use analysis::ReVisitor;
use parser::ReBlock;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod analysis;
mod parser;

#[proc_macro]
pub fn rerust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ReBlock);
    println!("AST: {:#?}", input);
    let mut visitor = ReVisitor::new();
    visitor.visit_reblock(&input);
    let graph = visitor.reactive_graph(&input);
    TokenStream::new()
}
