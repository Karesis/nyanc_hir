// hir/src/lib.rs

use nyanc_core::{Span, Symbol}; // Symbol 是我们为字符串设计的唯一ID，暂时可以 type Symbol = String;

// --- 核心 ID 定义 ---

/// 一个独一无二的定义ID，可以指向任何定义（函数、结构体、变量等）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefId(pub u32);

/// 代表一个 HIR 节点的 ID，用于在 HIR 内部进行引用
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HirId {
    pub owner: DefId, // 这个 HIR 节点属于哪个顶层定义
    pub local_id: u32,
}

// 声明并导出子模块
pub mod def;
pub mod expr;
pub mod stmt;
pub mod ty;

pub use def::*;
pub use expr::*;
pub use stmt::*;
pub use ty::*;

/// 代表一个模块的完整 HIR。
/// 这是 Analyzer 生成的主要产物之一。
#[derive(Debug, Clone)]
pub struct Module {
    pub items: std::collections::HashMap<DefId, Item>,
}