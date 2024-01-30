# Verification

## Enumeration definitions

* [ ] Lazily initialize special methods (normal and set enumerations)
* [ ] `extends_class` defaults to an Unresolved symbol
* [ ] `extends_class` resolves to Object as immediately as possible.

## Type definitions

* [ ] `alias_of` may have to be deferred in resolution (`Unresolved`).

## JetDoc

* [ ] Invoke `set_jetdoc()` appropriately for the symbol of definitions such as package definitions, variable definitions and virtual properties.*
  * [ ] Verification delegates JetDoc from getter or setter to the corresponding property.