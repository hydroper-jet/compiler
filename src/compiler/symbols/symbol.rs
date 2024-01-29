use crate::ns::*;
use std::cell::{Cell, RefCell};
use std::hash::Hash;
use std::ops::Deref;
use std::rc::{Rc, Weak};

use bitflags::bitflags;

#[derive(Clone)]
pub struct Symbol(pub(crate) Weak<SymbolKind>);

impl Eq for Symbol {}

impl PartialEq for Symbol {
    fn eq(&self, other: &Self) -> bool {
        self.0.ptr_eq(&other.0)
    }
}

impl Hash for Symbol {
    /// Performs hashing of the symbol by reference.
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.0.as_ptr().hash(state)
    }
}

impl Symbol {
    pub fn is_unresolved(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Unresolved)
    }

    pub fn is_type(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(_))
    }

    pub fn is_any_type(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::AnyType))
    }

    pub fn is_void_type(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::VoidType))
    }

    pub fn is_class_type(&self) -> bool {
        match self.0.upgrade().unwrap().as_ref() {
            SymbolKind::Type(TypeKind::ClassType(_)) => true,
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(ref t)) => t.origin.is_class_type(),
            _ => false,
        }
    }

    pub fn is_enum_type(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::EnumType(_)))
    }

    pub fn is_interface_type(&self) -> bool {
        match self.0.upgrade().unwrap().as_ref() {
            SymbolKind::Type(TypeKind::InterfaceType(_)) => true,
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(ref t)) => t.origin.is_interface_type(),
            _ => false,
        }
    }

    pub fn is_function_type(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::FunctionType(_)))
    }

    pub fn is_tuple_type(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::TupleType(_)))
    }

    pub fn is_nullable_type(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::NullableType(_)))
    }

    pub fn is_type_parameter_type(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::TypeParameterType(_)))
    }

    pub fn is_type_after_explicit_type_substitution(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(_)))
    }

    pub fn name(&self) -> String {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref name, .. } = data.as_ref();
                name.clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref name, .. } = data.as_ref();
                name.clone()
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref name, .. } = data.as_ref();
                name.clone()
            },
            SymbolKind::Type(TypeKind::TypeParameterType(data)) => {
                let TypeParameterTypeData { ref name, .. } = data.as_ref();
                name.clone()
            },
            _ => panic!(),
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        if let SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(ref t)) = self.0.upgrade().unwrap().as_ref() {
            return t.origin.fully_qualified_name();
        }
        let p: Option<Symbol> = self.parent_definition();
        (if let Some(p) = p { p.fully_qualified_name() + "." } else { "".to_owned() }) + &self.name()
    }

    pub fn is_set_enumeration(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { is_set_enumeration, .. } = data.as_ref();
                *is_set_enumeration
            },
            _ => panic!(),
        }
    }

    pub fn is_abstract(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref flags, .. } = data.as_ref();
                flags.get().contains(ClassTypeFlags::IS_ABSTRACT)
            },
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.is_abstract(),
            _ => panic!(),
        }
    }

    pub fn set_is_abstract(&self, value: bool) {
        let mut symbol = self.0.upgrade().unwrap();
        match Rc::get_mut(&mut symbol).unwrap() {
            SymbolKind::Type(TypeKind::ClassType(ref mut data)) => {
                let ClassTypeData { ref mut flags, .. } = Rc::get_mut(data).unwrap();
                flags.get_mut().set(ClassTypeFlags::IS_ABSTRACT, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_final(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref flags, .. } = data.as_ref();
                flags.get().contains(ClassTypeFlags::IS_FINAL)
            },
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.is_final(),
            _ => panic!(),
        }
    }

    pub fn set_is_final(&self, value: bool) {
        let mut symbol = self.0.upgrade().unwrap();
        match Rc::get_mut(&mut symbol).unwrap() {
            SymbolKind::Type(TypeKind::ClassType(ref mut data)) => {
                let ClassTypeData { ref mut flags, .. } = Rc::get_mut(data).unwrap();
                flags.get_mut().set(ClassTypeFlags::IS_FINAL, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_static(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref flags, .. } = data.as_ref();
                flags.get().contains(ClassTypeFlags::IS_STATIC)
            },
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.is_static(),
            _ => panic!(),
        }
    }

    pub fn set_is_static(&self, value: bool) {
        let mut symbol = self.0.upgrade().unwrap();
        match Rc::get_mut(&mut symbol).unwrap() {
            SymbolKind::Type(TypeKind::ClassType(ref mut data)) => {
                let ClassTypeData { ref mut flags, .. } = Rc::get_mut(data).unwrap();
                flags.get_mut().set(ClassTypeFlags::IS_STATIC, value);
            },
            _ => panic!(),
        }
    }

    pub fn allow_literal(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref flags, .. } = data.as_ref();
                flags.get().contains(ClassTypeFlags::ALLOW_LITERAL)
            },
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.allow_literal(),
            _ => panic!(),
        }
    }

    pub fn set_allow_literal(&self, value: bool) {
        let mut symbol = self.0.upgrade().unwrap();
        match Rc::get_mut(&mut symbol).unwrap() {
            SymbolKind::Type(TypeKind::ClassType(ref mut data)) => {
                let ClassTypeData { ref mut flags, .. } = Rc::get_mut(data).unwrap();
                flags.get_mut().set(ClassTypeFlags::ALLOW_LITERAL, value);
            },
            _ => panic!(),
        }
    }

    pub fn implements(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref implements, .. } = data.as_ref();
                implements.clone()
            },
            _ => panic!(),
        }
    }

    pub fn extends_interfaces(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref extends_interfaces, .. } = data.as_ref();
                extends_interfaces.clone()
            },
            _ => panic!(),
        }
    }

    pub fn parent_definition(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref parent_definition, .. } = data.as_ref();
                parent_definition.borrow().clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref parent_definition, .. } = data.as_ref();
                parent_definition.borrow().clone()
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref parent_definition, .. } = data.as_ref();
                parent_definition.borrow().clone()
            },
            _ => panic!(),
        }
    }

    pub fn set_parent_definition(&self, value: Option<Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref parent_definition, .. } = data.as_ref();
                parent_definition.replace(value);
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref parent_definition, .. } = data.as_ref();
                parent_definition.replace(value);
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref parent_definition, .. } = data.as_ref();
                parent_definition.replace(value);
            },
            _ => panic!(),
        }
    }

    pub fn extends_class(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref extends_class, .. } = data.as_ref();
                extends_class.borrow().clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref extends_class, .. } = data.as_ref();
                extends_class.borrow().clone()
            },
            _ => panic!(),
        }
    }

    pub fn set_extends_class(&self, value: Option<Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref extends_class, .. } = data.as_ref();
                extends_class.replace(value);
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref extends_class, .. } = data.as_ref();
                extends_class.replace(value);
            },
            _ => panic!(),
        }
    }

    /// Enumeration representation type. It may be `Unresolved` in certain occasions.
    /// 
    /// **Default**: `None`.
    pub fn enumeration_representation_type(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref representation_type, .. } = data.as_ref();
                representation_type.borrow().clone()
            },
            _ => panic!(),
        }
    }

    /// Enumeration representation type. It may be `Unresolved` in certain occasions.
    ///
    /// **Default**: `None`.
    pub fn set_enumeration_representation_type(&self, value: Option<Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref representation_type, .. } = data.as_ref();
                representation_type.replace(value);
            },
            _ => panic!(),
        }
    }

    pub fn type_parameters(&self) -> Option<SharedArray<Symbol>> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref type_parameters, .. } = data.as_ref();
                type_parameters.borrow().clone()
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref type_parameters, .. } = data.as_ref();
                type_parameters.borrow().clone()
            },
            _ => panic!(),
        }
    }

    pub fn set_type_parameters(&self, value: Option<SharedArray<Symbol>>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref type_parameters, .. } = data.as_ref();
                type_parameters.replace(value);
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref type_parameters, .. } = data.as_ref();
                type_parameters.replace(value);
            },
            _ => panic!(),
        }
    }

    pub fn static_properties(&self) -> SharedMap<String, Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref static_properties, .. } = data.as_ref();
                static_properties.clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref static_properties, .. } = data.as_ref();
                static_properties.clone()
            },
            _ => panic!(),
        }
    }

    pub fn prototype(&self) -> SharedMap<String, Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref prototype, .. } = data.as_ref();
                prototype.clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref prototype, .. } = data.as_ref();
                prototype.clone()
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref prototype, .. } = data.as_ref();
                prototype.clone()
            },
            _ => panic!(),
        }
    }

    pub fn proxies(&self) -> SharedMap<ProxyKind, Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref proxies, .. } = data.as_ref();
                proxies.clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref proxies, .. } = data.as_ref();
                proxies.clone()
            },
            _ => panic!(),
        }
    }

    pub fn list_of_to_proxies(&self) -> SharedMap<Symbol, Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref list_of_to_proxies, .. } = data.as_ref();
                list_of_to_proxies.clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref list_of_to_proxies, .. } = data.as_ref();
                list_of_to_proxies.clone()
            },
            _ => panic!(),
        }
    }

    /// Members of an enumeration type as (*string*, *number*) groups.
    pub fn enumeration_members(&self) -> SharedMap<String, AbstractRangeNumber> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref enumeration_members, .. } = data.as_ref();
                enumeration_members.clone()
            },
            _ => panic!(),
        }
    }

    pub fn constructor_function(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref constructor_function, .. } = data.as_ref();
                constructor_function.borrow().clone()
            },
            _ => panic!(),
        }
    }

    pub fn set_constructor_function(&self, value: Option<Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref constructor_function, .. } = data.as_ref();
                constructor_function.replace(value);
            },
            _ => panic!(),
        }
    }

    pub fn limited_subclasses(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref limited_subclasses, .. } = data.as_ref();
                limited_subclasses.clone()
            },
            _ => panic!(),
        }
    }

    pub fn limited_implementors(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref limited_implementors, .. } = data.as_ref();
                limited_implementors.clone()
            },
            _ => panic!(),
        }
    }

    pub fn plain_metadata(&self) -> SharedArray<Rc<PlainMetadata>> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref plain_metadata, .. } = data.as_ref();
                plain_metadata.clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref plain_metadata, .. } = data.as_ref();
                plain_metadata.clone()
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref plain_metadata, .. } = data.as_ref();
                plain_metadata.clone()
            },
            _ => panic!(),
        }
    }

    pub fn visibility(&self) -> Visibility {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref visibility, .. } = data.as_ref();
                visibility.get()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref visibility, .. } = data.as_ref();
                visibility.get()
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref visibility, .. } = data.as_ref();
                visibility.get()
            },
            _ => panic!(),
        }
    }

    pub fn set_visibility(&self, value: Visibility) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref visibility, .. } = data.as_ref();
                visibility.set(value);
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref visibility, .. } = data.as_ref();
                visibility.set(value);
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref visibility, .. } = data.as_ref();
                visibility.set(value);
            },
            _ => panic!(),
        }
    }

    pub fn jetdoc(&self) -> Option<Rc<JetDoc>> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref jetdoc, .. } = data.as_ref();
                jetdoc.borrow().clone()
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref jetdoc, .. } = data.as_ref();
                jetdoc.borrow().clone()
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref jetdoc, .. } = data.as_ref();
                jetdoc.borrow().clone()
            },
            _ => panic!(),
        }
    }

    pub fn set_jetdoc(&self, value: Option<Rc<JetDoc>>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref jetdoc, .. } = data.as_ref();
                jetdoc.replace(value);
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref jetdoc, .. } = data.as_ref();
                jetdoc.replace(value);
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref jetdoc, .. } = data.as_ref();
                jetdoc.replace(value);
            },
            _ => panic!(),
        }
    }

    pub fn parameters(&self) -> SharedArray<Rc<FunctionTypeParameter>> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::FunctionType(data)) => {
                let FunctionTypeData { ref parameters, .. } = data.as_ref();
                parameters.clone()
            },
            _ => panic!(),
        }
    }

    pub fn result_type(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::FunctionType(data)) => {
                let FunctionTypeData { ref result_type, .. } = data.as_ref();
                result_type.clone()
            },
            _ => panic!(),
        }
    }

    /// Element types of a tuple type.
    pub fn element_types(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::TupleType(data)) => {
                let TupleTypeData { ref element_types } = data.as_ref();
                element_types.clone()
            },
            _ => panic!(),
        }
    }

    pub fn base(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::NullableType(ref base)) => {
                base.clone()
            },
            _ => panic!(),
        }
    }

    /// Indicates whether a type includes the `null` value or not.
    pub fn includes_null(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::AnyType) => { return true; },
            SymbolKind::Type(TypeKind::NullableType(_)) => { return true; },
            _ => {
                if self.is_type() {
                    return false;
                }
                panic!();
            },
        }
    }

    pub fn origin(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.clone(),
            _ => panic!(),
        }
    }

    pub fn substitute_types(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.substitute_types.clone(),
            _ => panic!(),
        }
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::AnyType) => "*".into(),
            SymbolKind::Type(TypeKind::VoidType) => "void".into(),
            SymbolKind::Type(TypeKind::ClassType(_)) |
            SymbolKind::Type(TypeKind::InterfaceType(_)) => {
                let name_1 = self.fully_qualified_name();
                let mut p = String::new();
                if let Some(type_parameters) = self.type_parameters() {
                    p = ".<".to_owned() + &type_parameters.iter().map(|p| p.to_string()).collect::<Vec<String>>().join(", ") + ">";
                }
                name_1 + &p
            },
            SymbolKind::Type(TypeKind::EnumType(_)) => self.fully_qualified_name(),
            SymbolKind::Type(TypeKind::FunctionType(ft)) => {
                let mut p = vec![];
                for p_1 in ft.parameters.iter() {
                    match p_1.kind {
                        ParameterKind::Required => {
                            p.push(format!("{}: {}", p_1.name, p_1.static_type.to_string()));
                        },
                        ParameterKind::Optional => {
                            p.push(format!("{}?: {}", p_1.name, p_1.static_type.to_string()));
                        },
                        ParameterKind::Rest => {
                            p.push(format!("...{}: {}", p_1.name, p_1.static_type.to_string()));
                        },
                    }
                }
                format!("function({}): {}", p.join(", "), ft.result_type.to_string())
            },
            SymbolKind::Type(TypeKind::TupleType(tt)) => {
                format!("[{}]", tt.element_types.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", "))
            },
            SymbolKind::Type(TypeKind::NullableType(base)) => {
                if base.is_function_type() {
                    format!("?{}", base.to_string())
                } else {
                    format!("{}?", base.to_string())
                }
            },
            SymbolKind::Type(TypeKind::TypeParameterType(_)) => self.name(),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(t)) => {
                let name_1 = self.fully_qualified_name();
                let p = ".<".to_owned() + &t.substitute_types.iter().map(|t| t.to_string()).collect::<Vec<String>>().join(", ") + ">";
                name_1 + &p
            },
            _ => panic!(),
        }
    }
}

