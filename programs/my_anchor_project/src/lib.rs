use std::vec;

use anchor_lang::prelude::*;

declare_id!("AgBHPG2xUS1iWmbvwdrjWxcNTqudqtwGoYGe1iPF7gDF");

// #[program]
// pub mod my_anchor_project {
//     use super::*;

//     pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
//         msg!("Greetings from: {:?}", ctx.program_id);
//         Ok(())
//     }
// }

// #[derive(Accounts)]
// pub struct Initialize {}

#[account]
pub struct ComplexCounter {
    pub count: u64,
    pub authority: Pubkey,
    pub last_updated: i64,
    pub number_of_iteration: u64,
}

#[derive(Accounts)]
pub struct Create<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 + 8 + 8, seeds = [b"complex_counter"], bump)]
    pub counter: Account<'info, ComplexCounter>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct Increment<'info> {
    #[account(mut, seeds = [b"complex_counter"], bump)]
    pub counter:Account<'info, ComplexCounter>,
    pub authority: Signer<'info>,
}

#[program]
pub mod complex_counter_and_user_profile {
    use super::*;

    pub fn create(ctx: Context<Create>, authority: Pubkey) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        counter.count = 0;
        counter.authority = authority;
        counter.last_updated = Clock::get()?.unix_timestamp;
        counter.number_of_iteration = 0;
        Ok(())
    }

    pub fn create_user(ctx: Context<CreateUser>, authority: Pubkey) -> Result<()>{
        let user=&mut ctx.accounts.user_profile;
        user.name = "vignesh".to_string();
        user.email = "v@gmail.com".to_string();
        user.programs = vec![authority];
        Ok(())
    }

    pub fn update_profile(ctx: Context<UpdateProfile>, authority: Pubkey) -> Result<()>{
        let user= &mut ctx.accounts.user_profile;
        user.name = "v".to_string();
        user.email = "t".to_string();
        user.programs = vec![authority];
        Ok(())
    }

    pub fn increment(ctx: Context<Increment>) -> Result<()> {
        let counter = &mut ctx.accounts.counter;
        require!(ctx.accounts.authority.key() == counter.authority, ErrorCode::Unauthorized);
        counter.count += 1;
        counter.last_updated =Clock::get()?.unix_timestamp;
        counter.number_of_iteration +=1;
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("You are not authorized to perform this action")]
    Unauthorized,
}

#[account]
pub struct UserProfile{
    pub name: String,
    pub  email: String,
    pub programs: Vec<Pubkey>,
}


#[derive(Accounts)]
pub struct CreateUser<'info> {
    #[account(init, payer = user, space = 8 + 8 + 32 + 8 + 8, seeds = [b"user_profile"], bump)]
    pub user_profile: Account<'info, UserProfile>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
pub struct UpdateProfile<'info> {
    #[account(mut, seeds = [b"user_profile"], bump)]
    pub user_profile:Account<'info, UserProfile>,
    pub authority: Signer<'info>,
}

