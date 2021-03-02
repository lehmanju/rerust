// build graph
// check incoming and outgoing types
// prefix for anonymous reactives

use crate::parser::{ReBlock, ReClosure, ReExpr, ReLocal};
use petgraph::{graph::NodeIndex, Graph};
use syn::{Error, Result, Type};

use super::{
    ChangedNode, ChoiceNode, EvtNode, Family, FilterNode, FoldNode, MapNode, NameNode, NodeData,
    ReData, ReEdge, ReNode, VarNode,
};

pub struct ReVisitor<'ast> {
    pub graph: Graph<ReNode<'ast>, ReEdge<'ast>>,
    name_nodes: Vec<(&'ast NameNode<'ast>, NodeIndex)>,
    node_count: u32,
}

// #[derive(Debug)]
// pub struct ReNode<'ast> {
//     pub id: u32,
//     pub family: Family,
//     pub ty: &'ast Type,
//     pub pin: bool,
//     pub data: ReData<'ast>,
// }

impl<'ast> ReVisitor<'ast> {
    pub fn visit_reblock(&mut self, i: &'ast ReBlock) -> Result<()> {
        for local in &i.stmts {
            self.visit_relocal(local)?;
        }
        Ok(())
    }
    fn visit_relocal(&mut self, i: &'ast ReLocal) -> Result<()> {
        let (mut last_idxs, last_fam) = self.visit_reexpr(&i.init)?;
        let name = &i.ident;
        let name_str = name.ident.to_string();
        if self.name_nodes.iter().any(|(n, _)| n.id.ident == name_str) {
            return Err(Error::new(
                i.ident.ident.span(),
                "identifier already occupied",
            ));
        }
        let (last_idx, last_ty) = last_idxs.remove(0);
        let mut last_node = self.graph.node_weight_mut(last_idx).unwrap();
        let mut pin = false;
        if i.pin_token.is_some() {
            if last_idxs.len() != 1 {
                return Err(Error::new(
                    i.pin_token.unwrap().span,
                    "cannot pin group of reactives",
                ));
            }
            pin = true;
            *last_node.pin_mut() = true;
        }
        let name_node = NameNode {
            id: name,
            data: ReData {
                id: self.next_idx(),
                family: last_fam,
                ty: last_ty,
                pin,
            },
        };
        let new_node = ReNode::Name(name_node);
        let node_idx = self.graph.add_node(new_node);
        self.name_nodes.push((&name_node, node_idx));
        let edge = ReEdge { ty: last_ty };
        self.graph.add_edge(last_idx, node_idx, edge);
        Ok(())
    }

