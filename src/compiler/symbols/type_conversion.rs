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
        ()
    }

    pub fn implicit_conversion(&mut self, value: &Symbol, target_type: &Symbol, optional: bool) -> Option<Symbol> {
        ()
    }

    pub fn explicit_conversion(&mut self, value: &Symbol, target_type: &Symbol, optional: bool) -> Option<Symbol> {
        ()
    }
}