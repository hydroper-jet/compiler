use crate::ns::*;
use std::fmt::Debug;
use std::hash::Hash;
use std::ops::Deref;

use bitflags::bitflags;

/// An union data type that represents one of several symbols of the
/// Jet semantics.
///
/// The several `is` prefix methods are used to test a `Symbol` reference
/// against a symbol kind. The miscellaneous structures that wrap a `Symbol` such as `ClassType`
/// and `Package` are used both for kind assertions and to describe supported methods.
/// Methods that are not compatible with a symbol kind result in a panic.
#[derive(Clone)]
pub struct Symbol(pub(crate) Weak<SymbolKind>);

impl Debug for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Symbol()")
    }
}

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

    pub fn is_block_statement(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::BlockStatement(_))
    }

    pub fn is_variable_definition_directive(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::VariableDefinitionDirective(_))
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

    pub fn is_origin_class_type(&self) -> bool {
        match self.0.upgrade().unwrap().as_ref() {
            SymbolKind::Type(TypeKind::ClassType(_)) => true,
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

    pub fn is_origin_interface_type(&self) -> bool {
        match self.0.upgrade().unwrap().as_ref() {
            SymbolKind::Type(TypeKind::InterfaceType(_)) => true,
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

    pub fn is_alias(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Alias(_))
    }

    pub fn is_package(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Package(_))
    }

    pub fn is_package_set(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::PackageSet(_))
    }

    pub fn is_variable_property(&self) -> bool {
        matches!(
            self.0.upgrade().unwrap().as_ref(),
            SymbolKind::VariableProperty(_) | SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(_)
        )
    }

    pub fn is_origin_variable_property(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::VariableProperty(_))
    }

    pub fn is_variable_property_after_indirect_type_substitution(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(_))
    }

    pub fn is_virtual_property(&self) -> bool {
        matches!(
            self.0.upgrade().unwrap().as_ref(),
            SymbolKind::VirtualProperty(_) | SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(_)
        )
    }

    pub fn is_origin_virtual_property(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::VirtualProperty(_))
    }

    pub fn is_virtual_property_after_indirect_type_substitution(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(_))
    }

    pub fn is_function(&self) -> bool {
        matches!(
            self.0.upgrade().unwrap().as_ref(),
            SymbolKind::Function(_) | SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(_)
        )
    }

    pub fn is_origin_function(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Function(_))
    }

    pub fn is_function_after_explicit_or_indirect_type_substitution(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(_))
    }

    pub fn is_scope(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Scope(_, _))
    }

    pub fn is_with_scope(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Scope(_, Some(ScopeKind::With { .. })))
    }

    pub fn is_filter_operator_scope(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Scope(_, Some(ScopeKind::FilterOperator { .. })))
    }

    pub fn is_activation_scope(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Scope(_, Some(ScopeKind::Activation(_))))
    }

    pub fn is_class_scope(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Scope(_, Some(ScopeKind::Class { .. })))
    }

    pub fn is_enum_scope(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Scope(_, Some(ScopeKind::Enum { .. })))
    }

    pub fn is_interface_scope(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Scope(_, Some(ScopeKind::Interface { .. })))
    }

    pub fn is_package_scope(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Scope(_, Some(ScopeKind::Package { .. })))
    }

    pub fn is_value(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Value(_, _))
    }

    pub fn is_embed_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Embed(_))
    }

    pub fn is_import_meta(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::ImportMeta)
    }

    pub fn is_import_meta_env(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::ImportMetaEnv)
    }

    pub fn is_constant(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Constant(_))
    }

    pub fn is_undefined_constant(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Constant(ConstantKind::Undefined))
    }

    pub fn is_null_constant(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Constant(ConstantKind::Null))
    }

    pub fn is_string_constant(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Constant(ConstantKind::String(_)))
    }

    pub fn is_char_constant(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Constant(ConstantKind::Char(_)))
    }

    pub fn is_boolean_constant(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Constant(ConstantKind::Boolean(_)))
    }

    pub fn is_number_constant(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Constant(ConstantKind::Number(_)))
    }

    pub fn is_enum_constant(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Constant(ConstantKind::Enum(_)))
    }

    pub fn is_this_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::This)
    }

    pub fn is_conversion_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Conversion(_))
    }

    pub fn is_import_meta_output(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::ImportMetaOutput)
    }

    pub fn is_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Reference(_))
    }

    pub fn is_type_as_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Type { .. })
    }

    pub fn is_xml_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Xml { .. })
    }

    pub fn is_dynamic_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Dynamic { .. })
    }

    pub fn is_static_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Static { .. })
    }

    pub fn is_instance_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Instance { .. })
    }

    pub fn is_proxy_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Proxy { .. })
    }

    pub fn is_tuple_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Tuple { .. })
    }

    pub fn is_scope_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Scope { .. })
    }

    pub fn is_dynamic_scope_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::DynamicScope { .. })
    }

    pub fn is_package_reference_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else { return false; };
        let ValueKind::Reference(data) = data.as_ref() else { return false; };
        matches!(data.as_ref(), ReferenceValueKind::Package { .. })
    }

    pub fn is_function_value(&self) -> bool {
        let data = self.0.upgrade().unwrap();
        let SymbolKind::Value(_, Some(data)) = data.as_ref() else {
            return false;
        };
        matches!(data.as_ref(), ValueKind::Function { .. })
    }

    /// Performs type substitution. Invoking this method is equivalent to
    /// `TypeSubstitution(&mut host).execute(&symbol, &type_parameters, &substitute_types)`.
    pub fn type_substitution(&self, host: &SymbolHost, type_parameters: &SharedArray<Symbol>, substitute_types: &SharedArray<Symbol>) -> Self {
        TypeSubstitution(host).execute(self, type_parameters, substitute_types)
    }

    pub fn name(&self) -> String {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => data.name.clone(),
            SymbolKind::Type(TypeKind::EnumType(data)) => data.name.clone(),
            SymbolKind::Type(TypeKind::InterfaceType(data)) => data.name.clone(),
            SymbolKind::Type(TypeKind::TypeParameterType(data)) => data.name.clone(),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.name(),
            SymbolKind::Alias(data) => data.name.clone(),
            SymbolKind::Package(data) => data.name.clone(),
            SymbolKind::PackageSet(data) => data.name.clone(),
            SymbolKind::VariableProperty(data) => data.name.clone(),
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.origin.name(),
            SymbolKind::VirtualProperty(data) => data.name.clone(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.origin.name(),
            SymbolKind::Function(data) => data.name.clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.name(),
            _ => panic!(),
        }
    }

    pub fn fully_qualified_name(&self) -> String {
        self.fully_qualified_name_list().join(".")
    }

    pub fn fully_qualified_name_list(&self) -> Vec<String> {
        if let SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(ref t)) = self.0.upgrade().unwrap().as_ref() {
            return t.origin.fully_qualified_name_list();
        }
        let mut r: Vec<String> = vec![];
        let mut p = Some(self.clone());
        while let Some(p1) = p {
            let name = p1.name();
            if !name.is_empty() {
                r.insert(0, name);
            }
            p = p1.parent_definition();
        }
        r
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
            SymbolKind::Type(TypeKind::ClassType(data)) => data.flags.borrow().contains(ClassTypeFlags::IS_ABSTRACT),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.is_abstract(),
            SymbolKind::Function(data) => data.flags.borrow().contains(FunctionSymbolFlags::IS_ABSTRACT),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.is_abstract(),
            _ => panic!(),
        }
    }

    pub fn set_is_abstract(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                data.flags.borrow_mut().set(ClassTypeFlags::IS_ABSTRACT, value);
            },
            SymbolKind::Function(data) => {
                data.flags.borrow_mut().set(FunctionSymbolFlags::IS_ABSTRACT, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_final(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => data.flags.borrow().contains(ClassTypeFlags::IS_FINAL) || data.flags.borrow().contains(ClassTypeFlags::ALLOW_LITERAL),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.is_final(),
            SymbolKind::Function(data) => data.flags.borrow().contains(FunctionSymbolFlags::IS_FINAL),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.is_final(),
            _ => panic!(),
        }
    }

    pub fn set_is_final(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                data.flags.borrow_mut().set(ClassTypeFlags::IS_FINAL, value);
            },
            SymbolKind::Function(data) => {
                data.flags.borrow_mut().set(FunctionSymbolFlags::IS_FINAL, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_static(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => data.flags.borrow().contains(ClassTypeFlags::IS_STATIC),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.is_static(),
            _ => panic!(),
        }
    }

    pub fn set_is_static(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                data.flags.borrow_mut().set(ClassTypeFlags::IS_STATIC, value);
            },
            _ => panic!(),
        }
    }

    pub fn allow_literal(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => data.flags.borrow().contains(ClassTypeFlags::ALLOW_LITERAL),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.allow_literal(),
            _ => panic!(),
        }
    }

    pub fn set_allow_literal(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                data.flags.borrow_mut().set(ClassTypeFlags::ALLOW_LITERAL, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_generator(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.flags.borrow().contains(FunctionSymbolFlags::IS_GENERATOR),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.is_generator(),
            _ => panic!(),
        }
    }

    pub fn set_is_generator(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.flags.borrow_mut().set(FunctionSymbolFlags::IS_GENERATOR, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_async(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.flags.borrow().contains(FunctionSymbolFlags::IS_ASYNC),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.is_async(),
            _ => panic!(),
        }
    }

    pub fn set_is_async(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.flags.borrow_mut().set(FunctionSymbolFlags::IS_ASYNC, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_native(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.flags.borrow().contains(FunctionSymbolFlags::IS_NATIVE),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.is_native(),
            _ => panic!(),
        }
    }

    pub fn set_is_native(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.flags.borrow_mut().set(FunctionSymbolFlags::IS_NATIVE, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_optional_interface_method(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.flags.borrow().contains(FunctionSymbolFlags::IS_OPTIONAL_INTERFACE_METHOD),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.is_optional_interface_method(),
            _ => panic!(),
        }
    }

    pub fn set_is_optional_interface_method(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.flags.borrow_mut().set(FunctionSymbolFlags::IS_OPTIONAL_INTERFACE_METHOD, value);
            },
            _ => panic!(),
        }
    }

    pub fn is_overriding(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.flags.borrow().contains(FunctionSymbolFlags::IS_OVERRIDING),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.is_overriding.get(),
            _ => panic!(),
        }
    }

    pub fn set_is_overriding(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.flags.borrow_mut().set(FunctionSymbolFlags::IS_OVERRIDING, value);
            },
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => {
                data.is_overriding.set(value);
            },
            _ => panic!(),
        }
    }

    pub fn is_constructor(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.flags.borrow().contains(FunctionSymbolFlags::IS_CONSTRUCTOR),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.is_constructor(),
            _ => panic!(),
        }
    }

    pub fn set_is_constructor(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.flags.borrow_mut().set(FunctionSymbolFlags::IS_CONSTRUCTOR, value);
            },
            _ => panic!(),
        }
    }

    pub fn clone_constant_value(&self, host: &SymbolHost) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(data_1, Some(data_2)) => {
                match data_2.as_ref() {
                    ValueKind::Constant(k) => match k {
                        ConstantKind::Undefined => host.factory().create_undefined_constant(&data_1.static_type.borrow().clone()),
                        ConstantKind::Null => host.factory().create_null_constant(&data_1.static_type.borrow().clone()),
                        ConstantKind::Boolean(v) => host.factory().create_boolean_constant(*v, &data_1.static_type.borrow().clone()),
                        ConstantKind::Number(v) => host.factory().create_number_constant(v.clone(), &data_1.static_type.borrow().clone()),
                        ConstantKind::String(v) => host.factory().create_string_constant(v.clone(), &data_1.static_type.borrow().clone()),
                        ConstantKind::Enum(v) => host.factory().create_enum_constant(v.clone(), &data_1.static_type.borrow().clone()),
                        ConstantKind::Char(v) => host.factory().create_char_constant(*v, &data_1.static_type.borrow().clone()),
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn implements(&self, host: &SymbolHost) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref implements, .. } = data.as_ref();
                implements.clone()
            },
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => {
                if let Some(r) = data.implements.borrow().as_ref() {
                    return r.clone();
                }
                let r: SharedArray<Symbol> = data.origin.implements(host).iter().map(|t| TypeSubstitution(host).execute(&t, &data.origin.type_parameters().unwrap(), &data.substitute_types)).collect();
                data.implements.replace(Some(r.clone()));
                r
            },
            _ => panic!(),
        }
    }

    pub fn extends_interfaces(&self, host: &SymbolHost) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                let InterfaceTypeData { ref extends_interfaces, .. } = data.as_ref();
                extends_interfaces.clone()
            },
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => {
                if let Some(r) = data.extends_interfaces.borrow().as_ref() {
                    return r.clone();
                }
                let r: SharedArray<Symbol> = data.origin.extends_interfaces(host).iter().map(|t| TypeSubstitution(host).execute(&t, &data.origin.type_parameters().unwrap(), &data.substitute_types)).collect();
                data.extends_interfaces.replace(Some(r.clone()));
                r
            },
            _ => panic!(),
        }
    }

    pub fn parent_definition(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => data.parent_definition.borrow().clone(),
            SymbolKind::Type(TypeKind::EnumType(data)) => data.parent_definition.borrow().clone(),
            SymbolKind::Type(TypeKind::InterfaceType(data)) => data.parent_definition.borrow().clone(),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.parent_definition(),
            SymbolKind::Alias(data) => data.parent_definition.borrow().clone(),
            SymbolKind::Package(data) => data.parent_definition.borrow().clone(),
            SymbolKind::PackageSet(data) => data.parent_definition.borrow().clone(),
            SymbolKind::VariableProperty(data) => data.parent_definition.borrow().clone(),
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.origin.parent_definition(),
            SymbolKind::VirtualProperty(data) => data.parent_definition.borrow().clone(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.origin.parent_definition(),
            SymbolKind::Function(data) => data.parent_definition.borrow().clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.parent_definition(),
            _ => None,
        }
    }

    pub fn set_parent_definition(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            SymbolKind::Alias(data) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            SymbolKind::Package(data) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            SymbolKind::PackageSet(data) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            SymbolKind::VariableProperty(data) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            SymbolKind::VirtualProperty(data) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            SymbolKind::Function(data) => {
                data.parent_definition.replace(value.map(|v| v.clone()));
            },
            _ => panic!(),
        }
    }

    /// The class which a class extends. This field may possibly be
    /// `Unresolved`.
    pub fn extends_class(&self, host: &SymbolHost) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => data.extends_class.borrow().clone(),
            SymbolKind::Type(TypeKind::EnumType(_)) => Some(host.object_type()),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => {
                if let Some(r) = data.extends_class.borrow().as_ref() {
                    return Some(r.clone());
                }
                let r = data.origin.extends_class(host);
                if r.is_none() {
                    return None;
                }
                let r = r.unwrap();
                if r.is_unresolved() {
                    return Some(r.clone());
                }
                let r = TypeSubstitution(host).execute(&r, &data.origin.type_parameters().unwrap(), &data.substitute_types);
                data.extends_class.replace(Some(r.clone()));
                Some(r)
            },
            _ => panic!(),
        }
    }

    pub fn set_extends_class(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                data.extends_class.replace(value.map(|c| c.clone()));
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
    pub fn set_enumeration_representation_type(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                let EnumTypeData { ref representation_type, .. } = data.as_ref();
                representation_type.replace(value.map(|t| t.clone()));
            },
            _ => panic!(),
        }
    }

    pub fn type_parameters(&self) -> Option<SharedArray<Symbol>> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => data.type_parameters.borrow().clone(),
            SymbolKind::Type(TypeKind::InterfaceType(data)) => data.type_parameters.borrow().clone(),
            SymbolKind::Function(data) => data.type_parameters.borrow().clone(),
            _ => panic!(),
        }
    }

    pub fn set_type_parameters(&self, value: Option<&SharedArray<Symbol>>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                data.type_parameters.replace(value.map(|p| p.clone()));
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                data.type_parameters.replace(value.map(|p| p.clone()));
            },
            SymbolKind::Function(data) => {
                data.type_parameters.replace(value.map(|p| p.clone()));
            },
            _ => panic!(),
        }
    }

    pub fn static_properties(&self, host: &SymbolHost) -> SharedMap<String, Symbol> {
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
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => {
                let TypeAfterExplicitTypeSubstitutionData { ref static_properties, .. } = data.as_ref();
                if let Some(r) = static_properties.borrow().as_ref() {
                    return r.clone();
                }
                let r: SharedMap<String, Symbol> = data.origin.static_properties(host).borrow().iter().map(|(name, p)| {
                    let p = TypeSubstitution(host).execute(&p, &data.origin.type_parameters().unwrap(), &data.substitute_types);
                    (name.clone(), p)
                }).collect();
                static_properties.replace(Some(r.clone()));
                r
            },
            _ => panic!(),
        }
    }

    pub fn prototype(&self, host: &SymbolHost) -> SharedMap<String, Symbol> {
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
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => {
                let TypeAfterExplicitTypeSubstitutionData { ref prototype, .. } = data.as_ref();
                if let Some(r) = prototype.borrow().as_ref() {
                    return r.clone();
                }
                let r: SharedMap<String, Symbol> = data.origin.prototype(host).borrow().iter().map(|(name, p)| {
                    let p = TypeSubstitution(host).execute(&p, &data.origin.type_parameters().unwrap(), &data.substitute_types);
                    (name.clone(), p)
                }).collect();
                prototype.replace(Some(r.clone()));
                r
            },
            _ => panic!(),
        }
    }

    pub fn proxies(&self, host: &SymbolHost) -> SharedMap<ProxyKind, Symbol> {
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
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => {
                let TypeAfterExplicitTypeSubstitutionData { ref proxies, .. } = data.as_ref();
                if let Some(r) = proxies.borrow().as_ref() {
                    return r.clone();
                }
                let r: SharedMap<ProxyKind, Symbol> = data.origin.proxies(host).borrow().iter().map(|(kind, p)| {
                    let p = TypeSubstitution(host).execute(&p, &data.origin.type_parameters().unwrap(), &data.substitute_types);
                    (*kind, p)
                }).collect();
                proxies.replace(Some(r.clone()));
                r
            },
            _ => panic!(),
        }
    }

    pub fn list_of_to_proxies(&self, host: &SymbolHost) -> SharedArray<Symbol> {
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
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => {
                let TypeAfterExplicitTypeSubstitutionData { ref list_of_to_proxies, .. } = data.as_ref();
                if let Some(r) = list_of_to_proxies.borrow().as_ref() {
                    return r.clone();
                }
                let type_parameters = data.origin.type_parameters().unwrap();
                let r: SharedArray<Symbol> = data.origin.list_of_to_proxies(host).iter().map(|proxy_function| {
                    TypeSubstitution(host).execute(&proxy_function, &type_parameters, &data.substitute_types)
                }).collect();
                list_of_to_proxies.replace(Some(r.clone()));
                r
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

    pub fn constructor_function(&self, host: &SymbolHost) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref constructor_function, .. } = data.as_ref();
                constructor_function.borrow().clone()
            },
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => {
                if let Some(r) = data.constructor_function.borrow().as_ref() {
                    return Some(r.clone());
                }
                let r = data.origin.constructor_function(host);
                if r.is_none() {
                    return None;
                }
                let r = r.unwrap();
                let r = TypeSubstitution(host).execute(&r, &data.origin.type_parameters().unwrap(), &data.substitute_types);
                data.constructor_function.replace(Some(r.clone()));
                Some(r)
            },
            _ => panic!(),
        }
    }

    pub fn set_constructor_function(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                let ClassTypeData { ref constructor_function, .. } = data.as_ref();
                constructor_function.replace(value.map(|f| f.clone()));
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
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.plain_metadata(),
            SymbolKind::Alias(data) => data.plain_metadata.clone(),
            SymbolKind::VariableProperty(data) => data.plain_metadata.clone(),
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.origin.plain_metadata(),
            SymbolKind::Function(data) => data.plain_metadata.clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.plain_metadata(),
            SymbolKind::BlockStatement(data) => data.plain_metadata.clone(),
            _ => panic!(),
        }
    }

    pub fn visibility(&self) -> Visibility {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => data.visibility.get(),
            SymbolKind::Type(TypeKind::EnumType(data)) => data.visibility.get(),
            SymbolKind::Type(TypeKind::InterfaceType(data)) => data.visibility.get(),
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.visibility(),
            SymbolKind::Alias(data) => data.visibility.get(),
            SymbolKind::PackageSet(data) => data.visibility.get(),
            SymbolKind::VariableProperty(data) => data.visibility.get(),
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.origin.visibility(),
            SymbolKind::VirtualProperty(data) => data.visibility.get(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.origin.visibility(),
            SymbolKind::Function(data) => data.visibility.get(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.visibility(),
            _ => panic!(),
        }
    }

    pub fn set_visibility(&self, value: Visibility) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::ClassType(data)) => {
                data.visibility.set(value);
            },
            SymbolKind::Type(TypeKind::EnumType(data)) => {
                data.visibility.set(value);
            },
            SymbolKind::Type(TypeKind::InterfaceType(data)) => {
                data.visibility.set(value);
            },
            SymbolKind::Alias(data) => {
                data.visibility.set(value);
            },
            SymbolKind::PackageSet(data) => {
                data.visibility.set(value);
            },
            SymbolKind::VariableProperty(data) => {
                data.visibility.set(value);
            },
            SymbolKind::VirtualProperty(data) => {
                data.visibility.set(value);
            },
            SymbolKind::Function(data) => {
                data.visibility.set(value);
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
            SymbolKind::Type(TypeKind::TypeAfterExplicitTypeSubstitution(data)) => data.origin.jetdoc(),
            SymbolKind::Alias(data) => data.jetdoc.borrow().clone(),
            SymbolKind::Package(data) => data.jetdoc.borrow().clone(),
            SymbolKind::PackageSet(data) => data.jetdoc.borrow().clone(),
            SymbolKind::VariableProperty(data) => data.jetdoc.borrow().clone(),
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.origin.jetdoc(),
            SymbolKind::VirtualProperty(data) => data.jetdoc.borrow().clone(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.origin.jetdoc(),
            SymbolKind::Function(data) => data.jetdoc.borrow().clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.jetdoc(),
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
            SymbolKind::Alias(data) => {
                data.jetdoc.replace(value);
            },
            SymbolKind::Package(data) => {
                data.jetdoc.replace(value);
            },
            SymbolKind::PackageSet(data) => {
                data.jetdoc.replace(value);
            },
            SymbolKind::VariableProperty(data) => {
                data.jetdoc.replace(value);
            },
            SymbolKind::VirtualProperty(data) => {
                data.jetdoc.replace(value);
            },
            SymbolKind::Function(data) => {
                data.jetdoc.replace(value);
            },
            _ => panic!(),
        }
    }

    pub fn parameters(&self) -> SharedArray<Rc<ParameterOfFunctionType>> {
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

    pub fn embedded_byte_array(&self) -> Option<Rc<Vec<u8>>> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Embed(data) => data.embedded_byte_array.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn embedded_string(&self) -> Option<Rc<String>> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Embed(data) => data.embedded_string.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn base(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::NullableType(ref base)) => base.clone(),
            SymbolKind::Scope(_, Some(ScopeKind::FilterOperator { base, .. })) => base.clone(),
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Conversion(data) => data.base.clone(),
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Xml { base, .. } => base.clone(),
                            ReferenceValueKind::Dynamic { base, .. } => base.clone(),
                            ReferenceValueKind::Static { base, .. } => base.clone(),
                            ReferenceValueKind::Instance { base, .. } => base.clone(),
                            ReferenceValueKind::Proxy { base, .. } => base.clone(),
                            ReferenceValueKind::Tuple { base, .. } => base.clone(),
                            ReferenceValueKind::Scope { base, .. } => base.clone(),
                            ReferenceValueKind::DynamicScope { base, .. } => base.clone(),
                            ReferenceValueKind::Package { base, .. } => base.clone(),
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    /// Indicates whether a type includes the `undefined` value or not.
    pub fn includes_undefined(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::AnyType) |
            SymbolKind::Type(TypeKind::VoidType) => { return true; },
            _ => {
                if self.is_type() {
                    return false;
                }
                panic!();
            },
        }
    }

    /// Indicates whether a type includes the `null` value or not.
    pub fn includes_null(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::AnyType) |
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
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.origin.clone(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.origin.clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.clone(),
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

    pub fn indirect_type_parameters(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.indirect_type_parameters.clone(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.indirect_type_parameters.clone(),
            _ => panic!(),
        }
    }

    pub fn indirect_substitute_types(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.indirect_substitute_types.clone(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.indirect_substitute_types.clone(),
            _ => panic!(),
        }
    }

    pub fn explicit_or_indirect_type_parameters(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.explicit_or_indirect_type_parameters.clone(),
            _ => panic!(),
        }
    }

    pub fn explicit_or_indirect_substitute_types(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.explicit_or_indirect_substitute_types.clone(),
            _ => panic!(),
        }
    }

    /// The symbol aliased by an `Alias` symbol. Possibly `Unresolved`.
    pub fn alias_of(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Alias(data) => data.alias_of.borrow().clone(),
            _ => panic!(),
        }
    }

    pub fn set_alias_of(&self, alias_of: &Symbol) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Alias(data) => {
                data.alias_of.replace(alias_of.clone());
            },
            _ => panic!(),
        }
    }

    pub fn resolve_alias(&self) -> Symbol {
        if self.is_alias() {
            return self.alias_of().resolve_alias();
        }
        return self.clone();
    }

    pub fn resolve_property(&self, qual: Option<Symbol>, key: SemanticPropertyKey, host: &SymbolHost) -> Result<Option<Symbol>, PropertyResolutionError> {
        PropertyResolution(host).resolve_property(self, qual, key)
    }

    pub fn resolve_property_with_disambiguation(&self, qual: Option<Symbol>, key: SemanticPropertyKey, host: &SymbolHost, disamb: PropertyDisambiguation) -> Result<Option<Symbol>, PropertyResolutionError> {
        PropertyResolution(host).resolve_property_with_disambiguation(self, qual, key, disamb)
    }

    pub fn properties(&self, _host: &SymbolHost) -> SharedMap<String, Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Package(data) => data.properties.clone(),
            SymbolKind::Scope(data, _) => data.properties.clone(),
            _ => panic!(),
        }
    }

    pub fn redirect_packages(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Package(data) => data.redirect_packages.clone(),
            _ => panic!(),
        }
    }

    /// Indicates subpackages of a package from their unqualified name to their `Package` symbol.
    pub fn subpackages(&self) -> SharedMap<String, Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Package(data) => data.subpackages.clone(),
            _ => panic!(),
        }
    }

    /// The packages of a package set.
    pub fn packages(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::PackageSet(data) => data.packages.clone(),
            _ => panic!(),
        }
    }

    /// Static type of a value or property. Possibly `Unresolved`.
    pub fn static_type(&self, host: &SymbolHost) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariableProperty(data) => data.static_type.borrow().clone(),
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => {
                if let Some(r) = data.static_type.borrow().as_ref() {
                    return r.clone();
                }
                let r = data.origin.static_type(host);
                if r.is_unresolved() {
                    return r.clone();
                }
                let r = TypeSubstitution(host).execute(&r, &data.indirect_type_parameters, &data.indirect_substitute_types);
                data.static_type.replace(Some(r.clone()));
                r
            },
            SymbolKind::VirtualProperty(data) => {
                if let Some(r) = data.static_type.borrow().as_ref() {
                    return r.clone();
                }

                let mut deduced_type: Option<Symbol> = None;

                // Deduce [[Type]] from getter
                let getter = data.getter.borrow();
                let getter = getter.as_ref();
                if let Some(getter) = getter {
                    let signature: Symbol = getter.signature(host);
                    if !signature.is_unresolved() {
                        deduced_type = Some(signature.result_type());
                    }
                }

                // Deduce [[Type]] from setter
                let setter = data.setter.borrow();
                let setter = setter.as_ref();
                if let Some(setter) = setter {
                    let signature: Symbol = setter.signature(host);
                    if !signature.is_unresolved() {
                        deduced_type = Some(signature.parameters().get(0).unwrap().static_type.clone());
                    }
                }

                if deduced_type.is_none() {
                    return host.unresolved();
                }

                data.static_type.replace(deduced_type.clone());
                deduced_type.unwrap()
            },
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => {
                if let Some(r) = data.static_type.borrow().as_ref() {
                    return r.clone();
                }
                let r = data.origin.static_type(host);
                if r.is_unresolved() {
                    return r.clone();
                }
                let r = TypeSubstitution(host).execute(&r, &data.indirect_type_parameters, &data.indirect_substitute_types);
                data.static_type.replace(Some(r.clone()));
                r
            },
            SymbolKind::Value(data, _) => data.static_type.borrow().clone(),
            _ => panic!(),
        }
    }

    pub fn set_static_type(&self, value: &Symbol) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariableProperty(data) => {
                data.static_type.replace(value.clone());
            },
            SymbolKind::Value(data, _) => {
                data.static_type.replace(value.clone());
            },
            _ => panic!(),
        }
    }

    pub fn read_only(&self, host: &SymbolHost) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariableProperty(data) => data.read_only.get(),
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.origin.read_only(host),
            SymbolKind::VirtualProperty(data) => data.setter.borrow().is_none(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.origin.read_only(host),
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Type { .. } => true,
                            ReferenceValueKind::Xml { .. } => false,
                            ReferenceValueKind::Dynamic { .. } => false,
                            ReferenceValueKind::Static { property, .. } => property.read_only(host),
                            ReferenceValueKind::Instance { property, .. } => property.read_only(host),
                            ReferenceValueKind::Proxy { base, .. } => {
                                let r = base.static_type(host).has_set_property_proxy(host);
                                if r.is_err() {
                                    return true;
                                }
                                r.unwrap()
                            },
                            ReferenceValueKind::Tuple { .. } => false,
                            ReferenceValueKind::Scope { property, .. } => property.read_only(host),
                            ReferenceValueKind::DynamicScope { .. } => false,
                            ReferenceValueKind::Package { property, .. } => property.read_only(host),
                        }
                    }
                    _ => true,
                }
            },
            _ => true,
        }
    }

    pub fn set_read_only(&self, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariableProperty(data) => {
                data.read_only.set(value);
            },
            _ => panic!(),
        }
    }

    pub fn write_only(&self, host: &SymbolHost) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VirtualProperty(data) => data.getter.borrow().is_none(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.origin.write_only(host),
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Type { .. } => false,
                            ReferenceValueKind::Xml { .. } => false,
                            ReferenceValueKind::Dynamic { .. } => false,
                            ReferenceValueKind::Static { property, .. } => property.write_only(host),
                            ReferenceValueKind::Instance { property, .. } => property.write_only(host),
                            ReferenceValueKind::Proxy { .. } => false,
                            ReferenceValueKind::Tuple { .. } => false,
                            ReferenceValueKind::Scope { property, .. } => property.write_only(host),
                            ReferenceValueKind::DynamicScope { .. } => false,
                            ReferenceValueKind::Package { property, .. } => property.write_only(host),
                        }
                    }
                    _ => true,
                }
            },
            _ => false,
        }
    }

    pub fn deletable(&self, host: &SymbolHost) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Xml { .. } => true,
                            ReferenceValueKind::Dynamic { .. } => true,
                            ReferenceValueKind::Proxy { base, .. } => {
                                let r = base.static_type(host).find_proxy(ProxyKind::DeleteProperty, host);
                                if r.is_err() {
                                    return true;
                                }
                                r.unwrap().is_some()
                            },
                            ReferenceValueKind::DynamicScope { .. } => true,
                            _ => false,
                        }
                    }
                    _ => false,
                }
            },
            _ => false,
        }
    }

    /// If a type is a nullable type, return it as a non nullable type.
    pub fn non_null_type(&self) -> Symbol {
        if self.is_nullable_type() {
            self.base()
        } else {
            self.clone()
        }
    }

    /// Finds proxy in the class inheritance.
    pub fn find_proxy(&self, kind: ProxyKind, host: &SymbolHost) -> Result<Option<Symbol>, DeferVerificationError> {
        if self.is_unresolved() {
            return Ok(None);
        }
        for class in self.non_null_type().descending_class_hierarchy(host).collect::<Vec<_>>() {
            class.throw_if_unresolved()?;
            let proxy = class.proxies(host).get(&kind);
            if proxy.is_some() {
                return Ok(proxy);
            }
        }
        Ok(None)
    }

    /// Indicates whether a `class` or `enum` defines the `setProperty()` proxy.
    pub fn has_set_property_proxy(&self, host: &SymbolHost) -> Result<bool, DeferVerificationError> {
        Ok(self.find_proxy(ProxyKind::SetProperty, host)?.is_some())
    }

    pub fn constant_initializer(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariableProperty(data) => data.constant_initializer.borrow().clone(),
            _ => panic!(),
        }
    }

    pub fn set_constant_initializer(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariableProperty(data) => {
                data.constant_initializer.replace(value.map(|v| v.clone()));
            },
            _ => panic!(),
        }
    }

    /// Indicates whether a variable property is optional for an object initializer.
    pub fn is_optional_variable(&self, host: &SymbolHost) -> Result<bool, DeferVerificationError> {
        let st = self.static_type(host);
        if st.is_unresolved() {
            Err(DeferVerificationError)
        } else {
            Ok(st.includes_null() || st.includes_undefined())
        }
    }

    pub fn getter(&self, host: &SymbolHost) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VirtualProperty(data) => data.getter.borrow().clone(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => {
                if let Some(r) = data.getter.borrow().as_ref() {
                    return Some(r.clone());
                }
                let r = data.origin.getter(host);
                if r.is_none() {
                    return r;
                }
                let r = TypeSubstitution(host).execute(&r.unwrap(), &data.indirect_type_parameters, &data.indirect_substitute_types);
                data.getter.replace(Some(r.clone()));
                Some(r)
            },
            _ => panic!(),
        }
    }

    pub fn set_getter(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VirtualProperty(data) => {
                data.getter.replace(value.map(|v| v.clone()));
            },
            _ => panic!(),
        }
    }

    pub fn setter(&self, host: &SymbolHost) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VirtualProperty(data) => data.setter.borrow().clone(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => {
                if let Some(r) = data.setter.borrow().as_ref() {
                    return Some(r.clone());
                }
                let r = data.origin.setter(host);
                if r.is_none() {
                    return r;
                }
                let r = TypeSubstitution(host).execute(&r.unwrap(), &data.indirect_type_parameters, &data.indirect_substitute_types);
                data.setter.replace(Some(r.clone()));
                Some(r)
            },
            _ => panic!(),
        }
    }

    pub fn set_setter(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VirtualProperty(data) => {
                data.setter.replace(value.map(|v| v.clone()));
            },
            _ => panic!(),
        }
    }

    /// Signature of a function symbol as a structural function type.
    /// Possibly `Unresolved`.
    pub fn signature(&self, host: &SymbolHost) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.signature.borrow().clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => {
                if let Some(r) = data.signature.borrow().as_ref() {
                    return r.clone();
                }
                let r = data.origin.signature(host);
                if r.is_unresolved() {
                    return r.clone();
                }
                let r = TypeSubstitution(host).execute(&r, &data.explicit_or_indirect_substitute_types, &data.explicit_or_indirect_substitute_types);
                data.signature.replace(Some(r.clone()));
                r
            },
            _ => panic!(),
        }
    }

    pub fn set_signature(&self, value: &Symbol) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.signature.replace(value.clone());
            },
            _ => panic!(),
        }
    }

    /// The virtual property to which a function symbol belongs.
    pub fn of_virtual_property(&self, host: &SymbolHost) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.of_virtual_property.borrow().clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => {
                if let Some(r) = data.of_virtual_property.borrow().as_ref() {
                    return Some(r.clone());
                }
                let r = data.origin.of_virtual_property(host);
                if r.is_none() {
                    return None;
                }
                let r = TypeSubstitution(host).execute(&r.unwrap(), &data.explicit_or_indirect_substitute_types, &data.explicit_or_indirect_substitute_types);
                data.of_virtual_property.replace(Some(r.clone()));
                Some(r)
            },
            _ => panic!(),
        }
    }

    pub fn set_of_virtual_property(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.of_virtual_property.replace(value.map(|v| v.clone()));
            },
            _ => panic!(),
        }
    }

    /// Set of function symbols used to override a function symbol.
    pub fn overriden_by(&self, host: &SymbolHost) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.overriden_by.clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => {
                if let Some(r) = data.overriden_by.borrow().as_ref() {
                    return r.clone();
                }
                let r = data.origin.overriden_by(host);
                let r: SharedArray<Symbol> = r.iter().map(|r| TypeSubstitution(host).execute(&r, &data.explicit_or_indirect_substitute_types, &data.explicit_or_indirect_substitute_types)).collect();
                data.overriden_by.replace(Some(r.clone()));
                r
            },
            _ => panic!(),
        }
    }

    pub fn overrides_method(&self, host: &SymbolHost) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.overrides_method.borrow().clone(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => {
                if let Some(r) = data.overrides_method.borrow().as_ref() {
                    return Some(r.clone());
                }
                let r = data.origin.overrides_method(host);
                if r.is_none() {
                    return None;
                }
                let r = TypeSubstitution(host).execute(&r.unwrap(), &data.explicit_or_indirect_substitute_types, &data.explicit_or_indirect_substitute_types);
                data.overrides_method.replace(Some(r.clone()));
                Some(r)
            },
            _ => panic!(),
        }
    }

    pub fn set_overrides_method(&self, value: Option<Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.overrides_method.replace(value);
            },
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => {
                data.overrides_method.replace(value);
            },
            _ => panic!(),
        }
    }

    pub fn parent_scope(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(data, _) => data.parent_scope.borrow().clone(),
            _ => panic!(),
        }
    }

    pub fn set_parent_scope(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(data, _) => {
                data.parent_scope.replace(value.map(|v| v.clone()));
            },
            _ => panic!(),
        }
    }

    pub fn imports(&self) -> SharedMap<String, Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(data, _) => data.imports.clone(),
            _ => panic!(),
        }
    }

    pub fn open_packages(&self) -> SharedArray<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(data, _) => data.open_packages.clone(),
            _ => panic!(),
        }
    }

    pub fn package_aliases(&self) -> SharedMap<String, Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(data, _) => data.package_aliases.clone(),
            _ => panic!(),
        }
    }

    pub fn local_variable_scope_count(&self) -> usize {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(data, _) => data.local_variable_scope_count.get(),
            _ => panic!(),
        }
    }

    pub fn set_local_variable_scope_count(&self, value: usize) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(data, _) => {
                data.local_variable_scope_count.set(value);
            },
            _ => panic!(),
        }
    }

    pub fn object(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::With { object, .. })) => object.clone(),
            _ => panic!(),
        }
    }

    pub fn function(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::Activation(data))) => data.function.clone(),
            _ => panic!(),
        }
    }

    pub fn this(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::Activation(data))) => data.this.borrow().clone(),
            _ => panic!(),
        }
    }

    pub fn set_this(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::Activation(data))) => {
                data.this.replace(value.map(|v| v.clone()));
            },
            _ => panic!(),
        }
    }

    pub fn class(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::Class { class, .. })) => class.clone(),
            SymbolKind::Scope(_, Some(ScopeKind::Enum { class, .. })) => class.clone(),
            _ => panic!(),
        }
    }

    pub fn interface(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::Interface { interface, .. })) => interface.clone(),
            _ => panic!(),
        }
    }

    pub fn package(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::Package { package, .. })) => package.clone(),
            _ => panic!(),
        }
    }

    pub fn string_value(&self) -> String {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Constant(ConstantKind::String(v)) => v.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn char_value(&self) -> char {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Constant(ConstantKind::Char(v)) => v.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn boolean_value(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Constant(ConstantKind::Boolean(v)) => v.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn number_value(&self) -> AbstractRangeNumber {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Constant(ConstantKind::Number(v)) => v.clone(),
                    ValueKind::Constant(ConstantKind::Enum(v)) => v.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn conversion_relationship(&self) -> TypeConversionRelationship {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Conversion(data) => data.relationship.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn conversion_is_optional(&self) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Conversion(data) => data.optional.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn conversion_target(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Conversion(data) => data.target.clone(),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn qualifier(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Xml { qualifier, .. } => qualifier.clone(),
                            ReferenceValueKind::Dynamic { qualifier, .. } => qualifier.clone(),
                            ReferenceValueKind::DynamicScope { qualifier, .. } => qualifier.clone(),
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn key(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Xml { key, .. } => key.clone(),
                            ReferenceValueKind::Dynamic { key, .. } => key.clone(),
                            ReferenceValueKind::DynamicScope { key, .. } => key.clone(),
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn property(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Static { property, .. } => property.clone(),
                            ReferenceValueKind::Instance { property, .. } => property.clone(),
                            ReferenceValueKind::Scope { property, .. } => property.clone(),
                            ReferenceValueKind::Package { property, .. } => property.clone(),
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn disambiguation(&self) -> PropertyDisambiguation {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Xml { disambiguation, .. } => *disambiguation,
                            ReferenceValueKind::Dynamic { disambiguation, .. } => *disambiguation,
                            ReferenceValueKind::DynamicScope { disambiguation, .. } => *disambiguation,
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn proxy(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Proxy { proxy, .. } => proxy.clone(),
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn tuple_index(&self) -> usize {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Tuple { index, .. } => index.clone(),
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn property_is_visible(&self, scope: &Symbol, host: &SymbolHost) -> bool {
        let mut prop = self.clone();
        if prop.is_value() {
            if prop.is_static_reference_value() {
                prop = prop.property();
            } else if prop.is_instance_reference_value() {
                prop = prop.property();
            } else if prop.is_package_reference_value() {
                prop = prop.property();
            } else {
                return true;
            }
        }

        match prop.visibility() {
            Visibility::Public => true,
            Visibility::Internal => {
                let mut p: Option<Symbol> = None;
                if let Some(p1) = prop.parent_definition() {
                    for p1 in p1.descending_definition_hierarchy() {
                        if p1.is_package() {
                            p = Some(p1);
                            break;
                        }
                    }
                }
                if p.is_none() {
                    return true;
                }
                let p = p.unwrap();
                for scope in scope.descending_scope_hierarchy() {
                    if scope.is_package_scope() && scope.package() == p {
                        return true;
                    }
                }
                return false;
            },
            Visibility::Private => {
                let mut t: Option<Symbol> = None;
                if let Some(p) = prop.parent_definition() {
                    for p in p.descending_definition_hierarchy() {
                        if p.is_class_type() || p.is_enum_type() {
                            t = Some(p);
                            break;
                        }
                    }
                }
                if t.is_none() {
                    return false;
                }
                let t = t.unwrap();
                for scope in scope.descending_scope_hierarchy() {
                    if (scope.is_class_scope() || scope.is_enum_scope()) && scope.class() == t {
                        return true;
                    }
                }
                return false;
            },
            Visibility::Protected => {
                let mut t: Option<Symbol> = None;
                if let Some(p) = prop.parent_definition() {
                    for p in p.descending_definition_hierarchy() {
                        if p.is_class_type() || p.is_enum_type() {
                            t = Some(p);
                            break;
                        }
                    }
                }
                if t.is_none() {
                    return false;
                }
                let t = t.unwrap();
                for scope in scope.descending_scope_hierarchy() {
                    if (scope.is_class_scope() || scope.is_enum_scope()) && scope.class().is_equals_or_subtype_of(&t, host) {
                        return true;
                    }
                }
                return false;
            },
        }
    }

    pub fn is_ascending_type_of(&self, possibly_subtype: &Symbol, host: &SymbolHost) -> bool {
        possibly_subtype.is_subtype_of(self, host)
    }

    pub fn is_subtype_of(&self, possibly_ascending_type: &Symbol, host: &SymbolHost) -> bool {
        possibly_ascending_type.is_any_type() || self.all_ascending_types(host).contains(possibly_ascending_type)
    }

    pub fn is_equals_or_subtype_of(&self, other: &Symbol, host: &SymbolHost) -> bool {
        self == other || self.is_subtype_of(other, host)
    }

    /// Returns all ascending types of a type in ascending type order.
    pub fn all_ascending_types(&self, host: &SymbolHost) -> Vec<Symbol> {
        let mut r = vec![];
        let mut r2 = vec![];
        for type_symbol in self.direct_ascending_types(host) {
            if !type_symbol.is_unresolved() {
                for type_symbol in type_symbol.all_ascending_types(host) {
                    if !r.contains(&type_symbol) {
                        r.push(type_symbol.clone());
                    }
                }
            }
            if !r.contains(&type_symbol) {
                r2.push(type_symbol.clone());
            }
        }
        r.extend(r2);
        r
    }

    /// Returns direct ascending types of a type.
    pub fn direct_ascending_types(&self, host: &SymbolHost) -> Vec<Symbol> {
        if self.is_class_type() {
            let mut r: Vec<Symbol> = self.implements(host).iter().collect();
            if let Some(ascending_class) = self.extends_class(host) {
                r.push(ascending_class);
            }
            return r;
        } else if self.is_enum_type() {
            return vec![self.extends_class(host).unwrap()];
        } else if self.is_interface_type() {
            return self.extends_interfaces(host).iter().collect();
        } else if self.is_function_type() {
            return vec![host.function_type()];
        } else if self.is_tuple_type() {
            return vec![host.object_type()];
        }
        return vec![];
    }

    pub fn expect_type(&self) -> Result<Symbol, ExpectedTypeError> {
        if self.is_type_as_reference_value() {
            return Ok(self.referenced_type());
        }
        if self.is_package_reference_value() || self.is_scope_reference_value() {
            return self.property().expect_type();
        }
        if self.is_type() {
            Ok(self.clone())
        } else {
            Err(ExpectedTypeError)
        }
    }

    /// Given a type base, returns its default value.
    pub fn type_default_value(&self, host: &SymbolHost) -> Option<Symbol> {
        if self.is_void_type() {
            Some(host.factory().create_undefined_constant(self))
        } else if self.is_nullable_type() {
            Some(host.factory().create_null_constant(self))
        } else if host.is_numeric_type(self) {
            let v = AbstractRangeNumber::zero(self, host);
            Some(host.factory().create_number_constant(v, self))
        } else if self == &host.boolean_type() {
            Some(host.factory().create_boolean_constant(false, self))
        } else if self == &host.string_type() {
            Some(host.factory().create_string_constant(String::new(), self))
        } else if self == &host.char_type() {
            Some(host.factory().create_char_constant('\x00', self))
        } else if self.is_enum_type() && self.is_set_enumeration() {
            let v = AbstractRangeNumber::zero(&self.enumeration_representation_type().unwrap(), host);
            Some(host.factory().create_enum_constant(v, self))
        } else {
            None
        }
    }

    /// The internal *PropertyStaticType*() function.
    pub fn property_static_type(&self, host: &SymbolHost) -> Symbol {
        if self.is_variable_property() || self.is_virtual_property() {
            return self.static_type(host);
        }
        if self.is_function() {
            return self.signature(host);
        }
        assert!(self.is_type());
        return host.class_type();
    }

    /// Throws `DeferVerificationError` error if the symbol is `Unresolved`.
    pub fn throw_if_unresolved(&self) -> Result<(), DeferVerificationError> {
        if self.is_unresolved() {
            Err(DeferVerificationError)
        } else {
            Ok(())
        }
    }

    /// Iterator over a descending class hierarchy.
    pub fn descending_class_hierarchy<'a>(&self, host: &'a SymbolHost) -> DescendingClassHierarchy<'a> {
        DescendingClassHierarchy(Some(self.clone()), host)
    }

    /// Iterator over a descending scope hierarchy.
    pub fn descending_scope_hierarchy(&self) -> DescendingScopeHierarchy {
        DescendingScopeHierarchy(Some(self.clone()))
    }

    /// Iterator over a descending definition hierarchy.
    pub fn descending_definition_hierarchy(&self) -> DescendingDefinitionHierarchy {
        DescendingDefinitionHierarchy(Some(self.clone()))
    }

    /// The internal *WrapPropertyReference*() function.
    pub fn wrap_property_reference(&self, host: &SymbolHost) -> Symbol {
        if self.is_type() && (self.is_void_type() || self.is_any_type() || self.is_function_type() || self.is_tuple_type() || self.is_nullable_type()) {
            return host.factory().create_type_as_reference_value(&self);
        }
        let parent = self.parent_definition().unwrap();
        if parent.is_class_type() || parent.is_enum_type() {
            return host.factory().create_static_reference_value(&parent, &self);
        }
        if parent.is_package() {
            return host.factory().create_package_reference_value(&parent, &self);
        }
        assert!(self.is_scope());
        return host.factory().create_scope_reference_value(&parent, &self);
    }

    pub fn is_floating_point_type_of_wider_range_than(&self, other: &Symbol, host: &SymbolHost) -> bool {
        let number_type = host.number_type();
        let single_type = host.single_type();

        if self == &number_type {
            other == &single_type
        } else if self == &single_type {
            false
        } else {
            false
        }
    }

    pub fn is_integer_type_of_wider_range_than(&self, other: &Symbol, host: &SymbolHost) -> bool {
        let long_type = host.long_type();
        let big_int_type = host.big_int_type();

        if self == &long_type {
            false
        } else if self == &big_int_type {
            [long_type].contains(other)
        } else {
            false
        }
    }

    pub fn type_after_substitution_has_origin(&self, origin: &Symbol) -> bool {
        self.is_type_after_explicit_type_substitution() && &self.origin() == origin
    }

    /// If a type is `[T]`, returns `T`, either as an origin type parameter
    /// or as a substitute type.
    pub fn array_element_type(&self, host: &SymbolHost) -> Result<Option<Symbol>, DeferVerificationError> {
        let array_type = host.array_type();
        array_type.throw_if_unresolved()?;
        if self == &array_type {
            Ok(Some(array_type.type_parameters().unwrap().get(0).unwrap()))
        } else if self.type_after_substitution_has_origin(&array_type) {
            Ok(Some(self.substitute_types().get(0).unwrap()))
        } else {
            Ok(None)
        }
    }

    /// If a type is `Map.<K, V>`, returns (`K`, `V`), either as origin type parameters
    /// or as substitute types.
    pub fn map_key_value_types(&self, host: &SymbolHost) -> Result<Option<(Symbol, Symbol)>, DeferVerificationError> {
        let map_type = host.map_type();
        map_type.throw_if_unresolved()?;
        if self == &map_type {
            let params = map_type.type_parameters().unwrap();
            Ok(Some((params.get(0).unwrap(), params.get(1).unwrap())))
        } else if self.type_after_substitution_has_origin(&map_type) {
            let sub = map_type.substitute_types();
            Ok(Some((sub.get(0).unwrap(), sub.get(1).unwrap())))
        } else {
            Ok(None)
        }
    }

    pub(crate) fn not_overriden_abstract_getter(&self, getter_from_base_class: &Symbol, subclass: &Symbol, host: &SymbolHost) -> bool {
        if getter_from_base_class.is_abstract() {
            let prop2 = subclass.prototype(host).get(&getter_from_base_class.name());
            prop2.is_none() || !prop2.clone().unwrap().is_virtual_property() || prop2.unwrap().getter(host).is_none()
        } else {
            false
        }
    }

    pub(crate) fn not_overriden_abstract_setter(&self, setter_from_base_class: &Symbol, subclass: &Symbol, host: &SymbolHost) -> bool {
        if setter_from_base_class.is_abstract() {
            let prop2 = subclass.prototype(host).get(&setter_from_base_class.name());
            prop2.is_none() || !prop2.clone().unwrap().is_virtual_property() || prop2.unwrap().setter(host).is_none()
        } else {
            false
        }
    }

    pub fn activation_scope(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => data.activation_scope.borrow().clone(),
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Function { activation_scope } => Some(activation_scope.clone()),
                    _ => panic!(),
                }
            },
            _ => panic!(),
        }
    }

    pub fn set_activation_scope(&self, value: Option<Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Function(data) => {
                data.activation_scope.replace(value);
            },
            _ => panic!(),
        }
    }

    pub fn find_activation(&self) -> Option<Symbol> {
        for scope in self.descending_scope_hierarchy() {
            if scope.is_activation_scope() {
                return Some(scope);
            }
        }
        return None
    }

    pub fn property_has_capture(&self, property: &Symbol) -> bool {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::Activation(data))) => {
                if let Some(set) = data.property_has_capture.borrow().as_ref() {
                    set.includes(property)
                } else {
                    false
                }
            },
            _ => panic!(),
        }
    }

    pub fn set_property_has_capture(&self, property: &Symbol, value: bool) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Scope(_, Some(ScopeKind::Activation(data))) => {
                if let Some(set) = data.property_has_capture.borrow_mut().as_mut() {
                    if value {
                        if !set.includes(property) {
                            set.push(property.clone());
                        }
                    } else {
                        let i = set.index_of(property);
                        if let Some(i) = i {
                            set.remove(i);
                        }
                    }
                } else if value {
                    data.property_has_capture.replace(Some(shared_array![property.clone()]));
                }
            },
            _ => panic!(),
        }
    }

    /// Assuming a `Scope` base, if the specified reference is a property that has been captured by another
    /// activation in the base *current scope*, `check_property_has_capture()`
    /// marks this property as captured in its respective activation.
    ///
    /// `check_property_has_capture` performs the following action:
    /// 
    /// ```
    /// let current_scope = self;
    /// if reference.is_scope_reference_value() && reference.base().find_activation().unwrap() != current_scope.find_activation().unwrap() {
    ///     reference.base().find_activation().unwrap().set_property_has_capture(reference.property(), true);
    /// }
    /// ```
    /// 
    /// # Example
    /// 
    /// ```ignore
    /// current_scope.check_property_has_capture(&reference);
    /// ```
    pub fn check_property_has_capture(&self, reference: &Symbol) {
        let current_scope = self;
        if reference.is_scope_reference_value() && reference.base().find_activation().unwrap() != current_scope.find_activation().unwrap() {
            reference.base().find_activation().unwrap().set_property_has_capture(&reference.property(), true);
        }
    }

    pub fn is_array_type_of_any(&self, host: &SymbolHost) -> bool {
        self.type_after_substitution_has_origin(&host.array_type()) &&
        self.substitute_types().get(0).unwrap().is_any_type()
    }

    pub fn is_map_type_of_any_any(&self, host: &SymbolHost) -> bool {
        self.type_after_substitution_has_origin(&host.map_type()) &&
        self.substitute_types().get(0).unwrap().is_any_type() &&
        self.substitute_types().get(1).unwrap().is_any_type()
    }

    pub fn shadow_scope(&self) -> Option<Symbol> {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariableDefinitionDirective(data) => data.shadow_scope.borrow().clone(),
            _ => panic!(),
        }
    }

    pub fn set_shadow_scope(&self, value: Option<&Symbol>) {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::VariableDefinitionDirective(data) => {
                data.shadow_scope.replace(value.cloned());
            },
            _ => panic!(),
        }
    }

    pub fn referenced_type(&self) -> Symbol {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Value(_, Some(data)) => {
                match data.as_ref() {
                    ValueKind::Conversion(data) => data.base.clone(),
                    ValueKind::Reference(data) => {
                        match data.as_ref() {
                            ReferenceValueKind::Type { referenced_type, .. } => referenced_type.clone(),
                            _ => panic!(),
                        }
                    },
                    _ => panic!(),
                }
            },
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
            SymbolKind::Alias(_) |
            SymbolKind::Package(_) |
            SymbolKind::PackageSet(_) |
            SymbolKind::VariableProperty(_) |
            SymbolKind::VirtualProperty(_) |
            SymbolKind::Function(_) => self.fully_qualified_name(),
            SymbolKind::VariablePropertyAfterIndirectTypeSubstitution(data) => data.origin.fully_qualified_name(),
            SymbolKind::VirtualPropertyAfterIndirectTypeSubstitution(data) => data.origin.fully_qualified_name(),
            SymbolKind::FunctionAfterExplicitOrIndirectTypeSubstitution(data) => data.origin.fully_qualified_name(),
            _ => panic!(),
        }
    }
}

