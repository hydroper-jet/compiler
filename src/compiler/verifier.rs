use crate::ns::*;
use std::rc::Rc;

pub struct Verifier {
    host: Rc<SymbolHost>,
    ast_to_symbol: Rc<AstToSymbol>,
    deferred_expressions: Vec<Rc<Expression>>,
    deferred_directives: Vec<Rc<Directive>>,
}

impl Verifier {
    pub fn new(host: &Rc<SymbolHost>) -> Self {
        Verifier {
            host: host.clone(),
            ast_to_symbol: AstToSymbol::new(),
            deferred_expressions: vec![],
            deferred_directives: vec![],
        }
    }
}