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
    * [x] JetDoc
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
  * [x] `is_package()`
  * [x] `name()`
  * [x] `parent_definition()`
  * [x] `set_parent_definition()`
  * [x] `properties()`
  * [x] `redirect_packages()`
  * [x] `subpackages()`
  * [x] `jetdoc()`
  * [x] `set_jetdoc()`
  * [x] Factory creation through a fully qualified name (`Vec<&str>`)
* [ ] `PackageSet`
* [ ] `VariableProperty`
  * [ ] JetDoc
* [ ] `VariablePropertyAfterIndirectTypeSubstitution`
  * Static type, if unresolved, mutates on next retrieval.
* [ ] `VirtualProperty`
  * [ ] JetDoc
    * [ ] Verification delegates JetDoc from getter or setter to the corresponding property.
* [ ] `VirtualPropertyAfterIndirectTypeSubstitution`
  * Static type, if unresolved, mutates on next retrieval.
* [ ] `FunctionSymbol`
  * [ ] JetDoc
* [ ] `FunctionAfterExplicitOrIndirectTypeSubstitution`
  * Static type, if unresolved, mutates on next retrieval.
* [ ] `Scope`
* [ ] `Value`
  * [ ] Constants
  * [ ] `ConversionValue`
  * [ ] Reference values mentioned in property resolution