pub struct DescendingClassHierarchy<'a>(Option<Symbol>, &'a SymbolHost);

impl<'a> Iterator for DescendingClassHierarchy<'a> {
    type Item = Symbol;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.0.clone() {
            if r.is_unresolved() {
                self.0 = None;
            } else {
                self.0 = r.extends_class(self.1);
            }
            Some(r)
        } else {
            None
        }
    }
}

pub struct DescendingScopeHierarchy(Option<Symbol>);

impl Iterator for DescendingScopeHierarchy {
    type Item = Symbol;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.0.clone() {
            self.0 = r.parent_scope();
            Some(r)
        } else {
            None
        }
    }
}

pub struct DescendingDefinitionHierarchy(Option<Symbol>);

impl Iterator for DescendingDefinitionHierarchy {
    type Item = Symbol;
    fn next(&mut self) -> Option<Self::Item> {
        if let Some(r) = self.0.clone() {
            self.0 = r.parent_definition();
            Some(r)
        } else {
            None
        }
    }
}

pub(crate) enum SymbolKind {
    Unresolved,
    Type(TypeKind),
    Alias(Rc<AliasData>),
    Package(Rc<PackageData>),
    PackageSet(Rc<PackageSetData>),
    VariableProperty(Rc<VariablePropertyData>),
    VariablePropertyAfterIndirectTypeSubstitution(Rc<VariablePropertyAfterIndirectTypeSubstitutionData>),
    VirtualProperty(Rc<VirtualPropertyData>),
    VirtualPropertyAfterIndirectTypeSubstitution(Rc<VirtualPropertyAfterIndirectTypeSubstitutionData>),
    Function(Rc<FunctionSymbolData>),
    FunctionAfterExplicitOrIndirectTypeSubstitution(Rc<FunctionAfterExplicitOrIndirectTypeSubstitutionData>),
    Scope(Rc<ScopeData>, Option<ScopeKind>),
    Value(ValueData, Option<Rc<ValueKind>>),
    BlockStatement(Rc<BlockStatementSymbolData>),
    VariableDefinitionDirective(Rc<VariableDefinitionDirectiveSymbolData>),
}

