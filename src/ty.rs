use crate::DefId;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Type {
    Int,
    Bool,
    Unit,
    Struct(DefId), // 直接指向结构体的定义 ID
    Pointer(Box<Type>),
    Error, // 一个特殊的类型，用于在出错后继续进行类型检查
}