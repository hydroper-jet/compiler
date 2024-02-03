# Symbols

## Covered

* [x] `Visibility`
* [x] `PlainMetadata`
* [x] `SymbolHost`
* [x] `Symbol`
* [x] `SymbolFactory`
* [x] `Unresolved`
* Types
  * [x] `AnyType`
  * [x] `VoidType`
  * [x] `ClassType`
  * [x] `EnumType`
  * [x] `InterfaceType`
  * [x] `FunctionType`
  * [x] `TupleType`
  * [x] `NullableType`
  * [x] `TypeParameterType`
  * [x] `TypeAfterExplicitTypeSubstitution` (`T.<...>`)
* [x] `Alias`
* [x] `Package`
* [x] `PackageSet`
* [x] `VariableProperty`
* [x] `VariablePropertyAfterIndirectTypeSubstitution`
* [x] `VirtualProperty`
* [x] `VirtualPropertyAfterIndirectTypeSubstitution`
* [x] `FunctionSymbol`
* [x] `FunctionAfterExplicitOrIndirectTypeSubstitution`
* [x] `Scope`
* [x] Miscellaneous scopes
* [x] `ImportMetaSymbol`
* [x] `ImportMetaEnvSymbol`
* [x] `Value`
* [x] Miscellaneous values

## Covered (logic)

* [x] Type substitution
* [x] Type relationship
* [x] Default value
* [x] `property_is_visible(&scope)`
* [x] Property resolution
* [x] Type conversions
* [x] Method overriding
  * [x] `abstract_methods_not_overriden()`
  * [x] `override_method()`
* [x] Interface implementations
  * [x] `verify(): Result<Vec<InterfaceImplementationLog>, DeferVerificationError>`