pub(crate) struct BlockStatementSymbolData {
    pub plain_metadata: SharedArray<Rc<PlainMetadata>>,
}

pub(crate) struct VariableDefinitionDirectiveSymbolData {
    pub shadow_scope: RefCell<Option<Symbol>>,
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
    pub flags: RefCell<ClassTypeFlags>,
    pub type_parameters: RefCell<Option<SharedArray<Symbol>>>,
    pub static_properties: SharedMap<String, Symbol>,
    pub constructor_function: RefCell<Option<Symbol>>,
    pub prototype: SharedMap<String, Symbol>,
    pub proxies: SharedMap<ProxyKind, Symbol>,
    pub list_of_to_proxies: SharedArray<Symbol>,
    pub limited_subclasses: SharedArray<Symbol>,
    pub plain_metadata: SharedArray<Rc<PlainMetadata>>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
}

pub(crate) struct EnumTypeData {
    pub name: String,
    pub visibility: Cell<Visibility>,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub representation_type: RefCell<Option<Symbol>>,
    pub is_set_enumeration: bool,
    pub static_properties: SharedMap<String, Symbol>,
    pub prototype: SharedMap<String, Symbol>,
    pub proxies: SharedMap<ProxyKind, Symbol>,
    pub list_of_to_proxies: SharedArray<Symbol>,
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
    pub parameters: SharedArray<Rc<ParameterOfFunctionType>>,
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
    pub extends_class: RefCell<Option<Symbol>>,
    pub extends_interfaces: RefCell<Option<SharedArray<Symbol>>>,
    pub implements: RefCell<Option<SharedArray<Symbol>>>,
    pub static_properties: RefCell<Option<SharedMap<String, Symbol>>>,
    pub constructor_function: RefCell<Option<Symbol>>,
    pub prototype: RefCell<Option<SharedMap<String, Symbol>>>,
    pub proxies: RefCell<Option<SharedMap<ProxyKind, Symbol>>>,
    pub list_of_to_proxies: RefCell<Option<SharedArray<Symbol>>>,
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub(crate) struct ClassTypeFlags: u16 {
        const IS_FINAL = 0b00000001;
        const IS_STATIC = 0b00000010;
        const IS_ABSTRACT = 0b00000100;
        const ALLOW_LITERAL = 0b00001000;
    }
}

