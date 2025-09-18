// hir/src/expr.rs

use crate::{DefId, HirId, Type};
use nyanc_core::{Span, Symbol};

/// HIR 中的表达式节点。
/// 它包含了表达式的种类、位置，以及一个用于类型检查的类型槽。
#[derive(Debug, Clone)]
pub struct Expr {
    pub hir_id: HirId,
    pub kind: ExprKind,
    pub span: Span,
}

/// HIR 表达式的种类枚举。
#[derive(Debug, Clone)]
pub enum ExprKind {
    // 新增
    Assignment(AssignExpr),
    MemberAccess(MemberAccessExpr),
    StructInit(StructInitExpr),
    
    // 原有
    Binary(BinaryExpr),
    Unary(UnaryExpr),
    Call(CallExpr),
    Variable(VariableExpr),
    Literal(Literal),
    
    // 注意：Grouping 表达式已被“降级”，不存在于 HIR 中
}

// --- 具体表达式的 HIR 结构 ---

#[derive(Debug, Clone)]
pub struct AssignExpr {
    pub target: Box<Expr>,
    pub value: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct MemberAccessExpr {
    pub object: Box<Expr>,
    pub field: Symbol, // 字段名被转换为 Symbol
}

#[derive(Debug, Clone)]
pub struct StructInitExpr {
    pub struct_def_id: DefId, // 结构体名称被解析为唯一的 DefId
    pub fields: Vec<(Symbol, Expr)>, // 字段名是 Symbol，值是 HIR 表达式
}

#[derive(Debug, Clone, Copy)]
pub enum BinaryOp { Add, Sub, Mul, Div, Eq, NotEq, Gt, GtEq, Lt, LtEq }

#[derive(Debug, Clone)]
pub struct BinaryExpr {
    pub op: BinaryOp,
    pub left: Box<Expr>,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone, Copy)]
pub enum UnaryOp { Neg, Not } // 我们可以做得比 AST 更具体

#[derive(Debug, Clone)]
pub struct UnaryExpr {
    pub op: UnaryOp,
    pub right: Box<Expr>,
}

#[derive(Debug, Clone)]
pub struct CallExpr {
    pub callee: Box<Expr>,
    pub args: Vec<Expr>,
}

#[derive(Debug, Clone)]
pub struct VariableExpr {
    pub def_id: DefId, // 不再是 Token，而是直接指向定义的 ID！
}

#[derive(Debug, Clone)]
pub enum Literal {
    Int(i64),
    Float(f64),
    Bool(bool),
    String(Symbol),
    Unit,
}