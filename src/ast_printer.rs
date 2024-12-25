use std::io::Write;
use std::fs::File;
use crate::expr::ExprVisitor;
use crate::Expr;

// expression     → literal
//                | unary
//                | binary
//                | grouping ;

// literal        → NUMBER | STRING | "true" | "false" | "nil" ;
// grouping       → "(" expression ")" ;
// unary          → ( "-" | "!" ) expression ;
// binary         → expression operator expression ;
// operator       → "==" | "!=" | "<" | "<=" | ">" | ">="
//                | "+"  | "-"  | "*" | "/" ;

pub struct Ast {
    pub output_dir: String,
    pub base_name: String,
    pub types: Vec<String>,
}

impl Ast {
    pub fn new(output_dir: &String, base_name: &str, types: Vec<String>) -> Self {
        Self {
            output_dir: output_dir.to_string(),
            base_name: base_name.to_string(),
            types: types,
        }
    }

    pub fn write_output(&self) {
        let file_path = format!("{}/{}.rs", self.output_dir, self.base_name);
        let mut file = File::create(&file_path).expect("Failed to create file");

        file.write_all(b"use crate::tokens::TokenType;\n\n").unwrap();
        file.write_all(b"#[derive(Debug)]\n").unwrap();
        file.write_all(format!("pub enum {} {{\n", self.base_name).as_bytes()).unwrap();

        for type_line in &self.types {
            let type_name = type_line.split(':').next().unwrap().trim();
            file.write_all(format!("    {}({}{}),\n", type_name, type_name, "Expr").as_bytes())
                .unwrap();
        }

        file.write_all(b"}\n\n").unwrap();

        for type_line in &self.types {
            let parts: Vec<&str> = type_line.split(':').collect();
            let type_name = parts[0].trim();
            let fields = parts[1]
                .split(',')
                .map(|s| {
                    let pair: Vec<&str> = s.trim().split(' ').collect();
                    format!("pub {}: {},", pair[1], pair[0])
                })
                .collect::<Vec<String>>()
                .join("\n    ");
            
            file.write_all(b"#[derive(Debug)]\n").unwrap();
            file.write_all(format!("pub struct {}Expr {{\n    {}\n}}\n\n", type_name, fields).as_bytes())
                .unwrap();
        }

        // Write the visitor functions for each type
        file.write_all(format!("pub trait {}Visitor<T> {{\n", self.base_name).as_bytes()).unwrap();
        for type_line in &self.types {
            let type_name = type_line.split(':').next().unwrap().trim();
            file.write_all(format!("    fn visit_{}(&mut self, expr: &{}{}) -> T;\n", type_name.to_lowercase(), type_name, self.base_name).as_bytes()).unwrap();
        }

        file.write_all(b"}\n\n").unwrap();

        // Accept functions for visitor
        file.write_all(format!("impl {} {{\n", self.base_name).as_bytes()).unwrap();
        file.write_all(format!("    pub fn accept<T>(&self, visitor: &mut dyn {}Visitor<T>) -> T {{\n", self.base_name).as_bytes()).unwrap();
        file.write_all(b"       match self {\n").unwrap();

        for type_line in &self.types {
            let type_name = type_line.split(':').next().unwrap().trim();
            file.write_all(format!("            {}::{}({}) => visitor.visit_{}({}),\n", self.base_name, type_name, type_name.to_lowercase(), type_name.to_lowercase(), type_name.to_lowercase()).as_bytes()).unwrap();
        }

        file.write_all(b"       }\n").unwrap();
        file.write_all(b"   }\n").unwrap();
        file.write_all(b"}").unwrap();
    }
}

pub struct AstPrinter;

impl ExprVisitor<String> for AstPrinter {
    fn visit_binary(&mut self, expr: &crate::BinaryExpr) -> String {
        format!("({:?} {} {})", expr.operator, expr.left.accept(self), expr.right.accept(self))
    }

    fn visit_grouping(&mut self, expr: &crate::expr::GroupingExpr) -> String {
        format!("(group {:?})", expr.expression.accept(self))
    }

    fn visit_literal(&mut self, expr: &crate::LiteralExpr) -> String {
        expr.value.clone()
    }

    fn visit_unary(&mut self, expr: &crate::expr::UnaryExpr) -> String {
        format!("({:?} {:?})", expr.operator, expr.right.accept(self))
    }
}

impl AstPrinter {
    pub fn print(&mut self, expr: &Expr) -> String {
        expr.accept(self)
    }
}