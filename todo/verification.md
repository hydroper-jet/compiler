# Verification

## Function definitions

* [ ] Invoke `symbol.set_is_constructor(true)` for constructor definitions.
* [ ] Invoke `symbol.set_of_virtual_property(p)` appropriately for getters and setters.

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
* There are miscellaneous other cases not written here yet, so be ultra careful with compounds in syntax constructs and certain contexts involving `Unresolved` including when using standard objects such as `jet.lang.Array` and `jet.lang.String`.

## Parent definitions

Set parent definitions of all properties properly, including for these that belong to scopes.

## Infinity and NaN

* [ ] The `jet.lang.Infinity` and `jet.lang.NaN` constants are processed in a special way such that they contain their respective values. Although they are assigned zero in the following example, they must be assigned a different IEEE 754 constant initializer.

```
package jet.lang {
    public const Infinity = 0
    public const NaN = 0
}
```