use syn::{Expr, Type};

use enum_dispatch::enum_dispatch;

use crate::parser::{ReClosure, ReIdent};

pub mod visitor;

#[derive(Debug, Clone)]
pub struct ReData {
    pub id: u32,
    pub family: Family,
    pub ty: Type,
    pub pin: bool,
}

#[enum_dispatch]
pub trait NodeData {
    fn outgoing_family(&self) -> Family;
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
    Evt(EvtNode),
    Name(NameNode<'ast>),
    Fold(FoldNode<'ast>),
    Map(MapNode<'ast>),
    Filter(FilterNode<'ast>),
    Changed(ChangedNode),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Event,
    Variable,
}

#[derive(Debug)]
pub struct ChangedNode {
    pub data: ReData,
}

#[derive(Debug)]
pub struct EvtNode {
    pub data: ReData,
}

#[derive(Debug)]
pub struct VarNode<'ast> {
    pub initial: &'ast Expr,
    pub data: ReData,
}

#[derive(Clone, Debug)]
pub struct NameNode<'ast> {
    pub id: &'ast ReIdent,
    pub data: ReData,
}

#[derive(Debug)]
pub struct FoldNode<'ast> {
    pub initial: &'ast Expr,
    pub update_expr: &'ast ReClosure,
    pub data: ReData,
}

#[derive(Debug)]
pub struct MapNode<'ast> {
    pub update_expr: &'ast ReClosure,
    pub data: ReData,
}

#[derive(Debug)]
pub struct FilterNode<'ast> {
    pub filter_expr: &'ast ReClosure,
    pub data: ReData,
}

#[derive(Clone, Debug)]
pub struct ReEdge {
    ty: Type,
}

impl NodeData for VarNode<'_> {
    fn outgoing_family(&self) -> Family {
        self.family()
    }
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

impl NodeData for EvtNode {
    fn outgoing_family(&self) -> Family {
        self.family()
    }
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
    fn outgoing_family(&self) -> Family {
        self.family()
    }

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
    fn outgoing_family(&self) -> Family {
        Family::Variable
    }

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
    fn outgoing_family(&self) -> Family {
        self.family()
    }

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
    fn outgoing_family(&self) -> Family {
        self.family()
    }

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

impl NodeData for ChangedNode {
    fn outgoing_family(&self) -> Family {
        Family::Event
    }

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

impl NodeData for ReData {
    fn outgoing_family(&self) -> Family {
        self.family()
    }
    fn family(&self) -> Family {
        self.family
    }

    fn ty(&self) -> &Type {
        &self.ty
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
