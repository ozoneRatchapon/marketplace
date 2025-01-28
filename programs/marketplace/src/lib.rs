use anchor_lang::prelude::*;

declare_id!("HKA9qcoQnB2DyV5N1xpBujcVcqTzDvMhakMmVg96Sry9");

#[program]
pub mod marketplace {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        msg!("Greetings from: {:?}", ctx.program_id);
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {
}
