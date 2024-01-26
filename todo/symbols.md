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
  * [ ] `ClassType`
    * [ ] Factory creation
    * [ ] JetDoc
    * [ ] Limited subclasses
    * [ ] `fully_qualified_name()`
    * [ ] `ToString`
      * Consider type parameters
  * [ ] `EnumType`
    * [ ] JetDoc
  * [ ] `InterfaceType`
    * [ ] JetDoc
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