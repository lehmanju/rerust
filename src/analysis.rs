// build graph
// check incoming and outgoing types
// prefix for anonymous reactives

use crate::parser::{ReBlock, ReClosure, ReExpr, ReIdent, ReLocal};
use enum_dispatch::enum_dispatch;
use petgraph::{graph::NodeIndex, Graph};
use proc_macro2::Span;
use syn::{
    punctuated::Punctuated,
    token::{Comma, Paren},
    Error, Expr, Result, Type, TypeTuple,
};

pub struct ReVisitor<'ast> {
    pub graph: Graph<ReNode<'ast>, ReEdge>,
    name_nodes: Vec<(NameNode<'ast>, NodeIndex)>,
    node_count: u32,
}
#[enum_dispatch(Generate)]
#[derive(Debug)]
pub enum ReNode<'ast> {
    Var(VarNode<'ast>),
    Evt(EvtNode<'ast>),
    Name(NameNode<'ast>),
    Fold(FoldNode<'ast>),
    Map(MapNode<'ast>),
    Choice(ChoiceNode),
    Filter(FilterNode<'ast>),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Family {
    Event,
    Variable,
}

#[derive(Debug)]
pub struct ChoiceNode {
    pub family: Family,
    pub ty: Type,
    pub id: u32,
}

#[derive(Debug)]
pub struct EvtNode<'ast> {
    pub family: Family,
    pub ty: &'ast Type,
    pub id: u32,
}

#[derive(Debug)]
pub struct VarNode<'ast> {
    pub family: Family,
    pub initial: &'ast Expr,
    pub ty: &'ast Type,
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct NameNode<'ast> {
    pub family: Family,
    pub id: &'ast ReIdent,
    pub ty: Type,
}

#[derive(Debug)]
pub struct FoldNode<'ast> {
    pub family: Family,
    pub initial: &'ast Expr,
    pub ty: &'ast Type,
    pub update_expr: &'ast ReClosure,
    pub id: u32,
}

#[derive(Debug)]
pub struct MapNode<'ast> {
    pub family: Family,
    pub ty: &'ast Type,
    pub update_expr: &'ast ReClosure,
    pub id: u32,
}

#[derive(Debug)]
pub struct FilterNode<'ast> {
    pub family: Family,
    pub ty: Type,
    pub filter_expr: &'ast ReClosure,
    pub id: u32,
}

#[derive(Clone, Debug)]
pub struct ReEdge {
    ty: Type,
}

impl<'ast> ReVisitor<'ast> {
    pub fn visit_reblock(&mut self, i: &'ast ReBlock) -> Result<()> {
        for local in &i.stmts {
            self.visit_relocal(local)?;
        }
        Ok(())
    }
    fn visit_relocal(&mut self, i: &'ast ReLocal) -> Result<()> {
        let (mut last_idxs, last_fam) = self.visit_reexpr(&i.init.expr)?;
        let name = &i.ident;
        let name_str = name.ident.to_string();
        if self.name_nodes.iter().any(|(n, _)| n.id.ident == name_str) {
            return Err(Error::new(
                i.ident.ident.span(),
                "identifier already occupied",
            ));
        }
        assert!(last_idxs.len() == 1); // let name = ( ... ); is forbidden
        let (last_idx, last_ty) = last_idxs.remove(0);
        let name_node = NameNode {
            id: name,
            ty: last_ty.clone(),
            family: last_fam,
        };
        let node_idx = self.graph.add_node(ReNode::Name(name_node.clone()));
        self.name_nodes.push((name_node, node_idx));
        let edge = ReEdge { ty: last_ty };
        self.graph.add_edge(last_idx, node_idx, edge);
        Ok(())
    }

