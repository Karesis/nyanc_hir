use crate::{DefId, Block, Type, HirId};
use nyanc_core::Symbol;

#[derive(Debug, Clone)]
pub enum Item {
    Function(Function),
    Struct(Struct),
}

#[derive(Debug, Clone)]
pub struct Function {
    pub def_id: DefId,
    pub name: Symbol,
    pub params: Vec<HirId<Param>>,
    pub return_type: HirId<Type>,
    pub body: HirId<Block>,
}

#[derive(Debug, Clone)]
pub struct Struct {
    pub def_id: DefId,
    pub name: Symbol,
    pub fields: Vec<HirId<Param>>,
}

#[derive(Debug, Clone)]
pub struct Param {
    pub def_id: DefId, // 每个参数也是一个定义
    pub name: Symbol,
    pub param_type: HirId<Type>,
}