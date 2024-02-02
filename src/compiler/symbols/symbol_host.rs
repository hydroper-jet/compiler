use crate::ns::*;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
    pub(crate) unresolved: Symbol,
    pub(crate) any_type: Symbol,
    pub(crate) void_type: Symbol,
    pub(crate) import_meta: Symbol,
    pub(crate) import_meta_env: Symbol,

    pub(crate) env_cache: RefCell<Option<Rc<HashMap<String, String>>>>,

    pub(crate) function_types: HashMap<usize, Vec<Symbol>>,
    pub(crate) tuple_types: HashMap<usize, Vec<Symbol>>,
    pub(crate) nullable_types: HashMap<Symbol, Symbol>,

    /// Types after explicit type substitution.
    pub(crate) taets: HashMap<Symbol, Vec<Symbol>>,
    /// Variable properties after indirect type substitution.
    pub(crate) vapaits: HashMap<Symbol, HashMap<SharedArray<Symbol>, Vec<Symbol>>>,
    /// Virtual properties after indirect type substitution.
    pub(crate) vipaits: HashMap<Symbol, HashMap<SharedArray<Symbol>, Vec<Symbol>>>,
    /// Functions after explicit or indirect type substitution.
    pub(crate) faeoits: HashMap<Symbol, HashMap<SharedArray<Symbol>, Vec<Symbol>>>,

    pub(crate) top_level_package: Symbol,
    pub(crate) jet_lang_package: RefCell<Option<Symbol>>,
    pub(crate) object_type: RefCell<Option<Symbol>>,
    pub(crate) boolean_type: RefCell<Option<Symbol>>,
    pub(crate) string_type: RefCell<Option<Symbol>>,
    pub(crate) char_type: RefCell<Option<Symbol>>,
    pub(crate) char_index_type: RefCell<Option<Symbol>>,
    pub(crate) number_type: RefCell<Option<Symbol>>,
    pub(crate) single_type: RefCell<Option<Symbol>>,
    pub(crate) byte_type: RefCell<Option<Symbol>>,
    pub(crate) short_type: RefCell<Option<Symbol>>,
    pub(crate) int_type: RefCell<Option<Symbol>>,
    pub(crate) long_type: RefCell<Option<Symbol>>,
    pub(crate) unsigned_byte_type: RefCell<Option<Symbol>>,
    pub(crate) unsigned_short_type: RefCell<Option<Symbol>>,
    pub(crate) unsigned_int_type: RefCell<Option<Symbol>>,
    pub(crate) unsigned_long_type: RefCell<Option<Symbol>>,
    pub(crate) big_int_type: RefCell<Option<Symbol>>,
    pub(crate) function_type: RefCell<Option<Symbol>>,
    pub(crate) xml_type: RefCell<Option<Symbol>>,
    pub(crate) xml_list_type: RefCell<Option<Symbol>>,
    pub(crate) class_type: RefCell<Option<Symbol>>,

    pub(crate) infinity_constant: RefCell<Option<Symbol>>,
    pub(crate) nan_constant: RefCell<Option<Symbol>>,

    pub(crate) root_scope: RefCell<Option<Symbol>>,
}

