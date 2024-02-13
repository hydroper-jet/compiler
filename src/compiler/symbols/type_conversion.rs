use crate::ns::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TypeConversionRelationship {
    BetweenNumberTypes,

    // Implicit
    FromAny,
    ToAny,
    FromNonNullableToCovariantType,
    FromNullableToNullableCovariantType,
    /// Results in exact equal type, but nullable
    FromNonNullableToNullable,
    FromInterfaceToObject,
    FromNullableInterfaceToNullableObject,

    // Explicit
    ThroughToProxy,
    FromNonNullableToContravariantType,
    FromNullableToNonNullableContravariantType,
    ArrayToContravariantArray,
    ArrayToCovariantArray,
    FromStringToEnum,
    FromNumberToEnum,
    FromStringToChar,
    FromCharToString,
    FromCharToNumber,
    FromNumberToChar,
    FromTypeParameter,
    FromMapToLiteralClass,
    FromLiteralClassToMap,
}

pub struct TypeConversions<'a>(pub &'a SymbolHost);

impl<'a> TypeConversions<'a> {
    pub fn implicit_constant_conversion(&self, value: &Symbol, target_type: &Symbol) -> Option<Symbol> {
        let from_type = value.static_type(self.0);
        if &from_type == target_type {
            return Some(value.clone());
        }
        if !value.is_constant() {
            return None;
        }

        // undefined to *, nullable type, or set enumeration
        if value.is_undefined_constant() {
            if target_type.is_nullable_type() {
                return Some(self.0.factory().create_null_constant(target_type));
            } else if target_type.is_enum_type() && target_type.is_set_enumeration() {
                let rt = target_type.enumeration_representation_type().unwrap();
                if rt.is_unresolved() {
                    return None;
                }
                let v = AbstractRangeNumber::zero(&rt, self.0);
                return Some(self.0.factory().create_enum_constant(v, target_type));
            } else if target_type.is_any_type() {
                return Some(self.0.factory().create_undefined_constant(target_type));
            }
        }

        // null to * or nullable type
        if value.is_undefined_constant() && (target_type.is_nullable_type() || target_type.is_any_type()) {
            return Some(self.0.factory().create_null_constant(target_type));
        }

        let object_type = self.0.object_type();

        // N constant to *, Object or Object?
        if value.is_number_constant() && (target_type.is_any_type() || target_type.non_null_type() == object_type) {
            return Some(self.0.factory().create_number_constant(value.number_value(), target_type));
        }

        if value.is_number_constant() {
            // NF1 constant to NF2 or NF2?
            if target_type.non_null_type().is_floating_point_type_of_wider_range_than(&from_type, self.0) {
                let v = value.number_value().convert_type(target_type, self.0);
                return Some(self.0.factory().create_number_constant(v, target_type));
            }

            // NI1 constant to NI2 or NI2?
            if target_type.non_null_type().is_integer_type_of_wider_range_than(&from_type, self.0) {
                let v = value.number_value().convert_type(target_type, self.0);
                return Some(self.0.factory().create_number_constant(v, target_type));
            }

            let number_type = self.0.number_type();
            let nullable_number_type = self.0.factory().create_nullable_type(&number_type);

            // NaN constant to NI or NI?
            if value.number_value().is_nan() && self.0.is_integer_type(&target_type.non_null_type()) {
                let v = AbstractRangeNumber::zero(&target_type.non_null_type(), self.0);
                return Some(self.0.factory().create_number_constant(v, target_type));
            }

            let big_int_type = self.0.big_int_type();

            // -Infinity constant to NI or NI?
            if value.number_value().is_negative_infinity() && self.0.is_integer_type(&target_type.non_null_type()) && target_type.non_null_type() != big_int_type {
                let v = AbstractRangeNumber::minimum_value(&target_type.non_null_type(), self.0);
                return Some(self.0.factory().create_number_constant(v, target_type));
            }

            // +Infinity constant to NI or NI?
            if value.number_value().is_positive_infinity() && self.0.is_integer_type(&target_type.non_null_type()) && target_type.non_null_type() != big_int_type {
                let v = AbstractRangeNumber::maximum_value(&target_type.non_null_type(), self.0);
                return Some(self.0.factory().create_number_constant(v, target_type));
            }
        }

        // From non nullable T constant to T?
        if target_type.is_nullable_type() && target_type.base() == from_type {
            let new_k = value.clone_constant_value(self.0);
            new_k.set_static_type(target_type);
            return Some(new_k);
        }

        None
    }

