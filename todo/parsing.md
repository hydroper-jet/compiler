# Parsing

## Covered

* Expressions
  * [x] `*` (qualified identifier)
  * [x] Initial identifier expression
    * [x] `id`
    * [x] `q::x` (qualified identifier)
    * [x] `embed ObjectInitializer`
  * [x] Parentheses
    * [x] `(x)`
    * [x] `(q)::x` (qualified identifier)
  * [x] `@id`
    * Attribute qualified identifier.
  * [x] *NullLiteral*
  * [x] `true`
  * [x] `false`
  * [x] *NumericLiteral* (the value is kept as a raw string for context type flexibility)
  * [x] *StringLiteral*
  * [x] `this`
  * [x] *RegularExpressionLiteral* (initiates with either `/` or `/=`)
  * [x] *XMLInitializer*
  * [x] *ArrayInitializer*
  * [x] *ObjectInitializer*
  * [x] *PostfixExpression*
    * [x] `import.meta`
    * [x] New expression
    * [x] `o.x`
    * [x] `o..x` (descendants)
    * [x] `o.(condition)`
    * [x] `o.(q)::x` (qualified identifier)
    * [x] `o[k]`
    * [x] *SuperExpression* followed by *PropertyOperator* (**.** *QualifiedIdentifier* or *Brackets*)
    * [x] `o!`
    * [x] `o(...)`
    * [x] `o.<...>`
    * [x] `x++` (no line break in between)
    * [x] `x--` (no line break in between)
    * [x] `x` followed by optional chaining (`?.`)
      * Refine: when `?.(...)` is followed by `::` and the expression in the left parentheses is not a list expression, refine the optional chaining as a property operator.
  * [x] *UnaryExpression*
    * [x] `delete o.x`
    * [x] `void v`
    * [x] `await v`
    * [x] `typeof v`
    * [x] `++x`
    * [x] `--x`
    * [x] `+v`
    * [x] `-v`
    * [x] `~v`
    * [x] `!v`
  * [x] *BinaryExpression*
    * [x] `a ** b`
    * [x] `a * b`
    * [x] `a / b`
    * [x] `a % b`
    * [x] `a + b`
    * [x] `a - b`
    * [x] `a << b`
    * [x] `a >> b`
    * [x] `a >>> b`
    * [x] `a < b`
    * [x] `a > b`
    * [x] `a <= b`
    * [x] `a >= b`
    * [x] `v as T`
    * [x] `v is T`
    * [x] `v is not T`
    * [x] `k in v`
    * [x] `k not in v`
    * [x] `a == b`
    * [x] `a != b`
    * [x] `a === b`
    * [x] `a !== b`
    * [x] `a & b`
    * [x] `a ^ b`
    * [x] `a | b`
    * [x] `a && b`
    * [x] `a ^^ b`
    * [x] `a || b`
    * [x] `a ?? b`
  * [x] `a ? b : c`
  * [x] `yield`
  * [x] *FunctionExpression*
  * [x] `x = y`
  * [x] `{...} = v` or `[...] = v`
  * [x] `x [CompoundAssignment] v`
  * [x] `x, y`
* Destructuring
  * [x] Parse typed destructuring by reusing *ObjectInitializer* or *ArrayLiteral* and allowing *NonNull* followed by optional type annotation
* Type expressions
  * [x] `?T`
    * Results into `NullableTypeExpression`
  * [x] `*`
  * [x] `id`
  * [x] `void`
  * [x] `[...]`
    * [x] `[T]`
    * [x] `[T1, T2, ...]`
  * [x] `(x)` parenthesized
  * [x] `function(...): T` type expression
  * [x] `o.x`
  * [x] `T.<...>`
  * [x] `T?`
    * Results into `NullableTypeExpression`
  * [x] `T!`
    * Results into `NonNullableTypeExpression`
* Statements
  * [x] `;`
  * [x] Expression statement
    * [x] Resolve ambiguity with labeled statement
    * [x] Resolve ambiguity with super statement (`super` followed by *PropertyOperator* or `super()` followed by *PropertyOperator*)
  * [x] `super()`
    * [x] Resolve ambiguity with expression statement (`super` followed by *PropertyOperator* or `super()` followed by *PropertyOperator*)
  * [x] Block
  * [x] Labeled statement
    * [x] Resolve ambiguity with expression statement
  * [x] `if`
  * [x] `switch`
  * [x] `switch type`
  * [x] `do`
  * [x] `while`
  * [x] `for`
  * [x] `for..in`
  * [x] `for each`
  * [x] `continue [label]`
  * [x] `break [label]`
  * [x] `try`
  * [x] `with`
  * [x] `return [v]`
  * [x] `throw e`
  * [x] `default xml namespace = ns`
* Attributes
  * [x] `Expression::to_metadata`
  * [x] `parse_attribute_identifier_names`
  * [x] `peek_annotatable_directive_identifier_name`
  * [x] `peek_attribute_public_private_protected_internal`
  * [x] `consume_attribute_public_private_protected_internal`
    * `public`, `private`, `protected`, `internal`
  * [x] `keyword_attribute_from_previous_token`
    * [x] `public`, `private`, `protected`, `internal`
    * [x] `proxy`,  `final`,  `native`,  `static`,  `abstract`,  `override`
  * [x] Parse annotatable directives starting with meta-data followed by a annotatable directive identifier name
    * [x] Call `parse_attribute_identifier_names` to consume remaining atributes
  * [x] Parse annotatable directives starting with a reserved word
    * [x] Call `parse_attribute_identifier_names` to consume remaining atributes
  * [x] Parse annotatable directives starting with an identifier followed by a annotatable directive identifier name
    * [x] Call `parse_attribute_identifier_names` to consume remaining atributes
* Annotatable directives
  * [ ] `parse_annotatable_directive`
* Directives
  * [x] `configuration` directive
    * [x] Parse in parallel with expression statement (if identifier is `configuration` and followed by `{`)
  * [x] `import`
    * [x] Resolve ambiguity with expression statement beginning with `import.meta` expression
  * [ ] `use`
    * [ ] Parse it together with other definitions since it is annotatable.
* Configuration expressions
  * [x] Configuration expressions
    * [x] Translate `=` into `==`
    * [x] Translate right identifier in `=` or `!=` to a *StringLiteral*
* Definitions
  * [ ] Variable definition
    * [ ] Attribute validation
    * [ ] Within `enum` block, if the definition has no `static` attribute, prohibit destructuring and type annotation on bindings.
  * [ ] Function definition
    * [ ] Attribute validation
    * [ ] Verify body according to `native` attribute
    * [ ] Getter
    * [ ] Setter
    * [ ] Proxy
    * [ ] Constructor
  * [ ] `class`
    * [ ] Attribute validation
    * [ ] Allowed only at package block and top-level
  * [ ] `enum`
    * [ ] Attribute validation
    * [ ] Allowed only at package block and top-level
  * [ ] `interface`
    * [ ] Attribute validation
    * [ ] Allowed only at package block and top-level
  * [ ] `type`
    * [ ] Attribute validation
    * [ ] Allowed only at package block and top-level
  * [ ] `package`
  * [ ] Program