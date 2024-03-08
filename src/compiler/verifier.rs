use crate::ns::*;

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
                deferred_function_commons: vec![],
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

        todo_here();
    }

    /// Verifies an expression. Returns `None` if verification failed.
    ///
    /// # Panics
    ///
    /// Panics if the verifier is already invalidated before verifying.
    pub fn verify_expression(&mut self, exp: &Rc<Expression>, context: &ExpressionVerifyContext) -> Option<Symbol> {
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
    /// List of (phase, scope, directive).
    pub deferred_directives: Vec<(usize, Symbol, Rc<Directive>)>,
    /// List of (phase, scope, common).
    pub deferred_function_commons: Vec<(usize, Symbol, Rc<FunctionCommon>)>,
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
        self.deferred_function_commons.clear();
    }

    pub fn add_syntax_error(&mut self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        location.compilation_unit().add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
        self.invalidated = true;
    }

    pub fn add_verify_error(&mut self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
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

    pub fn verify_expression(&mut self, exp: &Rc<Expression>, context: &ExpressionVerifyContext) -> Result<Option<Symbol>, DeferVerificationError> {
        let pre_result = self.ast_to_symbol.get(exp);
        if let Some(pre_result) = pre_result {
            return Ok(Some(pre_result));
        }
        let result: Option<Symbol>;
        match exp.as_ref() {
            Expression::QualifiedIdentifier(id) => {
                result = id.verify_as_exp(self, &context)?;
            },
            Expression::Embed(emb) => {
                result = emb.verify(self, &context)?;
            },
            Expression::Paren(paren_exp) => {
                result = self.verify_expression(&paren_exp.expression, &context)?;
            },
            Expression::NullLiteral(nl) => {
                result = nl.verify(self, &context)?;
            },
            Expression::BooleanLiteral(bl) => {
                result = Some(self.host.factory().create_boolean_constant(bl.value, &self.host.boolean_type()));
            },
            Expression::NumericLiteral(nl) => {
                result = nl.verify(self, &context)?;
            },
            Expression::StringLiteral(sl) => {
                result = sl.verify(self, &context)?;
            },
            Expression::ThisLiteral(tl) => {
                result = tl.verify(self)?;
            },
            Expression::RegExpLiteral(rl) => {
                result = rl.verify(self)?;
            },
            Expression::Xml(xml) => {
                xml.element.verify(self)?;
                result = Some(self.host.factory().create_value(&self.host.xml_type()));
            },
            Expression::XmlMarkup(xml) => {
                result = Some(self.host.factory().create_value(&self.host.xml_type()));
            },
            Expression::XmlList(xml) => {
                for content in &xml.content {
                    content.verify(self)?;
                }
                result = Some(self.host.factory().create_value(&self.host.xml_list_type()));
            },
            Expression::ArrayLiteral(al) => {
                result = al.verify(self, context)?;
            },
            Expression::ObjectInitializer(oi) => {
                result = oi.verify(self, context)?;
            },
        }

        self.ast_to_symbol.set(exp, result.clone());

        if result.is_none() {
            return Ok(result);
        }
        let result = result.unwrap();

        match context.mode {
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

    pub fn verify_type_expression(&mut self, exp: &Rc<Expression>) -> Result<Option<Symbol>, DeferVerificationError> {
        let v = self.verify_expression(exp, &ExpressionVerifyContext { ..default() })?;
        if v.is_none() {
            return Ok(None);
        }
        let v = v.unwrap();
        let v = v.expect_type();
        if v.is_err() {
            self.add_verify_error(&exp.location(), DiagnosticKind::MustResolveToType, diagnostic_arguments![]);
            self.ast_to_symbol.set(exp, None);
            return Ok(None);
        }
        let v = v.unwrap();
        self.ast_to_symbol.set(exp, Some(v.clone()));
        Ok(Some(v))
    }

    pub fn limit_expression_type(&mut self, exp: &Rc<Expression>, limit_type: &Symbol) -> Result<Option<Symbol>, DeferVerificationError> {
        let v = self.verify_expression(exp, &ExpressionVerifyContext {
            context_type: Some(limit_type.clone()),
            ..default()
        })?;
        if v.is_none() {
            return Ok(None);
        }
        let v = v.unwrap();
        let got_type = v.static_type(&self.host);
        let v = TypeConversions(&self.host).implicit_conversion(&v, limit_type, false);
        if v.is_none() {
            self.add_verify_error(&exp.location(), DiagnosticKind::IncompatibleTypes, diagnostic_arguments![Symbol(got_type), Symbol(limit_type.clone())]);
            self.ast_to_symbol.set(exp, None);
            return Ok(None);
        }
        let v = v.unwrap();
        self.ast_to_symbol.set(exp, Some(v.clone()));
        Ok(Some(v))
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum VerifyMode {
    Read,
    Write,
    Delete,
}

#[derive(Clone)]
pub struct ExpressionVerifyContext {
    pub context_type: Option<Symbol>,
    pub followed_by_type_arguments: bool,
    pub mode: VerifyMode,
    pub preceded_by_negative: bool,
}

impl Default for ExpressionVerifyContext {
    fn default() -> Self {
        Self {
            context_type: None,
            followed_by_type_arguments: false,
            mode: VerifyMode::Read,
            preceded_by_negative: false,
        }
    }
}