pub mod constants;
pub mod error;
pub mod instructions;
pub mod state;

use anchor_lang::prelude::*;

pub use constants::*;
pub use instructions::*;
pub use state::*;

declare_id!("FG58R1a66Tk5aChAmGeFPHn3rffTFHnzBbnViAZHdckJ");

#[program]
pub mod swap {
    use super::*;

    pub fn make_offer(ctx: Context<MakeOffer>, token_a_offered_amount: u64) -> Result<()> {
        make_offer::send_offered_tokens_to_vault(&ctx, token_a_offered_amount)?;
        make_offer::save_offer()?;
        Ok(())
    }
}
