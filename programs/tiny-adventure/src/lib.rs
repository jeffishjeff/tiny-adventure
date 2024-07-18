use min_max::*;
use anchor_lang::system_program;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;

declare_id!("DXxLHjAjv1xyV367pNLzFGSVdmUSD9UJRGKhjuvxm79D");

#[program]
pub mod tiny_adventure {
    use super::*;

    const MIN_LOOT: u64 = LAMPORTS_PER_SOL / 10; // 0.1 SOL

    pub fn initialize(_ctx: Context<Initialize>) -> Result<()> {
        msg!("A Journey Begins!");
        Ok(())
    }

    pub fn new_game(ctx: Context<NewGame>, loot: u64) -> Result<()> {
        let player_account = &mut ctx.accounts.player_account;
        let has_loot = **ctx.accounts.loot_account.to_account_info().try_borrow_mut_lamports()? > 0;
        
        let cpi_context = CpiContext::new(
            ctx.accounts.system_program.to_account_info(),
            system_program::Transfer {
                from: ctx.accounts.payer.to_account_info().clone(),
                to: ctx.accounts.loot_account.to_account_info().clone(),
            },
        );
        system_program::transfer(cpi_context, max!(loot, MIN_LOOT))?;

        player_account.position = 0;
        msg!("New game started!");
        print_game(player_account.position, has_loot);
        Ok(())
    }

    pub fn travel(ctx: Context<Travel>, to_right: bool, password: String) -> Result<()> {
        let player_account = &mut ctx.accounts.player_account;
        let loot = **ctx.accounts.loot_account.to_account_info().try_borrow_mut_lamports()?;
        
        if to_right {
            if player_account.position < 4 {
                player_account.position += 1;
            }
        } else {
            if player_account.position > 0 {
                player_account.position -= 1;
            }
        }

        if player_account.position == 3 && loot > 0 {
            if password == "gib" {
                msg!("Loot received!");
                **ctx.accounts.loot_account.to_account_info().try_borrow_mut_lamports()? -= loot;
                **ctx.accounts.player.to_account_info().try_borrow_mut_lamports()? += loot;
            }
        }

        print_game(player_account.position, loot > 0);
        Ok(())
    }
}

fn print_game(player_position: u8, has_loot: bool) {
    if has_loot {
        match player_position {
            0 => { msg!("o......💎."); }
            1 => { msg!("..o....💎."); }
            2 => { msg!("....o..💎."); }
            3 => { msg!("......o💎."); }
            4 => { msg!(".......💎o"); }
            _ => ()
        }
    } else {
        match player_position {
            0 => { msg!("o........"); }
            1 => { msg!("..o......"); }
            2 => { msg!("....o...."); }
            3 => { msg!("......o.."); }
            4 => { msg!("........o"); }
            _ => ()
        }
    }
}

#[derive(Accounts)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(init_if_needed, seeds = [b"player"], bump, payer = signer, space = 8 + 1)]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(init_if_needed, seeds = [b"loot"], bump, payer = signer, space = 8)]
    pub loot_account: Account<'info, LootAccount>,
}

#[derive(Accounts)]
pub struct NewGame<'info> {
    #[account(mut)]
    pub payer: Signer<'info>,
    pub system_program: Program<'info, System>,
    #[account(mut)]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(mut, seeds = [b"loot"], bump)]
    pub loot_account: Account<'info, LootAccount>,
}

#[derive(Accounts)]
pub struct Travel<'info> {
    #[account(mut)]
    pub player: Signer<'info>,
    #[account(mut)]
    pub player_account: Account<'info, PlayerAccount>,
    #[account(mut, seeds = [b"loot"], bump)]
    pub loot_account: Account<'info, LootAccount>,
}

#[account]
pub struct LootAccount {}

#[account]
pub struct PlayerAccount {
    position: u8,
}