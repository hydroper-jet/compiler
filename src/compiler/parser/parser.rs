use crate::ns::*;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

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

    fn consume_context_keyword(&mut self, name: &str) -> Result<bool, ParsingFailure> {
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
    fn expect_generics_gt(&mut self) -> Result<(), ParsingFailure> {
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
                let identifier = self.finish_qualified_identifier(false, self.pop_location(), arguments[0].clone())?;
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
            self.expect_generics_gt()?;
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
                let id = self.finish_qualified_identifier(false, self.pop_location(), id)?;
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
        } else if let Token::NumericLiteral(n) = self.token.0 {
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
                let id = self.finish_qualified_identifier(false, self.pop_location(), id)?;
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
            Some(FunctionBody::Block(self.parse_block(block_context)?))
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
        let binding: Rc<VariableBinding> = self.parse_variable_binding(true)?;
        let has_initializer = binding.init.is_some();
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
                let id = self.finish_qualified_identifier(false, self.pop_location(), id)?;
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
        } else if let Token::NumericLiteral(n) = self.token.0 {
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
                let id = self.finish_qualified_identifier(false, self.pop_location(), id)?;
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
        self.next()?;
        let Expression::ObjectInitializer(descriptor) = self.parse_object_initializer()?.as_ref();
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
            let id = self.finish_qualified_identifier(false, self.pop_location(), left)?;
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
                return self.finish_qualified_identifier(attribute, self.pop_location(), id);
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
            return self.finish_qualified_identifier(attribute, self.pop_location(), qual);
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
            self.next()?;
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

    fn parse_type_expression(&mut self) -> Result<Rc<Expression>, ParsingFailure> {
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
                    self.expect_generics_gt()?;
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
        self.validate_parameter_list(&parameters);

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
    start: Location,
    metadata_exp: Vec<Rc<Expression>>,
    asdoc: Option<Rc<JetDoc>>,
    first_modifier: Option<Rc<Expression>>,
    previous_token_is_definition_keyword: bool,
    context: ParsingDirectiveContext,
}