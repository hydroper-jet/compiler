# Parsing

## New parsing challenges

* Error recovery
  * https://langdev.stackexchange.com/q/3311/606

## Covered

* Expressions
  * [ ] `*`
  * [ ] Initial identifier expression
    * [ ] `id`
    * [ ] `q::x` (qualified identifier)
    * [ ] `q`
    * [ ] `embed "path/to/file" ...`
      * [ ] `from` postfix (limit identifier name to `outputDirectory`)
      * [ ] `as` postfix
  * [ ] Parentheses
    * [ ] `()`
      * Produces an internal empty parentheses expression used for arrow functions.
    * [ ] `(x)`
    * [ ] `(q)::x` (qualified identifier)
  * [ ] `@id`
    * Attribute qualified identifier.
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
    * [ ] `v instanceof T`
    * [ ] `v not instanceof T`
    * [ ] `v is T`
    * [ ] `v is not T`
    * [ ] `k in v`
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
  * [ ] `yield`
  * [ ] `... => body`
    * Refines the left side into an *ArrowSignature* and parses an arrow function with an activation context.
  * [ ] `x = y`
  * [ ] `{...} = v` or `[...] = v`
  * [ ] `x [CompoundAssignment] v`
  * [ ] `x, y`
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
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
* Directives
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
* Configuration expressions
  * [ ] Configuration expressions
* Definitions
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`
  * [ ] `x`