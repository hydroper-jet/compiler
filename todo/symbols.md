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
    * [x] `is_type()`
    * [x] `is_function_type()`
    * [x] `to_string()`
    * [x] `parameters()`
    * [x] `result_type()`
    * [ ] Factory creation
  * [ ] `TupleType`
  * [ ] `NullableType`
  * [ ] `TypeParameter`
  * [ ] `TypeAfterExplicitTypeSubstitution` (`T.<...>`)
* [ ] `Alias`
  * [ ] Factory creation
  * [ ] JetDoc
* [ ] `Package`
  * [ ] Factory creation
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
* [ ] `FunctionAfterExplicitTypeSubstitution` (`f.<...>`)
* [ ] `Scope`
* [ ] `Value`
  * [ ] Constants
  * [ ] `ConversionValue`
  * [ ] Reference values mentioned in property resolution