pub(crate) enum SymbolKind {
    Unresolved,
    Type(TypeKind),
}

pub(crate) enum TypeKind {
    AnyType,
    VoidType,
    ClassType(Rc<ClassTypeData>),
    EnumType(Rc<EnumTypeData>),
    InterfaceType(Rc<InterfaceTypeData>),
    FunctionType(Rc<FunctionTypeData>),
    TupleType(Rc<TupleTypeData>),
    NullableType(Symbol),
    TypeParameterType(Rc<TypeParameterTypeData>),
    TypeAfterExplicitTypeSubstitution(Rc<TypeAfterExplicitTypeSubstitutionData>),
}

pub(crate) struct ClassTypeData {
    pub name: String,
    pub visibility: Cell<Visibility>,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub extends_class: RefCell<Option<Symbol>>,
    pub implements: SharedArray<Symbol>,
    pub flags: Cell<ClassTypeFlags>,
    pub type_parameters: RefCell<Option<SharedArray<Symbol>>>,
    pub static_properties: SharedMap<String, Symbol>,
    pub constructor_function: RefCell<Option<Symbol>>,
    pub prototype: SharedMap<String, Symbol>,
    pub proxies: SharedMap<ProxyKind, Symbol>,
    pub list_of_to_proxies: SharedMap<Symbol, Symbol>,
    pub limited_subclasses: SharedArray<Symbol>,
    pub plain_metadata: SharedArray<Rc<PlainMetadata>>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
}

