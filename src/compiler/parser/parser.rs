use crate::ns::*;
use lazy_regex::*;
use std::cell::Cell;
use std::rc::Rc;
use std::str::FromStr;

pub struct Parser<'input> {
    tokenizer: Tokenizer<'input>,
    previous_token: (Token, Location),
    token: (Token, Location),
    locations: Vec<Location>,
    activations: Vec<ParsingActivation>,
}

impl<'input> Parser<'input> {
    /// Constructs a parser.
    pub fn new(compilation_unit: &'input Rc<CompilationUnit>) -> Self {
        Self {
            tokenizer: Tokenizer::new(compilation_unit),
            previous_token: (Token::Eof, Location::with_line_and_offset(&compilation_unit, 1, 0)),
            token: (Token::Eof, Location::with_line_and_offset(&compilation_unit, 1, 0)),
            locations: vec![],
            activations: vec![],
        }
    }

    fn compilation_unit(&self) -> &Rc<CompilationUnit> {
        self.tokenizer.compilation_unit()
    }

    fn token_location(&self) -> Location {
        self.token.1.clone()
    }

    fn mark_location(&mut self) {
        self.locations.push(self.token.1.clone());
    }

    fn duplicate_location(&mut self) {
        self.locations.push(self.locations.last().unwrap().clone());
    }

    fn push_location(&mut self, location: &Location) {
        self.locations.push(location.clone());
    }

    fn pop_location(&mut self) -> Location {
        self.locations.pop().unwrap().combine_with_start_of(self.token.1.clone())
    }

    fn add_syntax_error(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        self.compilation_unit().add_diagnostic(Diagnostic::new_syntax_error(location, kind, arguments));
    }

    /*
    fn add_warning(&self, location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) {
        self.compilation_unit().add_diagnostic(Diagnostic::new_warning(location, kind, arguments));
    }
    */

    fn next(&mut self) -> Result<(), ParsingFailure> {
        self.previous_token = self.token.clone();
        self.token = self.tokenizer.scan_ie_div()?;
        Ok(())
    }

    fn next_ie_xml_tag(&mut self) -> Result<(), ParsingFailure> {
        self.previous_token = self.token.clone();
        self.token = self.tokenizer.scan_ie_xml_tag()?;
        Ok(())
    }

    fn next_ie_xml_content(&mut self) -> Result<(), ParsingFailure> {
        self.previous_token = self.token.clone();
        self.token = self.tokenizer.scan_ie_xml_content()?;
        Ok(())
    }

    fn peek(&self, token: Token) -> bool {
        self.token.0 == token
    }

