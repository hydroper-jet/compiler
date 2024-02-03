use crate::ns::*;

pub struct InterfaceImplementations<'a>(pub &'a mut SymbolHost);

impl<'a> InterfaceImplementations<'a> {
    pub fn verify(&mut self, implementor: &Symbol, interface: &Symbol) -> Result<Vec<InterfaceImplementationLog>, DeferVerificationError> {
        ()
    }
}

pub enum InterfaceImplementationLog {
    UnimplementedMethod { name: String },
    UnimplementedGetter { name: String },
    UnimplementedSetter { name: String },
    PropertyMustBeMethod { name: String },
    PropertyMustBeVirtualProperty { name: String },
    WrongMethodSignature { name: String, expected_signature: Symbol },
    WrongGetterSignature { name: String, expected_signature: Symbol },
    WrongSetterSignature { name: String, expected_signature: Symbol },
    NonConformingTypeParameters { name: String },
}