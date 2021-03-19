use analysis::visitor::ReVisitor;
use codegen::generate;
use parser::ReBlock;
use proc_macro::TokenStream;
use syn::parse_macro_input;

mod analysis;
mod codegen;
mod parser;

#[proc_macro]
pub fn rerust(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ReBlock);
    let mut visitor = ReVisitor::new();
    let result = visitor.visit_reblock(&input);
    if result.is_err() {
        return result.unwrap_err().to_compile_error().into();
    }
    let graph = visitor.reactive_graph();
    generate(&graph).into()
}
