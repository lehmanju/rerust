// build graph
// check incoming and outgoing types
// prefix for anonymous reactives

use crate::parser::{ReBlock, ReClosure, ReExpr, ReIdent, ReLocal};
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
}

pub enum ReNode<'ast> {
    Source(SourceNode<'ast>),
    Name(NameNode<'ast>),
    Reactive(ReactiveNode<'ast>),
    Group(Type),
    Choice(Type),
}

pub struct SourceNode<'ast> {
    pub initial: Option<&'ast Expr>,
    pub ty: &'ast Type,
}

#[derive(Clone)]
pub struct NameNode<'ast> {
    pub id: &'ast ReIdent,
    pub ty: Type,
}

pub struct ReactiveNode<'ast> {
    pub initial: Option<&'ast Expr>,
    pub ty: &'ast syn::Type,
    pub update_expr: &'ast ReClosure,
}

#[derive(Clone)]
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
        let (last_idx, last_ty) = self.visit_reexpr(&i.init)?;
        let name = &i.ident;
        let name_str = name.ident.to_string();
        if self
            .name_nodes
            .iter()
            .find(|(n, idx)| n.id.ident.to_string() == name_str)
            .is_some()
        {
            return Err(Error::new(
                i.ident.ident.span(),
                "identifier already occupied",
            ));
        }
        let name_node = NameNode {
            id: name,
            ty: last_ty.clone(),
        };
        let node_idx = self.graph.add_node(ReNode::Name(name_node.clone()));
        self.name_nodes.push((name_node, node_idx));
        let edge = ReEdge { ty: last_ty };
        self.graph.add_edge(last_idx, node_idx, edge);
        Ok(())
    }
    fn visit_reexpr(&mut self, i: &'ast ReExpr) -> Result<(NodeIndex, Type)> {
        match i {
            ReExpr::Var(varexpr) => {
                let node = ReNode::Source(SourceNode {
                    initial: Some(&varexpr.expr),
                    ty: &varexpr.ty,
                });
                let idx = self.graph.add_node(node);
                Ok((idx, varexpr.ty.clone()))
            }
            ReExpr::Evt(evtexpr) => {
                let node = ReNode::Source(SourceNode {
                    initial: None,
                    ty: &evtexpr.ty,
                });
                let idx = self.graph.add_node(node);
                Ok((idx, evtexpr.ty.clone()))
            }
            ReExpr::Ident(identexpr) => {
                let idx = self
                    .name_nodes
                    .iter()
                    .find(|(n, idx)| n.id.ident.to_string() == identexpr.ident.to_string());
                match idx {
                    Some((name, idx)) => Ok((idx.clone(), name.ty.clone())),
                    None => Err(Error::new(identexpr.ident.span(), "unknown reactive")),
                }
            }
            ReExpr::Group(groupexpr) => {
                let mut incoming_types: Punctuated<Type, Comma> = Punctuated::new();
                let mut incoming_nodes = Vec::new();
                for pair in groupexpr.exprs.pairs() {
                    let (expr, _) = pair.into_tuple();
                    let (idx, ty) = self.visit_reexpr(expr)?;
                    incoming_nodes.push((idx, ty.clone()));
                    incoming_types.push(ty.clone());
                }
                let ty = Type::Tuple(TypeTuple {
                    paren_token: Paren {
                        span: Span::call_site(),
                    },
                    elems: incoming_types,
                });
                let node = ReNode::Group(ty.clone());
                let idx = self.graph.add_node(node);
                for (nidx, nty) in incoming_nodes {
                    let edge = ReEdge { ty: nty };
                    self.graph.add_edge(nidx, idx, edge);
                }
                Ok((idx, ty))
            }
            ReExpr::Fold(foldexpr) => {
                let (incoming_idx, incoming_ty) = self.visit_reexpr(&foldexpr.left_expr)?;
                let ty = self.visit_reclosure(&foldexpr.closure)?;
                let node = ReNode::Reactive(ReactiveNode {
                    initial: Some(&foldexpr.init_expr),
                    ty,
                    update_expr: &foldexpr.closure,
                });
                let idx = self.graph.add_node(node);
                let edge = ReEdge { ty: ty.clone() };
                self.graph.add_edge(incoming_idx, idx, edge);
                Ok((idx, ty.clone()))
            }
            ReExpr::Choice(choiceexpr) => {
                let (a_idx, a_ty) = self.visit_reexpr(&choiceexpr.left_expr)?;
                let (b_idx, b_ty) = self.visit_reexpr(&choiceexpr.right_expr)?;
                let span = choiceexpr.oror.spans[0];
                if a_ty != b_ty {
                    return Err(Error::new(span, "mismatching types"));
                }
                let node = ReNode::Choice(a_ty.clone());
                let idx = self.graph.add_node(node);
                let edge = ReEdge { ty: a_ty.clone() };
                self.graph.add_edge(a_idx, idx, edge.clone());
                self.graph.add_edge(b_idx, idx, edge);
                Ok((idx, a_ty))
            }
            ReExpr::Map(mapexpr) => {
                let (incoming_idx, incoming_ty) = self.visit_reexpr(&mapexpr.left_expr)?;
                let ty = self.visit_reclosure(&mapexpr.closure)?;
                let node = ReNode::Reactive(ReactiveNode {
                    initial: None,
                    ty,
                    update_expr: &mapexpr.closure,
                });
                let idx = self.graph.add_node(node);
                let edge = ReEdge { ty: ty.clone() };
                self.graph.add_edge(incoming_idx, idx, edge);
                Ok((idx, ty.clone()))
            }
            ReExpr::Filter(filterexpr) => {
                todo!("replace with map/fold")
            }
        }
    }
    fn visit_reclosure(&mut self, i: &'ast ReClosure) -> Result<&'ast Type> {
        Ok(&i.return_type)
    }
    pub fn reactive_graph(&self, i: &'ast ReBlock) -> Graph<ReNode<'ast>, ReEdge> {
        todo!()
    }
    pub fn new() -> Self {
        todo!()
    }
}
