use anchor_lang::prelude::*;
use anchor_spl::token::Token;

declare_id!("7qsbu3sAHjpvFsqEimWss5EX5AShpwPgu6ibanjUENC8");

#[program]
pub mod margin_loan_cpi {
    use super::*;

    /// Initializes a new account.
    #[derive(Accounts)]
    pub struct InitializeAccount<'info> {
        /// The MarginFi program account.
        /// CHECK: no validation, for educational purpose only
        pub marginfi_program: AccountInfo<'info>,

        /// The MarginFi group account.
        /// CHECK: no validation, for educational purpose only
        pub marginfi_group: AccountInfo<'info>,

        /// The MarginFi account to be initialized.
        /// CHECK: no validation, for educational purpose only
        #[account(init)]
        pub marginfi_account: AccountInfo<'info>,

        /// The signer of the transaction.
        #[account(mut)]
        pub signer: Signer<'info>,

        /// System program.
        pub system_program: Program<'info, System>,
    }

    pub fn initialize_account(ctx: Context<InitializeAccount>) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.marginfi_program.to_account_info(),
            marginfi_cpi::cpi::accounts::InitMarginfiAccount {
                authority: ctx.accounts.signer.to_account_info(),
                marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
                marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
                system_program: ctx.accounts.system_program.to_account_info(),
            },
        );
        marginfi_cpi::cpi::init_marginfi_account(cpi_ctx)
    }

    /// A structure for Margin loop operations.
    pub struct MarginLoop<'info> {
        pub marginfi_program: AccountInfo<'info>,
        #[account(mut)]
        pub marginfi_account: AccountInfo<'info>,
        #[account(mut)]
        pub marginfi_group: AccountInfo<'info>,

        pub signer: Signer<'info>,
        pub margin_bank_authority: AccountInfo<'info>,

        /// User token account.
        /// CHECK: no validation, for educational purpose only
        #[account(mut)]
        pub user_token_account: AccountInfo<'info>,

        #[account(mut)]
        pub token_vault: AccountInfo<'info>,

        pub token_program: Program<'info, Token>,
    }

    /// Performs a margin loop operation.
    pub fn loop(ctx: Context<MarginLoop>, amount: u64) -> Result<()> {
        let cpi_ctx = CpiContext::new(
            ctx.accounts.marginfi_program.to_account_info(),
            marginfi_cpi::cpi::accounts::MarginDepositCollateral {
                marginfi_account: ctx.accounts.marginfi_account.to_account_info(),
                marginfi_group: ctx.accounts.marginfi_group.to_account_info(),
                user_token_account: ctx.accounts.user_token_account.to_account_info(),
                token_vault: ctx.accounts.token_vault.to_account_info(),
                margin_bank_authority: ctx.accounts.margin_bank_authority.to_account_info(),
                token_program: ctx.accounts.token_program.to_account_info(),
                amount,
            },
        );
        marginfi_cpi::cpi::deposit_and_borrow(cpi_ctx, amount)?;
        // Check if the repeated call is intentional
        marginfi_cpi::cpi::deposit_and_borrow(cpi_ctx, amount)?;
        Ok(())
    }
}