pub(crate) struct AliasData {
    pub name: String,
    pub visibility: Cell<Visibility>,
    pub alias_of: RefCell<Symbol>,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub plain_metadata: SharedArray<Rc<PlainMetadata>>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
}

pub(crate) struct PackageData {
    pub name: String,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub properties: SharedMap<String, Symbol>,
    pub redirect_packages: SharedArray<Symbol>,
    pub subpackages: SharedMap<String, Symbol>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
}

pub(crate) struct PackageSetData {
    pub name: String,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub visibility: Cell<Visibility>,
    pub packages: SharedArray<Symbol>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
}

pub(crate) struct VariablePropertyData {
    pub name: String,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub visibility: Cell<Visibility>,
    pub static_type: RefCell<Symbol>,
    pub read_only: Cell<bool>,
    pub constant_initializer: RefCell<Option<Symbol>>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
    pub plain_metadata: SharedArray<Rc<PlainMetadata>>,
}

pub(crate) struct VariablePropertyAfterIndirectTypeSubstitutionData {
    pub origin: Symbol,
    pub indirect_type_parameters: SharedArray<Symbol>,
    pub indirect_substitute_types: SharedArray<Symbol>,
    pub static_type: RefCell<Option<Symbol>>,
}

pub(crate) struct VirtualPropertyData {
    pub name: String,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub visibility: Cell<Visibility>,
    pub static_type: RefCell<Option<Symbol>>,
    pub getter: RefCell<Option<Symbol>>,
    pub setter: RefCell<Option<Symbol>>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
}