pub(crate) struct EnumTypeData {
    pub name: String,
    pub visibility: Cell<Visibility>,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub extends_class: RefCell<Option<Symbol>>,
    pub representation_type: RefCell<Option<Symbol>>,
    pub is_set_enumeration: bool,
    pub static_properties: SharedMap<String, Symbol>,
    pub prototype: SharedMap<String, Symbol>,
    pub proxies: SharedMap<ProxyKind, Symbol>,
    pub list_of_to_proxies: SharedMap<Symbol, Symbol>,
    pub enumeration_members: SharedMap<String, AbstractRangeNumber>,
    pub plain_metadata: SharedArray<Rc<PlainMetadata>>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
}

pub(crate) struct InterfaceTypeData {
    pub name: String,
    pub visibility: Cell<Visibility>,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub extends_interfaces: SharedArray<Symbol>,
    pub type_parameters: RefCell<Option<SharedArray<Symbol>>>,
    pub prototype: SharedMap<String, Symbol>,
    pub limited_implementors: SharedArray<Symbol>,
    pub plain_metadata: SharedArray<Rc<PlainMetadata>>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
}

pub(crate) struct FunctionTypeData {
    pub parameters: SharedArray<Rc<FunctionTypeParameter>>,
    pub result_type: Symbol,
}

