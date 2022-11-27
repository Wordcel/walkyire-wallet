use anchor_lang::prelude::*;
use std::mem::size_of;

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS");

#[program]
pub mod walkyire {
    use super::*;

    pub fn initialize_wallet(ctx: Context<InitializeWallet>) -> Result<()> {
        let wallet_account = &mut ctx.accounts.wallet_account;
        wallet_account.wallet_pubkey = ctx.accounts.wallet_pubkey.key();
        wallet_account.bump = ctx.bumps["wallet_account"];
        Ok(())
    }

    pub fn initialize_proof(ctx: Context<InitializeProof>, status: bool) -> Result<()> {
        let proof_account = &mut ctx.accounts.proof;
        proof_account.status = status;
        proof_account.wallet_account = ctx.accounts.wallet_account.key();
        proof_account.bump = ctx.bumps["proof_account"];
        Ok(())
    }

    pub fn recover_wallet(ctx: Context<RecoverWallet>) -> Result<()> {
        // This verification is simplistic and thus centralizes the proof creation.
        // One way to decentralize is for devices/users to generate zk proof, further research here
        // would be helpful.
        require!(ctx.accounts.proof.status, ProofError::ProofFailure);
        let wallet_account = &mut ctx.accounts.wallet_account;
        wallet_account.wallet_pubkey = ctx.accounts.new_wallet_pubkey.key();
        Ok(())
    }
}

#[derive(Accounts)]
pub struct InitializeWallet<'info> {
    #[account(init,
        seeds = [b"walkyire_wallet"],
        bump,
        payer = protocol_authority,
        space= WalkyireWallet::LEN
    )]
    pub wallet_account: Account<'info, WalkyireWallet>,
    pub wallet_pubkey: Signer<'info>,
    #[account(mut, constraint = is_proto_authority(protocol_authority.key()) @AdminError::UnAuthorizedAccess)]
    pub protocol_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(status:bool)]
pub struct InitializeProof<'info> {
    // Note that this creates an account that is rent exempt. The optimal way to do this would be
    // to just create the account for one epoch and let the runtime gc it.
    #[account(init,
        seeds = [b"walkyire_wallet_recovery", wallet_account.key().as_ref()],
        bump,
        payer = protocol_authority,
        space= Proof::LEN
    )]
    pub proof: Account<'info, Proof>,
    pub wallet_account: Account<'info, WalkyireWallet>,
    #[account(mut, constraint = is_proto_authority(protocol_authority.key()) @AdminError::UnAuthorizedAccess)]
    pub protocol_authority: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct RecoverWallet<'info> {
    #[account(
        seeds = [b"walkyire_wallet_recovery", wallet_account.key().as_ref()],
        bump,
        has_one = wallet_account
    )]
    pub proof: Account<'info, Proof>,
    #[account(mut)]
    pub wallet_account: Account<'info, WalkyireWallet>,
    pub system_program: Program<'info, System>,
    pub new_wallet_pubkey: Signer<'info>,
    #[account(mut, constraint = is_proto_authority(protocol_authority.key()) @AdminError::UnAuthorizedAccess)]
    pub protocol_authority: Signer<'info>,
}

#[account]
pub struct WalkyireWallet {
    pub bump: u8,
    pub wallet_pubkey: Pubkey,
}

impl WalkyireWallet {
    pub const LEN: usize = 8 + size_of::<Self>();
}

#[account]
pub struct Proof {
    pub bump: u8,
    pub wallet_account: Pubkey,
    pub status: bool,
}

impl Proof {
    pub const LEN: usize = 8 + size_of::<Self>();
}

#[error_code]
pub enum AdminError {
    UnAuthorizedAccess,
}

#[error_code]
pub enum ProofError {
    ProofFailure,
}

fn is_proto_authority(key: Pubkey) -> bool {
    // replace with actual admin public key
    // All the otp proofs should be signed by the admin
    let protocol_authority = Pubkey::new_from_array([0u8; 32]);
    key == protocol_authority
}
