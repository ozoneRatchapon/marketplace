use anchor_lang::prelude::*;

use crate::errors::MarketplaceError;
use crate::state::{Listing, Marketplace};

// HW
// reward the maker and/or the taker for their participation in the marketplace
// use the reward token mint as a reward

#[derive(Accounts)]
pub struct Purchase<'info> {
    #[account(mut)]
    pub taker: Signer<'info>,
    #[account(mut)]
    pub maker: AccountInfo<'info>,
    pub maker_mint: InterfaceAccount<'info, Mint>,
    #[account(
        seeds = [b"marketplace",marketplace.name.as_str().as_bytes()],
        bump = marketplace.bump,
    )]
    pub marketplace: Account<'info, Marketplace>,
    #[account(
        init_if_needed,
        payer = taker,
        associated_token::mint = maker_mint,
        associated_token::authority = maker,
    )]
    pub taker_ata: InterfaceAccount<'info, TokenAccount>,
    #[account(
        mut,
        associated_token::mint = maker_mint,
        associated_token::authority = listing,
    )]
    pub vault: InterfaceAccount<'info, TokenAccount>,
    #[account(
    mut,
    close = maker,
seeds = [b"marketplace",key().as_ref(),maker_mint.key().as_ref()],
bump = listing.bump,
)]
    pub listing: Account<'info, Listing>,
    #[account(
    mut,
    seeds = [b"treasury",marketplace.key().as_ref()],
    bump = marketplace.rewards_mint_bump,
)]
    pub treasury: SystemAccount<'info>,
    #[account(
    seeds = [b"rewards",marketplace.key().as_ref()],
    bump = marketplace.rewards_mint_bump,
    mint::authority = marketplace,
    mint::decimals = 6,
)]
    pub rewards_mint: InterfaceAccount<'info, Mint>,

    pub system_program: Program<'info, System>,
    pub token_program: Interface<'info, TokenInterface>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

impl<'info> Purchase<'info> {
    // send sol
    pub fn pay(&mut self) -> Result<()> {
        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.maker.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        let marketplace_fee = self
            .listing
            .price
            .checked_mul(self.marketplace.fee as u64)
            .ok_or(MarketplaceError::ArithmeticOverflow)?
            .checked_div(1000_u64)
            .ok_or(MarketplaceError::ArithmeticOverflow)?;

        let amount = self.listing.price - marketplace_fee;

        transfer(cpi_ctx, amount)?;

        let cpi_program = self.system_program.to_account_info();

        let cpi_accounts = Transfer {
            from: self.taker.to_account_info(),
            to: self.treasury.to_account_info(),
        };

        let cpi_ctx = CpiContext::new(cpi_program, cpi_accounts);

        transfer(cpi_ctx, marketplace_fee)?;

        Ok(())
    }
    // transfer the nft
    pub fn transfer_nft(&mut self) -> Result<()> {
        let programs = self.token_program.to_account_info();

        let accounts = TransferChecked {
            from: self.vault.to_account_info(),
            mint: self.maker_mint.to_account_info(),
            to: self.taker_ata.to_account_info(),
            authority: self.listing.to_account_info(),
        };

        let seeds = &[
            &b"marketplace"[..],
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let cpi_ctx = CpiContext::new_with_signer(programs, accounts, signer_seeds);

        transfer_checked(cpi_ctx, self.maker_mint.decimals)?;
        Ok(())
    }
    // close the account
    pub fn close_vault_account(&mut self) -> Result<()> {
        let seeds = &[
            &b"marketplace"[..],
            &self.marketplace.key().to_bytes()[..],
            &self.maker_mint.key().to_bytes()[..],
            &[self.listing.bump],
        ];
        let signer_seeds = &[&seeds[..]];

        let accounts = CloseAccount {
            account: self.vault.to_account_info(),
            destination: self.treasury.to_account_info(),
            authority: self.maker.to_account_info(),
        };
        Ok(())
    }
}
