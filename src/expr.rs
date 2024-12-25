use crate::tokens::TokenType;

#[derive(Debug)]
pub enum Expr {
    Binary(BinaryExpr),
    Grouping(GroupingExpr),
    Literal(LiteralExpr),
    Unary(UnaryExpr),
}

#[derive(Debug)]
pub struct BinaryExpr {
    pub left: Box<Expr>,
    pub operator: TokenType,
    pub right: Box<Expr>,
}

#[derive(Debug)]
pub struct GroupingExpr {
    pub expression: Box<Expr>,
}

#[derive(Debug)]
pub struct LiteralExpr {
    pub value: String,
}

#[derive(Debug)]
pub struct UnaryExpr {
    pub operator: TokenType,
    pub right: Box<Expr>,
}

pub trait ExprVisitor<T> {
    fn visit_binary(&mut self, expr: &BinaryExpr) -> T;
    fn visit_grouping(&mut self, expr: &GroupingExpr) -> T;
    fn visit_literal(&mut self, expr: &LiteralExpr) -> T;
    fn visit_unary(&mut self, expr: &UnaryExpr) -> T;
}

impl Expr {
    pub fn accept<T>(&self, visitor: &mut dyn ExprVisitor<T>) -> T {
       match self {
            Expr::Binary(binary) => visitor.visit_binary(binary),
            Expr::Grouping(grouping) => visitor.visit_grouping(grouping),
            Expr::Literal(literal) => visitor.visit_literal(literal),
            Expr::Unary(unary) => visitor.visit_unary(unary),
       }
   }
}