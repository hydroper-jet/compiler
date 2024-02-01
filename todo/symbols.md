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
  * [x] Factory creation
* [x] `VirtualPropertyAfterIndirectTypeSubstitution`
  * [x] Factory creation
* [x] `FunctionSymbol`
  * [x] Factory creation
* [x] `FunctionAfterExplicitOrIndirectTypeSubstitution`
  * [x] Factory creation
* [x] `Scope`
  * [x] Factory creation
* [x] Miscellaneous scopes
  * [x] With scope
    * [x] `object()`
  * [x] Filter operator scope
    * [x] `base()`
  * [x] Activation scope
    * [x] `function()`
    * [x] `this()`
    * [x] `set_this()`
  * [x] Class scope
    * [x] `class()`
  * [x] Enum scope
    * [x] `class()`
  * [x] Interface scope
    * [x] `interface()`
  * [x] Package scope
    * [x] `package()`
* [x] `ImportMetaSymbol`
* [x] `ImportMetaEnvSymbol`
* [ ] `Value`
  * [ ] Constants
  * [ ] `ConversionValue`
  * [ ] Reference values mentioned in property resolution
  * [ ] `ImportMetaOutputValue`
  * [ ] Factory creation

## Covered (logic)

* [x] Type substitution
* Type relationship
  * [ ] `is_subtype_of()`
    * Class hierarchy
    * Class implementors
    * Interface hierarchy
  * [ ] `is_ascending_type_of()`
* [ ] Default value
* [ ] Property resolution
  * Take care with unresolved parts (consult symbol-operations.md for cases)
* [ ] `property_is_visible(&scope)`