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

    /// # Panics
    ///
    /// Panics if the verifier is already invalidated before verifying.
    pub fn verify_programs(&mut self, programs: Vec<Rc<Program>>) {
        if self.verifier.invalidated {
            panic!("Verifier already invalidated.");
        }
        self.verifier.reset_state();

        to_do_here();
    }

    /// # Panics
    ///
    /// Panics if the verifier is already invalidated before verifying.
    pub fn verify_expression(&mut self, exp: &Rc<Expression>, context_type: Option<Symbol>, mode: VerifyMode) -> Option<Symbol> {
        if self.verifier.invalidated {
            panic!("Verifier already invalidated.");
        }
        self.verifier.reset_state();

        ()
    }

    pub fn enter_scope(&mut self, scope: &Symbol) {
        self.verifier.enter_scope(scope);
    }

    pub fn exit_scope(&mut self) {
        self.verifier.exit_scope();
    }
}

pub(crate) struct VerifierVerifier {
    pub host: Rc<SymbolHost>,
    pub ast_to_symbol: Rc<AstToSymbol>,
    pub deferred_directives: Vec<Rc<Directive>>,
    invalidated: bool,
    pub deferred_counter: usize,
    pub scope: Symbol,
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

    pub fn verify_expression(&mut self, exp: &Rc<Expression>, context_type: Option<Symbol>, followed_by_type_arguments: bool, mode: VerifyMode) -> Result<Option<Symbol>, DeferVerificationError> {
        let pre_result = self.ast_to_symbol.get(exp);
        if let Some(pre_result) = pre_result {
            return Ok(Some(pre_result));
        }
        let mut result: Option<Symbol>;
        match exp.as_ref() {
            Expression::QualifiedIdentifier(id) => {
                result = id.verify_as_exp(self, exp, followed_by_type_arguments)?;
            },
        }

        if result.is_none() {
            return Ok(result);
        }
        let result = result.unwrap();

        match mode {
            VerifyMode::Read => {
                if result.write_only(&self.host) {
                    self.add_verify_error(&exp.location(), DiagnosticKind::ReferenceIsWriteOnly, diagnostic_arguments![]);
                }
            },
            VerifyMode::Write => {
                if result.read_only(&self.host) {
                    self.add_verify_error(&exp.location(), DiagnosticKind::ReferenceIsReadOnly, diagnostic_arguments![]);
                }
            },
            VerifyMode::Delete => {
                if !result.deletable(&self.host) {
                    self.add_verify_error(&exp.location(), DiagnosticKind::ReferenceIsNotDeletable, diagnostic_arguments![]);
                }
            },
        }

        Ok(Some(result))
    }

    pub fn limit_expression_type(&mut self, exp: &Rc<Expression>, limit_type: &Symbol) -> Result<Option<Symbol>, DeferVerificationError> {
        let v = self.verify_expression(exp, Some(limit_type.clone()), false, VerifyMode::Read)?;
        if v.is_none() {
            return Ok(None);
        }
        let v = v.unwrap();
        let got_type = v.static_type(&self.host);
        let v = TypeConversions(&self.host).implicit_conversion(&v, limit_type, false);
        if v.is_none() {
            self.add_verify_error(&exp.location(), DiagnosticKind::IncompatibleTypes, diagnostic_arguments![Symbol(got_type), Symbol(limit_type.clone())]);
            self.ast_to_symbol.delete(exp);
            return Ok(None);
        }
        let v = v.unwrap();
        self.ast_to_symbol.set(exp, Some(v));
        Ok(Some(v))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VerifyMode {
    Read,
    Write,
    Delete,
}