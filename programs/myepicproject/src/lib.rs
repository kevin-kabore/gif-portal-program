use anchor_lang::prelude::*; // Kinda like an import statement. We want to import in a lot of the tools Anchor provides for us to make writing Solana programs easier.

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); // "program id":  has info for Solana on how to run our program

// #[<macroName>] is how we use macros in Rust. Similiar class inheritance.
#[program] // lets us call our program from the frontend via fetch request
pub mod myepicproject { // module is kinda like a class.
    use super::*;
    /* Step 3️⃣: In the program, grab the base account (state) from the Context */
    pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult { // a public function that takes a context "StartStuffOff" and returns a ProgramResult.
        
        let base_account = &mut ctx.accounts.base_account; // Get a mutable reference to the base account

        base_account.total_gifs = 0; // Initialize total_gifs to 0

        Ok(()) // like a return statement
    }

    /* Step 6️⃣: Use AddGif Context to grab state from the program */
    pub fn add_gif(ctx: Context<AddGif>) -> ProgramResult { 
        let base_account = &mut ctx.accounts.base_account; // get a mutable reference to the account
        base_account.total_gifs += 1; // increment total_gifs by 1
        Ok(()) 
    }
}

/* Step 2️⃣: Define Context of StartStuffOff using base account (program state) */
#[derive(Accounts)]
// Attach certain variables to the StartStuffOff context
pub struct StartStuffOff<'info> {
    /* 
        Specify to solana how to initialize the base account 
        - init: create a new account owned by our current program
        - payer = user: tells program who is paying for the account to be created (user calling the function in this case - use)
        - space = 9000: allocates 9000 bytes of soace for our account
    */
    #[account(init, payer = user, space = 9000)] // space is the max amount of bytes we can store in the account.
    pub base_account: Account<'info, BaseAccount>,
    
    /* and specify what to hold in the context*/
    #[account(mut)]
    // user: data passed into the program that proves to the program that the user calling this program actually owns their wallet account.
    pub user: Signer<'info>, 
    // a reference to the SystemProgram, which basically runs Solana. Does a lot, but main for ex: create accounts on Solana.
    pub system_program: Program<'info, System>,  // SystemProgram is a Solana program with programId = 1111111111111111111111111111111
}

/* Step 4️⃣: Define Context of AddGif */
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)] // has mutable reference to base_account
    pub base_account: Account<'info, BaseAccount>,
}
// How does Context work?: "Hey, when someone calls add_gif be sure to attach the AddGif context to it as well so the user can access the base_account and whatever else is attached to AddGif."


/* Step 1️⃣: Define the base account (program state) */
#[account]
// Tell the program what kind of account it can make and what to hold in it.
pub struct BaseAccount {
    // base account has a total_gifs field whitch is an integer
    pub total_gifs: u64,
}

