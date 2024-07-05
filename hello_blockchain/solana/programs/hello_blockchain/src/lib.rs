use anchor_lang::prelude::*;

declare_id!("6hH58heVcNjvqUKnVXrejeFSJBXxArxSANSJE3835ZAy");

const CONTENT_MAX_LEN: usize = 1024;

#[program]
pub mod hello_blockchain {
    use super::*;

    pub fn create_message(ctx: Context<CreateMessage>, content: String) -> Result<()> {
        msg!("Creating message from: {:?}", ctx.program_id);
        let message = &mut ctx.accounts.message;
        message.content = content;
        message.signer = ctx.accounts.signer.key();
        message.bump = ctx.bumps.message;
        Ok(())
    }

    pub fn update_message(ctx: Context<UpdateMessage>, content: String) -> Result<()>{
        msg!("Updating message from: {:?}", ctx.program_id);
        let message = &mut ctx.accounts.message;
        message.content = content;
        Ok(())
    }


}

#[derive(Accounts)]
pub struct CreateMessage<'info> {
    #[account(init, payer = signer, seeds = [signer.key().as_ref()], bump, space = 8 + CONTENT_MAX_LEN)]
    pub message: Account<'info, Message>,
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>
}

#[account]
pub struct Message{
    pub content: String,
    pub signer: Pubkey,
    pub bump: u8
}

#[derive(Accounts)]
pub struct UpdateMessage<'info>{
    #[account(
        mut,
        seeds = [signer.key().as_ref()],
        bump = message.bump,
        has_one = signer
    )]
    pub message: Account<'info, Message>,
    #[account(mut)]
    pub signer: Signer<'info>
}

// TODO: Write tests in /test/hello_blockchain.ts