use anchor_lang::prelude::*;
use anchor_spl::token::{self, Token, TokenAccount, Mint};
use anchor_spl::associated_token::AssociatedToken;
use std::mem::size_of;

declare_id!("ClawClubNFT11111111111111111111111111111111");

#[program]
pub mod claw_club {
    use super::*;

    /// Initialize the Claw Club NFT collection
    pub fn initialize_collection(
        ctx: Context<InitializeCollection>,
        collection_name: String,
        collection_symbol: String,
        max_supply: u64,
        royalty_basis_points: u16,
    ) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        collection.authority = ctx.accounts.authority.key();
        collection.name = collection_name;
        collection.symbol = collection_symbol;
        collection.max_supply = max_supply;
        collection.royalty_basis_points = royalty_basis_points;
        collection.current_supply = 0;
        collection.is_active = true;
        collection.created_at = Clock::get()?.unix_timestamp;

        emit!(CollectionCreated {
            collection: collection.key(),
            authority: ctx.accounts.authority.key(),
            max_supply,
            royalty_basis_points,
        });

        Ok(())
    }

    /// Enable whitelist (for WL phase)
    pub fn enable_whitelist(
        ctx: Context<EnableWhitelist>,
        whitelist_addresses: Vec<Pubkey>,
        duration_minutes: u32,
    ) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        require!(collection.authority == ctx.accounts.authority.key(), InvalidAuthority);

        let whitelist = &mut ctx.accounts.whitelist;
        whitelist.collection = collection.key();
        whitelist.addresses = whitelist_addresses.clone();
        whitelist.enabled = true;
        whitelist.enabled_at = Clock::get()?.unix_timestamp;
        whitelist.duration_minutes = duration_minutes;

        emit!(WhitelistEnabled {
            collection: collection.key(),
            count: whitelist_addresses.len() as u32,
            duration_minutes,
        });

        Ok(())
    }

    /// Disable whitelist (for public phase)
    pub fn disable_whitelist(
        ctx: Context<DisableWhitelist>,
    ) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        require!(collection.authority == ctx.accounts.authority.key(), InvalidAuthority);

        let whitelist = &mut ctx.accounts.whitelist;
        whitelist.enabled = false;

        emit!(WhitelistDisabled {
            collection: collection.key(),
        });

        Ok(())
    }

    /// Mint an NFT (FREE MINT)
    pub fn mint_nft(
        ctx: Context<MintNFT>,
        nft_id: u32,
        metadata_uri: String,
        rarity_score: u16,
    ) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        require!(collection.is_active, CollectionNotActive);
        require!(collection.current_supply < collection.max_supply, MaxSupplyReached);

        // Check if whitelist is active
        if ctx.accounts.whitelist.enabled {
            let now = Clock::get()?.unix_timestamp;
            let whitelist_end = ctx.accounts.whitelist.enabled_at + (ctx.accounts.whitelist.duration_minutes as i64 * 60);
            require!(now <= whitelist_end, WhitelistPeriodEnded);
            
            // Check if minter is whitelisted
            require!(
                ctx.accounts.whitelist.addresses.contains(&ctx.accounts.minter.key()),
                NotWhitelisted
            );
        }

        // Check if minter already has NFT (max 1 per wallet)
        let nft_count = ctx.accounts.nft_account.nft_count;
        require!(nft_count == 0, MaxNFTPerWallet);

        // Create NFT record
        let nft = &mut ctx.accounts.nft;
        nft.collection = collection.key();
        nft.nft_id = nft_id;
        nft.owner = ctx.accounts.minter.key();
        nft.metadata_uri = metadata_uri.clone();
        nft.rarity_score = rarity_score;
        nft.minted_at = Clock::get()?.unix_timestamp;
        nft.mint = ctx.accounts.mint.key();

        // Update NFT account
        let nft_account = &mut ctx.accounts.nft_account;
        nft_account.minted_count += 1;

        // Update collection supply
        collection.current_supply += 1;

        emit!(NFTMinted {
            collection: collection.key(),
            nft_id,
            owner: ctx.accounts.minter.key(),
            rarity_score,
            current_supply: collection.current_supply,
        });

        Ok(())
    }

    /// Transfer NFT to new owner
    pub fn transfer_nft(
        ctx: Context<TransferNFT>,
    ) -> Result<()> {
        let nft = &mut ctx.accounts.nft;
        require!(nft.owner == ctx.accounts.owner.key(), InvalidOwner);

        let old_owner = nft.owner;
        nft.owner = ctx.accounts.new_owner.key();

        emit!(NFTTransferred {
            nft_id: nft.nft_id,
            from: old_owner,
            to: ctx.accounts.new_owner.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }

    /// Withdraw treasury (Amine only)
    pub fn withdraw_treasury(
        ctx: Context<WithdrawTreasury>,
        amount: u64,
    ) -> Result<()> {
        let collection = &mut ctx.accounts.collection;
        require!(collection.authority == ctx.accounts.authority.key(), InvalidAuthority);

        // Transfer from treasury to authority
        **ctx.accounts.treasury.try_borrow_mut_lamports()? -= amount;
        **ctx.accounts.authority.try_borrow_mut_lamports()? += amount;

        emit!(TreasuryWithdrawn {
            amount,
            withdrawn_by: ctx.accounts.authority.key(),
            timestamp: Clock::get()?.unix_timestamp,
        });

        Ok(())
    }
}

// ==================== ACCOUNTS ====================

#[derive(Accounts)]
pub struct InitializeCollection<'info> {
    #[account(init, payer = authority, space = size_of::<Collection>() + 1000)]
    pub collection: Account<'info, Collection>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct EnableWhitelist<'info> {
    pub collection: Account<'info, Collection>,
    #[account(init, payer = authority, space = size_of::<Whitelist>() + 10000)]
    pub whitelist: Account<'info, Whitelist>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct DisableWhitelist<'info> {
    pub collection: Account<'info, Collection>,
    #[account(mut)]
    pub whitelist: Account<'info, Whitelist>,
    pub authority: Signer<'info>,
}

#[derive(Accounts)]
pub struct MintNFT<'info> {
    #[account(mut)]
    pub collection: Account<'info, Collection>,
    pub whitelist: Account<'info, Whitelist>,
    
    #[account(
        init,
        payer = minter,
        space = size_of::<NFT>() + 500,
        seeds = [b"nft", collection.key().as_ref(), minter.key().as_ref()],
        bump
    )]
    pub nft: Account<'info, NFT>,
    
    #[account(
        init_if_needed,
        payer = minter,
        space = size_of::<NFTAccount>(),
        seeds = [b"nft_account", minter.key().as_ref()],
        bump
    )]
    pub nft_account: Account<'info, NFTAccount>,
    
    pub mint: Account<'info, Mint>,
    pub token_account: Option<Account<'info, TokenAccount>>,
    
    #[account(mut)]
    pub minter: Signer<'info>,
    
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct TransferNFT<'info> {
    #[account(mut)]
    pub nft: Account<'info, NFT>,
    pub owner: Signer<'info>,
    /// CHECK: New owner address
    pub new_owner: UncheckedAccount<'info>,
}