impl SymbolHost {
    pub fn new() -> Self {
        let arena = Arena::new();
        Self {
            arena: Arena::new(),
            unresolved: Symbol(arena.allocate(SymbolKind::Unresolved)),
            any_type: Symbol(arena.allocate(SymbolKind::Type(TypeKind::AnyType))),
            void_type: Symbol(arena.allocate(SymbolKind::Type(TypeKind::VoidType))),
            import_meta: Symbol(arena.allocate(SymbolKind::ImportMeta)),
            import_meta_env: Symbol(arena.allocate(SymbolKind::ImportMetaEnv)),

            env_cache: RefCell::new(None),

            function_types: HashMap::new(),
            tuple_types: HashMap::new(),
            nullable_types: HashMap::new(),
            taets: HashMap::new(),
            vapaits: HashMap::new(),
            vipaits: HashMap::new(),
            faeoits: HashMap::new(),

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
            boolean_type: RefCell::new(None),
            string_type: RefCell::new(None),
            char_type: RefCell::new(None),
            char_index_type: RefCell::new(None),
            number_type: RefCell::new(None),
            single_type: RefCell::new(None),
            byte_type: RefCell::new(None),
            short_type: RefCell::new(None),
            int_type: RefCell::new(None),
            long_type: RefCell::new(None),
            unsigned_byte_type: RefCell::new(None),
            unsigned_short_type: RefCell::new(None),
            unsigned_int_type: RefCell::new(None),
            unsigned_long_type: RefCell::new(None),
            big_int_type: RefCell::new(None),
            function_type: RefCell::new(None),
            xml_type: RefCell::new(None),
            xml_list_type: RefCell::new(None),
            class_type: RefCell::new(None),

            infinity_constant: RefCell::new(None),
            nan_constant: RefCell::new(None),

            root_scope: RefCell::new(None),
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

    /// The `import.meta` symbol.
    pub fn import_meta(&self) -> Symbol {
        (self.import_meta).clone()
    }

    /// The `import.meta.env` symbol.
    pub fn import_meta_env(&self) -> Symbol {
        (self.import_meta_env).clone()
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
        if let Some(r) = self.lookup_at_jet_lang("Object") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Boolean` class, possibly `Unresolved`.
    pub fn boolean_type(&mut self) -> Symbol {
        if let Some(r) = self.boolean_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Boolean") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.String` class, possibly `Unresolved`.
    pub fn string_type(&mut self) -> Symbol {
        if let Some(r) = self.string_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("String") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Char` class, possibly `Unresolved`.
    pub fn char_type(&mut self) -> Symbol {
        if let Some(r) = self.char_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Char") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.CharIndex` class, possibly `Unresolved`.
    pub fn char_index_type(&mut self) -> Symbol {
        if let Some(r) = self.char_index_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("CharIndex") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Number` class, possibly `Unresolved`.
    pub fn number_type(&mut self) -> Symbol {
        if let Some(r) = self.number_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Number") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Single` class, possibly `Unresolved`.
    pub fn single_type(&mut self) -> Symbol {
        if let Some(r) = self.single_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Single") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Byte` class, possibly `Unresolved`.
    pub fn byte_type(&mut self) -> Symbol {
        if let Some(r) = self.byte_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Byte") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Short` class, possibly `Unresolved`.
    pub fn short_type(&mut self) -> Symbol {
        if let Some(r) = self.short_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Short") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Int` class, possibly `Unresolved`.
    pub fn int_type(&mut self) -> Symbol {
        if let Some(r) = self.int_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Int") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Long` class, possibly `Unresolved`.
    pub fn long_type(&mut self) -> Symbol {
        if let Some(r) = self.long_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Long") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.UnsignedByte` class, possibly `Unresolved`.
    pub fn unsigned_byte_type(&mut self) -> Symbol {
        if let Some(r) = self.unsigned_byte_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("UnsignedByte") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.UnsignedShort` class, possibly `Unresolved`.
    pub fn unsigned_short_type(&mut self) -> Symbol {
        if let Some(r) = self.unsigned_short_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("UnsignedShort") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.UnsignedInt` class, possibly `Unresolved`.
    pub fn unsigned_int_type(&mut self) -> Symbol {
        if let Some(r) = self.unsigned_int_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("UnsignedInt") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.UnsignedLong` class, possibly `Unresolved`.
    pub fn unsigned_long_type(&mut self) -> Symbol {
        if let Some(r) = self.unsigned_long_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("UnsignedLong") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.BigInt` class, possibly `Unresolved`.
    pub fn big_int_type(&mut self) -> Symbol {
        if let Some(r) = self.big_int_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("BigInt") {
            self.object_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Function` class, possibly `Unresolved`.
    pub fn function_type(&mut self) -> Symbol {
        if let Some(r) = self.function_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Function") {
            self.function_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.XML` class, possibly `Unresolved`.
    pub fn xml_type(&mut self) -> Symbol {
        if let Some(r) = self.xml_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("XML") {
            self.xml_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.XMLList` class, possibly `Unresolved`.
    pub fn xml_list_type(&mut self) -> Symbol {
        if let Some(r) = self.xml_list_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("XMLList") {
            self.xml_list_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Class` class, possibly `Unresolved`.
    pub fn class_type(&mut self) -> Symbol {
        if let Some(r) = self.class_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Class") {
            self.class_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Infinity` constant, possibly `Unresolved`.
    pub fn infinity_constant(&mut self) -> Symbol {
        if let Some(r) = self.infinity_constant.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Infinity") {
            self.infinity_constant.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.NaN` constant, possibly `Unresolved`.
    pub fn nan_constant(&mut self) -> Symbol {
        if let Some(r) = self.nan_constant.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("NaN") {
            self.nan_constant.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    fn lookup_at_jet_lang(&mut self, name: &str) -> Option<Symbol> {
        self.jet_lang_package().properties(self).get(&name.to_owned())
    }

    pub fn is_numeric_type(&mut self, symbol: &Symbol) -> bool {
        [
            self.number_type(),
            self.int_type(),
            self.unsigned_int_type(),
            self.single_type(),
            self.byte_type(),
            self.unsigned_byte_type(),
            self.short_type(),
            self.unsigned_short_type(),
            self.long_type(),
            self.unsigned_long_type(),
            self.big_int_type(),
        ].contains(symbol)
    }

    pub fn is_integer_type(&mut self, symbol: &Symbol) -> bool {
        [
            self.int_type(),
            self.unsigned_int_type(),
            self.byte_type(),
            self.unsigned_byte_type(),
            self.short_type(),
            self.unsigned_short_type(),
            self.long_type(),
            self.unsigned_long_type(),
            self.big_int_type(),
        ].contains(symbol)
    }

    /// The root scope that imports the top-level package and `jet.lang.*`.
    pub fn root_scope(&mut self) -> Symbol {
        if let Some(r) = self.root_scope.borrow().as_ref() {
            return r.clone();
        }
        let r = self.factory().create_scope();
        r.open_packages().push(self.top_level_package());
        r.open_packages().push(self.jet_lang_package());
        r
    }

    pub(crate) fn preload_environment_variables(&mut self) -> Rc<HashMap<String, String>> {
        if let Some(env) = self.env_cache.borrow().as_ref() {
            return env.clone();
        }
        let mut r = HashMap::<String, String>::new();
        if let Ok(iterator) = dotenvy::dotenv_iter() {
            for item in iterator {
                if let Ok((key, value)) = item {
                    r.insert(key, value);
                }
            }
        }
        let r = Rc::new(r);
        self.env_cache.replace(Some(r.clone()));
        r
    }
}