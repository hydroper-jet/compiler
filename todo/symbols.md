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
  * [ ] `TypeAfterExplicitTypeSubstitution` (`T.<...>`)
    * https://github.com/violetscript/violetc/blob/master/parser/src/semantic/model/typeSystem.cs#L918
    * Constructor, if unresolved, mutates on next retrieval.
    * [x] `is_type()`
    * [x] `is_type_after_explicit_type_substitution()`
    * [x] `is_class_type()`
    * [x] `is_interface_type()`
    * [x] `fully_qualified_name()`
    * [x] `to_string()`
    * [x] `origin()`
    * [x] `substitute_types()`
    * [x] `is_abstract()`
    * [x] `is_final()`
    * [x] `is_static()`
    * [x] `allow_literal()`
    * [ ] `implements()` — Implements list of a class.
    * [ ] `super_interfaces()` — Extends list of an interface.
    * [ ] `name()` — Unqualified name.
    * [ ] `parent_definition()`
    * [ ] `super_class()`
    * [ ] `type_parameters()`
    * [ ] `static_properties()`
    * [ ] `constructor_function()`
    * [ ] `prototype()`
    * [ ] `proxies()`
    * [ ] `list_of_to_proxies()`
    * [ ] `plain_metadata()`
    * [ ] `visibility()`
    * [ ] `jetdoc()`
    * [ ] `includes_null()` — Returns `false`.
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
* [ ] `FunctionAfterIndirectTypeSubstitution`
  * Static type, if unresolved, mutates on next retrieval.
* [ ] `FunctionAfterExplicitTypeSubstitution` (`f.<...>`)
  * Static type, if unresolved, mutates on next retrieval.
* [ ] `Scope`
* [ ] `Value`
  * [ ] Constants
  * [ ] `ConversionValue`
  * [ ] Reference values mentioned in property resolution