use crate::{DefId, Expr, Type, HirId};
use nyanc_core::Span;

#[derive(Debug, Clone)]
pub struct Stmt {
    pub kind: StmtKind,
    pub span: Span,
}

#[derive(Debug, Clone)]
pub enum StmtKind {
    Let { 
        var_def_id: DefId, 
        initializer: HirId<Expr> 
    },

    Var { 
        var_def_id: DefId, 
        var_type: HirId<Type>, 
        initializer: Option<HirId<Expr>> 
    },

    Return { 
        value: Option<HirId<Expr>> 
    },

    Block(HirId<Block>),
    Expression(HirId<Expr>),
}

#[derive(Debug, Clone)]
pub struct Block {
    pub stmts: Vec<HirId<Stmt>>,
}