    pub fn implicit_conversion(&self, value: &Symbol, target_type: &Symbol, optional: bool) -> Option<Symbol> {
        let from_type = value.static_type(self.0);
        if &from_type == target_type {
            return Some(value.clone());
        }

        let const_conv = self.implicit_constant_conversion(value, target_type);
        if const_conv.is_some() {
            return const_conv;
        }

        // From *
        if from_type.is_any_type() {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromAny, optional, target_type));
        }

        // To *
        if target_type.is_any_type() {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::ToAny, optional, target_type));
        }

        let from_type_non_null = from_type.non_null_type();
        let target_type_non_null = target_type.non_null_type();
        let from_nullable = from_type.is_nullable_type();
        let to_nullable = target_type.is_nullable_type();
        let from_non_nullable = !from_nullable;
        let to_non_nullable = !to_nullable;

        // N1 to N2, N1? to N2? or N1 to N2?
        if self.0.is_numeric_type(&from_type_non_null) && self.0.is_numeric_type(&target_type_non_null) && !(from_nullable && to_non_nullable) {
            let are_integer_types = self.0.is_integer_type(&from_type_non_null) && self.0.is_integer_type(&target_type_non_null);
            let are_floating_point_types = self.0.is_floating_point_type(&from_type_non_null) && self.0.is_floating_point_type(&target_type_non_null);
            let compatible = (are_integer_types && target_type_non_null.is_integer_type_of_wider_range_than(&from_type_non_null, self.0))
                || (are_floating_point_types && target_type_non_null.is_floating_point_type_of_wider_range_than(&from_type_non_null, self.0));
            if compatible {
                return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::BetweenNumberTypes, optional, target_type));
            }
        }

        // From non-nullable to covariant type or nullable covariant type
        if from_non_nullable && target_type_non_null.is_ascending_type_of(&from_type, self.0) {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromNonNullableToCovariantType, optional, target_type));
        }

        // From nullable to nullable covariant type
        if from_nullable && to_nullable && target_type.base().is_ascending_type_of(&from_type.base(), self.0) {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromNullableToNullableCovariantType, optional, target_type));
        }

        // From T to T?
        if from_non_nullable && to_nullable && from_type == target_type_non_null {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromNonNullableToNullable, optional, target_type));
        }

        let from_interface = from_type.is_interface_type();
        let object_type = self.0.object_type();

        // From ìnterface to Object or Object?
        if from_interface && target_type_non_null == object_type {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromInterfaceToObject, optional, target_type));
        }

        // From nullable ìnterface to nullable Object
        if from_nullable && from_type.base().is_interface_type() && to_nullable && target_type.base() == object_type {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromNullableInterfaceToNullableObject, optional, target_type));
        }

        None
    }

    pub fn explicit_conversion(&self, value: &Symbol, target_type: &Symbol, optional: bool) -> Option<Symbol> {
        let from_type = value.static_type(self.0);
        if &from_type == target_type {
            return Some(value.clone());
        }

        let implicit_conv = self.implicit_conversion(value, target_type, optional);
        if implicit_conv.is_some() {
            return implicit_conv;
        }

        let from_type_non_null = from_type.non_null_type();
        let target_type_non_null = target_type.non_null_type();
        let from_nullable = from_type.is_nullable_type();
        let to_nullable = target_type.is_nullable_type();
        let from_non_nullable = !from_nullable;
        let to_non_nullable = !to_nullable;

        // To T through proxy::to
        if from_type.is_class_type() || from_type.is_enum_type() {
            for proxy in from_type.list_of_to_proxies(self.0).iter() {
                let signature = proxy.signature(self.0);
                if signature.is_unresolved() {
                    continue;
                }
                let result_type = signature.result_type();
                if result_type.is_equals_or_subtype_of(&target_type, self.0) {
                    return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::ThroughToProxy, optional, target_type));
                }
            }
        }

        // From non-nullable to contravariant type
        if from_non_nullable && target_type.is_subtype_of(&from_type, self.0) {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromNonNullableToContravariantType, optional, target_type));
        }

        // From nullable to non-nullable contravariant type
        if from_nullable && target_type.is_subtype_of(&from_type, self.0) {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromNullableToNonNullableContravariantType, optional, target_type));
        }

        let array_type = self.0.array_type();

        if from_type.type_after_substitution_has_origin(&array_type)
        && target_type_non_null.type_after_substitution_has_origin(&array_type) {
            let from_el_type = from_type.substitute_types().get(0).unwrap();
            let to_el_type = target_type_non_null.substitute_types().get(0).unwrap();

            // Array to contravariant Array
            if to_el_type.is_subtype_of(&from_el_type, self.0) {
                return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::ArrayToContravariantArray, optional, target_type));
            }

            // Array to covariant Array
            if to_el_type.is_ascending_type_of(&from_el_type, self.0) {
                return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::ArrayToCovariantArray, optional, target_type));
            }
        }

        // N1 to N2, N1? to N2? or N1 to N2?
        if self.0.is_numeric_type(&from_type_non_null) && self.0.is_numeric_type(&target_type_non_null) && !(from_nullable && to_non_nullable) {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::BetweenNumberTypes, optional, target_type));
        }

        let string_type = self.0.string_type();

        // From String to enum
        if from_type == string_type && target_type_non_null.is_enum_type() {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromStringToEnum, optional, target_type));
        }

        // From number to enum
        if target_type_non_null.is_enum_type() && from_type == target_type_non_null.enumeration_representation_type().unwrap() {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromNumberToEnum, optional, target_type));
        }

        let char_type = self.0.char_type();

        // From String to Char
        if from_type == string_type && target_type_non_null == char_type {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromStringToChar, optional, target_type));
        }

        // From Char to String
        if from_type == char_type && target_type_non_null == string_type {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromCharToString, optional, target_type));
        }

        let number_type = self.0.number_type();

        // From Char to Number
        if from_type == char_type && target_type_non_null == number_type {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromCharToNumber, optional, target_type));
        }

        // From Number to Char
        if from_type == number_type && target_type_non_null == char_type {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromNumberToChar, optional, target_type));
        }

        // From Map.<*, *> to C where C is a class with C[[AllowLiteral]] = true
        if from_type.is_map_type_of_any_any(self.0) && target_type_non_null.is_class_type() && target_type_non_null.allow_literal() {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromMapToLiteralClass, optional, target_type));
        }

        // From C to Map.<*, *> where C is a class with C[[AllowLiteral]] = true
        if from_type.is_class_type() && from_type.allow_literal() && target_type_non_null.is_map_type_of_any_any(self.0) {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromLiteralClassToMap, optional, target_type));
        }

        // From type parameter T to W
        if from_type.is_type_parameter_type() {
            return Some(self.0.factory().create_conversion_value(value, TypeConversionRelationship::FromTypeParameter, optional, target_type));
        }

        None
    }
}