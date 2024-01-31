use crate::ns::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
    pub(crate) unresolved: Symbol,
    pub(crate) any_type: Symbol,
    pub(crate) void_type: Symbol,

    pub(crate) function_types: HashMap<usize, Vec<Symbol>>,
    pub(crate) tuple_types: HashMap<usize, Vec<Symbol>>,
    pub(crate) nullable_types: HashMap<Symbol, Symbol>,

    /// Types after explicit type substitution.
    pub(crate) taets: HashMap<Symbol, Vec<Symbol>>,
    /// Variable properties after indirect type substitution.
    pub(crate) vpaits: HashMap<SharedArray<Symbol>, Vec<Symbol>>,

    pub(crate) top_level_package: Symbol,
    pub(crate) jet_lang_package: RefCell<Option<Symbol>>,
    pub(crate) object_type: RefCell<Option<Symbol>>,
}

impl SymbolHost {
    pub fn new() -> Self {
        let arena = Arena::new();
        Self {
            arena: Arena::new(),
            unresolved: Symbol(arena.allocate(SymbolKind::Unresolved)),
            any_type: Symbol(arena.allocate(SymbolKind::Type(TypeKind::AnyType))),
            void_type: Symbol(arena.allocate(SymbolKind::Type(TypeKind::VoidType))),
            function_types: HashMap::new(),
            tuple_types: HashMap::new(),
            nullable_types: HashMap::new(),
            taets: HashMap::new(),
            vpaits: HashMap::new(),
            top_level_package: Symbol(arena.allocate(SymbolKind::Package(Rc::new(PackageData {
                name: String::new(),
                parent_definition: RefCell::new(None),
                properties: SharedMap::new(),
                redirect_packages: SharedArray::new(),
                subpackages: SharedMap::new(),
                jetdoc: RefCell::new(None),
            })))),
            jet_lang_package: RefCell::new(None),
            object_type: RefCell::new(None),
        }
    }

    pub fn factory(&mut self) -> SymbolFactory {
        SymbolFactory { host: self }
    }

    /// Returns the unique `Unresolved` symbol.
    pub fn unresolved(&self) -> Symbol {
        (self.unresolved).clone()
    }

    pub fn any_type(&self) -> Symbol {
        (self.any_type).clone()
    }

    pub fn void_type(&self) -> Symbol {
        (self.void_type).clone()
    }

    pub fn top_level_package(&self) -> Symbol {
        (self.top_level_package).clone()
    }

    /// The `jet.lang.*` package.
    pub fn jet_lang_package(&mut self) -> Symbol {
        if let Some(r) = self.jet_lang_package.borrow().as_ref() {
            return r.clone();
        }
        let r = self.factory().create_package(["jet", "lang"]);
        self.jet_lang_package.replace(Some(r.clone()));
        r
    }

    /// The `jet.lang.Object` class, possibly `Unresolved`.
    pub fn object_type(&mut self) -> Symbol {
        if let Some(r) = self.object_type.borrow().as_ref() {
            return r.clone();
        }
        let pckg = self.jet_lang_package();
        if let Some(r) = pckg.properties(self).get(&"Object".to_owned()) {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }
}