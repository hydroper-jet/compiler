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
        self.next()?;
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
        } else if self.peek(Token::Return) {
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
        let variable_kind = left.kind.0.clone();
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
            if self.peek_annotatable_directive_identifier_name() && self.lookbehind_is_annotatable_directive_identifier_name() {
                if ["enum", "type"].contains(&id.0.as_ref()) && id.1.character_count() == id.0.len() {
                    let mut context = AnnotatableContext {
                        start_location: id.1.clone(),
                        jetdoc,
                        attributes: vec![],
                        context: context.clone(),
                        directive_context_keyword: Some(id.clone()),
                    };
                    // self.parse_attribute_identifier_names(&mut context)?;
                } else {
                    let mut context = AnnotatableContext {
                        start_location: id.1.clone(),
                        jetdoc,
                        attributes: vec![self.keyword_attribute_from_previous_token().unwrap()],
                        context: context.clone(),
                        directive_context_keyword: None,
                    };
                    self.parse_attribute_identifier_names(&mut context)?;
                }
                return self.parse_annotatable_directive(&context)?;
            } else if self.peek(Token::LeftBrace) && &id.0 == "configuration" && id.1.character_count() == "configuration".len() {
                self.parse_configuration_directive(context, id.1)
            } else {
                self.parse_statement_starting_with_identifier(context, id)
            }
        } else if self.peek(Token::Import) {
            self.parse_import_directive_or_expression_statement(context)
        } else if self.peek(Token::LeftBrace) {
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
                    return self.parse_annotatable_directive(&context)?;
                }
            }
            let semicolon_inserted = self.parse_semicolon()?;
            Ok((Rc::new(Directive::ExpressionStatement(ExpressionStatement {
                location: self.pop_location(),
                expression: exp,
            })), semicolon_inserted))
        } else if self.peek(Token::Public) || self.peek(Token::Private) || self.peek(Token::Protected)
        || self.peek(Token::Internal) || self.peek(Token::Var) || self.peek(Token::Const)
        || self.peek(Token::Function) || self.peek(Token::Class) || self.peek(Token::Interface)
        || self.peek(Token::Use) {
            let mut context = AnnotatableContext {
                start_location: self.pop_location(),
                jetdoc,
                attributes: vec![],
                context: context.clone(),
                directive_context_keyword: None,
            };
            self.parse_attribute_identifier_names(&mut context)?;
            return self.parse_annotatable_directive(&context)?;
        } else {
            self.parse_statement(context)
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
                if let Some((value_1, location)) = self.consume_identifier(false)? {
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

    fn consume_attribute_public_private_protected_internal(&mut self) -> Result<Option<Attribute>, ParsingFailure> {
        if let Some(a) = self.peek_attribute_public_private_protected_internal() {
            self.next()?;
            Ok(Some(a))
        } else {
            Ok(None)
        }
    }

    fn peek_attribute_public_private_protected_internal(&self) -> Option<Attribute> {
        match self.token.0 {
            Token::Public => Some(Attribute::Public(self.token.1.clone())),
            Token::Private => Some(Attribute::Private(self.token.1.clone())),
            Token::Protected => Some(Attribute::Protected(self.token.1.clone())),
            Token::Internal => Some(Attribute::Internal(self.token.1.clone())),
            _ => None,
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
        || Token::is_context_keyword(self.previous_token, "enum")
        || Token::is_context_keyword(self.previous_token, "type")
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
    start_location: Location,
    jetdoc: Option<Rc<JetDoc>>,
    attributes: Vec<Attribute>,
    context: ParsingDirectiveContext,
    /// Previous token as a directive context keyword.
    directive_context_keyword: Option<(String, Location)>,
}