pub(crate) struct VirtualPropertyAfterIndirectTypeSubstitutionData {
    pub origin: Symbol,
    pub indirect_type_parameters: SharedArray<Symbol>,
    pub indirect_substitute_types: SharedArray<Symbol>,
    pub static_type: RefCell<Option<Symbol>>,
    pub getter: RefCell<Option<Symbol>>,
    pub setter: RefCell<Option<Symbol>>,
}

pub(crate) struct FunctionSymbolData {
    pub name: String,
    pub parent_definition: RefCell<Option<Symbol>>,
    pub visibility: Cell<Visibility>,
    pub flags: RefCell<FunctionSymbolFlags>,
    pub signature: RefCell<Symbol>,
    pub type_parameters: RefCell<Option<SharedArray<Symbol>>>,
    pub of_virtual_property: RefCell<Option<Symbol>>,
    pub overriden_by: SharedArray<Symbol>,
    pub overrides_method: RefCell<Option<Symbol>>,
    pub jetdoc: RefCell<Option<Rc<JetDoc>>>,
    pub plain_metadata: SharedArray<Rc<PlainMetadata>>,
    pub activation_scope: RefCell<Option<Symbol>>,
}

bitflags! {
    #[derive(Copy, Clone, PartialEq, Eq)]
    pub(crate) struct FunctionSymbolFlags: u16 {
        const IS_GENERATOR = 0b0000_0001;
        const IS_ASYNC = 0b0000_0010;
        const IS_NATIVE = 0b0000_0100;
        const IS_OPTIONAL_INTERFACE_METHOD = 0b0000_1000;
        const IS_OVERRIDING = 0b0001_0000;
        const IS_FINAL = 0b0010_0000;
        const IS_ABSTRACT = 0b0100_0000;
        const IS_CONSTRUCTOR = 0b1000_0000;
    }
}

