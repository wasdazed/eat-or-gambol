use anchor_lang::prelude::*;
use anchor_spl::token::{Mint, Token, TokenAccount, Transfer};

declare_id!("FdugZTVqxDJZfuxi3HNpzDmSSuAQ2YZkwAJyPmcFCmM8");

#[program]
pub mod eog {
    use anchor_spl::token;

    use super::*;

    pub fn initialize(ctx: Context<Initialize>) -> Result<()> {
        let state = &mut ctx.accounts.state;
        state.mint = ctx.accounts.mint.key();
        state.house_vault = ctx.accounts.house_vault.key();
        state.kitchen_vault = ctx.accounts.kitchen_vault.key();
        state.dev_vault = ctx.accounts.dev_vault.key();
        state.bump = ctx.bumps.state;
        Ok(())
    }

    pub fn eat(ctx: Context<Eat>, amount: u64) -> Result<()> {
        sacrifice(ctx, amount, 500, "Eat".to_string())
    }

    pub fn sex(ctx: Context<Eat>, amount: u64) -> Result<()> {
        sacrifice(ctx, amount, 1000, "Sex".to_string())
    }

    pub fn lambo(ctx: Context<Eat>, amount: u64) -> Result<()> {
        sacrifice(ctx, amount, 1000, "Lambo".to_string())
    }

    pub fn rolex(ctx: Context<Eat>, amount: u64) -> Result<()> {
        sacrifice(ctx, amount, 2000, "Rolex".to_string())
    }

    pub fn gambol(ctx: Context<Gambol>, amount: u64) -> Result<()> {
        require!(amount > 0, Error::ZeroAmount);

        let dev_fee = amount * 300 / 10_000; // 3%

        // Player → house vault
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.player_token.to_account_info(),
                    to: ctx.accounts.house_vault.to_account_info(),
                    authority: ctx.accounts.player.to_account_info(),
                },
            ),
            amount,
        )?;

        // Dev takes 3%
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.house_vault.to_account_info(),
                    to: ctx.accounts.dev_vault.to_account_info(),
                    authority: ctx.accounts.state.to_account_info(),
                },
                &[&[b"state", &[ctx.accounts.state.bump]]],
            ),
            dev_fee,
        )?;

        // Fake randomness for localnet (50/50)
        let clock = Clock::get()?;
        let won = (clock.unix_timestamp.unsigned_abs() % 2) == 0;
        let payout = if won { amount * 2 - dev_fee } else { 0 };

        if won {
            token::transfer(
                CpiContext::new_with_signer(
                    ctx.accounts.token_program.to_account_info(),
                    Transfer {
                        from: ctx.accounts.house_vault.to_account_info(),
                        to: ctx.accounts.player_token.to_account_info(),
                        authority: ctx.accounts.state.to_account_info(),
                    },
                    &[&[b"state", &[ctx.accounts.state.bump]]],
                ),
                payout,
            )?;
        }

        emit!(GambolResult {
            player: ctx.accounts.player.key(),
            amount,
            won,
            payout,
        });

        Ok(())
    }

    pub fn sacrifice(ctx: Context<Eat>, amount: u64, dev_bps: u64, action: String) -> Result<()> {
        require!(amount > 0, Error::ZeroAmount);

        let dev_take = amount * dev_bps / 10_000;

        // Player → kitchen vault
        token::transfer(
            CpiContext::new(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.player_token.to_account_info(),
                    to: ctx.accounts.kitchen_vault.to_account_info(),
                    authority: ctx.accounts.player.to_account_info(),
                },
            ),
            amount,
        )?;

        // Dev takes a cut
        token::transfer(
            CpiContext::new_with_signer(
                ctx.accounts.token_program.to_account_info(),
                Transfer {
                    from: ctx.accounts.kitchen_vault.to_account_info(),
                    to: ctx.accounts.dev_vault.to_account_info(),
                    authority: ctx.accounts.state.to_account_info(),
                },
                &[&[b"state", &[ctx.accounts.state.bump]]],
            ),
            dev_take,
        )?;

        emit!(Sacrificed {
            player: ctx.accounts.player.key(),
            action: action.to_string(),
            amount,
            dev_take,
        });

        Ok(())
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(
        init,
        payer = authority,
        space = 8 + 32 * 4 + 1,
        seeds = [b"state"],
        bump
    )]
    pub state: Account<'info, GlobalState>,

    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = state,
        seeds = [b"house"],
        bump
    )]
    pub house_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = state,
        seeds = [b"kitchen"],
        bump
    )]
    pub kitchen_vault: Account<'info, TokenAccount>,

    #[account(
        init,
        payer = authority,
        token::mint = mint,
        token::authority = state,
        seeds = [b"dev"],
        bump
    )]
    pub dev_vault: Account<'info, TokenAccount>,

    pub mint: Account<'info, Mint>,
    #[account(mut)]
    pub authority: Signer<'info>,
    pub token_program: Program<'info, Token>,
    pub system_program: Program<'info, System>,
    pub rent: Sysvar<'info, Rent>,
}

#[derive(Accounts)]
pub struct Eat<'info> {
    #[account(seeds = [b"state"], bump = state.bump)]
    pub state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [b"kitchen"],
        bump,
        token::mint = state.mint,
        token::authority = state
    )]
    pub kitchen_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"dev"],
        bump,
        token::mint = state.mint,
        token::authority = state
    )]
    pub dev_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        // player's token account (ATA) must be for the same mint
        token::mint = state.mint,
        // authority is the signer `player` — no token::authority constraint required
    )]
    pub player_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[derive(Accounts)]
pub struct Gambol<'info> {
    #[account(seeds = [b"state"], bump = state.bump)]
    pub state: Account<'info, GlobalState>,

    #[account(
        mut,
        seeds = [b"house"],
        bump,
        token::mint = state.mint,
        token::authority = state
    )]
    pub house_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        seeds = [b"dev"],
        bump,
        token::mint = state.mint,
        token::authority = state
    )]
    pub dev_vault: Account<'info, TokenAccount>,

    #[account(
        mut,
        token::mint = state.mint,
    )]
    pub player_token: Account<'info, TokenAccount>,

    #[account(mut)]
    pub player: Signer<'info>,

    pub token_program: Program<'info, Token>,
}

#[account]
pub struct GlobalState {
    pub mint: Pubkey,
    pub house_vault: Pubkey,
    pub kitchen_vault: Pubkey,
    pub dev_vault: Pubkey,
    pub bump: u8,
}

#[event]
pub struct Sacrificed {
    pub player: Pubkey,
    pub action: String,
    pub amount: u64,
    pub dev_take: u64,
}

#[event]
pub struct GambolResult {
    pub player: Pubkey,
    pub amount: u64,
    pub won: bool,
    pub payout: u64,
}

#[error_code]
pub enum Error {
    #[msg("Amount must be greater than zero")]
    ZeroAmount,
}
