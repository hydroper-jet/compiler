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
    * [x] Throws `DeferVerificationError` when an involved ascending type is unresolved (an inherited class)
  * [x] `override_method()`
    * [x] Throws `MethodOverridingError::DeferVerification` when an involved ascending type is unresolved (like an inherited class)
    * [x] Throws `MethodOverridingError::DeferVerification` when an involved signature is unresolved
    * [x] Throws `MethodOverridingError::CannotIntroduceTypeParameters` where necessary.
* [ ] Interface implementation
  * [ ] `verify(): Result<InterfaceImplementationVerification, DeferVerificationError>`
    * [ ] Throws `DeferVerificationError` when an involved ascending type is unresolved (like an inherited interface or inherited class)
    * [ ] Throws `DeferVerificationError` when an involved signature is unresolved