use crate::ns::*;
use std::cell::{Cell, RefCell};
use std::collections::HashMap;
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

impl Symbol {
    pub fn is_unresolved(&self) -> bool {
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Unresolved(_))
    }

    pub fn unresolved_count(&self) -> u32 {
        let symbol = self.0.upgrade().unwrap();
        let SymbolKind::Unresolved(ref symbol) = symbol.as_ref() else {
            panic!();
        };
        symbol.get()
    }

    pub fn increment_unresolved_count(&self) {
        let symbol = self.0.upgrade().unwrap();
        let SymbolKind::Unresolved(ref symbol) = symbol.as_ref() else {
            panic!();
        };
        symbol.set(symbol.get() + 1);
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
        matches!(self.0.upgrade().unwrap().as_ref(), SymbolKind::Type(TypeKind::ClassType(_)))
    }
}

impl ToString for Symbol {
    fn to_string(&self) -> String {
        let symbol = self.0.upgrade().unwrap();
        match symbol.as_ref() {
            SymbolKind::Type(TypeKind::AnyType) => "*".into(),
            SymbolKind::Type(TypeKind::VoidType) => "void".into(),
            _ => panic!(),
        }
    }
}

pub(crate) enum SymbolKind {
    Unresolved(Cell<u32>),
    Type(TypeKind),
}

pub(crate) enum TypeKind {
    AnyType,
    VoidType,
    ClassType(Rc<ClassTypeData>),
}

pub(crate) struct ClassTypeData {
    pub(crate) name: String,
    pub(crate) visibility: Cell<Visibility>,
    pub(crate) parent_definition: RefCell<Option<Symbol>>,
    pub(crate) super_class: RefCell<Option<Symbol>>,
    pub(crate) implements: Rc<RefCell<Vec<Symbol>>>,
    pub(crate) flags: Cell<ClassTypeFlags>,
    pub(crate) type_parameters: Rc<RefCell<Vec<Symbol>>>,
    pub(crate) static_properties: Rc<RefCell<HashMap<String, Symbol>>>,
    pub(crate) prototype: Rc<RefCell<HashMap<String, Symbol>>>,
    pub(crate) proxies: Rc<RefCell<HashMap<ProxyKind, Symbol>>>,
    pub(crate) list_of_to_proxies: Rc<RefCell<Vec<Symbol>>>,
    pub(crate) list_of_to_optional_proxies: Rc<RefCell<Vec<Symbol>>>,
    pub(crate) limited_subclasses: Rc<RefCell<Vec<Symbol>>>,
    pub(crate) plain_metadata: RefCell<Vec<Rc<PlainMetadata>>>,
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
/// * `unresolved_count()` — Counter counting from zero (0).
/// * `increment_unresolved_count()`
#[derive(Clone)]
pub struct UnresolvedSymbol(pub Symbol);

impl Deref for UnresolvedSymbol {
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
#[derive(Clone)]
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
#[derive(Clone)]
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
#[derive(Clone)]
pub struct ClassType(pub Symbol);

impl Deref for ClassType {
    type Target = Symbol;
    fn deref(&self) -> &Self::Target {
        assert!(self.0.is_class_type());
        &self.0
    }
}