    fn visit_reexpr(&mut self, i: &'ast ReExpr) -> Result<(Vec<(NodeIndex, Type)>, Family)> {
        match i {
            ReExpr::Group(groupexpr) => {
                let mut incoming_nodes = Vec::new();
                let mut family = Family::Variable;
                for expr in &groupexpr.exprs {
                    let (mut nodes, fam) = self.visit_reexpr(expr)?;
                    assert!(nodes.len() == 1);
                    incoming_nodes.push(nodes.remove(0));
                    if fam == Family::Event {
                        family = Family::Event;
                    }
                }
                Ok((incoming_nodes, family))
            }
            ReExpr::Var(varexpr) => {
                let node = ReNode::Var(VarNode {
                    initial: &varexpr.expr,
                    ty: &varexpr.ty,
                    family: Family::Variable,
                    id: self.next_idx(),
                });
                let idx = self.graph.add_node(node);
                Ok((vec![(idx, varexpr.ty.clone())], Family::Variable))
            }
            ReExpr::Evt(evtexpr) => {
                let node = ReNode::Evt(EvtNode {
                    ty: &evtexpr.ty,
                    id: self.next_idx(),
                    family: Family::Event,
                });
                let idx = self.graph.add_node(node);
                Ok((vec![(idx, evtexpr.ty.clone())], Family::Event))
            }
            ReExpr::Ident(identexpr) => {
                let idx = self
                    .name_nodes
                    .iter()
                    .find(|(n, _)| identexpr.ident == n.id.ident);
                match idx {
                    Some((name, idx)) => Ok((vec![(*idx, name.ty.clone())], name.family)),
                    None => Err(Error::new(identexpr.ident.span(), "unknown reactive")),
                }
            }
            ReExpr::Fold(foldexpr) => {
                let (incoming, fam) = self.visit_reexpr(&foldexpr.left_expr)?;
                let ty = self.visit_reclosure(&foldexpr.closure)?;
                let node = ReNode::Fold(FoldNode {
                    initial: &foldexpr.init_expr,
                    ty,
                    update_expr: &foldexpr.closure,
                    id: self.next_idx(),
                    family: fam,
                });
                let idx = self.graph.add_node(node);
                for (node, ty) in incoming {
                    let edge = ReEdge { ty };
                    self.graph.add_edge(node, idx, edge);
                }
                Ok((vec![(idx, ty.clone())], Family::Variable))
            }
            ReExpr::Choice(choiceexpr) => {
                let (mut a_nodes, a_fam) = self.visit_reexpr(&choiceexpr.left_expr)?;
                let (mut b_nodes, b_fam) = self.visit_reexpr(&choiceexpr.right_expr)?;
                assert!(a_nodes.len() == 1 && b_nodes.len() == 1); //only allow two inputs
                let span = choiceexpr.oror.spans[0];
                let (a_idx, a_ty) = a_nodes.remove(0);
                let (b_idx, b_ty) = b_nodes.remove(0);
                if a_ty != b_ty {
                    return Err(Error::new(span, "mismatching types"));
                }
                if a_fam != b_fam {
                    return Err(Error::new(span, "mismatching reactive family"));
                }
                let node = ReNode::Choice(ChoiceNode {
                    ty: a_ty.clone(),
                    id: self.next_idx(),
                    family: a_fam,
                });
                let idx = self.graph.add_node(node);
                let edge = ReEdge { ty: a_ty.clone() };
                self.graph.add_edge(a_idx, idx, edge.clone());
                self.graph.add_edge(b_idx, idx, edge);
                Ok((vec![(idx, a_ty)], a_fam))
            }
            ReExpr::Map(mapexpr) => {
                let (incoming, incoming_fam) = self.visit_reexpr(&mapexpr.left_expr)?;
                let ty = self.visit_reclosure(&mapexpr.closure)?;
                let node = ReNode::Map(MapNode {
                    ty,
                    update_expr: &mapexpr.closure,
                    id: self.next_idx(),
                    family: incoming_fam,
                });
                let idx = self.graph.add_node(node);
                for (node, ty) in incoming {
                    let edge = ReEdge { ty };
                    self.graph.add_edge(node, idx, edge);
                }

                Ok((vec![(idx, ty.clone())], incoming_fam))
            }
            ReExpr::Filter(filterexpr) => {
                let (mut incoming, incoming_fam) = self.visit_reexpr(&filterexpr.left_expr)?;
                self.visit_reclosure(&filterexpr.closure)?;
                assert!(incoming.len() == 1); // only one reactive as input
                let (idx, ty) = incoming.remove(0);
                let node = ReNode::Filter(FilterNode {
                    ty: ty.clone(),
                    filter_expr: &filterexpr.closure,
                    id: self.next_idx(),
                    family: incoming_fam,
                });
                let idx_filter = self.graph.add_node(node);
                let edge = ReEdge { ty: ty.clone() };
                self.graph.add_edge(idx, idx_filter, edge);
                Ok((vec![(idx, ty)], incoming_fam))
            }
        }
    }
    fn next_idx(&mut self) -> u32 {
        let res = self.node_count;
        self.node_count += 1;
        res
    }
    fn visit_reclosure(&mut self, i: &'ast ReClosure) -> Result<&'ast Type> {
        Ok(&i.return_type)
    }
    pub fn reactive_graph(self) -> Graph<ReNode<'ast>, ReEdge> {
        self.graph
    }
    pub fn new() -> Self {
        Self {
            graph: Graph::new(),
            name_nodes: Vec::new(),
            node_count: 0u32,
        }
    }
}
