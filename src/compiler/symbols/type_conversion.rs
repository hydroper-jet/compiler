use crate::ns::*;

#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum TypeConversionRelationship {
    // Implicit
    FromAny,
    ToAny,
    N1ToN2,
    FromNonNullableToCovariantType,
    FromNonNullableToNullableCovariantType,
    FromNullableToNullableCovariantType,
    FromTToNullableT,
    FromInterfaceToObject,
    FromInterfaceToNullableObject,

    // Explicit
    ThroughToProxy,
    FromNonNullableToContravariantType,
    FromNullableToNonNullableContravariantType,
    ArrayToContravariantArray,
    ArrayToCovariantArray,
    BetweenNumberTypes,
    FromStringToEnum,
    FromNumberToEnum,
    FromCharToInt,
    FromIntToChar,
    FromTypeParameter,
}

pub struct TypeConversions<'a>(pub &'a mut SymbolHost);

impl<'a> TypeConversions<'a> {
    pub fn implicit_constant_conversion(&mut self, value: &Symbol, target_type: &Symbol) -> Option<Symbol> {
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
                return Some(self.0.factory().create_enum_constant(AbstractRangeNumber::zero(&rt, self.0), target_type));
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
                return Some(self.0.factory().create_number_constant(value.number_value().convert_type(target_type, self.0), target_type));
            }

            // NI1 constant to NI2 or NI2?
            if target_type.non_null_type().is_integer_type_of_wider_range_than(&from_type, self.0) {
                return Some(self.0.factory().create_number_constant(value.number_value().convert_type(target_type, self.0), target_type));
            }

            let number_type = self.0.number_type();
            let nullable_number_type = self.0.factory().create_nullable_type(&number_type);

            let possibly_from_number_types = [
                self.0.byte_type(),
                self.0.short_type(),
                self.0.int_type(),
                self.0.unsigned_byte_type(),
                self.0.unsigned_short_type(),
                self.0.unsigned_int_type(),
            ];

            // NI constant to Number or Number?
            if [number_type, nullable_number_type].contains(target_type) && possibly_from_number_types.contains(&from_type) {
                return Some(self.0.factory().create_number_constant(value.number_value().convert_type(target_type, self.0), target_type));
            }

            // NaN constant to NI or NI?
            if value.number_value().is_nan() && self.0.is_integer_type(&target_type.non_null_type()) {
                return Some(self.0.factory().create_number_constant(AbstractRangeNumber::zero(&target_type.non_null_type(), self.0), target_type));
            }

            let big_int_type = self.0.big_int_type();

            // -Infinity constant to NI or NI?
            if value.number_value().is_negative_infinity() && self.0.is_integer_type(&target_type.non_null_type()) && target_type.non_null_type() != big_int_type {
                return Some(self.0.factory().create_number_constant(AbstractRangeNumber::minimum_value(&target_type.non_null_type(), self.0), target_type));
            }

            // +Infinity constant to NI or NI?
            if value.number_value().is_positive_infinity() && self.0.is_integer_type(&target_type.non_null_type()) && target_type.non_null_type() != big_int_type {
                return Some(self.0.factory().create_number_constant(AbstractRangeNumber::maximum_value(&target_type.non_null_type(), self.0), target_type));
            }
        }

        // From non nullable T constant to T?
        if target_type.is_nullable_type() && target_type.base() == from_type {
            value.set_static_type(target_type);
            return Some(value.clone());
        }

        None
    }

    pub fn implicit_conversion(&mut self, value: &Symbol, target_type: &Symbol, optional: bool) -> Option<Symbol> {
        ()
    }

    pub fn explicit_conversion(&mut self, value: &Symbol, target_type: &Symbol, optional: bool) -> Option<Symbol> {
        ()
    }
}