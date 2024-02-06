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
    verifier: VerifierVerifier,
}

impl Verifier {
    pub fn new(host: &Rc<SymbolHost>) -> Self {
        Self {
            verifier: VerifierVerifier {
                host: host.clone(),
                ast_to_symbol: AstToSymbol::new(),
                deferred_directives: vec![],
                invalidated: false,
                deferred_counter: 0,
                scope: host.root_scope(),
            },
        }
    }

    pub fn ast_to_symbol(&self) -> &Rc<AstToSymbol> {
        &self.verifier.ast_to_symbol
    }

    /// Indicates whether a syntax or verify error occurred while
    /// verifying.
    pub fn invalidated(&self) -> bool {
        self.verifier.invalidated
    }

    pub fn verify_programs(&mut self, programs: Vec<Rc<Program>>) {
        if self.verifier.invalidated {
            return;
        }
        self.verifier.reset_state();

        to_do_here();
    }

    pub fn verify_expression(&mut self, exp: &Rc<Expression>, context_type: Option<Symbol>) {
        if self.verifier.invalidated {
            return;
        }
        self.verifier.reset_state();

        to_do_here();
    }

    pub fn enter_scope(&mut self, scope: &Symbol) {
        self.verifier.enter_scope(scope);
    }

    pub fn exit_scope(&mut self) {
        self.verifier.exit_scope();
    }
}

pub(crate) struct VerifierVerifier {
    host: Rc<SymbolHost>,
    ast_to_symbol: Rc<AstToSymbol>,
    deferred_directives: Vec<Rc<Directive>>,
    invalidated: bool,
    deferred_counter: usize,
    scope: Symbol,
}

impl VerifierVerifier {
    pub fn ast_to_symbol(&self) -> &Rc<AstToSymbol> {
        &self.ast_to_symbol
    }

    /// Indicates whether a syntax or verify error occurred while
    /// verifying.
    pub fn invalidated(&self) -> bool {
        self.invalidated
    }

    fn reset_state(&mut self) {
        self.deferred_counter = 0;
        self.deferred_directives.clear();
    }

    pub fn add_syntax_error(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        location.compilation_unit().add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
        self.invalidated = true;
    }

    pub fn add_verify_error(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        location.compilation_unit().add_diagnostic(Diagnostic::new_verify_error(location, kind, arguments));
        self.invalidated = true;
    }

    pub fn add_warning(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        location.compilation_unit().add_diagnostic(Diagnostic::new_warning(location, kind, arguments));
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

    pub fn verify_expression(&mut self, exp: &Rc<Expression>, context_type: Option<Symbol>, diagnostics: bool, type_argumented: bool) -> Result<Option<Symbol>, DeferVerificationError> {
        match exp.as_ref() {
            Expression::QualifiedIdentifier(id) => {
                id.verify_as_exp(self, exp, diagnostics, type_argumented)
            },
        }
    }
}