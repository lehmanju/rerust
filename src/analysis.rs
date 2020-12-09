// build graph
// check incoming and outgoing types
// prefix for anonymous reactives

use syn::{Error, Expr, Result, visit::Visit};
use crate::parser::{ReLocal, ReBlock, ReIdent, ReExpr, VarExpr, EvtExpr, GroupExpr, FoldExpr, MapExpr, ChoiceExpr, FilterExpr, ReClosure};
use std::marker::PhantomData;
use petgraph::Graph;

pub struct ReVisitor<'ast> {
    reactive_ids: Vec<&'ast ReIdent>,
    pub graph: Graph<ReNode<'ast>, ReEdge>,
}

pub struct ReNode<'ast> {
    pub id: &'ast ReIdent,
    pub initial: Option<&'ast Expr>,
    pub ty: &'ast syn::Type,
    pub update_expr: &'ast ReClosure,
    pub source: bool
}

pub struct ReEdge {
    ty: syn::Type,
}


impl<'ast> ReVisitor<'ast> {
    pub fn visit_reblock(&mut self, i: &'ast ReBlock) -> Result<()>{
        for local in &i.stmts {
           self.visit_relocal(local)?;
        }
        Ok(())
    }
    fn visit_relocal(&mut self, i: &'ast ReLocal) -> Result<()>{
        let name = &i.ident;
        if self.reactive_ids.contains(&name) {
            return Err(Error::new(name.ident.span(), "identifier already occupied"))
        }
        self.reactive_ids.push(name);
        let node;
        match i.init {
            Some((_, init)) => {}
            None => {}
        }
        Ok(())
    }
    fn visit_reident(&mut self, i: &'ast ReIdent)-> Result<()> {
        
    }
    fn visit_reexpr(&mut self, i: &'ast ReExpr)-> Result<()> {
        
    }
    fn visit_reexpr_var(&mut self, i: &'ast VarExpr)-> Result<()> {
        
    }
    fn visit_reexpr_evt(&mut self, i: &'ast EvtExpr)-> Result<()>  {
        
    }
    fn visit_reexpr_group(&mut self, i: &'ast GroupExpr) -> Result<()> {
        
    }
    fn visit_reexpr_fold(&mut self, i: &'ast FoldExpr) -> Result<()> {

    }
    fn visit_reexpr_map(&mut self, i: &'ast MapExpr)-> Result<()>  {
        
    }
    fn visit_reexpr_choice(&mut self, i: &'ast ChoiceExpr)-> Result<()>  {

    }
    fn visit_reexpr_filter(&mut self, i: &'ast FilterExpr) -> Result<()> {
        
    }
    fn visit_reclosure(&mut self, i: &'ast ReClosure) -> Result<()> {

    }
    pub fn reactive_graph(&self, i: &'ast ReBlock) -> Graph<ReNode<'ast>, ReEdge> {
        todo!()
    }
    pub fn new() -> Self {
        todo!()
    }
}

impl<'ast> Visit<'ast> for ReVisitor<'ast> {

}