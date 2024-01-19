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
  * [ ] Parse typed destructuring by reusing *ObjectInitializer* or *ArrayLiteral* and allowing *NonNull* followed by optional type annotation
* Type expressions
  * [ ] `?T`
    * Results into `NullableTypeExpression`
  * [ ] `*`
  * [ ] `id`
  * [ ] `void`
  * [ ] `[...]`
    * [ ] `[T]`
    * [ ] `[T1, T2, ...]`
  * [ ] `(x)` parenthesized
  * [ ] `function(...): T` type expression
  * [ ] `o.x`
  * [ ] `T.<...>`
  * [ ] `T?`
    * Results into `NullableTypeExpression`
  * [ ] `T!`
    * Results into `NonNullableTypeExpression`
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
    * [ ] Resolve ambiguity with expression statement beginning with `import.meta` expression
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