pub(crate) struct FunctionAfterExplicitOrIndirectTypeSubstitutionData {
    pub origin: Symbol,
    pub explicit_or_indirect_type_parameters: SharedArray<Symbol>,
    pub explicit_or_indirect_substitute_types: SharedArray<Symbol>,
    pub signature: RefCell<Option<Symbol>>,
    pub of_virtual_property: RefCell<Option<Symbol>>,
    pub overriden_by: RefCell<Option<SharedArray<Symbol>>>,
    pub overrides_method: RefCell<Option<Symbol>>,
    pub is_overriding: Cell<bool>,
}

pub(crate) struct ScopeData {
    pub parent_scope: RefCell<Option<Symbol>>,
    pub properties: SharedMap<String, Symbol>,
    pub imports: SharedMap<String, Symbol>,
    pub open_packages: SharedArray<Symbol>,
    pub package_aliases: SharedMap<String, Symbol>,
    pub local_variable_scope_count: Cell<usize>,
}

pub(crate) struct ActivationScopeData {
    pub function: Symbol,
    pub this: RefCell<Option<Symbol>>,
    pub property_has_capture: RefCell<Option<SharedArray<Symbol>>>,
}

pub(crate) enum ScopeKind {
    With {
        object: Symbol,
    },
    FilterOperator {
        base: Symbol,
    },
    Activation(Rc<ActivationScopeData>),
    Class {
        class: Symbol,
    },
    Enum {
        class: Symbol,
    },
    Interface {
        interface: Symbol,
    },
    Package {
        package: Symbol,
    },
}

pub(crate) struct ValueData {
    pub static_type: RefCell<Symbol>,
}

pub(crate) enum ValueKind {
    Constant(ConstantKind),
    This,
    Conversion(Rc<ConversionValueData>),
    ImportMeta,
    ImportMetaEnv,
    ImportMetaOutput,
    Reference(Rc<ReferenceValueKind>),
    Function {
        activation_scope: Symbol,
    },
    Embed(Rc<EmbedValueData>),
}

pub(crate) struct EmbedValueData {
    pub embedded_byte_array: Option<Rc<Vec<u8>>>,
    pub embedded_string: Option<Rc<String>>,
}

pub(crate) enum ConstantKind {
    Undefined,
    Null,
    String(String),
    Char(char),
    Boolean(bool),
    Number(AbstractRangeNumber),
    Enum(AbstractRangeNumber),
}

