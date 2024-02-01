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
* [x] `Value`
  * [x] `is_value()`
  * [x] `static_type()`
  * [x] `set_static_type()`
  * [x] Factory creation
* [ ] Miscellaneous values
  * [x] `UndefinedConstant`
    * [x] Factory creation
  * [x] `NullConstant`
    * [x] Factory creation
  * [x] `StringConstant`
    * [x] Factory creation
  * [x] `CharConstant`
    * [x] Factory creation
  * [x] `CharIndexConstant`
    * [x] Factory creation
  * [x] `BooleanConstant`
    * [x] Factory creation
  * [x] `NumberConstant`
    * [x] Factory creation
  * [x] `EnumConstant`
    * [x] Factory creation
  * [x] `ThisValue`
    * [x] Factory creation
  * [x] `ConversionValue`
    * [x] Factory creation
  * [x] `ImportMetaOutputValue`
    * [x] Factory creation
  * [ ] `XMLReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_xml_reference_value()`
    * [x] `base()`
    * [x] `qualifier()`
    * [x] `key_name()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `DynamicReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_dynamic_reference_value()`
    * [x] `base()`
    * [x] `qualifier()`
    * [x] `key_name()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `StaticReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_static_reference_value()`
    * [x] `base()`
    * [x] `property()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `InstanceReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_instance_reference_value()`
    * [x] `base()`
    * [x] `property()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `ProxyReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_proxy_reference_value()`
    * [x] `base()`
    * [x] `proxy()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `TupleReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_tuple_reference_value()`
    * [x] `base()`
    * [x] `tuple_index()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `ScopeReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_scope_reference_value()`
    * [x] `base()`
    * [x] `property()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `DynamicScopeReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_dynamic_scope_reference_value()`
    * [x] `base()`
    * [x] `qualifier()`
    * [x] `key_name()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `PackageReferenceValue
    * [x] `is_reference_value()`
    * [x] `is_package_reference_value()`
    * [x] `base()`
    * [x] `property()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [x] `property_is_visible()`
    * [ ] Factory creation

## Covered (logic)

* [x] Type substitution
* [x] Type relationship
* [x] Default value
* [ ] Property resolution
  * Take care with unresolved parts (consult symbol-operations.md for cases)
  * Return a `Result` that throws `PropertyResolutionError::AmbiguousReference` and `PropertyResolutionError::DeferVerification` variants.
* [x] `property_is_visible(&scope)`
* [ ] Type conversions