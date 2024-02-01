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
    * [ ] `is_reference_value()`
    * [ ] `is_xml_reference_value()`
    * [ ] `base()`
    * [ ] `qualifier()`
    * [ ] `key_name()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `DynamicReferenceValue
    * [ ] `is_reference_value()`
    * [ ] `is_dynamic_reference_value()`
    * [ ] `base()`
    * [ ] `qualifier()`
    * [ ] `key_name()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `StaticReferenceValue
    * [ ] `is_reference_value()`
    * [ ] `is_static_reference_value()`
    * [ ] `base()`
    * [ ] `property()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `InstanceReferenceValue
    * [ ] `is_reference_value()`
    * [ ] `is_instance_reference_value()`
    * [ ] `base()`
    * [ ] `property()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `ProxyReferenceValue
    * [ ] `is_reference_value()`
    * [ ] `is_proxy_reference_value()`
    * [ ] `base()`
    * [ ] `proxy()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `TupleReferenceValue
    * [ ] `is_reference_value()`
    * [ ] `is_tuple_reference_value()`
    * [ ] `base()`
    * [ ] `tuple_index()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `ScopeReferenceValue
    * [ ] `is_reference_value()`
    * [ ] `is_scope_reference_value()`
    * [ ] `base()`
    * [ ] `property()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `DynamicScopeReferenceValue
    * [ ] `is_reference_value()`
    * [ ] `is_dynamic_scope_reference_value()`
    * [ ] `base()`
    * [ ] `qualifier()`
    * [ ] `key_name()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
    * [ ] Factory creation
  * [ ] `PackageReferenceValue
    * [ ] `is_reference_value()`
    * [ ] `is_package_reference_value()`
    * [ ] `base()`
    * [ ] `property()`
    * [ ] `read_only()`
    * [ ] `write_only()`
    * [ ] `property_is_visible()`
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
  * Return a `Result` that throws `PropertyResolutionError::AmbiguousReference` and `PropertyResolutionError::DeferVerification` variants.
* [ ] `property_is_visible(&scope)`
* [ ] Type conversions