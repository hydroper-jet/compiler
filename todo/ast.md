# AST

## Changes

* Now, for the Jet parser, different from `as3_parser`, the AST will mix expressions and type expressions. Few type expressions will have dedicated expression nodes, such as the `void` type expression and function type expressions.
* Different from `as3_parser`, destructuring now reuses the same nodes from object initializer and array literal.

## Covered

* Expressions
  * [x] `*` (qualified identifier)
  * [x] Initial identifier expression
    * [x] `id`
    * [x] `q::x` (qualified identifier)
    * [x] `embed "path/to/file" ...`
      * [x] `from` postfix (limit identifier name to `outputDirectory`)
      * [x] `as` postfix
  * [x] Parentheses
    * [x] `()`
      * Produces an internal empty parentheses expression (*ArrowEmptyParameters*) used for arrow functions.
    * [x] `(x)`
    * [x] `(q)::x` (qualified identifier)
  * [x] `@id`
    * Attribute qualified identifier.
  * [x] *UndefinedLiteral*
  * [x] `true`
  * [x] `false`
  * [x] *NumericLiteral* (the value is kept as a raw string for context type flexibility)
  * [x] *StringLiteral*
  * [x] `this`
  * [x] *RegularExpressionLiteral* (initiates with either `/` or `/=`)
  * [x] *XMLInitializer*
  * [x] *ArrayInitializer*
  * [x] *ObjectInitializer*
  * [x] *FunctionExpression*
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
  * [x] `v: T` lookahead != `:` (*ExpressionWithTypeAnnotation*, used internally for arrow function signatures)
    * One precedence less than unary expressions and one precedence greater than binary expressions
  * [x] *BinaryExpression*
  * [x] `a ? b : c`
  * [x] `yield`
  * [x] `... => body`
    * Refines the left side into an *ArrowSignature* and parses an arrow function with an activation context.
  * [x] `x = y`
  * [x] `{...} = v` or `[...] = v`
  * [x] `x [CompoundAssignment] v`
  * [x] `...x` as *Rest* (used internally for arrays and arrow function signatures)
    * Same precedence as assignment
  * [x] `x, y`
* Type expressions
  * [x] `?T`
  * [x] `*`
  * [x] `id`
  * [x] `void`
  * [x] `undefined`
  * [x] `[...]`
    * [x] `[T]`
    * [x] `[T1, T2, ...]`
  * [x] `(...)`
    * [x] `(x)`
    * [x] `() => T`
    * [x] `(a: T) => T`
    * [x] `(a: T, ...) => T`
    * [x] `(a?: T, ...) => T`
    * [x] `(...a: [T]) => T`
    * [x] `(...) => T`
  * [x] `o.x`
  * [x] `T.<...>`
  * [x] `T?`
* Statements
  * [x] `;`
  * [x] Expression statement
  * [x] `super()`
  * [x] Block
  * [x] Labeled statement
  * [x] `if`
  * [x] `switch`
  * [x] `switch type`
  * [x] `do`
  * [x] `while`
  * [x] `for`
  * [x] `for..in`
  * [x] `for each`
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