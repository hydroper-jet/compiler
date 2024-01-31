# Symbols

## Covered

* [x] `Visibility`
* [x] `PlainMetadata`
* [ ] `SymbolHost`
* [ ] `Symbol`
* [ ] `SymbolFactory`
* [x] `Unresolved`
* Types
  * [x] `AnyType`
  * [x] `VoidType`
  * [x] `ClassType`
    * [x] Factory creation
  * [x] `EnumType`
    * [x] Factory creation
  * [x] `InterfaceType`
    * [x] Factory creation
  * [x] `FunctionType`
    * [x] Factory creation
  * [x] `TupleType`
    * [x] Factory creation
  * [x] `NullableType`
    * [x] Factory creation
  * [x] `TypeParameterType`
    * [x] Factory creation
  * [x] `TypeAfterExplicitTypeSubstitution` (`T.<...>`)
    * [x] Factory creation
* [x] `Alias`
  * [x] Factory creation
* [x] `Package`
  * [x] Factory creation
* [x] `PackageSet`
  * [x] Factory creation
* [x] `VariableProperty`
  * [x] Factory creation
* [x] `VariablePropertyAfterIndirectTypeSubstitution`
  * [x] Factory creation
* [x] `VirtualProperty`
  * [x] `is_virtual_property()`
  * [x] `name()`
  * [x] `fully_qualified_name()`
  * [x] `to_string()`
  * [x] `parent_definition()`
  * [x] `set_parent_definition()`
  * [x] `visibility()`
  * [x] `set_visibility()`
  * [x] `static_type()`
  * [x] `read_only()`
  * [x] `write_only()`
  * [x] `getter()`
  * [x] `set_getter()`
  * [x] `setter()`
  * [x] `set_setter()`
  * [x] `jetdoc()`
  * [x] `set_jetdoc()`
  * [x] Factory creation
* [ ] `VirtualPropertyAfterIndirectTypeSubstitution`
  * [ ] Static type, if unresolved, mutates on next retrieval.
  * [ ] Factory creation
* [ ] `FunctionSymbol`
  * [ ] JetDoc
  * [ ] Factory creation
* [ ] `FunctionAfterExplicitOrIndirectTypeSubstitution`
  * [ ] Static type, if unresolved, mutates on next retrieval.
  * [ ] Factory creation
* [ ] `Scope`
  * [ ] Factory creation
* [ ] `Value`
  * [ ] Constants
  * [ ] `ConversionValue`
  * [ ] Reference values mentioned in property resolution
  * [ ] `ImportMetaOutputValue`
  * [ ] Factory creation
* [ ] `ImportMetaSymbol`
* [ ] `ImportMetaEnvSymbol`