#[derive(Accounts)]
pub struct WithdrawTreasury<'info> {
    pub collection: Account<'info, Collection>,
    /// CHECK: Treasury account
    #[account(mut)]
    pub treasury: UncheckedAccount<'info>,
    #[account(mut)]
    pub authority: Signer<'info>,
}

// ==================== ACCOUNT STRUCTS ====================

#[account]
pub struct Collection {
    pub authority: Pubkey,           // 32
    pub name: String,                // 4 + name length
    pub symbol: String,              // 4 + symbol length
    pub max_supply: u64,             // 8
    pub royalty_basis_points: u16,   // 2 (7% = 700 basis points)
    pub current_supply: u64,         // 8
    pub is_active: bool,             // 1
    pub created_at: i64,             // 8
}

#[account]
pub struct Whitelist {
    pub collection: Pubkey,          // 32
    pub addresses: Vec<Pubkey>,      // 4 + addresses
    pub enabled: bool,               // 1
    pub enabled_at: i64,             // 8
    pub duration_minutes: u32,       // 4
}

#[account]
pub struct NFT {
    pub collection: Pubkey,          // 32
    pub nft_id: u32,                 // 4
    pub owner: Pubkey,               // 32
    pub metadata_uri: String,        // 4 + uri length
    pub rarity_score: u16,           // 2
    pub minted_at: i64,              // 8
    pub mint: Pubkey,                // 32
}

#[account]
pub struct NFTAccount {
    pub owner: Pubkey,               // 32
    pub minted_count: u8,            // 1
    pub nft_count: u8,               // 1
}

// ==================== EVENTS ====================

#[event]
pub struct CollectionCreated {
    pub collection: Pubkey,
    pub authority: Pubkey,
    pub max_supply: u64,
    pub royalty_basis_points: u16,
}

#[event]
pub struct WhitelistEnabled {
    pub collection: Pubkey,
    pub count: u32,
    pub duration_minutes: u32,
}

#[event]
pub struct WhitelistDisabled {
    pub collection: Pubkey,
}

#[event]
pub struct NFTMinted {
    pub collection: Pubkey,
    pub nft_id: u32,
    pub owner: Pubkey,
    pub rarity_score: u16,
    pub current_supply: u64,
}

#[event]
pub struct NFTTransferred {
    pub nft_id: u32,
    pub from: Pubkey,
    pub to: Pubkey,
    pub timestamp: i64,
}

#[event]
pub struct TreasuryWithdrawn {
    pub amount: u64,
    pub withdrawn_by: Pubkey,
    pub timestamp: i64,
}

// ==================== ERRORS ====================

#[error_code]
pub enum ClawClubError {
    #[msg("Invalid authority")]
    InvalidAuthority,
    
    #[msg("Collection not active")]
    CollectionNotActive,
    
    #[msg("Max supply reached")]
    MaxSupplyReached,
    
    #[msg("Whitelist period ended")]
    WhitelistPeriodEnded,
    
    #[msg("Not whitelisted")]
    NotWhitelisted,
    
    #[msg("Max 1 NFT per wallet")]
    MaxNFTPerWallet,
    
    #[msg("Invalid owner")]
    InvalidOwner,
}
