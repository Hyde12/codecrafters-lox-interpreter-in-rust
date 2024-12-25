use std::env;
use std::fs;
use std::fs::metadata;
use std::io::{self, Write};
use std::process::exit;
use lib::{throw_error, Scanner, Ast, TokenType, Expr, BinaryExpr, LiteralExpr, GroupingExpr, AstPrinter};

fn main() {
    let args: Vec<String> = env::args().collect();
    let binding = "".to_string();

    let command = args.get(1).unwrap_or(&binding);
    let argument_1 = args.get(2).unwrap_or(&binding);

    match command.as_str() {
        "tokenize" => {
            if args.len() < 3 { throw_error(format!("Usage: {} tokenize <filename>", args[0]), true); }

            writeln!(io::stderr(), "Logs from your program will appear here!").unwrap();

            let file_contents = fs::read_to_string(argument_1).unwrap_or_else(|_| {
                throw_error(format!("Failed to read file {}", argument_1), false);
                String::new()
            });

            // Use Scanner from the lib module
            let mut to_scan = Scanner::new(file_contents);
            to_scan.scan_tokens();

            for token in to_scan.tokens {
                let (token_type, text, literal, _line) = token;
                println!("{token_type:?} {text} {literal}");
            }

            if to_scan.errors {
                exit(65);
            }
        }
        "generate_ast" => {
            if args.len() < 3 {
                throw_error(format!("Usage: {} generate_ast <output directory>", args[0]), true);
            }

            let ast = Ast::new(argument_1, "Expr", vec![
                "Binary   : Box<Expr> left, TokenType operator, Box<Expr> right".to_string(),
                "Grouping : Box<Expr> expression".to_string(),
                "Literal  : String value".to_string(),
                "Unary    : TokenType operator, Box<Expr> right".to_string()
            ]);

            if metadata(&ast.output_dir).unwrap().is_dir() {
                ast.write_output();
            }
        }
        _ => {
            if args.len() < 3 {
                throw_error(format!("Usage: {} <command> <arguments>", args[0]), true);
            }

            throw_error(format!("Unknown command: {}", command), true);
        }
    }
}
