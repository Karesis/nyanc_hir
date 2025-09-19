// hir/src/lib.rs

use nyanc_core::Symbol;
use std::collections::HashMap;
use std::marker::PhantomData;

// --- 1. 定义核心 ID 定义 ---

/// 一个独一无二的定义ID，可以指向任何定义（函数、结构体、变量等）
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct DefId(pub u32);

/// 代表一个 HIR 节点在“仓库”中的唯一ID。
/// 它的结构和 AstId 完全一样，但类型不同，以保证类型安全。
#[derive(Debug, PartialEq, Eq, Hash)]
pub struct HirId<N> {
    pub raw: u32,
    _marker: PhantomData<fn() -> N>,
}
// 为 HirId 手动实现 Clone 和 Copy，以绕开 N: Copy 的约束
impl<N> Clone for HirId<N> { fn clone(&self) -> Self { *self } }
impl<N> Copy for HirId<N> {}


// --- 2. 定义 HIR “中央仓库” (Arena) ---
#[derive(Debug, Default)]
pub struct Hir {
    // 为每种 HIR 节点都创建一个“货架”
    pub items: Vec<Item>,
    pub stmts: Vec<Stmt>,
    pub exprs: Vec<Expr>,
    // 注意：hir::Type 不是一个“入库”的节点，它被直接包含
    pub params: Vec<Param>,
    pub blocks: Vec<Block>, // Block 也需要入库
}

// --- 3. 为仓库添加方便的“入库”方法 ---
impl Hir {
    pub fn new() -> Self { Self::default() }

    pub fn alloc_expr(&mut self, expr: Expr) -> HirId<Expr> {
        let index = self.exprs.len() as u32;
        self.exprs.push(expr);
        HirId { raw: index, _marker: PhantomData }
    }
    // ... 未来将为其他节点添加类似的 alloc 方法 ...
}

// --- 4. 声明并导出子模块 ---
pub mod def;
pub mod expr;
pub mod stmt;
pub mod ty;

// --- 5. 重新导出所有 HIR 节点定义 ---
pub use def::*;
pub use expr::*;
pub use stmt::*;
pub use ty::*;