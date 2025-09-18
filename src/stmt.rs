use crate::{DefId, Expr, Type, HirId};
use nyanc_core::Span;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub hir_id: HirId,
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Let { var_def_id: DefId, initializer: Expr },
    Var { var_def_id: DefId, var_type: Type, initializer: Option<Expr> },
    Return { value: Option<Expr> },
    Block(Block),
    Expression(Expr),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<Stmt>,
}