#![allow(unused)]

mod ast {
    pub enum Stmt {
        Expr(Expr),
        Let(Name, Expr),
    }

    pub struct Name {
        value: String,
    }

    pub enum Expr {
        IntLit(i64),
        Add(Box<Expr>, Box<Expr>),
        Sub(Box<Expr>, Box<Expr>),
    }
}

mod visit {
    use super::ast::*;

    pub trait Visitor<T> {
        fn visit_name(&mut self, n: &Name) -> T;
        fn visit_stmt(&mut self, s: &Stmt) -> T;
        fn visit_expr(&mut self, e: &Expr) -> T;
    }
}

use ast::*;
use visit::*;

struct Interpreter;
impl Visitor<i64> for Interpreter {
    fn visit_name(&mut self, n: &Name) -> i64 {
        panic!()
    }

    fn visit_stmt(&mut self, s: &Stmt) -> i64 {
        match s {
            Stmt::Expr(e) => self.visit_expr(e),
            Stmt::Let(..) => unimplemented!(),
        }
    }

    fn visit_expr(&mut self, e: &Expr) -> i64 {
        match e {
            Expr::IntLit(n) => *n,
            Expr::Add(lhs, rhs) => self.visit_expr(lhs) + self.visit_expr(rhs),
            Expr::Sub(lhs, rhs) => self.visit_expr(lhs) - self.visit_expr(rhs),
        }
    }
}

fn main() {}