    fn peek_identifier(&self, reserved_words: bool) -> Result<Option<(String, Location)>, ParsingFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            Ok(Some((id, location)))
        } else {
            if reserved_words {
                if let Some(id) = self.token.0.reserved_word_name() {
                    let location = self.token.1.clone();
                    return Ok(Some((id, location)));
                }
            }
            Ok(None)
        }
    }

    fn peek_context_keyword(&self, name: &str) -> bool {
        if let Token::Identifier(id) = self.token.0.clone() { id == name && self.token.1.character_count() == name.len() } else { false }
    }

    fn consume(&mut self, token: Token) -> Result<bool, ParsingFailure> {
        if self.token.0 == token {
            self.next()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn consume_and_ie_xml_tag(&mut self, token: Token) -> Result<bool, ParsingFailure> {
        if self.token.0 == token {
            self.next_ie_xml_tag()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn consume_and_ie_xml_content(&mut self, token: Token) -> Result<bool, ParsingFailure> {
        if self.token.0 == token {
            self.next_ie_xml_content()?;
            Ok(true)
        } else {
            Ok(false)
        }
    }

    fn consume_identifier(&mut self, reserved_words: bool) -> Result<Option<(String, Location)>, ParsingFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            self.next()?;
            Ok(Some((id, location)))
        } else {
            if reserved_words {
                if let Some(id) = self.token.0.reserved_word_name() {
                    let location = self.token.1.clone();
                    self.next()?;
                    return Ok(Some((id, location)));
                }
            }
            Ok(None)
        }
    }

    fn _consume_context_keyword(&mut self, name: &str) -> Result<bool, ParsingFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            if id == name && self.token.1.character_count() == name.len() {
                self.next()?;
                Ok(true)
            } else {
                Ok(false)
            }
        } else {
            Ok(false)
        }
    }

    fn expect(&mut self, token: Token) -> Result<(), ParsingFailure> {
        if self.token.0 != token {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![Token(token), Token(self.token.0.clone())]);
            Err(ParsingFailure)
        } else {
            self.next()?;
            Ok(())
        }
    }

    fn expect_and_ie_xml_tag(&mut self, token: Token) -> Result<(), ParsingFailure> {
        if self.token.0 != token {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![Token(token), Token(self.token.0.clone())]);
            Err(ParsingFailure)
        } else {
            self.next_ie_xml_tag()?;
            Ok(())
        }
    }

    fn expect_and_ie_xml_content(&mut self, token: Token) -> Result<(), ParsingFailure> {
        if self.token.0 != token {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![Token(token), Token(self.token.0.clone())]);
            Err(ParsingFailure)
        } else {
            self.next_ie_xml_content()?;
            Ok(())
        }
    }

    fn expect_identifier(&mut self, reserved_words: bool) -> Result<(String, Location), ParsingFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let location = self.token.1.clone();
            self.next()?;
            Ok((id, location))
        } else {
            if reserved_words {
                if let Some(id) = self.token.0.reserved_word_name() {
                    let location = self.token.1.clone();
                    self.next()?;
                    return Ok((id, location));
                }
            }
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParsingFailure)
        }
    }

    fn expect_context_keyword(&mut self, name: &str) -> Result<(), ParsingFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            if id == name && self.token.1.character_count() == name.len() {
                self.next()?;
                return Ok(());
            }
        }
        self.add_syntax_error(&self.token_location(), DiagnosticKind::Expected, diagnostic_arguments![String(name.into()), Token(self.token.0.clone())]);
        Err(ParsingFailure)
    }

    /// Expects a greater-than symbol. If the facing token is not greater-than,
    /// but starts with a greater-than symbol, the first character is shifted off
    /// from the facing token.
    fn expect_type_parameters_gt(&mut self) -> Result<(), ParsingFailure> {
        match self.token.0 {
            Token::Gt => {
                self.next()?;
                Ok(())
            },
            Token::Ge => {
                self.token.0 = Token::Assign;
                self.token.1.first_offset += 1;
                Ok(())
            },
            Token::RightShift => {
                self.token.0 = Token::Gt;
                self.token.1.first_offset += 1;
                Ok(())
            },
            Token::RightShiftAssign => {
                self.token.0 = Token::Ge;
                self.token.1.first_offset += 1;
                Ok(())
            },
            Token::UnsignedRightShift => {
                self.token.0 = Token::RightShift;
                self.token.1.first_offset += 1;
                Ok(())
            },
            Token::UnsignedRightShiftAssign => {
                self.token.0 = Token::RightShiftAssign;
                self.token.1.first_offset += 1;
                Ok(())
            },
            _ => {
                self.expect(Token::Gt)
            },
        }
    }

    pub fn expect_eof(&mut self) -> Result<(), ParsingFailure> {
        self.expect(Token::Eof)
    }

    pub fn parse_expression(&mut self, context: ParsingExpressionContext) -> Result<Rc<Expression>, ParsingFailure> {
        if let Some(exp) = self.parse_opt_expression(context)? {
            Ok(exp)
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedExpression, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParsingFailure)
        }
    }

    pub fn parse_opt_expression(&mut self, context: ParsingExpressionContext) -> Result<Option<Rc<Expression>>, ParsingFailure> {
        let exp: Option<Rc<Expression>> = self.parse_opt_start_expression(context.clone())?;

        // Parse subexpressions
        if let Some(exp) = exp {
            return Ok(Some(self.parse_subexpressions(exp, context.clone())?));
        }
        Ok(None)
    }

    fn parse_subexpressions(&mut self, mut base: Rc<Expression>, context: ParsingExpressionContext) -> Result<Rc<Expression>, ParsingFailure> {
        loop {
            if self.consume(Token::Dot)? {
                base = self.parse_dot_subexpression(base)?;
            } else if self.consume(Token::OptionalChaining)? {
                base = self.parse_optional_chaining(base)?;
            } else if self.peek(Token::LeftBracket) {
                self.next()?;
                self.push_location(&base.location());
                let key = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
                self.expect(Token::RightBracket)?;
                base = Rc::new(Expression::ComputedMember(ComputedMemberExpression {
                    base, key, location: self.pop_location()
                }));
            } else if self.consume(Token::Descendants)? {
                self.push_location(&base.location());
                let id = self.parse_qualified_identifier()?;
                base = Rc::new(Expression::Descendants(DescendantsExpression {
                    location: self.pop_location(),
                    base,
                    identifier: id,
                }));
            } else if self.peek(Token::LeftParen) {
                self.push_location(&base.location());
                let arguments = self.parse_arguments()?;
                base = Rc::new(Expression::Call(CallExpression {
                    location: self.pop_location(),
                    base,
                    arguments,
                }));
            } else if self.peek(Token::Increment) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location());
                self.next()?;
                base = Rc::new(Expression::Unary(UnaryExpression {
                    location: self.pop_location(),
                    expression: base,
                    operator: Operator::PostIncrement,
                }));
            } else if self.peek(Token::Decrement) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location());
                self.next()?;
                base = Rc::new(Expression::Unary(UnaryExpression {
                    location: self.pop_location(),
                    expression: base,
                    operator: Operator::PostDecrement,
                }));
            } else if self.peek(Token::Exclamation) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location());
                self.next()?;
                base = Rc::new(Expression::Unary(UnaryExpression {
                    location: self.pop_location(),
                    expression: base, operator: Operator::NonNull,
                }));
            // `not in`
            } else if self.token.0 == Token::Not && context.allow_in && context.min_precedence.includes(&OperatorPrecedence::Relational) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location());
                self.next()?;
                self.expect(Token::In)?;
                base = self.parse_binary_operator(base, Operator::NotIn, OperatorPrecedence::Relational.add(1).unwrap(), context.clone())?;
            // ConditionalExpression
            } else if self.peek(Token::Question) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) {
                self.push_location(&base.location());
                self.next()?;
                let consequent = self.parse_expression(ParsingExpressionContext {
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    ..context.clone()
                })?;
                self.expect(Token::Colon)?;
                let alternative = self.parse_expression(ParsingExpressionContext {
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    ..context.clone()
                })?;
                base = Rc::new(Expression::Conditional(ConditionalExpression {
                    location: self.pop_location(),
                    test: base, consequent, alternative,
                }));
            } else if let Some(binary_operator) = self.check_binary_operator(context.clone()) {
                let BinaryOperator(operator, required_precedence, _) = binary_operator;
                if context.min_precedence.includes(&required_precedence) {
                    self.next()?;
                    base = self.parse_binary_operator(base, operator, binary_operator.right_precedence(), context.clone())?;
                } else {
                    break;
                }
            // AssignmentExpression
            } else if self.peek(Token::Assign) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) && context.allow_assignment {
                self.push_location(&base.location());
                self.next()?;
                let left = base.clone();
                let right = self.parse_expression(ParsingExpressionContext {
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    ..context.clone()
                })?;
                base = Rc::new(Expression::Assignment(AssignmentExpression {
                    location: self.pop_location(),
                    left, compound: None, right,
                }));
            // CompoundAssignment and LogicalAssignment
            } else if let Some(compound) = self.token.0.compound_assignment() {
                if context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) && context.allow_assignment {
                    self.push_location(&base.location());
                    self.next()?;
                    let left = base.clone();
                    let right = self.parse_expression(ParsingExpressionContext {
                        min_precedence: OperatorPrecedence::AssignmentAndOther,
                        ..context.clone()
                    })?;
                    base = Rc::new(Expression::Assignment(AssignmentExpression {
                        location: self.pop_location(),
                        left, compound: Some(compound), right,
                    }));
                } else {
                    break;
                }
            } else if self.peek(Token::Comma) && context.min_precedence.includes(&OperatorPrecedence::List) {
                self.push_location(&base.location());
                self.next()?;
                let right = self.parse_expression(ParsingExpressionContext {
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    ..context.clone()
                })?;
                base = Rc::new(Expression::Sequence(SequenceExpression {
                    location: self.pop_location(),
                    left: base, right,
                }));
            } else {
                break;
            }
        }

        Ok(base)
    }

    fn parse_binary_operator(&mut self, base: Rc<Expression>, mut operator: Operator, right_precedence: OperatorPrecedence, context: ParsingExpressionContext) -> Result<Rc<Expression>, ParsingFailure> {
        // The left operand of a null-coalescing operation must not be
        // a logical AND, XOR or OR operation.
        if operator == Operator::NullCoalescing {
            if let Expression::Unary(UnaryExpression { expression, operator, .. }) = base.as_ref() {
                if [Operator::LogicalAnd, Operator::LogicalXor, Operator::LogicalOr].contains(&operator) {
                    self.add_syntax_error(&expression.location(), DiagnosticKind::IllegalNullishCoalescingLeftOperand, vec![]);
                }
            }
        }

        if operator == Operator::Is && self.consume(Token::Not)? {
            operator = Operator::IsNot;
        }

        self.push_location(&base.location());
        let right = self.parse_expression(ParsingExpressionContext {
            min_precedence: right_precedence,
            ..context
        })?;
        Ok(Rc::new(Expression::Binary(BinaryExpression {
            location: self.pop_location(),
            left: base, operator, right,
        })))
    }

    fn check_binary_operator(&self, context: ParsingExpressionContext) -> Option<BinaryOperator> {
        if let Some(operator) = self.token.0.to_binary_operator() {
            if operator == Operator::In && !context.allow_in {
                return None;
            }
            Some(BinaryOperator::try_from(operator).unwrap())
        } else {
            None
        }
    }

    fn parse_optional_chaining(&mut self, base: Rc<Expression>) -> Result<Rc<Expression>, ParsingFailure> {
        self.push_location(&base.location());
        self.duplicate_location();
        let mut operation = Rc::new(Expression::OptionalChainingPlaceholder(OptionalChainingPlaceholder {
            location: base.location(),
        }));
        if self.peek(Token::LeftParen) {
            let arguments: Vec<Rc<Expression>> = self.parse_arguments()?;
            if arguments.len() == 1 && self.peek(Token::ColonColon) {
                self.duplicate_location();
                let ql = self.pop_location();
                let identifier = self.finish_qualified_identifier(false, ql, arguments[0].clone())?;
                operation = Rc::new(Expression::Member(MemberExpression {
                    location: self.pop_location(),
                    base: operation,
                    identifier,
                }));
            } else {
                operation = Rc::new(Expression::Call(CallExpression {
                    location: self.pop_location(),
                    base: operation, arguments
                }));
            }
        } else if self.consume(Token::LeftBracket)? {
            let key = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
            self.expect(Token::RightBracket)?;
            operation = Rc::new(Expression::ComputedMember(ComputedMemberExpression {
                location: self.pop_location(),
                base: operation, key
            }));
        } else {
            let identifier = self.parse_qualified_identifier()?;
            operation = Rc::new(Expression::Member(MemberExpression {
                location: self.pop_location(),
                base: operation, identifier
            }));
        }

        // Parse postfix subexpressions
        operation = self.parse_optional_chaining_subexpressions(operation)?;

        Ok(Rc::new(Expression::OptionalChaining(OptionalChainingExpression {
            location: self.pop_location(),
            base, expression: operation,
        })))
    }

    fn parse_optional_chaining_subexpressions(&mut self, mut base: Rc<Expression>) -> Result<Rc<Expression>, ParsingFailure> {
        loop {
            if self.consume(Token::Dot)? {
                base = self.parse_dot_subexpression(base)?;
            } else if self.consume(Token::OptionalChaining)? {
                base = self.parse_optional_chaining(base)?;
            } else if self.peek(Token::LeftBracket) {
                self.next()?;
                self.push_location(&base.location());
                let key = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
                self.expect(Token::RightBracket)?;
                base = Rc::new(Expression::ComputedMember(ComputedMemberExpression {
                    base, key, location: self.pop_location()
                }));
            } else if self.consume(Token::Descendants)? {
                self.push_location(&base.location());
                let id = self.parse_qualified_identifier()?;
                base = Rc::new(Expression::Descendants(DescendantsExpression {
                    location: self.pop_location(),
                    base,
                    identifier: id,
                }));
            } else if self.peek(Token::LeftParen) {
                self.push_location(&base.location());
                let arguments = self.parse_arguments()?;
                base = Rc::new(Expression::Call(CallExpression {
                    location: self.pop_location(),
                    base,
                    arguments,
                }));
            } else if self.peek(Token::Exclamation) && !self.previous_token.1.line_break(&self.token.1) {
                self.push_location(&base.location());
                self.next()?;
                base = Rc::new(Expression::Unary(UnaryExpression {
                    location: self.pop_location(),
                    expression: base, operator: Operator::NonNull,
                }));
            } else {
                break;
            }
        }

        Ok(base)
    }

    fn parse_dot_subexpression(&mut self, base: Rc<Expression>) -> Result<Rc<Expression>, ParsingFailure> {
        self.push_location(&base.location());
        if self.peek(Token::LeftParen) {
            let paren_location = self.token_location();
            let paren_exp = self.parse_paren_list_expression()?;
            if !matches!(paren_exp.as_ref(), Expression::Sequence(_)) && self.peek(Token::ColonColon) {
                let id = self.finish_qualified_identifier(false, paren_location, paren_exp.clone())?;
                Ok(Rc::new(Expression::Member(MemberExpression {
                    location: self.pop_location(),
                    base, identifier: id
                })))
            } else {
                Ok(Rc::new(Expression::Filter(FilterExpression {
                    location: self.pop_location(),
                    base, test: paren_exp
                })))
            }
        } else if self.consume(Token::Lt)? {
            let mut arguments = vec![];
            arguments.push(self.parse_type_expression()?);
            while self.consume(Token::Comma)? {
                arguments.push(self.parse_type_expression()?);
            }
            self.expect_type_parameters_gt()?;
            Ok(Rc::new(Expression::WithTypeArguments(ExpressionWithTypeArguments {
                location: self.pop_location(),
                base, arguments
            })))
        } else {
            let id = self.parse_qualified_identifier()?;
            Ok(Rc::new(Expression::Member(MemberExpression {
                location: self.pop_location(),
                base, identifier: id
            })))
        }
    }

    /// Ensures a parameter list consists of zero or more required parameters followed by
    /// zero or more optional parameters optionally followed by a rest parameter.
    fn validate_parameter_list(&mut self, params: &Vec<Rc<Parameter>>) -> Result<(), ParsingFailure> {
        let mut least_kind = ParameterKind::Required; 
        let mut has_rest = false;
        for param in params {
            if !least_kind.may_be_followed_by(param.kind) {
                self.add_syntax_error(&param.location.clone(), DiagnosticKind::WrongParameterPosition, vec![]);
            }
            least_kind = param.kind;
            if param.kind == ParameterKind::Rest && has_rest {
                self.add_syntax_error(&param.location.clone(), DiagnosticKind::DuplicateRestParameter, vec![]);
            }
            has_rest = param.kind == ParameterKind::Rest;
        }
        Ok(())
    }

    fn parse_opt_start_expression(&mut self, context: ParsingExpressionContext) -> Result<Option<Rc<Expression>>, ParsingFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let id_location = self.token_location();
            self.next()?;

            // EmbedExpression
            if self.peek(Token::LeftBrace) && id == "embed" && self.previous_token.1.character_count() == "embed".len() {
                return Ok(Some(self.finish_embed_expression(id_location)?));
            }

            let id = Rc::new(Expression::QualifiedIdentifier(QualifiedIdentifier {
                location: id_location.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id((id, id_location.clone())),
            }));
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
                self.duplicate_location();
                let ql = self.pop_location();
                let id = self.finish_qualified_identifier(false, ql, id)?;
                Ok(Some(Rc::new(Expression::QualifiedIdentifier(id))))
            } else {
                Ok(Some(id))
            }
        } else if self.peek(Token::Null) {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(Expression::NullLiteral(NullLiteral {
                location: self.pop_location(),
            }))))
        } else if self.peek(Token::False) {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(Expression::BooleanLiteral(BooleanLiteral {
                location: self.pop_location(),
                value: false,
            }))))
        } else if self.peek(Token::True) {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(Expression::BooleanLiteral(BooleanLiteral {
                location: self.pop_location(),
                value: true,
            }))))
        } else if let Token::NumericLiteral(n) = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(Expression::NumericLiteral(NumericLiteral {
                location: self.pop_location(),
                value: n,
            }))))
        } else if let Token::StringLiteral(ref s) = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(Expression::StringLiteral(StringLiteral {
                location: self.pop_location(),
                value: s.clone(),
            }))))
        } else if self.peek(Token::This) {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(Expression::ThisLiteral(ThisLiteral {
                location: self.pop_location(),
            }))))
        } else if let Token::RegExpLiteral { ref body, ref flags } = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Some(Rc::new(Expression::RegExpLiteral(RegExpLiteral {
                location: self.pop_location(),
                body: body.clone(), flags: flags.clone(),
            }))))
        // `@`
        } else if self.peek(Token::Attribute) {
            self.mark_location();
            let id = self.parse_qualified_identifier()?;
            Ok(Some(Rc::new(Expression::QualifiedIdentifier(id))))
        // Parentheses
        } else if self.peek(Token::LeftParen) {
            return Ok(Some(self.parse_paren_list_expr_or_qual_id()?));
        // `*`
        } else if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            let id = Rc::new(Expression::QualifiedIdentifier(QualifiedIdentifier {
                location: id_location.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id(("*".into(), id_location.clone())),
            }));
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
                self.duplicate_location();
                let ql = self.pop_location();
                let id = self.finish_qualified_identifier(false, ql, id)?;
                Ok(Some(Rc::new(Expression::QualifiedIdentifier(id))))
            } else {
                Ok(Some(id))
            }
        // XMLList, XMLElement, XMLMarkup
        } else if self.peek(Token::Lt) {
            if let Some(token) = self.tokenizer.scan_xml_markup(self.token_location())? {
                self.token = token;
            }
            let start = self.token_location();
            if let Token::XmlMarkup(content) = &self.token.0.clone() {
                self.mark_location();
                self.next()?;
                Ok(Some(Rc::new(Expression::XmlMarkup(XmlMarkupExpression {
                    location: self.pop_location(),
                    markup: content.clone(),
                }))))
            } else {
                Ok(Some(self.parse_xml_element_or_xml_list(start)?))
            }
        // ArrayInitializer
        } else if self.peek(Token::LeftBracket) {
            Ok(Some(self.parse_array_initializer()?))
        // NewExpression
        } else if self.peek(Token::New) && context.min_precedence.includes(&OperatorPrecedence::Unary) {
            let start = self.token_location();
            self.next()?;
            Ok(Some(self.parse_new_expression(start)?))
        } else if self.peek(Token::LeftBrace) {
            Ok(Some(self.parse_object_initializer()?))
        } else if self.peek(Token::Function) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) {
            Ok(Some(self.parse_function_expression(context.clone())?))
        // SuperExpression
        } else if self.peek(Token::Super) && context.min_precedence.includes(&OperatorPrecedence::Postfix) {
            Ok(Some(self.parse_super_expression_followed_by_property_operator()?))
        // AwaitExpression
        } else if self.peek(Token::Await) && context.min_precedence.includes(&OperatorPrecedence::Unary) {
            self.mark_location();
            let operator_token = self.token.clone();
            self.next()?;
            let base = self.parse_expression(ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::Unary,
                ..default()
            })?;
            if let Some(activation) = self.activations.last_mut() {
                activation.uses_await = true;
            } else {
                self.add_syntax_error(&operator_token.1, DiagnosticKind::NotAllowedHere, diagnostic_arguments![Token(operator_token.0)]);
            }
            Ok(Some(Rc::new(Expression::Unary(UnaryExpression {
                location: self.pop_location(),
                expression: base, operator: Operator::Await,
            }))))
        // YieldExpression
        } else if self.peek(Token::Yield) && context.min_precedence.includes(&OperatorPrecedence::AssignmentAndOther) {
            self.mark_location();
            let operator_token = self.token.clone();
            self.next()?;
            let base = self.parse_expression(ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?;
            if let Some(activation) = self.activations.last_mut() {
                activation.uses_yield = true;
            } else {
                self.add_syntax_error(&operator_token.1, DiagnosticKind::NotAllowedHere, diagnostic_arguments![Token(operator_token.0)]);
            }
            Ok(Some(Rc::new(Expression::Unary(UnaryExpression {
                location: self.pop_location(),
                expression: base, operator: Operator::Yield,
            }))))
        // Miscellaneous prefix unary expressions
        } else if let Some((operator, subexp_precedence)) = self.check_prefix_operator() {
            if context.min_precedence.includes(&OperatorPrecedence::Unary) {
                self.mark_location();
                self.next()?;
                let base = self.parse_expression(ParsingExpressionContext { min_precedence: subexp_precedence, ..default() })?;
                Ok(Some(Rc::new(Expression::Unary(UnaryExpression {
                    location: self.pop_location(),
                    expression: base, operator,
                }))))
            } else {
                Ok(None)
            }
        // ImportMeta
        } else if self.peek(Token::Import) && context.min_precedence.includes(&OperatorPrecedence::Postfix) {
            self.mark_location();
            self.next()?;
            self.expect(Token::Dot)?;
            self.expect_context_keyword("meta")?;
            Ok(Some(Rc::new(Expression::ImportMeta(ImportMeta {
                location: self.pop_location(),
            }))))
        } else {
            Ok(None)
        }
    }

    fn check_prefix_operator(&self) -> Option<(Operator, OperatorPrecedence)> {
        match self.token.0 {
            Token::Delete => Some((Operator::Delete, OperatorPrecedence::Postfix)),
            Token::Void => Some((Operator::Void, OperatorPrecedence::Unary)),
            Token::Typeof => Some((Operator::Typeof, OperatorPrecedence::Unary)),
            Token::Increment => Some((Operator::PreIncrement, OperatorPrecedence::Postfix)),
            Token::Decrement => Some((Operator::PreDecrement, OperatorPrecedence::Postfix)),
            Token::Plus => Some((Operator::Positive, OperatorPrecedence::Unary)),
            Token::Minus => Some((Operator::Negative, OperatorPrecedence::Unary)),
            Token::BitwiseNot => Some((Operator::BitwiseNot, OperatorPrecedence::Unary)),
            Token::Exclamation => Some((Operator::LogicalNot, OperatorPrecedence::Unary)),
            _ => None,
        }
    }

    fn parse_function_expression(&mut self, context: ParsingExpressionContext) -> Result<Rc<Expression>, ParsingFailure> {
        self.mark_location();
        self.next()?;
        let mut name = None;
        if let Token::Identifier(id) = self.token.0.clone() {
            name = Some((id, self.token.1.clone()));
            self.next()?;
        }
        let common = self.parse_function_common(true, ParsingDirectiveContext::Default, context.allow_in)?;
        Ok(Rc::new(Expression::Function(FunctionExpression {
            location: self.pop_location(),
            name,
            common,
        })))
    }

    fn parse_function_common(&mut self, function_expr: bool, block_context: ParsingDirectiveContext, allow_in: bool) -> Result<Rc<FunctionCommon>, ParsingFailure> {
        self.mark_location();
        self.duplicate_location();
        self.expect(Token::LeftParen)?;
        let mut params: Vec<Rc<Parameter>> = vec![];
        if !self.peek(Token::RightParen) {
            params.push(self.parse_parameter()?);
            while self.consume(Token::Comma)? {
                params.push(self.parse_parameter()?);
            }
        }
        self.expect(Token::RightParen)?;
        self.validate_parameter_list(&params)?;

        let return_annotation = if self.consume(Token::Colon)? { Some(self.parse_type_expression()?) } else { None };

        let signature_location = self.pop_location();

        // Enter activation
        self.activations.push(ParsingActivation::new());

        // Body
        let body = if self.peek(Token::LeftBrace) {
            Some(FunctionBody::Block(Rc::new(self.parse_block(block_context)?)))
        } else {
            self.parse_opt_expression(ParsingExpressionContext {
                allow_in,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?.map(|e| FunctionBody::Expression(e))
        };

        // Body is required by function expressions
        if body.is_none() && function_expr {
            self.expect(Token::LeftBrace)?;
        }

        // Exit activation
        let activation = self.activations.pop().unwrap();

        Ok(Rc::new(FunctionCommon {
            location: self.pop_location(),
            contains_await: activation.uses_await,
            contains_yield: activation.uses_yield,
            signature: FunctionSignature {
                location: signature_location,
                parameters: params,
                result_type: return_annotation,
            },
            body,
        }))
    }

    fn parse_parameter(&mut self) -> Result<Rc<Parameter>, ParsingFailure> {
        self.mark_location();
        let rest = self.consume(Token::Ellipsis)?;
        let binding: Rc<VariableBinding> = Rc::new(self.parse_variable_binding(true)?);
        let has_initializer = binding.initializer.is_some();
        let location = self.pop_location();
        if rest && has_initializer {
            self.add_syntax_error(&location.clone(), DiagnosticKind::MalformedRestParameter, vec![]);
        }
        Ok(Rc::new(Parameter {
            location,
            destructuring: binding.destructuring.clone(),
            default_value: binding.initializer.clone(),
            kind: if rest {
                ParameterKind::Rest
            } else if has_initializer {
                ParameterKind::Optional
            } else {
                ParameterKind::Required
            },
        }))
    }

    fn parse_object_initializer(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        self.mark_location();
        self.expect(Token::LeftBrace)?;
        let mut fields: Vec<Rc<InitializerField>> = vec![];
        while !self.peek(Token::RightBrace) {
            fields.push(self.parse_field()?);
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightBrace)?;

        Ok(Rc::new(Expression::ObjectInitializer(ObjectInitializer {
            location: self.pop_location(),
            fields,
        })))
    }

    fn parse_field(&mut self) -> Result<Rc<InitializerField>, ParsingFailure> {
        if self.peek(Token::Ellipsis) {
            self.mark_location();
            self.next()?;
            let subexp = self.parse_expression(ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?;
            return Ok(Rc::new(InitializerField::Rest((subexp, self.pop_location()))));
        }

        let name = self.parse_field_name()?;

        let non_null = self.consume(Token::Exclamation)?;
        let mut value = None;

        if self.consume(Token::Colon)? {
            value = Some(self.parse_expression(ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?);
        } else if !matches!(name.0, FieldName::Identifier(_)) {
            self.expect(Token::Colon)?;
        }

        Ok(Rc::new(InitializerField::Field {
            name,
            non_null,
            value,
        }))
    }

    fn parse_field_name(&mut self) -> Result<(FieldName, Location), ParsingFailure> {
        if let Token::StringLiteral(value) = &self.token.0.clone() {
            let location = self.token_location();
            self.next()?;
            Ok((FieldName::StringLiteral(Rc::new(Expression::StringLiteral(StringLiteral {
                location: location.clone(),
                value: value.clone(),
            }))), location))
        } else if let Token::NumericLiteral(value) = &self.token.0.clone() {
            let location = self.token_location();
            self.next()?;
            Ok((FieldName::NumericLiteral(Rc::new(Expression::NumericLiteral(NumericLiteral {
                location: location.clone(),
                value: value.clone(),
            }))), location))
        } else if self.peek(Token::LeftBracket) {
            self.mark_location();
            self.next()?;
            let key_expr = self.parse_expression(ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::List,
                ..default()
            })?;
            self.expect(Token::RightBracket)?;
            let location = self.pop_location();
            Ok((FieldName::Brackets(key_expr), location))
        } else {
            let (id, location) = self.expect_identifier(true)?;
            Ok((FieldName::Identifier(id), location))
        }
    }

    fn parse_new_expression(&mut self, start: Location) -> Result<Rc<Expression>, ParsingFailure> {
        self.push_location(&start);
        let base = self.parse_new_subexpression()?;
        let arguments = if self.peek(Token::LeftParen) { Some(self.parse_arguments()?) } else { None };
        Ok(Rc::new(Expression::New(NewExpression {
            location: self.pop_location(),
            base, arguments,
        })))
    }

    fn parse_new_expression_start(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        if self.peek(Token::New) {
            let start = self.token_location();
            self.next()?;
            self.parse_new_expression(start)
        } else if self.peek(Token::Super) {
            self.parse_super_expression_followed_by_property_operator()
        } else {
            self.parse_primary_expression()
        }
    }

    fn parse_super_expression_followed_by_property_operator(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        self.mark_location();
        self.duplicate_location();
        self.next()?;
        let arguments = if self.peek(Token::LeftParen) { Some(self.parse_arguments()?) } else { None };
        let super_expr = Rc::new(Expression::Super(SuperExpression {
            location: self.pop_location(),
            object: arguments,
        }));

        if self.consume(Token::LeftBracket)? {
            let key = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
            self.expect(Token::RightBracket)?;
            Ok(Rc::new(Expression::ComputedMember(ComputedMemberExpression {
                location: self.pop_location(),
                base: super_expr, key,
            })))
        } else {
            self.expect(Token::Dot)?;
            let identifier = self.parse_qualified_identifier()?;
            Ok(Rc::new(Expression::Member(MemberExpression {
                location: self.pop_location(),
                base: super_expr, identifier,
            })))
        }
    }

    fn parse_arguments(&mut self) -> Result<Vec<Rc<Expression>>, ParsingFailure> {
        self.expect(Token::LeftParen)?;
        let mut arguments = vec![];
        if !self.peek(Token::RightParen) {
            arguments.push(self.parse_expression(ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?);
            while self.consume(Token::Comma)? {
                arguments.push(self.parse_expression(ParsingExpressionContext {
                    allow_in: true,
                    min_precedence: OperatorPrecedence::AssignmentAndOther,
                    ..default()
                })?);
            }
        }
        self.expect(Token::RightParen)?;
        Ok(arguments)
    }

    fn parse_new_subexpression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        let mut base = self.parse_new_expression_start()?;
        loop {
            if self.consume(Token::LeftBracket)? {
                self.push_location(&base.location());
                let key = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
                self.expect(Token::RightBracket)?;
                base = Rc::new(Expression::ComputedMember(ComputedMemberExpression {
                    location: self.pop_location(),
                    base, key,
                }));
            } else if self.consume(Token::Dot)? {
                self.push_location(&base.location());
                let identifier = self.parse_qualified_identifier()?;
                base = Rc::new(Expression::Member(MemberExpression {
                    location: self.pop_location(),
                    base, identifier,
                }));
            } else {
                break;
            }
        }
        Ok(base)
    }

    fn parse_primary_expression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        if let Token::Identifier(id) = self.token.0.clone() {
            let id_location = self.token_location();
            self.next()?;

            // EmbedExpression
            if self.peek(Token::LeftBrace) && id == "embed" && self.previous_token.1.character_count() == "embed".len() {
                return Ok(self.finish_embed_expression(id_location)?);
            }

            let id = Rc::new(Expression::QualifiedIdentifier(QualifiedIdentifier {
                location: id_location.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id((id, id_location.clone())),
            }));
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
                self.duplicate_location();
                let ql = self.pop_location();
                let id = self.finish_qualified_identifier(false, ql, id)?;
                Ok(Rc::new(Expression::QualifiedIdentifier(id)))
            } else {
                Ok(id)
            }
        } else if self.peek(Token::Null) {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(Expression::NullLiteral(NullLiteral {
                location: self.pop_location(),
            })))
        } else if self.peek(Token::False) {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(Expression::BooleanLiteral(BooleanLiteral {
                location: self.pop_location(),
                value: false,
            })))
        } else if self.peek(Token::True) {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(Expression::BooleanLiteral(BooleanLiteral {
                location: self.pop_location(),
                value: true,
            })))
        } else if let Token::NumericLiteral(n) = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(Expression::NumericLiteral(NumericLiteral {
                location: self.pop_location(),
                value: n,
            })))
        } else if let Token::StringLiteral(ref s) = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(Expression::StringLiteral(StringLiteral {
                location: self.pop_location(),
                value: s.clone(),
            })))
        } else if self.peek(Token::This) {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(Expression::ThisLiteral(ThisLiteral {
                location: self.pop_location(),
            })))
        } else if let Token::RegExpLiteral { ref body, ref flags } = self.token.0.clone() {
            self.mark_location();
            self.next()?;
            Ok(Rc::new(Expression::RegExpLiteral(RegExpLiteral {
                location: self.pop_location(),
                body: body.clone(), flags: flags.clone(),
            })))
        // `@`
        } else if self.peek(Token::Attribute) {
            self.mark_location();
            let id = self.parse_qualified_identifier()?;
            Ok(Rc::new(Expression::QualifiedIdentifier(id)))
        // Parentheses
        } else if self.peek(Token::LeftParen) {
            return Ok(self.parse_paren_list_expr_or_qual_id()?);
        // `*`
        } else if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            let id = Rc::new(Expression::QualifiedIdentifier(QualifiedIdentifier {
                location: id_location.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id(("*".into(), id_location.clone())),
            }));
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
                self.duplicate_location();
                let ql = self.pop_location();
                let id = self.finish_qualified_identifier(false, ql, id)?;
                Ok(Rc::new(Expression::QualifiedIdentifier(id)))
            } else {
                Ok(id)
            }
        // XMLList, XMLElement, XMLMarkup
        } else if self.peek(Token::Lt) {
            if let Some(token) = self.tokenizer.scan_xml_markup(self.token_location())? {
                self.token = token;
            }
            let start = self.token_location();
            if let Token::XmlMarkup(content) = &self.token.0.clone() {
                self.mark_location();
                self.next()?;
                Ok(Rc::new(Expression::XmlMarkup(XmlMarkupExpression {
                    location: self.pop_location(),
                    markup: content.clone(),
                })))
            } else {
                Ok(self.parse_xml_element_or_xml_list(start)?)
            }
        // ArrayInitializer
        } else if self.peek(Token::LeftBracket) {
            Ok(self.parse_array_initializer()?)
        } else if self.peek(Token::LeftBrace) {
            Ok(self.parse_object_initializer()?)
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedExpression, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParsingFailure)
        }
    }

    fn finish_embed_expression(&mut self, start: Location) -> Result<Rc<Expression>, ParsingFailure> {
        self.push_location(&start);
        let descriptor = self.parse_object_initializer()?.clone();
        let Expression::ObjectInitializer(descriptor) = descriptor.as_ref() else {
            panic!();
        };
        return Ok(Rc::new(Expression::Embed(EmbedExpression {
            location: self.pop_location(),
            description: descriptor.clone(),
        })));
    }

    fn parse_array_initializer(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        self.mark_location();

        self.expect(Token::LeftBracket)?;

        let mut elements: Vec<Element> = vec![];

        while !self.peek(Token::RightBracket) {
            let mut ellipses = false;
            while self.consume(Token::Comma)? {
                elements.push(Element::Elision);
                ellipses = true;
            }
            if !ellipses  {
                if self.peek(Token::Ellipsis) {
                    self.mark_location();
                    self.next()?;
                    elements.push(Element::Rest((self.parse_expression(ParsingExpressionContext {
                        allow_in: true,
                        min_precedence: OperatorPrecedence::AssignmentAndOther,
                        ..default()
                    })?, self.pop_location())));
                } else {
                    elements.push(Element::Expression(self.parse_expression(ParsingExpressionContext {
                        allow_in: true,
                        min_precedence: OperatorPrecedence::AssignmentAndOther,
                        ..default()
                    })?));
                }
            }
            if !self.consume(Token::Comma)? {
                break;
            }
        }
        self.expect(Token::RightBracket)?;
        Ok(Rc::new(Expression::ArrayLiteral(ArrayLiteral {
            location: self.pop_location(),
            elements,
        })))
    }

    fn parse_xml_element_or_xml_list(&mut self, start: Location) -> Result<Rc<Expression>, ParsingFailure> {
        self.next_ie_xml_tag()?;
        if self.consume_and_ie_xml_content(Token::Gt)? {
            self.push_location(&start);
            let content = self.parse_xml_content()?;
            self.expect_and_ie_xml_tag(Token::XmlLtSlash)?;
            self.expect(Token::Gt)?;
            return Ok(Rc::new(Expression::XmlList(XmlListExpression {
                location: self.pop_location(),
                content,
            })));
        }

        self.push_location(&start);
        let element = Rc::new(self.parse_xml_element(start, true)?);
        return Ok(Rc::new(Expression::Xml(XmlExpression {
            location: self.pop_location(),
            element,
        })));
    }

    /// Parses XMLElement starting from its XMLTagContent.
    fn parse_xml_element(&mut self, start: Location, ends_at_ie_div: bool) -> Result<XmlElement, ParsingFailure> {
        self.push_location(&start);
        let name = self.parse_xml_tag_name()?;
        let mut attributes: Vec<Rc<XmlAttribute>> = vec![];
        let mut attribute_expression: Option<Rc<Expression>> = None;
        while self.consume_and_ie_xml_tag(Token::XmlWhitespace)? {
            if self.consume(Token::LeftBrace)? {
                let expr = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::AssignmentAndOther, ..default() })?;
                self.expect_and_ie_xml_tag(Token::RightBrace)?;
                attribute_expression = Some(expr);
                self.consume_and_ie_xml_tag(Token::XmlWhitespace)?;
                break;
            } else if matches!(self.token.0, Token::XmlName(_)) {
                self.mark_location();
                let name = self.parse_xml_name()?;
                self.consume_and_ie_xml_tag(Token::XmlWhitespace)?;
                self.expect_and_ie_xml_tag(Token::Assign)?;
                self.consume_and_ie_xml_tag(Token::XmlWhitespace)?;
                let value: XmlAttributeValue;
                if self.consume(Token::LeftBrace)? {
                    let expr = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::AssignmentAndOther, ..default() })?;
                    self.expect_and_ie_xml_tag(Token::RightBrace)?;
                    value = XmlAttributeValue::Expression(expr);
                } else {
                    value = XmlAttributeValue::Value(self.parse_xml_attribute_value()?);
                }
                attributes.push(Rc::new(XmlAttribute {
                    location: self.pop_location(),
                    name, value
                }));
            } else {
                break;
            }
        }

        let mut content: Option<Vec<Rc<XmlElementContent>>> = None;
        let mut closing_name: Option<XmlTagName> = None;

        let is_empty;

        if ends_at_ie_div {
            is_empty = self.consume(Token::XmlSlashGt)?;
        } else {
            is_empty = self.consume_and_ie_xml_content(Token::XmlSlashGt)?;
        }

        if !is_empty {
            self.expect_and_ie_xml_content(Token::Gt)?;
            content = Some(self.parse_xml_content()?);
            self.expect_and_ie_xml_tag(Token::XmlLtSlash)?;
            closing_name = Some(self.parse_xml_tag_name()?);
            self.consume_and_ie_xml_tag(Token::XmlWhitespace)?;
            if ends_at_ie_div {
                self.expect(Token::Gt)?;
            } else {
                self.expect_and_ie_xml_content(Token::Gt)?;
            }
        }

        Ok(XmlElement {
            location: self.pop_location(),
            name,
            attributes,
            attribute_expression,
            content,
            closing_name,
        })
    }
    
    fn parse_xml_attribute_value(&mut self) -> Result<(String, Location), ParsingFailure> {
        if let Token::XmlAttributeValue(value) = self.token.0.clone() {
            let location = self.token_location();
            self.next_ie_xml_tag()?;
            return Ok((value, location));
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedXmlAttributeValue, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParsingFailure)
        }
    }

    fn parse_xml_tag_name(&mut self) -> Result<XmlTagName, ParsingFailure> {
        if self.consume(Token::LeftBrace)? {
            let expr = self.parse_expression(ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?;
            self.expect_and_ie_xml_tag(Token::RightBrace)?;
            Ok(XmlTagName::Expression(expr))
        } else {
            Ok(XmlTagName::Name(self.parse_xml_name()?))
        }
    }

    fn parse_xml_name(&mut self) -> Result<(String, Location), ParsingFailure> {
        if let Token::XmlName(name) = self.token.0.clone() {
            let name_location = self.token_location();
            self.next_ie_xml_tag()?;
            return Ok((name, name_location));
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedXmlName, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParsingFailure)
        }
    }

    /// Parses XMLContent until a `</` token.
    fn parse_xml_content(&mut self) -> Result<Vec<Rc<XmlElementContent>>, ParsingFailure> {
        let mut content = vec![];
        while !self.peek(Token::XmlLtSlash) {
            if self.consume(Token::LeftBrace)? {
                let expr = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::AssignmentAndOther, ..default() })?;
                self.expect_and_ie_xml_content(Token::RightBrace)?;
                content.push(Rc::new(XmlElementContent::Expression(expr)));
            } else if let Token::XmlMarkup(markup) = self.token.0.clone() {
                let location = self.token_location();
                self.next_ie_xml_content()?;
                content.push(Rc::new(XmlElementContent::XmlMarkup((markup, location))));
            } else if let Token::XmlText(text) = self.token.0.clone() {
                let location = self.token_location();
                self.next_ie_xml_content()?;
                content.push(Rc::new(XmlElementContent::XmlText((text, location))));
            } else if self.consume_and_ie_xml_tag(Token::Lt)? {
                let start = self.token_location();
                let element = self.parse_xml_element(start, false)?;
                content.push(Rc::new(XmlElementContent::XmlElement(Rc::new(element))));
            } else {
                self.expect_and_ie_xml_content(Token::XmlLtSlash)?;
            }
        }
        Ok(content)
    }

    fn finish_paren_list_expr_or_qual_id(&mut self, start: Location, left: Rc<Expression>) -> Result<Rc<Expression>, ParsingFailure> {
        if self.peek(Token::ColonColon) && !matches!(left.as_ref(), Expression::Sequence(_)) {
            self.push_location(&start);
            let ql = self.pop_location();
            let id = self.finish_qualified_identifier(false, ql, left)?;
            return Ok(Rc::new(Expression::QualifiedIdentifier(id)));
        }
        self.push_location(&start);
        return Ok(Rc::new(Expression::Paren(ParenExpression {
            location: self.pop_location(),
            expression: left,
        })));
    }

    /// Parses either a ParenListExpression, (), or a QualifiedIdentifier
    fn parse_paren_list_expr_or_qual_id(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        let start = self.token_location();
        self.expect(Token::LeftParen)?;

        let expr = self.parse_expression(ParsingExpressionContext {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            ..default()
        })?;

        self.expect(Token::RightParen)?;
        self.finish_paren_list_expr_or_qual_id(start, expr)
    }

    fn parse_qualified_identifier(&mut self) -> Result<QualifiedIdentifier, ParsingFailure> {
        self.mark_location();

        let attribute = self.consume(Token::Attribute)?;
        if attribute && self.peek(Token::LeftBracket) {
            let brackets = self.parse_brackets()?;
            return Ok(QualifiedIdentifier {
                location: self.pop_location(),
                attribute,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Brackets(brackets),
            });
        }

        let mut id: Option<String> = None;

        // IdentifierName
        if let Token::Identifier(id_1) = self.token.0.clone() {
            id = Some(id_1);
        } else {
            if let Some(id_1) = self.token.0.reserved_word_name() {
                id = Some(id_1);
            } else if self.peek(Token::Times) {
                id = Some("*".to_owned());
            }
        }

        if let Some(id) = id {
            let id_location = self.token_location();
            self.next()?;
            if self.peek(Token::ColonColon) {
                let id = QualifiedIdentifier {
                    location: id_location.clone(),
                    attribute: false,
                    qualifier: None,
                    id: QualifiedIdentifierIdentifier::Id((id, id_location.clone())),
                };
                let id = Rc::new(Expression::QualifiedIdentifier(id));
                let ql = self.pop_location();
                return self.finish_qualified_identifier(attribute, ql, id);
            } else {
                let id = QualifiedIdentifier {
                    location: id_location.clone(),
                    attribute,
                    qualifier: None,
                    id: QualifiedIdentifierIdentifier::Id((id, id_location.clone())),
                };
                return Ok(id);
            }
        }

        // (q)::x
        if self.peek(Token::LeftParen) {
            let qual = self.parse_paren_expression()?;
            let ql = self.pop_location();
            return self.finish_qualified_identifier(attribute, ql, qual);
        }

        self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
        Err(ParsingFailure)
    }

    /// Expects a colon-colon and finishes a qualified identifier.
    fn finish_qualified_identifier(&mut self, attribute: bool, start_location: Location, qual: Rc<Expression>) -> Result<QualifiedIdentifier, ParsingFailure> {
        self.push_location(&start_location);
        self.expect(Token::ColonColon)?;

        // `::` may be followed by one of { IdentifierName, `*`, Brackets }

        // IdentifierName
        if let Some(id) = self.consume_identifier(true)? {
            Ok(QualifiedIdentifier {
                location: self.pop_location(),
                attribute,
                qualifier: Some(qual),
                id: QualifiedIdentifierIdentifier::Id(id),
            })
        // `*`
        } else if self.peek(Token::Times) {
            let id_location = self.token_location();
            self.next()?;
            Ok(QualifiedIdentifier {
                location: self.pop_location(),
                attribute,
                qualifier: Some(qual),
                id: QualifiedIdentifierIdentifier::Id(("*".into(), id_location)),
            })
        // Brackets
        } else if self.peek(Token::LeftBracket) {
            let brackets = self.parse_brackets()?;
            Ok(QualifiedIdentifier {
                location: self.pop_location(),
                attribute,
                qualifier: Some(qual),
                id: QualifiedIdentifierIdentifier::Brackets(brackets),
            })
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedIdentifier, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParsingFailure)
        }
    }

    fn parse_brackets(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        self.expect(Token::LeftBracket)?;
        let expr = self.parse_expression(ParsingExpressionContext {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            ..default()
        });
        self.expect(Token::RightBracket)?;
        expr
    }

    fn parse_paren_expression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        self.expect(Token::LeftParen)?;
        let expr = self.parse_expression(ParsingExpressionContext {
            min_precedence: OperatorPrecedence::AssignmentAndOther,
            allow_in: true,
            ..default()
        });
        self.expect(Token::RightParen)?;
        expr
    }

    fn parse_paren_list_expression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        self.expect(Token::LeftParen)?;
        let expr = self.parse_expression(ParsingExpressionContext {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            ..default()
        });
        self.expect(Token::RightParen)?;
        expr
    }

    fn parse_typed_destructuring(&mut self) -> Result<TypedDestructuring, ParsingFailure> {
        self.mark_location();
        let mut destructuring: Rc<Expression>;
        if self.peek(Token::LeftBrace) {
            destructuring = self.parse_object_initializer()?;
        } else if self.peek(Token::LeftBracket) {
            destructuring = self.parse_array_initializer()?;
        } else {
            let id = self.expect_identifier(true)?;
            let id = QualifiedIdentifier {
                location: id.1.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id(id.clone()),
            };
            destructuring = Rc::new(Expression::QualifiedIdentifier(id));
        }
        if self.consume(Token::Exclamation)? {
            self.push_location(&destructuring.location());
            destructuring = Rc::new(Expression::Unary(UnaryExpression {
                location: self.pop_location(),
                operator: Operator::NonNull,
                expression: destructuring.clone(),
            }));
        }
        let type_annotation = if self.consume(Token::Colon)? { Some(self.parse_type_expression()?) } else { None };
        Ok(TypedDestructuring {
            location: self.pop_location(),
            destructuring,
            type_annotation,
        })
    }

    pub fn parse_type_expression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        let start = self.token_location();
        let (mut base, wrap_nullable) = self.parse_type_expression_start()?;

        loop {
            if self.consume(Token::Dot)? {
                self.push_location(&base.location());
                if self.consume(Token::Lt)? {
                    let mut arguments = vec![self.parse_type_expression()?];
                    while self.consume(Token::Comma)? {
                        arguments.push(self.parse_type_expression()?);
                    }
                    self.expect_type_parameters_gt()?;
                    base = Rc::new(Expression::WithTypeArguments(ExpressionWithTypeArguments {
                        location: self.pop_location(),
                        base, arguments,
                    }));
                } else {
                    let id = self.expect_identifier(true)?;
                    base = Rc::new(Expression::Member(MemberExpression {
                        location: self.pop_location(),
                        base, identifier: QualifiedIdentifier {
                            location: id.1.clone(),
                            attribute: false,
                            qualifier: None,
                            id: QualifiedIdentifierIdentifier::Id(id),
                        },
                    }));
                }
            } else if self.consume(Token::Question)? {
                self.push_location(&base.location());
                base = Rc::new(Expression::NullableType(NullableTypeExpression {
                    location: self.pop_location(),
                    base,
                }));
            } else if self.consume(Token::Exclamation)? {
                self.push_location(&base.location());
                base = Rc::new(Expression::NonNullableType(NonNullableTypeExpression {
                    location: self.pop_location(),
                    base,
                }));
            } else {
                break;
            }
        }
        
        if wrap_nullable {
            self.push_location(&start);
            base = Rc::new(Expression::NullableType(NullableTypeExpression {
                location: self.pop_location(),
                base,
            }));
        }

        Ok(base)
    }

    fn parse_type_expression_start(&mut self) -> Result<(Rc<Expression>, bool), ParsingFailure> {
        // Allow a `?` prefix to wrap a type into nullable.
        let wrap_nullable = self.consume(Token::Question)?;

        // Parenthesized
        if self.peek(Token::LeftParen) {
            self.mark_location();
            let expression = self.parse_type_expression()?;
            Ok((Rc::new(Expression::Paren(ParenExpression {
                location: self.pop_location(),
                expression,
            })), wrap_nullable))
        }
        // `function`
        else if self.peek(Token::Function) {
            Ok((self.parse_function_type_expression()?, wrap_nullable))
        // `void`
        } else if self.peek(Token::Void) {
            self.mark_location();
            self.next()?;
            Ok((Rc::new(Expression::VoidType(VoidTypeExpression {
                location: self.pop_location(),
            })), wrap_nullable))
        // [T]
        // [T1, T2, ...Tn]
        } else if self.peek(Token::LeftBracket) {
            let mut elements = vec![];
            self.mark_location();
            self.next()?;
            elements.push(self.parse_type_expression()?);
            if self.consume(Token::RightBracket)? {
                Ok((Rc::new(Expression::ArrayType(ArrayTypeExpression {
                    location: self.pop_location(),
                    expression: elements[0].clone(),
                })), wrap_nullable))
            } else {
                self.expect(Token::Comma)?;
                elements.push(self.parse_type_expression()?);
                while self.consume(Token::Comma)? {
                    if self.peek(Token::RightBracket) {
                        break;
                    }
                    elements.push(self.parse_type_expression()?);
                }
                self.expect(Token::RightBracket)?;
                Ok((Rc::new(Expression::TupleType(TupleTypeExpression {
                    location: self.pop_location(),
                    expressions: elements,
                })), wrap_nullable))
            }
        } else if self.peek(Token::Times) {
            let location = self.token_location();
            self.next()?;
            return Ok((Rc::new(Expression::AnyType(AnyTypeExpression {
                location,
            })), wrap_nullable));
        // Identifier
        } else {
            let id = self.expect_identifier(false)?;
            Ok((Rc::new(Expression::QualifiedIdentifier(QualifiedIdentifier {
                location: id.1.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id(id),
            })), wrap_nullable))
        }
    }

    fn parse_function_type_expression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        self.mark_location();
        self.next()?;
        self.mark_location();

        self.expect(Token::LeftParen)?;
        let mut parameters = vec![];
        if !self.peek(Token::RightParen) {
            parameters.push(self.parse_function_type_parameter()?);
            while self.consume(Token::Comma)? {
                parameters.push(self.parse_function_type_parameter()?);
            }
        }
        self.expect(Token::RightParen)?;
        self.validate_parameter_list(&parameters)?;

        self.expect(Token::Colon)?;
        let result_type = self.parse_type_expression()?;
        let signature_location = self.pop_location();
        Ok(Rc::new(Expression::FunctionType(FunctionTypeExpression {
            location: self.pop_location(),
            signature: FunctionSignature {
                location: signature_location,
                parameters,
                result_type: Some(result_type),
            },
        })))
    }

    fn parse_function_type_parameter(&mut self) -> Result<Rc<Parameter>, ParsingFailure> {
        self.mark_location();
        let rest = self.consume(Token::Ellipsis)?;
        let id = self.expect_identifier(false)?;
        let optional = !rest && self.consume(Token::Question)?;
        let destructuring = TypedDestructuring {
            location: id.1.clone(),
            destructuring: Rc::new(Expression::QualifiedIdentifier(QualifiedIdentifier {
                location: id.1.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id(id),
            })),
            type_annotation: if self.consume(Token::Colon)? { Some(self.parse_type_expression()?) } else { None },
        };
        let location = self.pop_location();
        Ok(Rc::new(Parameter {
            location,
            destructuring,
            default_value: None,
            kind: if rest {
                ParameterKind::Rest
            } else if optional {
                ParameterKind::Optional
            } else {
                ParameterKind::Required
            },
        }))
    }

    fn parse_variable_binding(&mut self, allow_in: bool) -> Result<VariableBinding, ParsingFailure> {
        let destructuring = self.parse_typed_destructuring()?;
        let initializer = if self.consume(Token::Assign)? {
            Some(self.parse_expression(ParsingExpressionContext {
                allow_in,
                min_precedence: OperatorPrecedence::AssignmentAndOther,
                ..default()
            })?)
        } else {
            None
        };
        Ok(VariableBinding {
            destructuring,
            initializer,
        })
    }

    fn parse_semicolon(&mut self) -> Result<bool, ParsingFailure> {
        Ok(self.consume(Token::Semicolon)? || self.peek(Token::RightBrace) || self.previous_token.1.line_break(&self.token.1))
    }

    fn parse_substatement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.parse_statement(context)
    }

    fn parse_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        // ExpressionStatement or LabeledStatement
        if let Token::Identifier(id) = &self.token.0.clone() {
            let id = (id.clone(), self.token_location());
            self.next()?;
            self.parse_statement_starting_with_identifier(context, id)
        // SuperStatement or ExpressionStatement with `super`
        } else if self.peek(Token::Super) {
            self.mark_location();
            self.next()?;
            let arguments = if self.peek(Token::LeftParen) { Some(self.parse_arguments()?) } else { None };
            let mut semicolon_inserted = false;
            if arguments.is_some() {
                semicolon_inserted = self.parse_semicolon()?;
            }
            if arguments.is_none() || (!semicolon_inserted && (self.peek(Token::Dot) || self.peek(Token::LeftBracket))) {
                if !(self.peek(Token::Dot) || self.peek(Token::LeftBracket)) {
                    self.expect(Token::Dot)?;
                }
                self.duplicate_location();
                // ExpressionStatement (`super`...)
                let mut expr = Rc::new(Expression::Super(SuperExpression {
                    location: self.pop_location(),
                    object: arguments,
                }));
                expr = self.parse_subexpressions(expr, ParsingExpressionContext {
                    allow_in: true,
                    min_precedence: OperatorPrecedence::List,
                    ..default()
                })?;
                let semicolon_inserted = self.parse_semicolon()?;
                Ok((Rc::new(Directive::ExpressionStatement(ExpressionStatement {
                    location: self.pop_location(),
                    expression: expr,
                })), semicolon_inserted))
            } else {
                // SuperStatement
                let node = Rc::new(Directive::SuperStatement(SuperStatement {
                    location: self.pop_location(),
                    arguments: arguments.unwrap(),
                }));

                // Check whether super statement is allowed here
                let allowed_here;
                if let ParsingDirectiveContext::ConstructorBlock { super_statement_found } = &context {
                    allowed_here = !super_statement_found.get();
                    super_statement_found.set(true);
                } else {
                    allowed_here = false;
                }

                if !allowed_here {
                    self.add_syntax_error(&node.location(), DiagnosticKind::NotAllowedHere, diagnostic_arguments![Token(Token::Super)]);
                }

                Ok((node, semicolon_inserted))
            }
        // EmptyStatement
        } else if self.peek(Token::Semicolon) {
            self.mark_location();
            self.next()?;
            Ok((Rc::new(Directive::EmptyStatement(EmptyStatement {
                location: self.pop_location(),
            })), true))
        // Block
        } else if self.peek(Token::LeftBrace) {
            let context = context.override_control_context(true, ParsingControlContext {
                breakable: true,
                iteration: false,
            });
            let block = self.parse_block(context)?;
            Ok((Rc::new(Directive::Block(block)), true))
        // IfStatement
        } else if self.peek(Token::If) {
            self.parse_if_statement(context)
        // SwitchStatement
        // `switch type`
        } else if self.peek(Token::Switch) {
            self.parse_switch_statement(context)
        // DoStatement
        } else if self.peek(Token::Do) {
            self.parse_do_statement(context)
        // WhileStatement
        } else if self.peek(Token::While) {
            self.parse_while_statement(context)
        // ForStatement
        // `for..in`
        // `for each`
        } else if self.peek(Token::For) {
            self.parse_for_statement(context)
        // WithStatement
        } else if self.peek(Token::With) {
            self.parse_with_statement(context)
        // BreakStatement
        } else if self.peek(Token::Break) {
            self.parse_break_statement(context)
        // ContinueStatement
        } else if self.peek(Token::Continue) {
            self.parse_continue_statement(context)
        // ReturnStatement
        } else if self.peek(Token::Return) {
            self.parse_return_statement(context)
        // ThrowStatement
        } else if self.peek(Token::Throw) {
            self.parse_throw_statement(context)
        // TryStatement
        } else if self.peek(Token::Try) {
            self.parse_try_statement(context)
        // `default xml namespace = expression`
        } else if self.peek(Token::Default) {
            self.parse_default_xml_namespace_statement()
        // ExpressionStatement
        } else {
            self.mark_location();
            let exp = self.parse_expression(ParsingExpressionContext {
                allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
            })?;
            let semicolon_inserted = self.parse_semicolon()?;
            Ok((Rc::new(Directive::ExpressionStatement(ExpressionStatement {
                location: self.pop_location(),
                expression: exp,
            })), semicolon_inserted))
        }
    }

    fn parse_statement_starting_with_identifier(&mut self, context: ParsingDirectiveContext, id: (String, Location)) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.push_location(&id.1);
        let id_location = id.1.clone();

        // LabeledStatement
        if self.consume(Token::Colon)? {
            let (substatement, semicolon_inserted) = self.parse_substatement(context.put_label(id.0.clone()))?;
            let labeled = Rc::new(Directive::LabeledStatement(LabeledStatement {
                location: self.pop_location(),
                label: id.clone(),
                substatement,
            }));
            return Ok((labeled, semicolon_inserted));
        }

        let mut exp: Rc<Expression>;

        // EmbedExpression
        if self.peek(Token::LeftBrace) && id.0 == "embed" && self.previous_token.1.character_count() == "embed".len() {
            exp = self.finish_embed_expression(id_location)?;
        } else {
            let id = Rc::new(Expression::QualifiedIdentifier(QualifiedIdentifier {
                location: id_location.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id(id.clone()),
            }));
            if self.peek(Token::ColonColon) {
                self.push_location(&id_location.clone());
                self.duplicate_location();
                let ql = self.pop_location();
                let id = self.finish_qualified_identifier(false, ql, id)?;
                exp = Rc::new(Expression::QualifiedIdentifier(id));
            } else {
                exp = id;
            }
        }

        exp = self.parse_subexpressions(exp, ParsingExpressionContext {
            allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
        })?;
        let semicolon_inserted = self.parse_semicolon()?;
        Ok((Rc::new(Directive::ExpressionStatement(ExpressionStatement {
            location: self.pop_location(),
            expression: exp,
        })), semicolon_inserted))
    }

    fn parse_block(&mut self, context: ParsingDirectiveContext) -> Result<Block, ParsingFailure> {
        self.mark_location();
        self.expect(Token::LeftBrace)?;
        let mut directives = vec![];
        let mut semicolon_inserted = false;
        while !self.peek(Token::RightBrace) {
            if !directives.is_empty() && !semicolon_inserted {
                self.expect(Token::Semicolon)?;
            }
            let (directive, semicolon_inserted_1) = self.parse_directive(context.clone())?;
            directives.push(directive);
            semicolon_inserted = semicolon_inserted_1;
        }
        self.expect(Token::RightBrace)?;
        Ok(Block { 
            location: self.pop_location(),
            directives,
        })
    }

    fn parse_if_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let context = context.override_control_context(true, ParsingControlContext {
            breakable: true,
            iteration: false,
        });
        self.mark_location();
        self.next()?;
        self.expect(Token::LeftParen)?;
        let test = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;
        let semicolon_inserted;
        let (consequent, semicolon_inserted_1) = self.parse_substatement(context.clone())?;
        let mut alternative = None;
        if self.peek(Token::Else) {
            if !semicolon_inserted_1 {
                self.expect(Token::Semicolon)?;
            }
            self.next()?;
            let (alternative_2, semicolon_inserted_2) = self.parse_substatement(context.clone())?;
            alternative = Some(alternative_2);
            semicolon_inserted = semicolon_inserted_2;
        } else {
            semicolon_inserted = semicolon_inserted_1;
        }
        Ok((Rc::new(Directive::IfStatement(IfStatement {
            location: self.pop_location(),
            test, consequent, alternative,
        })), semicolon_inserted))
    }

    fn parse_switch_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.mark_location();
        self.next()?;
        if self.peek_context_keyword("type") {
            self.forbid_line_break_before_token();
            self.next()?;
            return self.parse_switch_type_statement(context);
        }
        let context = context.override_control_context(false, ParsingControlContext {
            breakable: true,
            iteration: false,
        });
        self.expect(Token::LeftParen)?;
        let discriminant = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        let cases = self.parse_case_elements(context)?;
        self.expect(Token::RightBrace)?;
        Ok((Rc::new(Directive::SwitchStatement(SwitchStatement {
            location: self.pop_location(),
            discriminant, cases,
        })), true))
    }

    fn parse_case_elements(&mut self, context: ParsingDirectiveContext) -> Result<Vec<Case>, ParsingFailure> {
        let mut cases = vec![];
        let mut semicolon_inserted = false;
        while !self.peek(Token::RightBrace) {
            if !cases.is_empty() && !semicolon_inserted {
                self.expect(Token::Semicolon)?;
            }
            if !(self.peek(Token::Case) || self.peek(Token::Default)) {
                break;
            }
            self.mark_location();
            let mut labels = vec![];
            loop {
                if self.peek(Token::Case) {
                    self.mark_location();
                    self.next()?;
                    let exp = self.parse_expression(ParsingExpressionContext {
                        allow_in: true,
                        min_precedence: OperatorPrecedence::List,
                        ..default()
                    })?;
                    self.expect(Token::Colon)?;
                    labels.push(CaseLabel::Case((exp, self.pop_location())));
                } else if self.peek(Token::Default) {
                    self.mark_location();
                    self.next()?;
                    self.expect(Token::Colon)?;
                    labels.push(CaseLabel::Default(self.pop_location()));
                } else {
                    break;
                }
            }
            let mut directives = vec![];
            semicolon_inserted = false;
            while !(self.peek(Token::RightBrace) || self.peek(Token::Case) || self.peek(Token::Default)) {
                if !directives.is_empty() && !semicolon_inserted {
                    self.expect(Token::Semicolon)?;
                }
                let (directive, semicolon_inserted_1) = self.parse_directive(context.clone())?;
                directives.push(directive);
                semicolon_inserted = semicolon_inserted_1;
            }
            cases.push(Case {
                location: self.pop_location(),
                labels,
                directives,
            });
        }
        Ok(cases)
    }

    fn parse_switch_type_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let context = context.override_control_context(true, ParsingControlContext {
            breakable: true,
            iteration: false,
        });
        self.expect(Token::LeftParen)?;
        let discriminant = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;
        self.expect(Token::LeftBrace)?;
        let cases = self.parse_type_case_elements(context)?;
        self.expect(Token::RightBrace)?;
        Ok((Rc::new(Directive::SwitchTypeStatement(SwitchTypeStatement {
            location: self.pop_location(),
            discriminant, cases,
        })), true))
    }

    fn parse_type_case_elements(&mut self, context: ParsingDirectiveContext) -> Result<Vec<TypeCase>, ParsingFailure> {
        let mut cases = vec![];
        while !self.peek(Token::RightBrace) {
            if self.peek(Token::Default) {
                self.mark_location();
                self.next()?;
                let block = Rc::new(self.parse_block(context.clone())?);
                cases.push(TypeCase {
                    location: self.pop_location(),
                    parameter: None,
                    block,
                });
            } else {
                self.mark_location();
                self.expect(Token::Case)?;
                self.expect(Token::LeftParen)?;
                let parameter = Some(self.parse_typed_destructuring()?);
                self.expect(Token::RightParen)?;
                let block = Rc::new(self.parse_block(context.clone())?);
                cases.push(TypeCase {
                    location: self.pop_location(),
                    parameter,
                    block,
                });
            }
        }
        Ok(cases)
    }

    fn parse_do_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let context = context.override_control_context(false, ParsingControlContext {
            breakable: true,
            iteration: true,
        });
        self.mark_location();
        self.next()?;

        // Body
        let (body, semicolon_inserted_1) = self.parse_substatement(context)?;
        if !semicolon_inserted_1 {
            self.expect(Token::Semicolon)?;
        }

        self.expect(Token::While)?;

        // Test
        self.expect(Token::LeftParen)?;
        let test = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;

        let semicolon_inserted = self.parse_semicolon()?;
        Ok((Rc::new(Directive::DoStatement(DoStatement {
            location: self.pop_location(),
            body, test,
        })), semicolon_inserted))
    }

    fn parse_while_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let context = context.override_control_context(false, ParsingControlContext {
            breakable: true,
            iteration: true,
        });
        self.mark_location();
        self.next()?;

        // Test
        self.expect(Token::LeftParen)?;
        let test = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(Directive::WhileStatement(WhileStatement {
            location: self.pop_location(),
            test, body,
        })), semicolon_inserted))
    }

    /// Parses `for`, `for..in` or `for each`.
    fn parse_for_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let context = context.override_control_context(false, ParsingControlContext {
            breakable: true,
            iteration: true,
        });
        self.mark_location();
        self.next()?;

        // `for each`
        if self.peek_context_keyword("each") {
            self.forbid_line_break_before_token();
            self.next()?;
            return self.parse_for_each_statement(context);
        }

        self.expect(Token::LeftParen)?;

        let init_variable = if self.peek(Token::Var) || self.peek(Token::Const) {
            Some(self.parse_simple_variable_definition(false)?)
        } else {
            None
        };

        if init_variable.is_some() && self.consume(Token::In)? {
            return self.parse_for_in_statement_with_left_variable(context, init_variable.unwrap());
        }

        let mut init_exp = if init_variable.is_none() && !self.peek(Token::Semicolon) {
            self.parse_opt_expression(ParsingExpressionContext {
                allow_in: false,
                min_precedence: OperatorPrecedence::Postfix,
                ..default()
            })?
        } else {
            None
        };

        if init_exp.is_some() && self.consume(Token::In)? {
            return self.parse_for_in_statement_with_left_exp(context, init_exp.unwrap());
        }

        if init_exp.is_none() && init_variable.is_none() && !self.peek(Token::Semicolon) {
            init_exp = Some(self.parse_expression(ParsingExpressionContext {
                allow_in: false, min_precedence: OperatorPrecedence::List, ..default()
            })?);
        } else if let Some(exp) = init_exp.as_ref() {
            init_exp = Some(self.parse_subexpressions(exp.clone(), ParsingExpressionContext {
                allow_in: false, min_precedence: OperatorPrecedence::List, ..default()
            })?);
        }

        let init = if let Some(exp) = init_exp.as_ref() {
            Some(ForInitializer::Expression(exp.clone()))
        } else if let Some(variable) = init_variable.as_ref() {
            Some(ForInitializer::VariableDefinition(variable.clone()))
        } else {
            None
        };

        self.expect(Token::Semicolon)?;
        let test = if self.peek(Token::Semicolon) {
            None
        } else {
            Some(self.parse_expression(ParsingExpressionContext {
                allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
            })?)
        };
        self.expect(Token::Semicolon)?;
        let update = if self.peek(Token::RightParen) {
            None
        } else {
            Some(self.parse_expression(ParsingExpressionContext {
                allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
            })?)
        };
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(Directive::ForStatement(ForStatement {
            location: self.pop_location(),
            init, test, update, body,
        })), semicolon_inserted))
    }

    fn parse_for_each_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.expect(Token::LeftParen)?;
        let left = if self.peek(Token::Var) || self.peek(Token::Const) {
            self.mark_location();
            let kind = (if self.peek(Token::Var) { VariableDefinitionKind::Var } else { VariableDefinitionKind::Const }, self.token_location());
            self.next()?;
            let binding = self.parse_variable_binding(false)?;
            if let Some(init) = &binding.initializer {
                self.add_syntax_error(&init.location(), DiagnosticKind::IllegalForInInitializer, vec![]);
            }
            ForInBinding::VariableDefinition(SimpleVariableDefinition {
                location: self.pop_location(),
                kind,
                bindings: vec![Rc::new(binding)],
            })
        } else {
            ForInBinding::Expression(self.parse_expression(ParsingExpressionContext {
                allow_in: false, min_precedence: OperatorPrecedence::Postfix, ..default()
            })?)
        };
        self.expect(Token::In)?;
        let right = self.parse_expression(ParsingExpressionContext {
            allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
        })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(Directive::ForInStatement(ForInStatement {
            location: self.pop_location(),
            each: true, left, right, body,
        })), semicolon_inserted))
    }

    fn parse_for_in_statement_with_left_variable(&mut self, context: ParsingDirectiveContext, left: SimpleVariableDefinition) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let variable_binding = left.bindings[0].clone();

        if let Some(init) = &variable_binding.initializer {
            self.add_syntax_error(&init.location(), DiagnosticKind::IllegalForInInitializer, vec![]);
        }

        if left.bindings.len() > 1 {
            self.add_syntax_error(&left.kind.1.clone(), DiagnosticKind::MultipleForInBindings, vec![]);
        }

        let right = self.parse_expression(ParsingExpressionContext {
            allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
        })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(Directive::ForInStatement(ForInStatement {
            location: self.pop_location(),
            each: false, left: ForInBinding::VariableDefinition(left), right, body,
        })), semicolon_inserted))
    }

    fn parse_for_in_statement_with_left_exp(&mut self, context: ParsingDirectiveContext, left: Rc<Expression>) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let right = self.parse_expression(ParsingExpressionContext {
            allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
        })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(Directive::ForInStatement(ForInStatement {
            location: self.pop_location(),
            each: false, left: ForInBinding::Expression(left), right, body,
        })), semicolon_inserted))
    }

    fn parse_simple_variable_definition(&mut self, allow_in: bool) -> Result<SimpleVariableDefinition, ParsingFailure> {
        self.mark_location();
        let kind: VariableDefinitionKind;
        let kind_location = self.token_location();
        if self.consume(Token::Const)? {
            kind = VariableDefinitionKind::Const;
        } else {
            self.expect(Token::Var)?;
            kind = VariableDefinitionKind::Var;
        }
        let mut bindings = vec![Rc::new(self.parse_variable_binding(allow_in)?)];
        while self.consume(Token::Comma)? {
            bindings.push(Rc::new(self.parse_variable_binding(allow_in)?));
        }
        Ok(SimpleVariableDefinition {
            location: self.pop_location(),
            kind: (kind, kind_location),
            bindings,
        })
    }

    fn parse_with_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let context = context.override_control_context(true, ParsingControlContext {
            breakable: true,
            iteration: false,
        });
        self.mark_location();
        self.next()?;

        // Object
        self.expect(Token::LeftParen)?;
        let object = self.parse_expression(ParsingExpressionContext { allow_in: true, min_precedence: OperatorPrecedence::List, ..default() })?;
        self.expect(Token::RightParen)?;

        // Body
        let (body, semicolon_inserted) = self.parse_substatement(context)?;

        Ok((Rc::new(Directive::WithStatement(WithStatement {
            location: self.pop_location(),
            object, body,
        })), semicolon_inserted))
    }

    fn parse_break_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.mark_location();
        self.next()?;

        let label = if self.previous_token.1.line_break(&self.token.1) { None } else { self.consume_identifier(false)? };
        let label_location = label.clone().map(|label| label.1.clone());
        let label = label.map(|label| label.0.clone());

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(Directive::BreakStatement(BreakStatement {
            location: self.pop_location(),
            label: label.clone().map(|l| (l.clone(), label_location.clone().unwrap())),
        }));

        if label.is_some() && !context.is_label_defined(label.clone().unwrap()) {
            self.add_syntax_error(&label_location.unwrap(), DiagnosticKind::UndefinedLabel, diagnostic_arguments![String(label.clone().unwrap())]);
        } else if !context.is_break_allowed(label) {
            self.add_syntax_error(&node.location(), DiagnosticKind::IllegalBreak, vec![]);
        }

        Ok((node, semicolon_inserted))
    }

    fn parse_continue_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.mark_location();
        self.next()?;

        let label = if self.previous_token.1.line_break(&self.token.1) { None } else { self.consume_identifier(false)? };
        let label_location = label.clone().map(|label| label.1.clone());
        let label = label.map(|label| label.0.clone());

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(Directive::ContinueStatement(ContinueStatement {
            location: self.pop_location(),
            label: label.clone().map(|l| (l.clone(), label_location.clone().unwrap())),
        }));

        if label.is_some() && !context.is_label_defined(label.clone().unwrap()) {
            self.add_syntax_error(&label_location.unwrap(), DiagnosticKind::UndefinedLabel, diagnostic_arguments![String(label.clone().unwrap())]);
        } else if !context.is_continue_allowed(label) {
            self.add_syntax_error(&node.location(), DiagnosticKind::IllegalContinue, vec![]);
        }

        Ok((node, semicolon_inserted))
    }

    fn parse_return_statement(&mut self, _context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.mark_location();
        self.next()?;

        let expression = if self.previous_token.1.line_break(&self.token.1) { None } else {
            self.parse_opt_expression(ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::List,
                ..default()
            })?
        };

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(Directive::ReturnStatement(ReturnStatement {
            location: self.pop_location(),
            expression,
        }));

        Ok((node, semicolon_inserted))
    }

    fn parse_throw_statement(&mut self, _context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.mark_location();
        self.next()?;

        let line_break = self.previous_token.1.line_break(&self.token.1);

        let expression = self.parse_expression(ParsingExpressionContext {
            allow_in: true,
            min_precedence: OperatorPrecedence::List,
            ..default()
        })?;

        if line_break {
            self.add_syntax_error(&expression.location(), DiagnosticKind::ExpressionMustNotFollowLineBreak, vec![]);
        }

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(Directive::ThrowStatement(ThrowStatement {
            location: self.pop_location(),
            expression,
        }));

        Ok((node, semicolon_inserted))
    }

    fn parse_try_statement(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.mark_location();
        self.next()?;
        let context = context.clone_control();
        let block = Rc::new(self.parse_block(context.clone())?);
        let mut catch_clauses: Vec<CatchClause> = vec![];
        let mut finally_clause: Option<FinallyClause> = None;
        loop {
            if self.peek(Token::Catch) {
                self.mark_location();
                self.next()?;
                self.expect(Token::LeftParen)?;
                let parameter = self.parse_typed_destructuring()?;
                self.expect(Token::RightParen)?;
                let block = Rc::new(self.parse_block(context.clone())?);
                catch_clauses.push(CatchClause {
                    location: self.pop_location(),
                    parameter,
                    block,
                });
            } else if self.peek(Token::Finally) {
                self.mark_location();
                self.next()?;
                let block = Rc::new(self.parse_block(context.clone())?);
                finally_clause = Some(FinallyClause {
                    location: self.pop_location(),
                    block,
                });
                break;
            } else {
                break;
            }
        }
        if catch_clauses.is_empty() && finally_clause.is_none() {
            self.expect(Token::Catch)?;
        }

        let node = Rc::new(Directive::TryStatement(TryStatement {
            location: self.pop_location(),
            block, catch_clauses, finally_clause,
        }));

        Ok((node, true))
    }

    fn parse_default_xml_namespace_statement(&mut self) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.mark_location();
        self.next()?;

        self.forbid_line_break_before_token();
        self.expect_context_keyword("xml")?;
        self.forbid_line_break_before_token();
        self.expect_context_keyword("namespace")?;
        self.expect(Token::Assign)?;

        let expression = self.parse_expression(ParsingExpressionContext {
            allow_in: true,
            allow_assignment: false,
            min_precedence: OperatorPrecedence::AssignmentAndOther,
            ..default()
        })?;

        let semicolon_inserted = self.parse_semicolon()?;

        let node = Rc::new(Directive::DefaultXmlNamespaceStatement(DefaultXmlNamespaceStatement {
            location: self.pop_location(),
            right: expression,
        }));

        Ok((node, semicolon_inserted))
    }

    fn forbid_line_break_before_token(&mut self) {
        if self.previous_token.1.line_break(&self.token.1) {
            self.add_syntax_error(&self.token.1.clone(), DiagnosticKind::TokenMustNotFollowLineBreak, vec![]);
        }
    }

    fn parse_directive(&mut self, context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let jetdoc: Option<Rc<JetDoc>> = self.parse_jetdoc()?;
        // ConfigurationDirective or Statement
        if let Token::Identifier(id) = &self.token.0 {
            let id = (id.clone(), self.token_location());
            self.next()?;
            if self.peek_annotatable_directive_identifier_name() || self.lookbehind_is_annotatable_directive_identifier_name() {
                let mut context1: AnnotatableContext;
                if ["enum", "type"].contains(&id.0.as_ref()) && id.1.character_count() == id.0.len() {
                    context1 = AnnotatableContext {
                        start_location: id.1.clone(),
                        jetdoc,
                        attributes: vec![],
                        context: context.clone(),
                        directive_context_keyword: Some(id.clone()),
                    };
                    // self.parse_attribute_identifier_names(&mut context)?;
                } else {
                    context1 = AnnotatableContext {
                        start_location: id.1.clone(),
                        jetdoc,
                        attributes: vec![self.keyword_attribute_from_previous_token().unwrap()],
                        context: context.clone(),
                        directive_context_keyword: None,
                    };
                    self.parse_attribute_identifier_names(&mut context1)?;
                }
                return self.parse_annotatable_directive(context1);
            } else if self.peek(Token::LeftBrace) && &id.0 == "configuration" && id.1.character_count() == "configuration".len() {
                self.parse_configuration_directive(context, id.1)
            } else {
                self.parse_statement_starting_with_identifier(context, id)
            }
        } else if self.peek(Token::Import) {
            self.parse_import_directive_or_expression_statement(context)
        } else if self.peek(Token::LeftBracket) {
            self.mark_location();
            let exp = self.parse_expression(ParsingExpressionContext {
                allow_in: true, min_precedence: OperatorPrecedence::List, ..default()
            })?;
            if self.peek_annotatable_directive_identifier_name() {
                if let Some(metadata) = exp.to_metadata() {
                    let mut context = AnnotatableContext {
                        start_location: self.pop_location(),
                        jetdoc,
                        attributes: metadata,
                        context: context.clone(),
                        directive_context_keyword: None,
                    };
                    self.parse_attribute_identifier_names(&mut context)?;
                    return self.parse_annotatable_directive(context);
                }
            }
            let semicolon = self.parse_semicolon()?;
            Ok((Rc::new(Directive::ExpressionStatement(ExpressionStatement {
                location: self.pop_location(),
                expression: exp,
            })), semicolon))
        } else if self.peek(Token::Public) || self.peek(Token::Private) || self.peek(Token::Protected)
        || self.peek(Token::Internal) || self.peek(Token::Var) || self.peek(Token::Const)
        || self.peek(Token::Function) || self.peek(Token::Class) || self.peek(Token::Interface)
        || self.peek(Token::Use) {
            let mut context = AnnotatableContext {
                start_location: self.token_location(),
                jetdoc,
                attributes: vec![],
                context: context.clone(),
                directive_context_keyword: None,
            };
            self.parse_attribute_identifier_names(&mut context)?;
            return self.parse_annotatable_directive(context);
        } else {
            self.parse_statement(context)
        }
    }

    fn parse_directives(&mut self, context: ParsingDirectiveContext) -> Result<Vec<Rc<Directive>>, ParsingFailure> {
        let mut directives = vec![];
        let mut semicolon = false;
        while !self.peek(Token::Eof) {
            if !directives.is_empty() && !semicolon {
                self.expect(Token::Semicolon)?;
            }
            let (directive, semicolon_1) = self.parse_directive(context.clone())?;
            directives.push(directive);
            semicolon = semicolon_1;
        }
        Ok(directives)
    }

    fn parse_annotatable_directive(&mut self, context: AnnotatableContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        if self.consume(Token::Use)? {
            self.parse_use_directive(context)
        } else if self.peek(Token::Var) || self.peek(Token::Const) {
            self.parse_variable_definition(context)
        } else if self.consume(Token::Function)? {
            self.parse_function_definition(context)
        } else if self.consume(Token::Class)? {
            self.parse_class_definition(context)
        } else if context.has_directive_context_keyword("enum") {
            self.parse_enum_definition(context)
        } else if self.consume(Token::Interface)? {
            self.parse_interface_definition(context)
        } else if context.has_directive_context_keyword("type") {
            self.parse_type_definition(context)
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedDirectiveKeyword, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParsingFailure)
        }
    }

    fn parse_import_directive_or_expression_statement(&mut self, _context: ParsingDirectiveContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.mark_location();
        self.next()?;
        if self.consume(Token::Dot)? {
            self.duplicate_location();
            self.expect_context_keyword("meta")?;
            let mut expression = Rc::new(Expression::ImportMeta(ImportMeta {
                location: self.pop_location(),
            }));
            expression = self.parse_subexpressions(expression, ParsingExpressionContext {
                allow_in: true,
                min_precedence: OperatorPrecedence::List,
                ..default()
            })?;
            let semicolon = self.parse_semicolon()?;
            Ok((Rc::new(Directive::ExpressionStatement(ExpressionStatement {
                location: self.pop_location(),
                expression,
            })), semicolon))
        } else {
            let mut alias: Option<(String, Location)> = None;
            let mut package_name: Vec<(String, Location)> = vec![];
            let mut import_specifier = ImportSpecifier::Wildcard(self.token_location());
            let id1 = self.expect_identifier(false)?;
            if self.consume(Token::Assign)? {
                alias = Some(id1.clone());
                package_name.push(self.expect_identifier(false)?);
            } else {
                package_name.push(id1);
            }
    
            if !self.peek(Token::Dot) {
                self.expect(Token::Dot)?;
            }
    
            while self.consume(Token::Dot)? {
                if self.peek(Token::Times) {
                    import_specifier = ImportSpecifier::Wildcard(self.token_location());
                    self.next()?;
                    break;
                } else {
                    let id1 = self.expect_identifier(true)?;
                    if !self.peek(Token::Dot) {
                        import_specifier = ImportSpecifier::Identifier(id1.clone());
                        break;
                    } else {
                        package_name.push(id1.clone());
                    }
                }
            }
    
            let semicolon = self.parse_semicolon()?;
    
            let node = Rc::new(Directive::ImportDirective(ImportDirective {
                location: self.pop_location(),
                alias,
                package_name,
                import_specifier,
            }));
    
            Ok((node, semicolon))
        }
    }

    fn parse_use_directive(&mut self, context: AnnotatableContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let AnnotatableContext { start_location, jetdoc, attributes, context, .. } = context;
        self.push_location(&start_location);
        let mut alias: Option<(String, Location)> = None;
        let mut package_name: Vec<(String, Location)> = vec![];
        let mut import_specifier = ImportSpecifier::Wildcard(self.token_location());
        let id1 = self.expect_identifier(false)?;
        if self.consume(Token::Assign)? {
            alias = Some(id1.clone());
            package_name.push(self.expect_identifier(false)?);
        } else {
            package_name.push(id1);
        }

        if !self.peek(Token::Dot) {
            self.expect(Token::Dot)?;
        }

        while self.consume(Token::Dot)? {
            if alias.is_none() && self.peek(Token::Times) {
                import_specifier = ImportSpecifier::Wildcard(self.token_location());
                self.next()?;
                break;
            } else {
                let id1 = self.expect_identifier(true)?;
                if !self.peek(Token::Dot) {
                    import_specifier = ImportSpecifier::Identifier(id1.clone());
                    break;
                } else {
                    package_name.push(id1.clone());
                }
            }
        }

        let semicolon = self.parse_semicolon()?;
        let location = self.pop_location();

        if !(matches!(context, ParsingDirectiveContext::PackageBlock { .. })) {
            self.add_syntax_error(&location, DiagnosticKind::NotAllowedHere, diagnostic_arguments![String("'use'".into()), Token(self.token.0.clone())]);
        }

        let mut has_public = false;

        for a in &attributes {
            if a.is_metadata() {
                continue;
            }
            if a.is_public() {
                has_public = true;
            } else {
                // Unallowed attribute
                self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
            }
        }

        if !has_public {
            self.add_syntax_error(&location, DiagnosticKind::UseDirectiveMustContainPublic, diagnostic_arguments![]);
        }

        let node = Rc::new(Directive::UseDirective(UseDirective {
            location,
            jetdoc,
            attributes,
            alias,
            package_name,
            import_specifier,
        }));

        Ok((node, semicolon))
    }

    fn parse_variable_definition(&mut self, context: AnnotatableContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let AnnotatableContext { start_location, jetdoc, attributes, context, .. } = context;
        let has_static = Attribute::find_static(&attributes).is_some();
        self.push_location(&start_location);
        let kind_location = self.token_location();
        let kind = if self.consume(Token::Const)? {
            VariableDefinitionKind::Const
        } else {
            self.expect(Token::Var)?;
            VariableDefinitionKind::Var
        };
        let mut bindings = vec![Rc::new(self.parse_variable_binding(true)?)];
        while self.consume(Token::Comma)? {
            bindings.push(Rc::new(self.parse_variable_binding(true)?));
        }

        // Forbid destructuring bindings in enumerations.
        if !has_static && matches!(context, ParsingDirectiveContext::EnumBlock) {
            for binding in &bindings {
                let malformed = matches!(binding.destructuring.destructuring.as_ref(), Expression::QualifiedIdentifier(_))
                    || binding.destructuring.type_annotation.is_some();
                if malformed {
                    self.add_syntax_error(&binding.location(), DiagnosticKind::MalformedEnumMember, diagnostic_arguments![]);
                }
            }
        }

        for a in &attributes {
            if a.is_metadata() {
                continue;
            }
            match a {
                Attribute::Static(_) => {
                    if !context.is_type_block() {
                        // Unallowed attribute
                        self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                    }
                },
                Attribute::Public(_) |
                Attribute::Private(_) |
                Attribute::Protected(_) |
                Attribute::Internal(_) => {
                    self.verify_visibility(&a, &context);
                },
                _ => {
                    // Unallowed attribute
                    self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                },
            }
        }

        let semicolon = self.parse_semicolon()?;
        let node = Rc::new(Directive::VariableDefinition(VariableDefinition {
            location: self.pop_location(),
            jetdoc,
            attributes,
            kind: (kind, kind_location),
            bindings,
        }));

        Ok((node, semicolon))
    }

    fn parse_function_definition(&mut self, context: AnnotatableContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let AnnotatableContext { start_location, jetdoc, attributes, context, .. } = context;
        let has_proxy = Attribute::find_proxy(&attributes).is_some();
        let has_native = Attribute::find_native(&attributes).is_some();
        self.push_location(&start_location);
        let mut name = self.expect_identifier(true)?;
        let mut getter = false;
        let mut setter = false;
        if self.peek_identifier(true)?.is_some() {
            getter = Token::is_context_keyword(&self.previous_token, "get");
            setter = Token::is_context_keyword(&self.previous_token, "set");
            if getter || setter {
                name = self.expect_identifier(true)?;
            }
        }
        let constructor = !getter && !setter && !has_proxy && context.function_name_is_constructor(&name);
        let name = if getter {
            FunctionName::Getter(name)
        } else if setter {
            FunctionName::Setter(name)
        } else if constructor {
            FunctionName::Constructor(name)
        } else if has_proxy {
            let proxy_kind = ProxyKind::from_str(&name.0);
            if proxy_kind.is_err() {
                self.add_syntax_error(&name.1, DiagnosticKind::UnrecognizedProxy, diagnostic_arguments![String(name.0.clone())]);
                return Err(ParsingFailure);
            }
            FunctionName::Proxy(proxy_kind.unwrap(), name)
        } else {
            FunctionName::Identifier(name)
        };
        let type_parameters = if !(has_proxy || constructor || getter || setter) {
            self.parse_type_parameters_opt()?
        } else {
            None
        };
        let block_context = if constructor {
            ParsingDirectiveContext::ConstructorBlock { super_statement_found: Cell::new(false) }
        } else {
            ParsingDirectiveContext::Default
        };
        let common = self.parse_function_common(false, block_context, true)?;
        let semicolon = if common.has_block_body() { true } else { self.parse_semicolon()? };

        // Not all kinds of functions may be generators.
        if common.contains_yield && (constructor || getter || setter) {
            self.add_syntax_error(&name.location(), DiagnosticKind::FunctionMayNotBeGenerator, diagnostic_arguments![]);
        }

        // Not all kinds of functions may be asynchronous.
        if common.contains_await && (constructor || getter || setter) {
            self.add_syntax_error(&name.location(), DiagnosticKind::FunctionMayNotBeAsynchronous, diagnostic_arguments![]);
        }

        let interface_method = matches!(context, ParsingDirectiveContext::InterfaceBlock);

        // Body verification.
        //
        // Interface methods are skipped in the verification as they
        // may omit body.
        if !interface_method {
            if has_native && common.body.is_some() {
                self.add_syntax_error(&name.location(), DiagnosticKind::FunctionMustNotContainBody, diagnostic_arguments![]);
            } else if !has_native && common.body.is_none() {
                self.add_syntax_error(&name.location(), DiagnosticKind::FunctionMustContainBody, diagnostic_arguments![]);
            }
        }

        // Interface methods must not contain any annotations except for meta-data.
        if !attributes.is_empty() && interface_method {
            if !attributes.last().unwrap().is_metadata() {
                self.add_syntax_error(&name.location(), DiagnosticKind::FunctionMustNotContainAnnotations, diagnostic_arguments![]);
            }
        }

        for a in &attributes {
            if a.is_metadata() {
                continue;
            }
            match a {
                Attribute::Static(_) => {
                    if !context.is_type_block() {
                        // Unallowed attribute
                        self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                    }
                },
                Attribute::Final(_) |
                Attribute::Override(_) |
                Attribute::Abstract(_) => {
                    if !context.is_type_block() || constructor || has_proxy {
                        // Unallowed attribute
                        self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                    }
                },

                Attribute::Native(_) => {},

                Attribute::Proxy(_) => {
                    if !context.is_type_block() || getter || setter || constructor {
                        // Unallowed attribute
                        self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                    }
                },
                Attribute::Public(_) |
                Attribute::Private(_) |
                Attribute::Protected(_) |
                Attribute::Internal(_) => {
                    if has_proxy {
                        // Unallowed visibility in proxy function
                        self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                    } else {
                        self.verify_visibility(&a, &context);
                    }
                },
                _ => {
                    // Unallowed attribute
                    self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                },
            }
        }

        let node = Rc::new(Directive::FunctionDefinition(FunctionDefinition {
            location: self.pop_location(),
            jetdoc,
            attributes,
            name: name.clone(),
            type_parameters,
            common,
        }));

        Ok((node, semicolon))
    }

    fn parse_class_definition(&mut self, context: AnnotatableContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let AnnotatableContext { start_location, jetdoc, attributes, context, .. } = context;
        self.push_location(&start_location);
        let name = self.expect_identifier(true)?;
        let type_parameters = self.parse_type_parameters_opt()?;
        let mut extends_clause: Option<Rc<Expression>> = None;
        if self.consume(Token::Extends)? {
            extends_clause = Some(self.parse_type_expression()?);
        }
        let mut implements_clause: Option<Vec<Rc<Expression>>> = None;
        if self.consume(Token::Implements)? {
            implements_clause = Some(self.parse_type_expression_list()?);
        }
        let block = Rc::new(self.parse_block(ParsingDirectiveContext::ClassBlock {
            name: name.0.clone(),
        })?);

        for a in &attributes {
            if a.is_metadata() {
                continue;
            }
            match a {
                Attribute::Static(_) => {},
                Attribute::Final(_) => {},
                Attribute::Abstract(_) => {},

                Attribute::Public(_) |
                Attribute::Private(_) |
                Attribute::Protected(_) |
                Attribute::Internal(_) => {
                    self.verify_visibility(&a, &context);
                },
                _ => {
                    // Unallowed attribute
                    self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                },
            }
        }

        // Nested classes not allowed
        if !context.is_top_level_or_package() {
            self.add_syntax_error(&name.1, DiagnosticKind::NestedClassesNotAllowed, diagnostic_arguments![]);
        }

        let node = Rc::new(Directive::ClassDefinition(ClassDefinition {
            location: self.pop_location(),
            jetdoc,
            attributes,
            name: name.clone(),
            type_parameters,
            extends_clause,
            implements_clause,
            block,
        }));

        Ok((node, true))
    }

    fn parse_enum_definition(&mut self, context: AnnotatableContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let AnnotatableContext { start_location, jetdoc, attributes, context, .. } = context;
        self.push_location(&start_location);
        let name = self.expect_identifier(true)?;
        let mut as_clause: Option<Rc<Expression>> = None;
        if self.consume(Token::As)? {
            as_clause = Some(self.parse_type_expression()?);
        }
        let block = Rc::new(self.parse_block(ParsingDirectiveContext::EnumBlock)?);

        for a in &attributes {
            if a.is_metadata() {
                continue;
            }
            match a {
                Attribute::Public(_) |
                Attribute::Private(_) |
                Attribute::Protected(_) |
                Attribute::Internal(_) => {
                    self.verify_visibility(&a, &context);
                },
                _ => {
                    // Unallowed attribute
                    self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                },
            }
        }

        // Nested classes not allowed
        if !context.is_top_level_or_package() {
            self.add_syntax_error(&name.1, DiagnosticKind::NestedClassesNotAllowed, diagnostic_arguments![]);
        }

        let node = Rc::new(Directive::EnumDefinition(EnumDefinition {
            location: self.pop_location(),
            jetdoc,
            attributes,
            name: name.clone(),
            as_clause,
            block,
        }));

        Ok((node, true))
    }

    fn parse_interface_definition(&mut self, context: AnnotatableContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let AnnotatableContext { start_location, jetdoc, attributes, context, .. } = context;
        self.push_location(&start_location);
        let name = self.expect_identifier(true)?;
        let type_parameters = self.parse_type_parameters_opt()?;
        let mut extends_clause: Option<Vec<Rc<Expression>>> = None;
        if self.consume(Token::Extends)? {
            extends_clause = Some(self.parse_type_expression_list()?);
        }
        let block = Rc::new(self.parse_block(ParsingDirectiveContext::InterfaceBlock)?);

        // Interface block must only contain function definitions
        for directive in block.directives.iter() {
            if !(matches!(directive.as_ref(), Directive::FunctionDefinition(_))) {
                self.add_syntax_error(&directive.location(), DiagnosticKind::DirectiveNotAllowedInInterface, diagnostic_arguments![]);
            }
        }

        for a in &attributes {
            if a.is_metadata() {
                continue;
            }
            match a {
                Attribute::Public(_) |
                Attribute::Private(_) |
                Attribute::Protected(_) |
                Attribute::Internal(_) => {
                    self.verify_visibility(&a, &context);
                },
                _ => {
                    // Unallowed attribute
                    self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                },
            }
        }

        // Nested classes not allowed
        if !context.is_top_level_or_package() {
            self.add_syntax_error(&name.1, DiagnosticKind::NestedClassesNotAllowed, diagnostic_arguments![]);
        }

        let node = Rc::new(Directive::InterfaceDefinition(InterfaceDefinition {
            location: self.pop_location(),
            jetdoc,
            attributes,
            name: name.clone(),
            type_parameters,
            extends_clause,
            block,
        }));

        Ok((node, true))
    }

    fn parse_type_definition(&mut self, context: AnnotatableContext) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        let AnnotatableContext { start_location, jetdoc, attributes, context, .. } = context;
        self.push_location(&start_location);
        let left = self.expect_identifier(true)?;
        self.expect(Token::Assign)?;
        let right: Rc<Expression> = self.parse_type_expression()?;

        for a in &attributes {
            if a.is_metadata() {
                continue;
            }
            match a {
                Attribute::Public(_) |
                Attribute::Private(_) |
                Attribute::Protected(_) |
                Attribute::Internal(_) => {
                    self.verify_visibility(&a, &context);
                },
                _ => {
                    // Unallowed attribute
                    self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
                },
            }
        }

        // Nested classes not allowed
        if !context.is_top_level_or_package() {
            self.add_syntax_error(&left.1, DiagnosticKind::NestedClassesNotAllowed, diagnostic_arguments![]);
        }

        let semicolon = self.parse_semicolon()?;

        let node = Rc::new(Directive::TypeDefinition(TypeDefinition {
            location: self.pop_location(),
            jetdoc,
            attributes,
            left: left.clone(),
            right,
        }));

        Ok((node, semicolon))
    }

    fn parse_type_expression_list(&mut self) -> Result<Vec<Rc<Expression>>, ParsingFailure> {
        let mut list = vec![self.parse_type_expression()?];
        while self.consume(Token::Comma)? {
            list.push(self.parse_type_expression()?);
        }
        Ok(list)
    }

    fn verify_visibility(&self, a: &Attribute, context: &ParsingDirectiveContext) {
        let mut unallowed = false;
        match a {
            Attribute::Public(_) => {},
            Attribute::Private(_) |
            Attribute::Protected(_) => {
                if !context.is_type_block() {
                    unallowed = true;
                }
            },
            Attribute::Internal(_) => {},
            _ => {}
        }
        if unallowed {
            // Unallowed attribute
            self.add_syntax_error(&a.location(), DiagnosticKind::UnallowedAttribute, diagnostic_arguments![]);
        }
    }
    
    fn parse_type_parameters_opt(&mut self) -> Result<Option<Vec<Rc<TypeParameter>>>, ParsingFailure> {
        if !self.consume(Token::Dot)? {
            return Ok(None);
        }
        self.expect(Token::Lt)?;
        let mut list = vec![self.parse_type_parameter()?];
        while self.consume(Token::Comma)? {
            list.push(self.parse_type_parameter()?);
        }
        self.expect_type_parameters_gt()?;
        Ok(Some(list))
    }
    
    fn parse_type_parameter(&mut self) -> Result<Rc<TypeParameter>, ParsingFailure> {
        self.mark_location();
        let name = self.expect_identifier(false)?;
        Ok(Rc::new(TypeParameter {
            location: self.pop_location(),
            name,
        }))
    }

    fn parse_configuration_directive(&mut self, context: ParsingDirectiveContext, start_location: Location) -> Result<(Rc<Directive>, bool), ParsingFailure> {
        self.push_location(&start_location);
        self.expect(Token::LeftBrace)?;
        let subdirective = self.parse_configuration_subdirective(context.clone())?;
        self.expect(Token::RightBrace)?;
        Ok((Rc::new(Directive::ConfigurationDirective(ConfigurationDirective {
            location: self.pop_location(),
            directive: subdirective,
        })), true))
    }

    fn parse_configuration_subdirective(&mut self, context: ParsingDirectiveContext) -> Result<Rc<Directive>, ParsingFailure> {
        if self.peek(Token::If) {
            self.mark_location();
            self.next()?;
            self.expect(Token::LeftParen)?;
            let test = self.parse_configuration_expression()?;
            self.expect(Token::RightParen)?;
            let consequent = Rc::new(Directive::Block(self.parse_block(context.clone())?));
            let mut alternative: Option<Rc<Directive>> = None;
            if self.consume(Token::Else)? {
                alternative = Some(self.parse_configuration_subdirective(context.clone())?);
            }
            Ok(Rc::new(Directive::IfStatement(IfStatement {
                location: self.pop_location(),
                test,
                consequent,
                alternative,
            })))
        } else {
            Ok(Rc::new(Directive::Block(self.parse_block(context.clone())?)))
        }
    }

    fn parse_configuration_expression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        let mut base = self.parse_configuration_primary_expression()?;
        if self.consume(Token::LogicalAnd)? {
            self.push_location(&base.location());
            let right = self.parse_configuration_expression()?;
            base = Rc::new(Expression::Binary(BinaryExpression {
                location: self.pop_location(),
                operator: Operator::LogicalAnd,
                left: base.clone(),
                right,
            }));
        } else if self.consume(Token::LogicalOr)? {
            self.push_location(&base.location());
            let right = self.parse_configuration_expression()?;
            base = Rc::new(Expression::Binary(BinaryExpression {
                location: self.pop_location(),
                operator: Operator::LogicalOr,
                left: base.clone(),
                right,
            }));
        }
        Ok(base)
    }

    fn parse_configuration_primary_expression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
        if let Token::Identifier(id) = &self.token.0.clone() {
            self.mark_location();
            self.next()?;
            let mut id = id.clone();
            if self.consume(Token::ColonColon)? {
                let (id_1, _) = self.expect_identifier(true)?;
                id = id + &"::".to_owned() + &id_1;
            }
            let id_location = self.pop_location();
            let id = Rc::new(Expression::QualifiedIdentifier(QualifiedIdentifier {
                location: id_location.clone(),
                attribute: false,
                qualifier: None,
                id: QualifiedIdentifierIdentifier::Id((id, id_location)),
            }));
            let equality: Option<Operator> = if self.consume(Token::Assign)? {
                Some(Operator::Equals)
            } else if self.consume(Token::NotEquals)? {
                Some(Operator::NotEquals)
            } else {
                None
            };
            if let Some(equality) = equality {
                self.push_location(&id.location());
                self.mark_location();
                let value: String;
                if let Some((value_1, _)) = self.consume_identifier(false)? {
                    value = value_1;
                } else {
                    let Token::StringLiteral(s) = &self.token.0 else {
                        self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedStringLiteral, diagnostic_arguments![Token(self.token.0.clone())]);
                        return Err(ParsingFailure);
                    };
                    value = s.clone();
                    self.next()?;
                }
                let right = Rc::new(Expression::StringLiteral(StringLiteral {
                    location: self.pop_location(),
                    value,
                }));
                Ok(Rc::new(Expression::Binary(BinaryExpression {
                    location: self.pop_location(),
                    operator: equality,
                    left: id.clone(),
                    right,
                })))
            } else {
                Ok(id)
            }
        } else if self.peek(Token::LeftParen) {
            self.mark_location();
            self.next()?;
            let expression = self.parse_configuration_expression()?;
            self.expect(Token::RightParen)?;
            Ok(Rc::new(Expression::Paren(ParenExpression {
                location: self.pop_location(),
                expression,
            })))
        } else if self.peek(Token::Exclamation) {
            self.mark_location();
            self.next()?;
            let expression = self.parse_configuration_primary_expression()?;
            Ok(Rc::new(Expression::Unary(UnaryExpression {
                location: self.pop_location(),
                operator: Operator::LogicalNot,
                expression,
            })))
        } else {
            self.add_syntax_error(&self.token_location(), DiagnosticKind::ExpectedExpression, diagnostic_arguments![Token(self.token.0.clone())]);
            Err(ParsingFailure)
        }
    }

    fn keyword_attribute_from_previous_token(&self) -> Option<Attribute> {
        self.previous_token.0.to_attribute(&self.previous_token.1)
    }

    fn peek_attribute(&self) -> Option<Attribute> {
        self.token.0.to_attribute(&self.token.1)
    }

    fn peek_annotatable_directive_identifier_name(&self) -> bool {
        if self.token.0.to_attribute(&self.token.1).is_some() {
            return true;
        }
        match self.token.0 {
            Token::Identifier(ref name) => {
                if self.token.1.character_count() != name.len() {
                    return false;
                }
                name == "enum" || name == "type"
            },
            Token::Var |
            Token::Const |
            Token::Function |
            Token::Class |
            Token::Interface |
            Token::Use => true,
            _ => false,
        }
    }

    fn parse_attribute_identifier_names(&mut self, context: &mut AnnotatableContext) -> Result<(), ParsingFailure> {
        if context.directive_context_keyword.is_some() {
            return Err(ParsingFailure);
        }
        loop {
            if let Some(a) = self.peek_attribute() {
                let last_attribute_is_identifier = context.attributes.last().map_or(false, |a| !a.is_metadata());
                if last_attribute_is_identifier {
                    self.forbid_line_break_before_token();
                }
                if Attribute::has(&context.attributes, &a) {
                    self.add_syntax_error(&a.location(), DiagnosticKind::DuplicateAttribute, diagnostic_arguments![]);
                }
                if Attribute::is_duplicate_visibility(&context.attributes, &a) {
                    self.add_syntax_error(&a.location(), DiagnosticKind::DuplicateVisibility, diagnostic_arguments![]);
                }
                context.attributes.push(a);
                self.next()?;
            } else {
                if let Some(id) = self.peek_identifier(false)? {
                    self.forbid_line_break_before_token();
                    if ["enum", "type"].contains(&id.0.as_ref()) {
                        self.next()?;
                        context.directive_context_keyword = Some(id);
                    }
                }
                break;
            }
        }
        Ok(())
    }

    fn lookbehind_is_annotatable_directive_identifier_name(&self) -> bool {
        self.keyword_attribute_from_previous_token().is_some()
        || Token::is_context_keyword(&self.previous_token, "enum")
        || Token::is_context_keyword(&self.previous_token, "type")
    }

    pub fn parse_program(&mut self) -> Result<Rc<Program>, ParsingFailure> {
        self.mark_location();
        let mut packages = vec![];
        while self.peek(Token::Package) {
            self.mark_location();
            let jetdoc = self.parse_jetdoc()?;
            self.next()?;
            let mut name = vec![];
            if let Some(name1) = self.consume_identifier(false)? {
                name.push(name1.clone());
                while self.consume(Token::Dot)? {
                    name.push(self.expect_identifier(true)?);
                }
            }
            let block = Rc::new(self.parse_block(ParsingDirectiveContext::PackageBlock)?);
            packages.push(Rc::new(PackageDefinition {
                location: self.pop_location(),
                jetdoc,
                name,
                block,
            }));
        }
        let directives = self.parse_directives(ParsingDirectiveContext::TopLevel)?;
        Ok(Rc::new(Program {
            location: self.pop_location(),
            packages,
            directives,
        }))
    }

    pub fn parse_jetdoc(&mut self) -> Result<Option<Rc<JetDoc>>, ParsingFailure> {
        let comments = self.compilation_unit().comments.borrow();
        let last_comment = comments.last().map(|last_comment| last_comment.clone());
        drop(comments);
        Ok(last_comment.and_then(|comment| {
            if comment.is_jetdoc(&self.token.1) {
                self.compilation_unit().comments_mut().pop();
                let location = comment.location();
                let comment_prefix_length: usize = 3;
                let location = Location::with_lines_and_offsets(self.compilation_unit(), location.first_line_number, location.last_line_number, location.first_offset + comment_prefix_length, location.last_offset - 2);
                let content = &comment.content.borrow()[1..];
                let (main_body, tags) = self.parse_jetdoc_content(&location, content);
                Some(Rc::new(JetDoc {
                    location,
                    main_body,
                    tags,
                }))
            } else {
                None
            }
        }))
    }

    fn parse_jetdoc_content(&mut self, location: &Location, content: &str) -> (Option<(String, Location)>, Vec<(JetDocTag, Location)>) {
        let lines = self.split_jetdoc_lines(location, content);

        let mut main_body: Option<(String, Location)> = None;
        let mut tags: Vec<(JetDocTag, Location)> = vec![];
        let mut i = 0;
        let line_count = lines.len();

        let mut building_content_tag_name: Option<(String, Location)> = None;
        let mut building_content: Vec<(String, Location)> = vec![];
        let mut inside_code_block = false;

        while i < line_count {
            let line = &lines[i];
            let tag = if inside_code_block { None } else {
                regex_captures!(r"^([\s\t]*\@)([^\s\t]+)(.*)", &line.content)
            };
            if let Some((_, tag_prefix, tag_name, tag_content)) = tag {
                self.parse_jetdoc_tag_or_main_body(
                    &mut building_content_tag_name,
                    &mut building_content,
                    &mut main_body,
                    &mut tags,
                );
                if regex_is_match!(r"^[\s\t]*```([^`]|$)", &tag_content) {
                    inside_code_block = true;
                }
                let tag_name_location = Location::with_line_and_offsets(self.compilation_unit(), line.location.first_line_number(), line.location.first_offset() + tag_prefix.len() - 1, line.location.first_offset() + tag_prefix.len() + tag_name.len());
                building_content_tag_name = Some((tag_name.into(), tag_name_location));
                let tag_content_location = Location::with_line_and_offsets(self.compilation_unit(), line.location.first_line_number(), line.location.first_offset() + tag_prefix.len() + tag_name.len(), line.location.last_offset());
                building_content.push((tag_content.into(), tag_content_location));
            } else {
                if regex_is_match!(r"^[\s\t]*```([^`]|$)", &line.content) {
                    inside_code_block = !inside_code_block;
                }
                building_content.push((line.content.clone(), line.location.clone()));
            }
            i += 1;
        }

        self.parse_jetdoc_tag_or_main_body(
            &mut building_content_tag_name,
            &mut building_content,
            &mut main_body,
            &mut tags,
        );

        (main_body, tags)
    }

    fn split_jetdoc_lines(&mut self, location: &Location, content: &str) -> Vec<ParsingJetDocLine> {
        let mut builder = String::new();
        let mut lines = vec![];
        let mut line_number = location.first_line_number();
        let mut index = location.first_offset();
        let mut line_first_offset = index;
        let mut characters = content.chars();
        while let Some(ch) = characters.next() {
            if CharacterValidator::is_line_terminator(ch) {
                lines.push(ParsingJetDocLine {
                    content: builder,
                    location: Location::with_line_and_offsets(self.compilation_unit(), line_number, line_first_offset, index),
                });
                index += ch.len_utf8();
                // <CR><LF> sequence
                if ch == '\r' && characters.clone().next().unwrap_or('\x00') == '\n' {
                    index += '\n'.len_utf8();
                    characters.next();
                }
                builder = String::new();
                line_number += 1;
                line_first_offset = index;
            } else {
                builder.push(ch);
                index += ch.len_utf8();
            }
        }
        lines.push(ParsingJetDocLine {
            content: builder,
            location: Location::with_line_and_offsets(self.compilation_unit(), line_number, line_first_offset, index),
        });
        for line in &mut lines {
            let line_content = line.content.to_owned();
            let prefix = regex_captures!(r"^\s*(\*\s?)?", &line_content);
            if let Some((prefix, _)) = prefix {
                line.content = line.content[prefix.len()..].to_owned();
                line.location = Location::with_line_and_offsets(self.compilation_unit(), line.location.first_line_number(), line.location.first_offset() + prefix.len(), line.location.last_offset());
            }
        }

        lines
    }

    fn parse_jetdoc_tag_or_main_body(
        &self,
        building_content_tag_name: &mut Option<(String, Location)>,
        building_content: &mut Vec<(String, Location)>,
        main_body: &mut Option<(String, Location)>,
        tags: &mut Vec<(JetDocTag, Location)>
    ) {
        if let Some((tag_name, ref tag_location)) = building_content_tag_name.as_ref() {
            match tag_name.as_ref() {
                // @default value
                "default" => {
                    let (reference, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);
                    tags.push((JetDocTag::Default(reference), location));
                },

                // @deprecated
                "deprecated" => {
                    let (text, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);

                    let mut message: Option<String> = None;

                    if !regex_is_match!(r"^\s*$", &text) {
                        message = Some(text.clone());
                    }

                    tags.push((JetDocTag::Deprecated { message }, location));
                },

                // @event eventName description
                "event" => {
                    let (content, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);
                    if let Some((_, name, description)) = regex_captures!(r"(?x) ([^\s]+) (.*)", &content) {
                        tags.push((JetDocTag::Event { name: name.into(), description: description.trim_start().into() }, location));
                    } else {
                        tags.push((JetDocTag::Event { name: content, description: "".into() }, location));
                    }
                },

                // @eventType typeOrConstant
                "eventType" => {
                    let (type_or_constant, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);
                    let compilation_unit_2 = CompilationUnit::new(None, type_or_constant, &self.tokenizer.compilation_unit().compiler_options);
                    if let Some(exp) = ParserFacade::parse_expression(&compilation_unit_2) {
                        tags.push((JetDocTag::EventType(exp), location));
                    } else {
                        self.add_syntax_error(&tag_location, DiagnosticKind::FailedParsingJetDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }
                },

                // @example text
                "example" => {
                    let (text, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);
                    tags.push((JetDocTag::Example(text), location));
                },

                // @internal text
                "internal" => {
                    let (text, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);

                    // Content must be non empty
                    if regex_is_match!(r"^\s*$", &text) {
                        self.add_syntax_error(&tag_location, DiagnosticKind::FailedParsingJetDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }

                    tags.push((JetDocTag::Internal(text), location));
                },

                // @param paramName description
                "param" => {
                    let (content, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);

                    if let Some((_, name, description)) = regex_captures!(r"(?x) ([^\s]+) (.*)", &content) {
                        tags.push((JetDocTag::Param { name: name.into(), description: description.trim_start().into() }, location));
                    } else {
                        tags.push((JetDocTag::Param { name: content, description: "".into() }, location));
                    }
                },

                // @private
                "private" => {
                    let (text, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);

                    // Content must be empty
                    if !regex_is_match!(r"^\s*$", &text) {
                        self.add_syntax_error(&tag_location, DiagnosticKind::FailedParsingJetDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }

                    tags.push((JetDocTag::Private, location));
                },

                // @return text
                "return" => {
                    let (text, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);
                    tags.push((JetDocTag::Return(text), location));
                },

                // @see reference [displayText]
                "see" => {
                    let (content, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);
                    let reference: String;
                    let display_text: Option<String>;
                    if let Some((_, reference_1, display_text_1)) = regex_captures!(r"(?x) ([^\s]+) (.*)", &content) {
                        reference = reference_1.to_owned();
                        display_text = Some(display_text_1.trim().to_owned());
                    } else {
                        reference = content;
                        display_text = None;
                    }
                    if let Some(reference) = self.parse_jetdoc_reference(&reference, &tag_location, &tag_name) {
                        tags.push((JetDocTag::See { reference, display_text }, location));
                    }
                },

                // @throws className description
                "throws" => {
                    let (class_name_and_description, location) = join_jetdoc_content(building_content);
                    let location = tag_location.combine_with(location);

                    let class_name_and_description = regex_captures!(r"^([^\s]+)(\s.*)?", &class_name_and_description);

                    if let Some((_, class_name, description)) = class_name_and_description {
                        let description = description.trim().to_owned();
                        let description = if description.is_empty() {
                            None
                        } else {
                            Some(description)
                        };
                        let compilation_unit_2 = CompilationUnit::new(None, class_name.into(), &self.tokenizer.compilation_unit().compiler_options);
                        if let Some(exp) = ParserFacade::parse_type_expression(&compilation_unit_2) {
                            tags.push((JetDocTag::Throws { class_reference: exp, description }, location));
                        } else {
                            self.add_syntax_error(&tag_location, DiagnosticKind::FailedParsingJetDocTag, diagnostic_arguments![String(tag_name.clone())]);
                        }
                    } else {
                        self.add_syntax_error(&tag_location, DiagnosticKind::FailedParsingJetDocTag, diagnostic_arguments![String(tag_name.clone())]);
                    }
                },

                // Unrecognized tag
                _ => {
                    self.add_syntax_error(&tag_location, DiagnosticKind::UnrecognizedJetDocTag, diagnostic_arguments![String(tag_name.clone())]);
                },
            }
        } else if !building_content.is_empty() {
            *main_body = Some(join_jetdoc_content(building_content));
        }

        *building_content_tag_name = None;
        building_content.clear();
    }

    fn parse_jetdoc_reference(&self, reference: &str, tag_location: &Location, tag_name: &str) -> Option<Rc<JetDocReference>> {
        let split: Vec<&str> = reference.split("#").collect();
        if split.len() > 2 {
            self.add_syntax_error(&tag_location, DiagnosticKind::FailedParsingJetDocTag, diagnostic_arguments![String(tag_name.to_owned())]);
            return None;
        }
        let mut base: Option<Rc<Expression>> = None;
        let instance_property: Option<String> = split.get(1).and_then(|&f| if f.is_empty() { None } else { Some(f.to_owned()) });
        let base_text: String = split[0].to_owned();

        if !base_text.is_empty() {
            let compilation_unit_2 = CompilationUnit::new(None, base_text, &self.tokenizer.compilation_unit().compiler_options);
            if let Some(exp) = ParserFacade::parse_expression(&compilation_unit_2) {
                base = Some(exp);
            } else {
                self.add_syntax_error(&tag_location, DiagnosticKind::FailedParsingJetDocTag, diagnostic_arguments![String(tag_name.to_owned())]);
                return None;
            }
        }

        if base.is_none() && instance_property.is_none() {
            self.add_syntax_error(&tag_location, DiagnosticKind::FailedParsingJetDocTag, diagnostic_arguments![String(tag_name.to_owned())]);
            return None;
        }
        Some(Rc::new(JetDocReference { base, instance_property, }))
    }
}

fn join_jetdoc_content(content: &Vec<(String, Location)>) -> (String, Location) {
    let s: Vec<String> = content.iter().map(|c| c.0.clone()).collect();
    let s = s.join("\n").trim().to_owned();
    let location = content.first().unwrap().1.combine_with(content.last().unwrap().1.clone());
    (s, location)
}

struct ParsingJetDocLine {
    content: String,
    location: Location,
}

#[derive(Clone)]
struct ParsingActivation {
    uses_yield: bool,
    uses_await: bool,
}

impl ParsingActivation {
    pub fn new() -> Self {
        Self {
            uses_yield: false,
            uses_await: false,
        }
    }
}

#[derive(Clone)]
struct AnnotatableContext {
    start_location: Location,
    jetdoc: Option<Rc<JetDoc>>,
    attributes: Vec<Attribute>,
    context: ParsingDirectiveContext,
    /// Previous token as a directive context keyword.
    directive_context_keyword: Option<(String, Location)>,
}

impl AnnotatableContext {
    pub fn has_directive_context_keyword(&self, name: &str) -> bool {
        if let Some((ref k, _)) = self.directive_context_keyword {
            k == name
        } else {
            false
        }
    }
}

pub struct ParserFacade;

impl ParserFacade {
    /// Parses `Program` until end-of-file.
    pub fn parse_program(compilation_unit: &Rc<CompilationUnit>) -> Option<Rc<Program>> {
        let mut parser = Parser::new(compilation_unit);
        if parser.next().is_ok() {
            let program = parser.parse_program().ok();
            if compilation_unit.invalidated() { None } else { program }
        } else {
            None
        }
    }

    /// Parses `ListExpression^allowIn` and expects end-of-file.
    pub fn parse_expression(compilation_unit: &Rc<CompilationUnit>) -> Option<Rc<Expression>> {
        let mut parser = Parser::new(compilation_unit);
        if parser.next().is_ok() {
            let exp = parser.parse_expression(ParsingExpressionContext {
                ..default()
            }).ok();
            if exp.is_some() {
                let _ = parser.expect_eof();
            }
            if compilation_unit.invalidated() { None } else { exp }
        } else {
            None
        }
    }

    /// Parses `TypeExpression` and expects end-of-file.
    pub fn parse_type_expression(compilation_unit: &Rc<CompilationUnit>) -> Option<Rc<Expression>> {
        let mut parser = Parser::new(compilation_unit);
        if parser.next().is_ok() {
            let exp = parser.parse_type_expression().ok();
            if exp.is_some() {
                let _ = parser.expect_eof();
            }
            if compilation_unit.invalidated() { None } else { exp }
        } else {
            None
        }
    }

    /// Parses `Directives` until end-of-file.
    pub fn parse_directives(compilation_unit: &Rc<CompilationUnit>, context: ParsingDirectiveContext) -> Option<Vec<Rc<Directive>>> {
        let mut parser = Parser::new(compilation_unit);
        if parser.next().is_ok() {
            let directives = parser.parse_directives(context).ok();
            if compilation_unit.invalidated() { None } else { directives }
        } else {
            None
        }
    }
}