pub(crate) struct ConversionValueData {
    pub base: Symbol,
    pub relationship: TypeConversionRelationship,
    pub optional: bool,
    pub target: Symbol,
}

pub(crate) enum ReferenceValueKind {
    Type {
        referenced_type: Symbol,
    },
    Xml {
        base: Symbol,
        qualifier: Option<Symbol>,
        key: Symbol,
        disambiguation: PropertyDisambiguation,
    },
    Dynamic {
        base: Symbol,
        qualifier: Option<Symbol>,
        key: Symbol,
        disambiguation: PropertyDisambiguation,
    },
    Static {
        base: Symbol,
        property: Symbol,
    },
    Instance {
        base: Symbol,
        property: Symbol,
    },
    Proxy {
        base: Symbol,
        proxy: Symbol,
    },
    Tuple {
        base: Symbol,
        index: usize,
    },
    Scope {
        base: Symbol,
        property: Symbol,
    },
    DynamicScope {
        base: Symbol,
        qualifier: Option<Symbol>,
        key: Symbol,
        disambiguation: PropertyDisambiguation,
    },
    Package {
        base: Symbol,
        property: Symbol,
    },
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

/// Type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `to_string()`
/// * `includes_undefined()`
/// * `includes_null()`
/// * `property_is_visible()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Type(pub Symbol);

impl Deref for Type {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_type());
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
/// * `includes_undefined()` — Returns `true`.
/// * `includes_null()` — Returns `true`.
/// * `property_is_visible()`
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
/// * `includes_undefined()` — Returns `true`.
/// * `includes_null()` — Returns `false`.
/// * `property_is_visible()`
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
/// * `includes_undefined()` — Returns `false`.
/// * `includes_null()` — Returns `false`.
/// * `property_is_visible()`
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
/// * `includes_undefined()` — Returns `false`.
/// * `includes_null()` — Returns `false`.
/// * `property_is_visible()`
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
/// * `includes_undefined()` — Returns `false`.
/// * `includes_null()` — Returns `false`.
/// * `property_is_visible()`
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
/// * `includes_undefined()` — Returns `false`.
/// * `includes_null()` — Returns `false`.
/// * `property_is_visible()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionType(pub Symbol);

impl Deref for FunctionType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_function_type());
        &self.0
    }
}

/// Parameter of a function type.
pub struct ParameterOfFunctionType {
    pub kind: ParameterKind,
    pub name: String,
    /// Static type of the parameter. It is never `Unresolved` as
    /// function types are only created after all compound types are resolved.
    pub static_type: Symbol,
}

impl ParameterOfFunctionType {
    pub fn type_substitution(&self, host: &SymbolHost, type_parameters: &SharedArray<Symbol>, substitute_types: &SharedArray<Symbol>) -> Self {
        ParameterOfFunctionType {
            kind: self.kind,
            name: self.name.clone(),
            static_type: TypeSubstitution(host).execute(&self.static_type, type_parameters, substitute_types),
        }
    }
}

/// Tuple type symbol.
///
/// # Supported methods
///
/// * `is_type()`
/// * `is_tuple_type()`
/// * `to_string()`
/// * `element_types()`
/// * `includes_undefined()` — Returns `false`.
/// * `includes_null()` — Returns `false`.
/// * `property_is_visible()`
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
/// * `includes_undefined()` — Returns `false`.
/// * `includes_null()` — Returns `true`.
/// * `property_is_visible()`
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
/// * `includes_undefined()` — Returns `false`.
/// * `includes_null()` — Returns `false`.
/// * `property_is_visible()`
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
/// * `static_properties()`
/// * `constructor_function()`
/// * `prototype()`
/// * `proxies()`
/// * `list_of_to_proxies()`
/// * `plain_metadata()`
/// * `visibility()`
/// * `jetdoc()`
/// * `includes_undefined()` — Returns `false`.
/// * `includes_null()` — Returns `false`.
/// * `property_is_visible()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct TypeAfterExplicitTypeSubstitution(pub Symbol);

impl Deref for TypeAfterExplicitTypeSubstitution {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_type_after_explicit_type_substitution());
        &self.0
    }
}

/// Alias symbol.
///
/// # Supported methods
///
/// * `is_alias()`
/// * `name()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `visibility()`
/// * `set_visibility()`
/// * `alias_of()` — The aliased symbol, possibly `Unresolved`.
/// * `set_alias_of()`
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `plain_metadata()`
/// * `jetdoc()`
/// * `set_jetdoc()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Alias(pub Symbol);

impl Deref for Alias {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_alias());
        &self.0
    }
}

/// Package symbol.
///
/// # Supported methods
///
/// * `is_package()`
/// * `name()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `properties()`
/// * `redirect_packages()`
/// * `subpackages()`
/// * `jetdoc()`
/// * `set_jetdoc()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Package(pub Symbol);

impl Deref for Package {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_package());
        &self.0
    }
}

/// Package set symbol.
///
/// # Supported methods
///
/// * `is_package_set()`
/// * `name()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `visibility()`
/// * `set_visibility()`
/// * `packages()`
/// * `jetdoc()`
/// * `set_jetdoc()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PackageSet(pub Symbol);

impl Deref for PackageSet {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_package_set());
        &self.0
    }
}

/// Variable property symbol.
///
/// # Supported methods
///
/// * `is_variable_property()`
/// * `is_origin_variable_property()` — Returns `true`.
/// * `name()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `visibility()`
/// * `set_visibility()`
/// * `static_type()`
/// * `set_static_type()`
/// * `read_only()`
/// * `set_read_only()`
/// * `constant_initializer()`
/// * `set_constant_initializer()`
/// * `plain_metadata()`
/// * `jetdoc()`
/// * `set_jetdoc()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct VariableProperty(pub Symbol);

impl Deref for VariableProperty {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_variable_property());
        &self.0
    }
}

/// Symbol for variable property after indirect type substitution.
///
/// # Supported methods
///
/// * `is_variable_property()`
/// * `is_origin_variable_property()` — Returns `false`.
/// * `is_variable_property_after_indirect_type_substitution()`
/// * `name()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `parent_definition()`
/// * `origin()` — The original variable property.
/// * `indirect_type_parameters()` — Type parameters from indirect symbol.
/// * `indirect_substitute_types()` — Substitute types from indirect symbol.
/// * `visibility()`
/// * `static_type()`
/// * `read_only()`
/// * `plain_metadata()`
/// * `jetdoc()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct VariablePropertyAfterIndirectTypeSubstitution(pub Symbol);

impl Deref for VariablePropertyAfterIndirectTypeSubstitution {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_variable_property_after_indirect_type_substitution());
        &self.0
    }
}

/// Virtual property symbol.
///
/// # Supported methods
///
/// * `is_virtual_property()`
/// * `is_origin_virtual_property()` — Returns `true`.
/// * `name()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `visibility()`
/// * `set_visibility()`
/// * `static_type()`
/// * `read_only()`
/// * `write_only()`
/// * `getter()`
/// * `set_getter()`
/// * `setter()`
/// * `set_setter()`
/// * `jetdoc()`
/// * `set_jetdoc()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct VirtualProperty(pub Symbol);

impl Deref for VirtualProperty {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_virtual_property());
        &self.0
    }
}

/// Symbol for virtual property after indirect type substitution.
///
/// # Supported methods
///
/// * `is_virtual_property()`
/// * `is_origin_virtual_property()` — Returns `false`.
/// * `is_virtual_property_after_indirect_type_substitution()`
/// * `origin()`
/// * `indirect_type_parameters()`
/// * `indirect_substitute_types()`
/// * `name()`
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `parent_definition()`
/// * `visibility()`
/// * `static_type()`
/// * `read_only()`
/// * `write_only()`
/// * `getter()`
/// * `setter()`
/// * `jetdoc()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct VirtualPropertyAfterIndirectTypeSubstitution(pub Symbol);

impl Deref for VirtualPropertyAfterIndirectTypeSubstitution {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_virtual_property_after_indirect_type_substitution());
        &self.0
    }
}

/// Function symbol.
///
/// # Supported methods
///
/// * `is_function()`
/// * `is_origin_function()` — Returns `true`.
/// * `name()` — The function name. The name may be empty for special functions.
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `parent_definition()`
/// * `set_parent_definition()`
/// * `visibility()`
/// * `set_visibility()`
/// * `jetdoc()`
/// * `set_jetdoc()`
/// * `plain_metadata()`
/// * `is_generator()`
/// * `is_async()`
/// * `is_native()`
/// * `is_optional_interface_method()`
/// * `is_overriding()`
/// * `is_final()`
/// * `is_abstract()`
/// * `is_constructor()`
/// * `set_is_generator()`
/// * `set_is_async()`
/// * `set_is_native()`
/// * `set_is_optional_interface_method()`
/// * `set_is_overriding()`
/// * `set_is_final()`
/// * `set_is_abstract()`
/// * `set_is_constructor()`
/// * `signature()`
/// * `set_signature()`
/// * `type_parameters()`
/// * `set_type_parameters()`
/// * `of_virtual_property()`
/// * `set_of_virtual_property()`
/// * `overriden_by()` — List of function symbols used to override the function symbol.
/// * `overrides_method()` — Indicates an overriden method.
/// * `set_overrides_method()`
/// * `activation_scope()` — Activation scope, present for
///   non `abstract` non `native` functions and required interface methods.
/// * `set_activation_scope()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionSymbol(pub Symbol);

impl Deref for FunctionSymbol {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_function());
        &self.0
    }
}

/// Function symbol after explicit or indirect type substitution.
///
/// # Supported methods
///
/// * `is_function()`
/// * `is_origin_function()` — Returns `false`.
/// * `is_function_after_explicit_or_indirect_type_substitution()` — Returns `true`.
/// * `name()` — The function name. The name may be empty for special functions.
/// * `fully_qualified_name()`
/// * `to_string()`
/// * `parent_definition()`
/// * `visibility()`
/// * `jetdoc()`
/// * `plain_metadata()`
/// * `is_generator()`
/// * `is_async()`
/// * `is_native()`
/// * `is_optional_interface_method()`
/// * `is_overriding()`
/// * `is_final()`
/// * `is_abstract()`
/// * `is_constructor()`
/// * `set_is_overriding()`
/// * `signature()`
/// * `of_virtual_property()`
/// * `overriden_by()` — List of function symbols used to override the function symbol.
/// * `overrides_method()` — Indicates an overriden method.
/// * `set_overrides_method()`
/// * `origin()`
/// * `explicit_or_indirect_type_parameters()`
/// * `explicit_or_indirect_substitute_types()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FunctionAfterExplicitOrIndirectTypeSubstitution(pub Symbol);

impl Deref for FunctionAfterExplicitOrIndirectTypeSubstitution {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_function_after_explicit_or_indirect_type_substitution());
        &self.0
    }
}

/// Scope symbol.
///
/// # Supported methods
///
/// * `is_scope()`
/// * `parent_scope()`
/// * `set_parent_scope()`
/// * `properties()`
/// * `imports()`
/// * `open_packages()`
/// * `package_aliases()`
/// * `local_variable_scope_count()`
/// * `set_local_variable_scope_count()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct Scope(pub Symbol);