pub(crate) struct TupleTypeData {
    pub element_types: SharedArray<Symbol>,
}

pub(crate) struct TypeParameterTypeData {
    pub name: String,
}

pub(crate) struct TypeAfterExplicitTypeSubstitutionData {
    pub origin: Symbol,
    pub substitute_types: SharedArray<Symbol>,
    pub static_properties: RefCell<Option<SharedMap<String, Symbol>>>,
    pub constructor_function: RefCell<Option<Symbol>>,
    pub prototype: RefCell<Option<SharedMap<String, Symbol>>>,
    pub proxies: RefCell<Option<SharedMap<ProxyKind, Symbol>>>,
    pub list_of_to_proxies: RefCell<Option<SharedMap<Symbol, Symbol>>>,
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub(crate) struct ClassTypeFlags: u8 {
        const IS_FINAL = 0b00000001;
        const IS_STATIC = 0b00000010;
        const IS_ABSTRACT = 0b00000100;
        const ALLOW_LITERAL = 0b00001000;
    }
}

/// Unresolved symbol.
///
/// # Supported methods
/// 
/// * `is_unresolved()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Unresolved(pub Symbol);

impl Deref for Unresolved {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_unresolved());
        &self.0
    }
}

/// Any type (`*`) symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_any_type()`
/// * `to_string()`
/// * `includes_null()` — Returns `true`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct AnyType(pub Symbol);

