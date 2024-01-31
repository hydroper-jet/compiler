# Verification

## Enumeration definitions

* [ ] Lazily initialize special methods (normal and set enumerations)
  * [ ] What is `Unresolved` must be resolved whenever possible. For this guarantee, the lazy initialization of such special methods requires returning a little more complex result with unresolved parts for deferred verification. This could happen in the standard object compilation units, however with little chance (when using `jet.lang.*` definitions in the special methods).

## Type definitions

* [ ] `alias_of` may have to be deferred in resolution (`Unresolved`).

## JetDoc

* [ ] Invoke `set_jetdoc()` appropriately for the symbol of definitions such as package definitions, variable definitions and virtual properties.*
  * [ ] Verification delegates JetDoc from getter or setter to the corresponding property.

## Visibility

* [ ] Always remember to set visibility of properties properly, specially calling `definition.set_visibility(Attribute::visibility(&list, at_interface_block));`.