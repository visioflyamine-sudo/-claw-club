use anchor_lang::prelude::*;

declare_id!("ClawClubNFTProgram11111111111111111111111");

#[program]
pub mod claw_club {
    use super::*;

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        Ok(())
    }

    pub fn mint_nft(ctx: Context<MintNFT>, _uri: String) -> Result<()> {
        let nft = &mut ctx.accounts.nft_account;
        nft.owner = ctx.accounts.payer.key();
        nft.mint_count += 1;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize {}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    
    #[account(init, payer = payer, space = 8 + 32 + 8)]
    pub nft_account: Account<'info, NFTAccount>,
    
    pub system_program: Program<'info, System>,
}

#[account]
pub struct NFTAccount {
    pub owner: Pubkey,
    pub mint_count: u64,
}
