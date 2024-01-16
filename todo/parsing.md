# Parsing

## Covered

* Expressions
  * [ ] `*` (qualified identifier)
  * [ ] Initial identifier expression
    * [ ] `id`
    * [ ] `q::x` (qualified identifier)
    * [ ] `embed ObjectInitializer`
  * [ ] Parentheses
    * [ ] `()`
      * Produces an internal empty parentheses expression (*ArrowEmptyParameters*) used for arrow functions.
    * [ ] `(x)`
    * [ ] `(q)::x` (qualified identifier)
  * [ ] `@id`
    * Attribute qualified identifier.
  * [ ] *UndefinedLiteral*
  * [ ] `true`
  * [ ] `false`
  * [ ] *NumericLiteral* (the value is kept as a raw string for context type flexibility)
  * [ ] *StringLiteral*
  * [ ] `this`
  * [ ] *RegularExpressionLiteral* (initiates with either `/` or `/=`)
  * [ ] *XMLInitializer*
  * [ ] *ArrayInitializer*
  * [ ] *ObjectInitializer*
  * [ ] *FunctionExpression*
  * [ ] *PostfixExpression*
    * [ ] `import.meta`
    * [ ] New expression
    * [ ] `o.x`
    * [ ] `o..x` (descendants)
    * [ ] `o.(condition)`
    * [ ] `o.(q)::x` (qualified identifier)
    * [ ] `o[k]`
    * [ ] *SuperExpression* followed by *PropertyOperator* (**.** *QualifiedIdentifier* or *Brackets*)
    * [ ] `o!`
    * [ ] `o(...)`
    * [ ] `o.<...>`
    * [ ] `x++` (no line break in between)
    * [ ] `x--` (no line break in between)
    * [ ] `x` followed by optional chaining (`?.`)
      * Refine: when `?.(...)` is followed by `::` and the expression in the left parentheses is not a list expression, refine the optional chaining as a property operator.
  * [ ] *UnaryExpression*
    * [ ] `delete o.x`
    * [ ] `void v`
    * [ ] `await v`
    * [ ] `typeof v`
    * [ ] `++x`
    * [ ] `--x`
    * [ ] `+v`
    * [ ] `-v`
    * [ ] `~v`
    * [ ] `!v`
  * [ ] `v: T` lookahead != `:` (*ExpressionWithTypeAnnotation*, used internally for arrow function signatures)
    * One precedence less than unary expressions and one precedence greater than binary expressions
  * [ ] *BinaryExpression*
    * [ ] `a ** b`
    * [ ] `a * b`
    * [ ] `a / b`
    * [ ] `a % b`
    * [ ] `a + b`
    * [ ] `a - b`
    * [ ] `a << b`
    * [ ] `a >> b`
    * [ ] `a >>> b`
    * [ ] `a < b`
    * [ ] `a > b`
    * [ ] `a <= b`
    * [ ] `a >= b`
    * [ ] `v as T`
    * [ ] `v is T`
    * [ ] `v is not T`
    * [ ] `k in v`
    * [ ] `k not in v`
    * [ ] `a == b`
    * [ ] `a != b`
    * [ ] `a === b`
    * [ ] `a !== b`
    * [ ] `a & b`
    * [ ] `a ^ b`
    * [ ] `a | b`
    * [ ] `a && b`
    * [ ] `a ^^ b`
    * [ ] `a || b`
    * [ ] `a ?? b`
  * [ ] `a ? b : c`
    * [ ] `b` is a `with_type_annotation = false` context
  * [ ] `yield`
  * [ ] `... => body`
    * Refines the left side into an *ArrowSignature* and parses an arrow function with an activation context.
  * [ ] `x = y`
  * [ ] `{...} = v` or `[...] = v`
  * [ ] `x [CompoundAssignment] v`
  * [ ] `...x` as *Rest* (used internally for arrays and arrow function signatures)
    * Same precedence as assignment
  * [ ] `x, y`
* Destructuring
  * [ ] Parse typed destructuring by reusing *ObjectInitializer* or *ArrayLiteral* and allowing *NonNull* followed by optional type annotation
* Type expressions
  * [ ] `?T`
  * [ ] `*`
  * [ ] `id`
  * [ ] `void`
  * [ ] `undefined`
  * [ ] `[...]`
    * [ ] `[T]`
    * [ ] `[T1, T2, ...]`
  * [ ] `(...)`
    * [ ] `(x)`
    * [ ] `() => T`
    * [ ] `(a: T) => T`
    * [ ] `(a: T, ...) => T`
    * [ ] `(a?: T, ...) => T`
    * [ ] `(...a: [T]) => T`
    * [ ] `(...) => T`
  * [ ] `o.x`
  * [ ] `T.<...>`
  * [ ] `T?`
* Statements
  * [ ] `;`
  * [ ] Expression statement
    * [ ] Resolve ambiguity with labeled statement
    * [ ] Resolve ambiguity with super statement (`super` followed by *PropertyOperator* or `super()` followed by *PropertyOperator*)
  * [ ] `super()`
    * [ ] Resolve ambiguity with expression statement (`super` followed by *PropertyOperator* or `super()` followed by *PropertyOperator*)
  * [ ] Block
  * [ ] Labeled statement
    * [ ] Resolve ambiguity with expression statement
  * [ ] `if`
  * [ ] `switch`
  * [ ] `switch type`
  * [ ] `do`
  * [ ] `while`
  * [ ] `for`
  * [ ] `for..in`
  * [ ] `for each`
  * [ ] `continue [label]`
  * [ ] `break [label]`
  * [ ] `try`
  * [ ] `with`
  * [ ] `return [v]`
  * [ ] `throw e`
  * [ ] `default xml namespace = ns`
* Attributes
  * [ ] Attributes structure
* Directives
  * [ ] `configuration` directive
    * [ ] Parse in parallel with expression statement (if identifier is `configuration` and followed by `{`)
  * [ ] `import`
  * [ ] `use`
* Configuration expressions
  * [ ] Configuration expressions
    * [ ] Translate `=` into `==`
    * [ ] Translate right identifier in `=` or `!=` to a *StringLiteral*
* Definitions
  * [ ] Variable definition
  * [ ] Function definition
    * [ ] Getter
    * [ ] Setter
    * [ ] Proxy
    * [ ] Constructor
  * [ ] `class`
  * [ ] `enum`
  * [ ] `interface`
  * [ ] `type`
  * [ ] `package`
  * [ ] Program