impl Deref for AnyType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_any_type());
        &self.0
    }
}

/// Void type (`void`) symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_void_type()`
/// * `to_string()`
/// * `includes_null()` — Returns `false`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct VoidType(pub Symbol);

impl Deref for VoidType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_void_type());
        &self.0
    }
}

/// Class type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_class_type()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `is_abstract()`
/// * `set_is_abstract()`
/// * `is_final()`
/// * `set_is_final()`
/// * `is_static()`
/// * `set_is_static()`
/// * `allow_literal()`
/// * `set_allow_literal()`
/// * `implements()` — Implements list of the class.
/// * `name()` — Unqualified name.
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `extends_class()`
/// * `set_extends_class()`
/// * `type_parameters()`
/// * `set_type_parameters()`
/// * `static_properties()`
/// * `constructor_function()`
/// * `set_constructor_function()`
/// * `prototype()`
/// * `proxies()`
/// * `list_of_to_proxies()`
/// * `limited_subclasses()`
/// * `plain_metadata()`
/// * `visibility()`
/// * `set_visibility()`
/// * `jetdoc()`
/// * `set_jetdoc()`
/// * `includes_null()` — Returns `false`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ClassType(pub Symbol);

impl Deref for ClassType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_class_type());
        &self.0
    }
}