impl Deref for Scope {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_scope());
        &self.0
    }
}

/// `with` scope symbol.
///
/// # Supported methods
///
/// * `Scope` inherited methods
///   * `is_scope()`
///   * `parent_scope()`
///   * `set_parent_scope()`
///   * `properties()`
///   * `imports()`
///   * `open_packages()`
///   * `package_aliases()`
///   * `local_variable_scope_count()`
///   * `set_local_variable_scope_count()`
/// * `is_with_scope()`
/// * `object()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct WithScope(pub Symbol);

impl Deref for WithScope {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_with_scope());
        &self.0
    }
}

/// Filter operator scope symbol.
///
/// # Supported methods
///
/// * `Scope` inherited methods
///   * `is_scope()`
///   * `parent_scope()`
///   * `set_parent_scope()`
///   * `properties()`
///   * `imports()`
///   * `open_packages()`
///   * `package_aliases()`
///   * `local_variable_scope_count()`
///   * `set_local_variable_scope_count()`
/// * `is_filter_operator_scope()`
/// * `base()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct FilterOperatorScope(pub Symbol);

impl Deref for FilterOperatorScope {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_filter_operator_scope());
        &self.0
    }
}

/// Activation scope symbol.
///
/// Properties within an activation may be captured by subsequent
/// activations, therefore yielding `activation.property_has_capture(p) == true`.
///
/// # Supported methods
///
/// * `Scope` inherited methods
///   * `is_scope()`
///   * `parent_scope()`
///   * `set_parent_scope()`
///   * `properties()`
///   * `imports()`
///   * `open_packages()`
///   * `package_aliases()`
///   * `local_variable_scope_count()`
///   * `set_local_variable_scope_count()`
/// * `is_activation_scope()`
/// * `function()`
/// * `this()` — An optional `ThisValue` symbol.
/// * `set_this()`
/// * `property_has_capture()` — Indicates whether an activation's property has been captured by a subsequent activation.
///   Properties range from the activation scope to the innermost scope of an activation.
/// * `set_property_has_capture()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ActivationScope(pub Symbol);

impl Deref for ActivationScope {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_activation_scope());
        &self.0
    }
}

/// `class` scope symbol.
///
/// # Supported methods
///
/// * `Scope` inherited methods
///   * `is_scope()`
///   * `parent_scope()`
///   * `set_parent_scope()`
///   * `properties()`
///   * `imports()`
///   * `open_packages()`
///   * `package_aliases()`
///   * `local_variable_scope_count()`
///   * `set_local_variable_scope_count()`
/// * `is_class_scope()`
/// * `class()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct ClassScope(pub Symbol);

impl Deref for ClassScope {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_class_scope());
        &self.0
    }
}

/// `enum` scope symbol.
///
/// # Supported methods
///
/// * `Scope` inherited methods
///   * `is_scope()`
///   * `parent_scope()`
///   * `set_parent_scope()`
///   * `properties()`
///   * `imports()`
///   * `open_packages()`
///   * `package_aliases()`
///   * `local_variable_scope_count()`
///   * `set_local_variable_scope_count()`
/// * `is_enum_scope()`
/// * `class()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct EnumScope(pub Symbol);

impl Deref for EnumScope {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_enum_scope());
        &self.0
    }
}

/// Interface scope symbol.
///
/// # Supported methods
///
/// * `Scope` inherited methods
///   * `is_scope()`
///   * `parent_scope()`
///   * `set_parent_scope()`
///   * `properties()`
///   * `imports()`
///   * `open_packages()`
///   * `package_aliases()`
///   * `local_variable_scope_count()`
///   * `set_local_variable_scope_count()`
/// * `is_interface_scope()`
/// * `interface()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct InterfaceScope(pub Symbol);

impl Deref for InterfaceScope {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_interface_scope());
        &self.0
    }
}

/// Package scope symbol.
///
/// # Supported methods
///
/// * `Scope` inherited methods
///   * `is_scope()`
///   * `parent_scope()`
///   * `set_parent_scope()`
///   * `properties()`
///   * `imports()`
///   * `open_packages()`
///   * `package_aliases()`
///   * `local_variable_scope_count()`
///   * `set_local_variable_scope_count()`
/// * `is_package_scope()`
/// * `package()`
#[derive(Clone, Hash, PartialEq, Eq)]
pub struct PackageScope(pub Symbol);

impl Deref for PackageScope {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_package_scope());
        &self.0
    }
}

/// Value symbol.
///
/// # Supported methods
///
/// * `is_value()`
/// * `static_type()`
/// * `set_static_type()`
/// * `read_only()`
/// * `write_only()`
/// * `deletable()`
/// * `property_is_visible()`
pub struct Value(pub Symbol);

impl Deref for Value {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_value());
        &self.0
    }
}

/// `embed { ... }` value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_embed_value()`
/// * `embedded_byte_array()`
/// * `embedded_string()`
pub struct EmbedValue(pub Symbol);

impl Deref for EmbedValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_embed_value());
        &self.0
    }
}

#[derive(Clone)]
pub enum EmbedValueDataContent {
    String(String),
    ByteArray(Vec<u8>),
}

/// `import.meta` value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_import_meta()`
pub struct ImportMetaSymbol(pub Symbol);

impl Deref for ImportMetaSymbol {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_import_meta());
        &self.0
    }
}

/// `import.meta.output` value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_import_meta_output()`
pub struct ImportMetaOutputSymbol(pub Symbol);

impl Deref for ImportMetaOutputSymbol {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_import_meta_output());
        &self.0
    }
}

/// `import.meta.env` value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_import_meta_env()`
pub struct ImportMetaEnvSymbol(pub Symbol);

impl Deref for ImportMetaEnvSymbol {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_import_meta_env());
        &self.0
    }
}

/// Undefined constant value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_constant()`
/// * `is_undefined_constant()`
pub struct UndefinedConstant(pub Symbol);

impl Deref for UndefinedConstant {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_undefined_constant());
        &self.0
    }
}

/// Null constant value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_constant()`
/// * `is_null_constant()`
pub struct NullConstant(pub Symbol);

impl Deref for NullConstant {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_null_constant());
        &self.0
    }
}

/// String constant value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_constant()`
/// * `is_string_constant()`
/// * `string_value()`
pub struct StringConstant(pub Symbol);

impl Deref for StringConstant {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_string_constant());
        &self.0
    }
}

/// `Char` constant value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_constant()`
/// * `is_char_constant()`
/// * `char_value()`
pub struct CharConstant(pub Symbol);

impl Deref for CharConstant {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_char_constant());
        &self.0
    }
}

/// `Boolean` constant value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_constant()`
/// * `is_boolean_constant()`
/// * `boolean_value()`
pub struct BooleanConstant(pub Symbol);

impl Deref for BooleanConstant {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_boolean_constant());
        &self.0
    }
}

/// Numeric constant value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_constant()`
/// * `is_number_constant()`
/// * `number_value()` — The `AbstractRangeNumber` representing the numeric value.
pub struct NumberConstant(pub Symbol);

impl Deref for NumberConstant {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_number_constant());
        &self.0
    }
}

/// `enum` constant value symbol. The static type of this constant
/// is either `T` or `T?` where `T` is an `enum`.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_constant()`
/// * `is_enum_constant()`
/// * `number_value()` — The `AbstractRangeNumber` representing the `enum` value.
pub struct EnumConstant(pub Symbol);

impl Deref for EnumConstant {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_enum_constant());
        &self.0
    }
}

/// `this` value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_this_value()`
pub struct ThisValue(pub Symbol);

impl Deref for ThisValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_this_value());
        &self.0
    }
}

/// Conversion value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_conversion_value()`
/// * `base()` — Original value.
/// * `conversion_relationship()`
/// * `conversion_is_optional()` — Indicates whether the conversion is optional (`v as T` versus `T(v)`)
///   and whether the static type of the `ConversionValue` has been nullified.
/// * `conversion_target()` — Conversion target type, without being nullified in the case of the `as` operator.
pub struct ConversionValue(pub Symbol);

impl Deref for ConversionValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_conversion_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_type_as_reference_value()`
/// * `referenced_type()`
pub struct TypeAsReferenceValue(pub Symbol);

impl Deref for TypeAsReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_type_as_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_xml_reference_value()`
/// * `base()`
/// * `qualifier()`
/// * `key()`
/// * `disambiguation()`
pub struct XmlReferenceValue(pub Symbol);

impl Deref for XmlReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_xml_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_dynamic_reference_value()`
/// * `base()`
/// * `qualifier()`
/// * `key()`
/// * `disambiguation()`
pub struct DynamicReferenceValue(pub Symbol);

impl Deref for DynamicReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_dynamic_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_static_reference_value()`
/// * `base()`
/// * `property()`
pub struct StaticReferenceValue(pub Symbol);

impl Deref for StaticReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_static_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_instance_reference_value()`
/// * `base()`
/// * `property()`
pub struct InstanceReferenceValue(pub Symbol);

impl Deref for InstanceReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_instance_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_proxy_reference_value()`
/// * `base()`
/// * `proxy()` — The `getProperty` proxy.
pub struct ProxyReferenceValue(pub Symbol);

impl Deref for ProxyReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_proxy_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_tuple_reference_value()`
/// * `base()`
/// * `tuple_index()`
pub struct TupleReferenceValue(pub Symbol);

impl Deref for TupleReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_tuple_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_scope_reference_value()`
/// * `base()`
/// * `property()`
pub struct ScopeReferenceValue(pub Symbol);

impl Deref for ScopeReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_scope_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_dynamic_scope_reference_value()`
/// * `base()`
/// * `qualifier()`
/// * `key()`
/// * `disambiguation()`
pub struct DynamicScopeReferenceValue(pub Symbol);

impl Deref for DynamicScopeReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_dynamic_scope_reference_value());
        &self.0
    }
}

/// Reference value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_reference_value()`
/// * `is_package_reference_value()`
/// * `base()`
/// * `property()`
pub struct PackageReferenceValue(pub Symbol);

impl Deref for PackageReferenceValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_package_reference_value());
        &self.0
    }
}

/// Function value symbol.
///
/// # Supported methods
///
/// * Inherits methods from [`Value`].
/// * `is_function_value()`
/// * `activation_scope()`
pub struct FunctionValue(pub Symbol);

impl Deref for FunctionValue {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_function_value());
        &self.0
    }
}

/// Block statement symbol.
///
/// # Supported methods
///
/// * `is_block_statement()`
/// * `plain_metadata()`
pub struct BlockStatementSymbol(pub Symbol);

impl Deref for BlockStatementSymbol {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_block_statement());
        &self.0
    }
}

/// Variable definition directive symbol.
///
/// # Supported methods
///
/// * `is_variable_definition_directive()`
/// * `shadow_scope()`
/// * `set_shadow_scope()`
pub struct VariableDefinitionDirectiveSymbol(pub Symbol);

impl Deref for VariableDefinitionDirectiveSymbol {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_variable_definition_directive());
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use crate::ns::*;

    #[test]
    fn creating_packages() {
        let host = SymbolHost::new("");
        let p = host.factory().create_package(["y", "n"]);
        println!("{}", p.fully_qualified_name());
    }
}