# Symbols

## Covered

* [x] `Visibility`
* [x] `PlainMetadata`
  * [ ] Stored in other symbols behind a shared reference.
* [ ] `SymbolHost`
* [ ] `Symbol`
* [ ] `SymbolFactory`
* [x] `Unresolved`
* Types
  * [x] `AnyType`
  * [x] `VoidType`
  * [x] `ClassType`
    * [x] Factory creation
    * [x] JetDoc
  * [ ] `EnumType`
    * [ ] Factory creation
    * [x] `is_type()`
    * [x] `is_enum_type()`
    * [x] `fully_qualified_name()`
    * [x] `to_string()`
    * [x] `is_set_enumeration()`
    * [x] `set_is_set_enumeration()`
    * [x] `enumeration_representation_type()`
    * [x] `set_enumeration_representation_type()`
    * [x] `name()` — Unqualified name.
    * [x] `parent_definition()`
    * [x] `set_parent_definition()`
    * [ ] `super_class()`
    * [ ] `set_super_class()`
    * [ ] `static_properties()``
    * [ ] `prototype()`
    * [ ] `enumeration_members()`
    * [ ] `proxies()`
    * [ ] `list_of_to_proxies()`
    * [ ] `plain_metadata()`
    * [ ] `visibility()`
    * [ ] `set_visibility()`
    * [ ] `jetdoc()`
    * [ ] `set_jetdoc()`
  * [ ] `InterfaceType`
    * [ ] Factory creation
    * [ ] Limited implementors
  * [ ] `FunctionType`
  * [ ] `TupleType`
  * [ ] `NullableType`
  * [ ] `TypeParameter`
  * [ ] `TypeAfterExplicitTypeSubstitution` (`T.<...>`)
* [ ] `Package`
  * [ ] JetDoc
* [ ] `PackageSet`
* [ ] `VariableProperty`
  * [ ] JetDoc
* [ ] `VariablePropertyAfterIndirectTypeSubstitution`
* [ ] `VirtualProperty`
  * [ ] JetDoc
    * [ ] Verification delegates JetDoc from getter or setter to the corresponding property.
* [ ] `VirtualPropertyAfterIndirectTypeSubstitution`
* [ ] `FunctionSymbol`
  * [ ] JetDoc
* [ ] `FunctionAfterIndirectTypeSubstitution`
* [ ] `FunctionAfterIndirectTypeSubstitution`
* [ ] `FunctionAfterExplicitTypeSubstitution` (`f.<...>`)
* [ ] `Scope`
* [ ] `Value`