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
  * [x] `a ? b : c`
  * [x] `yield`
  * [x] *FunctionExpression*
  * [x] `x = y`
  * [x] `{...} = v` or `[...] = v`
  * [x] `x [CompoundAssignment] v`
  * [x] `x, y`
* Type expressions
  * [x] `?T` (`NullableTypeExpression`)
  * [x] `*`
  * [x] `id`
  * [x] `void`
  * [x] `[...]`
    * [x] `[T]`
    * [x] `[T1, T2, ...]`
  * [x] `(x)`
  * [x] `o.x`
  * [x] `T.<...>`
  * [x] `T?` (`NullableTypeExpression`)
  * [x] `T!` (`NonNullableTypeExpression`)
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
  * [x] `continue [label]`
  * [x] `break [label]`
  * [x] `try`
  * [x] `with`
  * [x] `return [v]`
  * [x] `throw e`
  * [x] `default xml namespace = ns`
* Attributes
  * [x] Attributes structure
* Directives
  * [x] `configuration` directive
  * [x] `import`
  * [x] `use`
* Definitions
  * [x] Variable definition
  * [x] Function definition
    * [x] Getter
    * [x] Setter
    * [x] Proxy
    * [x] Constructor
  * [x] `class`
  * [x] `enum`
  * [x] `interface`
  * [x] `type`
  * [x] `package`
  * [x] Program