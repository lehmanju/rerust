use syn::{Expr, Type};

use enum_dispatch::enum_dispatch;

use crate::parser::{ReClosure, ReIdent};

mod visitor;

#[derive(Debug, Clone)]
pub struct ReData<'ast> {
    pub id: u32,
    pub family: Family,
    pub ty: &'ast Type,
    pub pin: bool,
}

#[enum_dispatch]
pub trait NodeData {
    fn family(&self) -> Family;
    fn ty(&self) -> &Type;
    fn pin(&self) -> bool;
    fn pin_mut(&mut self) -> &mut bool;
    fn id(&self) -> u32;
}

#[enum_dispatch(NodeData, Generate)]
#[derive(Debug)]
pub enum ReNode<'ast> {
    Var(VarNode<'ast>),
    Evt(EvtNode<'ast>),
    Name(NameNode<'ast>),
    Fold(FoldNode<'ast>),
    Map(MapNode<'ast>),
    Choice(ChoiceNode<'ast>),
    Filter(FilterNode<'ast>),
    Changed(ChangedNode<'ast>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Event,
    Variable,
}

#[derive(Debug)]
pub struct ChangedNode<'ast> {
    pub data: ReData<'ast>,
}

#[derive(Debug)]
pub struct ChoiceNode<'ast> {
    pub data: ReData<'ast>,
}

#[derive(Debug)]
pub struct EvtNode<'ast> {
    pub data: ReData<'ast>,
}

#[derive(Debug)]
pub struct VarNode<'ast> {
    pub initial: &'ast Expr,
    pub data: ReData<'ast>,
}

#[derive(Clone, Debug)]
pub struct NameNode<'ast> {
    pub id: &'ast ReIdent,
    pub data: ReData<'ast>,
}

#[derive(Debug)]
pub struct FoldNode<'ast> {
    pub initial: &'ast Expr,
    pub update_expr: &'ast ReClosure,
    pub data: ReData<'ast>,
}

#[derive(Debug)]
pub struct MapNode<'ast> {
    pub update_expr: &'ast ReClosure,
    pub data: ReData<'ast>,
}

#[derive(Debug)]
pub struct FilterNode<'ast> {
    pub filter_expr: &'ast ReClosure,
    pub data: ReData<'ast>,
}

#[derive(Clone, Debug)]
pub struct ReEdge<'ast> {
    ty: &'ast Type,
}

impl NodeData for VarNode<'_> {
    fn family(&self) -> Family {
        self.data.family()
    }

    fn ty(&self) -> &Type {
        self.data.ty()
    }

    fn pin(&self) -> bool {
        self.data.pin()
    }

    fn pin_mut(&mut self) -> &mut bool {
        self.data.pin_mut()
    }

    fn id(&self) -> u32 {
        self.data.id()
    }
}

impl NodeData for EvtNode<'_> {
    fn family(&self) -> Family {
        self.data.family()
    }

    fn ty(&self) -> &Type {
        self.data.ty()
    }

    fn pin(&self) -> bool {
        self.data.pin()
    }

    fn pin_mut(&mut self) -> &mut bool {
        self.data.pin_mut()
    }

    fn id(&self) -> u32 {
        self.data.id()
    }
}

impl NodeData for ChoiceNode<'_> {
    fn family(&self) -> Family {
        self.data.family()
    }

    fn ty(&self) -> &Type {
        self.data.ty()
    }

    fn pin(&self) -> bool {
        self.data.pin()
    }

    fn pin_mut(&mut self) -> &mut bool {
        self.data.pin_mut()
    }

    fn id(&self) -> u32 {
        self.data.id()
    }
}

impl NodeData for NameNode<'_> {
    fn family(&self) -> Family {
        self.data.family()
    }

    fn ty(&self) -> &Type {
        self.data.ty()
    }

    fn pin(&self) -> bool {
        self.data.pin()
    }

    fn pin_mut(&mut self) -> &mut bool {
        self.data.pin_mut()
    }

    fn id(&self) -> u32 {
        self.data.id()
    }
}

impl NodeData for FoldNode<'_> {
    fn family(&self) -> Family {
        self.data.family()
    }

    fn ty(&self) -> &Type {
        self.data.ty()
    }

    fn pin(&self) -> bool {
        self.data.pin()
    }

    fn pin_mut(&mut self) -> &mut bool {
        self.data.pin_mut()
    }

    fn id(&self) -> u32 {
        self.data.id()
    }
}

impl NodeData for MapNode<'_> {
    fn family(&self) -> Family {
        self.data.family()
    }

    fn ty(&self) -> &Type {
        self.data.ty()
    }

    fn pin(&self) -> bool {
        self.data.pin()
    }

    fn pin_mut(&mut self) -> &mut bool {
        self.data.pin_mut()
    }

    fn id(&self) -> u32 {
        self.data.id()
    }
}

impl NodeData for FilterNode<'_> {
    fn family(&self) -> Family {
        self.data.family()
    }

    fn ty(&self) -> &Type {
        self.data.ty()
    }

    fn pin(&self) -> bool {
        self.data.pin()
    }

    fn pin_mut(&mut self) -> &mut bool {
        self.data.pin_mut()
    }

    fn id(&self) -> u32 {
        self.data.id()
    }
}

impl NodeData for ChangedNode<'_> {
    fn family(&self) -> Family {
        self.data.family()
    }

    fn ty(&self) -> &Type {
        self.data.ty()
    }

    fn pin(&self) -> bool {
        self.data.pin()
    }

    fn pin_mut(&mut self) -> &mut bool {
        self.data.pin_mut()
    }

    fn id(&self) -> u32 {
        self.data.id()
    }
}

impl NodeData for ReData<'_> {
    fn family(&self) -> Family {
        self.family
    }

    fn ty(&self) -> &Type {
        self.ty
    }

    fn pin(&self) -> bool {
        self.pin
    }

    fn pin_mut(&mut self) -> &mut bool {
        &mut self.pin
    }

    fn id(&self) -> u32 {
        self.id
    }
}
