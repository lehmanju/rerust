use parser::ReBlock;
use proc_macro::TokenStream;
use syn::parse_macro_input;
use analysis::ReVisitor;

mod analysis;
mod parser;

#[proc_macro]
pub fn rerust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ReBlock);
    println!("AST: {:#?}", input);
    let visitor = ReVisitor::new();
    visitor.visit_reblock(&input);
    let graph = visitor.reactive_graph(&input);
    TokenStream::new()
}