/// Enumeration type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_enum_type()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `is_set_enumeration()`
/// * `enumeration_representation_type()`
/// * `set_enumeration_representation_type()`
/// * `name()` — Unqualified name.
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `extends_class()`
/// * `set_extends_class()`
/// * `static_properties()``
/// * `prototype()`
/// * `enumeration_members()`
/// * `proxies()`
/// * `list_of_to_proxies()`
/// * `plain_metadata()`
/// * `visibility()`
/// * `set_visibility()`
/// * `jetdoc()`
/// * `set_jetdoc()`
/// * `includes_null()` — Returns `false`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct EnumType(pub Symbol);

impl Deref for EnumType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_enum_type());
        &self.0
    }
}

/// Interface type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_interface_type()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `name()` — Unqualified name.
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `extends_interfaces()` — Extends list of the interface.
/// * `type_parameters()`
/// * `set_type_parameters()`
/// * `prototype()`
/// * `limited_implementors()`
/// * `plain_metadata()`
/// * `visibility()`
/// * `set_visibility()`
/// * `jetdoc()`
/// * `set_jetdoc()`
/// * `includes_null()` — Returns `false`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct InterfaceType(pub Symbol);

impl Deref for InterfaceType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_interface_type());
        &self.0
    }
}

/// Function type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_function_type()`
/// * `to_string()`
/// * `parameters()`
/// * `result_type()`
/// * `includes_null()` — Returns `false`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionType(pub Symbol);

impl Deref for FunctionType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_function_type());
        &self.0
    }
}

pub struct FunctionTypeParameter {
    pub kind: ParameterKind,
    pub name: String,
    pub static_type: Symbol,
}

/// Tuple type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_tuple_type()`
/// * `to_string()`
/// * `element_types()`
/// * `includes_null()` — Returns `false`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TupleType(pub Symbol);

impl Deref for TupleType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_tuple_type());
        &self.0
    }
}

/// Nullable type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_nullable_type()`
/// * `to_string()`
/// * `base()`
/// * `includes_null()` — Returns `true`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct NullableType(pub Symbol);

impl Deref for NullableType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_nullable_type());
        &self.0
    }
}

/// Type parameter type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_type_parameter_type()`
/// * `to_string()`
/// * `name()`
/// * `includes_null()` — Returns `false`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TypeParameterType(pub Symbol);

impl Deref for TypeParameterType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_type_parameter_type());
        &self.0
    }
}

/// Symbol for a type after an explicit type substitution.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_type_after_explicit_type_substitution()`
/// * `is_class_type()`
/// * `is_interface_type()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `origin()`
/// * `substitute_types()`
/// * `is_abstract()`
/// * `is_final()`
/// * `is_static()`
/// * `allow_literal()`
/// * `implements()` — Implements list of a class.
/// * `extends_interfaces()` — Extends list of an interface.
/// * `name()` — Unqualified name.
/// * `parent_definition()`
/// * `extends_class()`
/// * `type_parameters()`
/// * `static_properties()`
/// * `constructor_function()`
/// * `prototype()`
/// * `proxies()`
/// * `list_of_to_proxies()`
/// * `plain_metadata()`
/// * `visibility()`
/// * `jetdoc()`
/// * `includes_null()` — Returns `false`.
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TypeAfterExplicitTypeSubstitution(pub Symbol);

impl Deref for TypeAfterExplicitTypeSubstitution {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_type_after_explicit_type_substitution());
        &self.0
    }
}