    fn visit_reexpr(&mut self, i: &'ast ReExpr) -> Result<(Vec<(NodeIndex, &'ast Type)>, Family)> {
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
                let data = ReData {
                    pin: true,
                    ty: &varexpr.ty,
                    family: Family::Variable,
                    id: self.next_idx(),
                };
                let node = ReNode::Var(VarNode {
                    initial: &varexpr.expr,
                    data,
                });
                let idx = self.graph.add_node(node);
                Ok((vec![(idx, &varexpr.ty)], Family::Variable))
            }
            ReExpr::Evt(evtexpr) => {
                let node = ReNode::Evt(EvtNode {
                    data: ReData {
                        pin: false,
                        ty: &evtexpr.ty,
                        id: self.next_idx(),
                        family: Family::Event,
                    },
                });
                let idx = self.graph.add_node(node);
                Ok((vec![(idx, &evtexpr.ty)], Family::Event))
            }
            ReExpr::Ident(identexpr) => {
                let idx = self
                    .name_nodes
                    .iter()
                    .find(|(n, _)| identexpr.ident == n.id.ident);
                match idx {
                    Some((name, idx)) => Ok((vec![(*idx, name.ty())], name.family())),
                    None => Err(Error::new(identexpr.ident.span(), "unknown reactive")),
                }
            }
            ReExpr::Fold(foldexpr) => {
                let (incoming, fam) = self.visit_reexpr(&foldexpr.left_expr)?;
                let ty = self.visit_reclosure(&foldexpr.closure)?;
                if fam != Family::Event {
                    return Err(Error::new(
                        foldexpr.fold_token.span,
                        "incoming node must be event",
                    ));
                }
                let node = ReNode::Fold(FoldNode {
                    initial: &foldexpr.init_expr,
                    update_expr: &foldexpr.closure,
                    data: ReData {
                        ty,
                        id: self.next_idx(),
                        family: fam,
                        pin: true,
                    },
                });
                let idx = self.graph.add_node(node);
                for (node, ty) in incoming {
                    let edge = ReEdge { ty };
                    self.graph.add_edge(node, idx, edge);
                }
                Ok((vec![(idx, ty)], Family::Variable))
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
                if a_fam != Family::Event || b_fam != Family::Event {
                    return Err(Error::new(
                        choiceexpr.oror.spans[0],
                        "changed only valid on variables",
                    ));
                }
                let node = ReNode::Choice(ChoiceNode {
                    data: ReData {
                        pin: false,
                        ty: a_ty,
                        id: self.next_idx(),
                        family: a_fam,
                    },
                });
                let idx = self.graph.add_node(node);
                let edge = ReEdge { ty: a_ty };
                self.graph.add_edge(a_idx, idx, edge.clone());
                self.graph.add_edge(b_idx, idx, edge);
                Ok((vec![(idx, a_ty)], a_fam))
            }
            ReExpr::Map(mapexpr) => {
                let (incoming, incoming_fam) = self.visit_reexpr(&mapexpr.left_expr)?;
                let ty = self.visit_reclosure(&mapexpr.closure)?;
                let node = ReNode::Map(MapNode {
                    update_expr: &mapexpr.closure,
                    data: ReData {
                        pin: false,
                        ty,
                        id: self.next_idx(),
                        family: incoming_fam,
                    },
                });
                let idx = self.graph.add_node(node);
                for (node, ty) in incoming {
                    let edge = ReEdge { ty };
                    self.graph.add_edge(node, idx, edge);
                }

                Ok((vec![(idx, ty)], incoming_fam))
            }
            ReExpr::Filter(filterexpr) => {
                let (mut incoming, incoming_fam) = self.visit_reexpr(&filterexpr.left_expr)?;
                self.visit_reclosure(&filterexpr.closure)?;
                assert!(incoming.len() == 1); // only one reactive as input
                if incoming_fam != Family::Event {
                    return Err(Error::new(
                        filterexpr.filter_token.span,
                        "filter only valid on events",
                    ));
                }
                let (idx, ty) = incoming.remove(0);
                let node = ReNode::Filter(FilterNode {
                    filter_expr: &filterexpr.closure,
                    data: ReData {
                        pin: false,
                        ty,
                        id: self.next_idx(),
                        family: incoming_fam,
                    },
                });
                let idx_filter = self.graph.add_node(node);
                let edge = ReEdge { ty };
                self.graph.add_edge(idx, idx_filter, edge);
                Ok((vec![(idx, ty)], incoming_fam))
            }
            ReExpr::Changed(changedexpr) => {
                let (incoming, incoming_fam) = self.visit_reexpr(&changedexpr.left_expr)?;
                assert!(incoming.len() == 1);
                let (idx, ty) = incoming.remove(0);
                if incoming_fam == Family::Event {
                    return Err(Error::new(
                        changedexpr.changed_token.span,
                        "changed only valid on variables",
                    ));
                }
                let node = ReNode::Changed(ChangedNode {
                    data: ReData {
                        pin: true,
                        ty,
                        id: self.next_idx(),
                        family: Family::Event,
                    },
                });
                let idx_changed = self.graph.add_node(node);
                let edge = ReEdge { ty };
                self.graph.add_edge(idx, idx_changed, edge);
                Ok((vec![(idx, ty)], Family::Event))
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
    pub fn reactive_graph(self) -> Graph<ReNode<'ast>, ReEdge<'ast>> {
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
