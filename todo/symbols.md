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
* [ ] `VariableProperty`
  * [x] `is_variable_property()`
  * [x] `name()`
  * [x] `fully_qualified_name()`
  * [x] `to_string()`
  * [x] `parent_definition()`
  * [x] `set_parent_definition()`
  * [x] `visibility()`
  * [x] `set_visibility()`
  * [x] `static_type()`
  * [x] `set_static_type()`
  * [x] `read_only()`
  * [x] `set_read_only()`
  * [x] `constant_initializer()`
  * [x] `set_constant_initializer()`
  * [x] `plain_metadata()`
  * [x] `jetdoc()`
  * [x] `set_jetdoc()`
  * [x] Factory creation
* [ ] `VariablePropertyAfterIndirectTypeSubstitution`
  * [ ] Static type, if unresolved, mutates on next retrieval.
  * [ ] JetDoc
  * [ ] Factory creation
* [ ] `VirtualProperty`
  * [ ] JetDoc
  * [ ] Factory creation
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
  * [ ] Factory creation