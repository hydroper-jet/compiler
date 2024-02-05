use crate::ns::*;
use std::rc::Rc;

/// Jet verifier.
///
/// `Verifier` is both a type checker, a symbol solver and strictness verifier,
/// resulting into diagnostics to respective `CompilationUnit`s.
///
/// # Verifying
/// 
/// A set of programs can be verified by invoking `verify_programs()`:
/// 
/// ```ignore
/// verifier.verify_programs(program_list);
/// ```
/// 
/// A single expression can be verified by invoking `verify_expression()`:
/// 
/// ```ignore
/// verifier.verify_expression(&expression, Some(context_type));
/// ```
/// 
/// # Scopes
/// 
/// Enter and exit scopes by invoking `enter_scope()` and `exit_scope()` respectively.
/// Such methods may alter the `parent_scope()` field of the scope to use the enclosing
/// scope as the parent.
///
/// ```
/// verifier.enter_scope(&scope);
/// verifier.exit_scope();
/// ```
///
/// # Symbol solving
///
/// As programs are verified, the `ast_to_symbol()` field is filled
/// with mappings from AST to symbols.
/// 
/// ```ignore
/// // expression: Rc<Expression>
/// let symbol: Option<Symbol> = verifier.ast_to_symbol().get(&expression);
/// ```
pub struct Verifier {
    host: Rc<SymbolHost>,
    ast_to_symbol: Rc<AstToSymbol>,
    deferred_expressions: Vec<Rc<Expression>>,
    deferred_directives: Vec<Rc<Directive>>,
    invalidated: bool,
    deferred_counter: usize,
    scope: Symbol,
}

impl Verifier {
    pub fn new(host: &Rc<SymbolHost>) -> Self {
        Verifier {
            host: host.clone(),
            ast_to_symbol: AstToSymbol::new(),
            deferred_expressions: vec![],
            deferred_directives: vec![],
            invalidated: false,
            deferred_counter: 0,
            scope: host.root_scope(),
        }
    }

    pub fn ast_to_symbol(&self) -> &Rc<AstToSymbol> {
        &self.ast_to_symbol
    }

    pub fn invalidated(&self) -> bool {
        self.invalidated
    }

    pub fn verify_programs(&mut self, programs: Vec<Rc<Program>>) {
        if self.invalidated {
            return;
        }
        self.reset_state();

        to_do_here();
    }

    pub fn verify_expression(&mut self, expression: &Rc<Expression>) {
        if self.invalidated {
            return;
        }
        self.reset_state();

        to_do_here();
    }

    fn reset_state(&mut self) {
        self.deferred_counter = 0;
        self.deferred_directives.clear();
        self.deferred_expressions.clear();
    }

    pub fn enter_scope(&mut self, scope: &Symbol) {
        let k = self.scope.clone();
        self.scope = scope.clone();
        if scope.parent_scope().is_none() {
            scope.set_parent_scope(Some(&k));
        }
    }

    pub fn exit_scope(&mut self) {
        self.scope = self.scope.parent_scope().unwrap();
    }
}