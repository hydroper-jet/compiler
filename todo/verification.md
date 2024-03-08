# Verification

## AST

* Expressions
  * Array literal
    * [x] Result into `enum` constant wherever possible (if the array literal is constant)
  * Object initializer
    * [x] Result into `enum` constant wherever possible (if the initializer is constant)
  * Function expression
    * [ ] Cache activation scope in the function expression node before verifying the signature. This prevents wrong diagnostic reports.
  * Unary expressions
    * [ ] Result into constant every wherever possible
    * [ ] Negative operator passes `context.preceded_by_negative = true`
    * [ ] Delete operator verifies base with `context.mode = delete`
  * Binary expressions
    * [ ] Result into constant every wherever possible
  * Assignment expressions
    * [ ] Non destructuring assignment verifies left-hand side with `context.mode = write`
  * Member expressions
    * [ ] Do not forget to pass `disamb` argument to property resolution
    * [ ] Fully qualified names shadow any other variables
  * Expressions with type arguments
    * [ ] Base's context is passed `followed_by_type_arguments = true`

* Destructuring
  * IdentifierName
    * [ ] Declarative
      * [ ] Skip if already assigned a symbol or `None` at `ast_to_symbol`.
      * [ ] Set parent definition of variable
      * [ ] Variable shadowing in activations
    * [ ] Assignment
  * Object
    * [ ] Declarative
      * [ ] Skip if already assigned a symbol or `None` at `ast_to_symbol`.
      * [ ] Set parent definition of variable
      * [ ] Variable shadowing in activations for a declarative field
    * [ ] Assignment
  * Array
    * [ ] Declarative
      * [ ] Skip if already assigned a symbol or `None` at `ast_to_symbol`.
    * [ ] Assignment
  * Non-null
    * [ ] Declarative
      * [ ] Skip if already assigned a symbol or `None` at `ast_to_symbol`.
    * [ ] Assignment
* Typed destructuring
  * [ ] `verify_typed_destructuring(&tdst, read_only, &properties_destination, &parent_definition, visibility, Some(context_type)): Result<(), DeferVerificationError>`

* Statements
  * Block
    * [ ] Parse plain meta-data

* Program
  * [ ] Create and attach activation scope (contains function whose signature is `function(): void`)

## Duplicate errors

* [ ] Variable bindings report duplicate error
* [ ] Functions report duplicate error
* [ ] Constructors report duplicate error
* [ ] Proxies report duplicate error
* [ ] Getter reports duplicate error if not overriding
* [ ] Setter reports duplicate error if not overriding
* [ ] Classes report duplicate error
* [ ] Enumerations report duplicate error
* [ ] Interfaces report duplicate error
* [ ] `type` definitions report duplicate error

## Plain meta-data

* [ ] Process plain meta-data in definitions
* [ ] Process plain meta-data in blocks by assigning to a `BlockStatementSymbol`

## Function definitions

* [ ] Invoke `symbol.set_activation_scope(Some(activation_scope))` for all non `abstract` and non `native` functions.
* [ ] Invoke `symbol.set_is_constructor(true)` for constructor definitions.
* [ ] Invoke `symbol.set_of_virtual_property(p)` appropriately for getters and setters.
* [ ] Invoke `symbol.set_name(name)` for getters and setters (required).
* [ ] Invoke `symbol.set_is_optional_interface_method()` for regular methods.
* [ ] Invoke `symbol.set_is_optional_interface_method()` for getters.
* [ ] Invoke `symbol.set_is_optional_interface_method()` for setters.

## Function common

* Map *FunctionCommon* to respective function symbol
  * [ ] In function expressions
  * [ ] In function definitions

## Captured properties

* [ ] Do the following after successful property resolutions:

```rust
current_scope.check_property_has_capture(&reference_value);
```

## Enumeration definitions

* [ ] Lazily initialize special methods (normal and set enumerations)
  * [ ] What is `Unresolved` must be resolved whenever possible. For this guarantee, the lazy initialization of such special methods requires returning a little more complex result with unresolved parts for deferred verification. This could happen in the standard object compilation units, however with little chance (when using `jet.lang.*` definitions in the special methods).

## Type definitions

* [ ] `alias_of` may have to be deferred in resolution (`Unresolved`).

## JetDoc

* [ ] Invoke `set_jetdoc()` appropriately for the symbol of definitions such as package definitions, variable definitions and virtual properties.*
  * [ ] Verification propagates JetDoc from getter or setter to the corresponding virtual property.

## Visibility

* [ ] Always remember to set visibility of properties properly, specially calling `definition.set_visibility(Attribute::visibility(&list, at_interface_block));`.
* [ ] Propagate visibility from getter or setter to the corresponding virtual property.

## Deferred verification

Verification must be deferred in cases where the semantic model produces the `Unresolved` symbol.

* Structural types such as function types and these after type substitution must not be constructed containing any `Unresolved` compound.
* Extends list and implements list must not contain `Unresolved` compound.
* Compounds in expressions and statements may cause deferred verification of the respective syntax constructs.
* Special enumeration properties and methods must be created without `Unresolved` signatures or static type.
* There are miscellaneous other cases not written here yet, so be ultra careful with compounds in syntax constructs and certain contexts involving `Unresolved` including when using standard objects such as `jet.lang.Array` and `jet.lang.String`.

## Parent definitions

* [ ] Set parent definitions of all properties properly, including for these that belong to scopes (aliases, types, variables, accessors and functions).
  * [ ] Set parent definition of getters and setters
  * [ ] Set parent definition of the virtual property from getters and setters

## undefined, Infinity and NaN

* [ ] `jet.lang.undefined` assigned `void 0` which yields an `undefined` constant of the `*` type

```
package jet.lang {
    public const undefined = void 0
}
```

* [ ] The `jet.lang.Infinity` and `jet.lang.NaN` constants are processed in a special way such that they contain their respective values. Although they are assigned zero in the following example, they must be assigned a different IEEE 754 constant initializer.

```
package jet.lang {
    public const Infinity = 0
    public const NaN = 0
}
```

## Overriding

* [ ] Use `MethodOverriding::abstract_methods_not_overriden()` to verify not overriden abstract methods in a class.
* [ ] Use `MethodOverriding::override_method()` to override a method.

## Interface implementation

* [ ] Use `InterfaceImplementation::verify()` to verify interface implementations of a class.
