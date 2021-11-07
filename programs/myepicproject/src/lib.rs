use anchor_lang::prelude::*; // Kinda like an import statement. We want to import in a lot of the tools Anchor provides for us to make writing Solana programs easier.

declare_id!("Fg6PaFpoGXkYsidMpWTK6W2BeZ7FEfcYkg476zPFsLnS"); // "program id":  has info for Solana on how to run our program

// #[<macroName>] is how we use macros in Rust. Similiar class inheritance.
#[program] // lets us call our program from the frontend via fetch request
pub mod myepicproject { // module is kinda like a class.
    use super::*;

    pub fn start_stuff_off() -> ProgramResult { // a public function that takes a context "StartStuffOff" and returns a ProgramResult.
        Ok(()) // like a return statement
    }
}
