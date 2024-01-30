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
    * [x] Factory creation
  * [x] `TupleType`
    * [x] `is_type()`
    * [x] `is_tuple_type()`
    * [x] `to_string()`
    * [x] `element_types()`
    * [x] Factory creation
  * [x] `NullableType`
    * [x] `is_type()`
    * [x] `is_nullable_type()`
    * [x] `to_string()`
    * [x] `base()`
    * [x] Factory creation
  * [x] `TypeParameterType`
    * [x] `is_type()`
    * [x] `is_type_parameter_type()`
    * [x] `to_string()`
    * [x] `name()`
    * [x] Factory creation
  * [x] `TypeAfterExplicitTypeSubstitution` (`T.<...>`)
    * [x] Factory creation
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