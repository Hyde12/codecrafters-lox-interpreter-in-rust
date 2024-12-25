// Declare all your modules
pub mod scanner;
pub mod expr;
pub mod tokens;
pub mod ast_printer;

// Re-export useful items from modules if needed
pub use scanner::Scanner;
pub use expr::{Expr, BinaryExpr, LiteralExpr, GroupingExpr};
pub use tokens::TokenType;
pub use ast_printer::{Ast, AstPrinter};

// Define shared utility functions
use std::process::exit;

pub fn throw_error(error_code: String, exit_program: bool) {
    eprintln!("{error_code}");
    if exit_program {
        exit(65);
    }
}
