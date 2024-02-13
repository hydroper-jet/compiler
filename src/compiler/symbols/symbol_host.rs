use crate::ns::*;
use std::cell::RefCell;
use std::rc::Rc;

pub struct SymbolHost {
    pub(crate) arena: Arena<SymbolKind>,
    pub(crate) unresolved: Symbol,
    pub(crate) any_type: Symbol,
    pub(crate) void_type: Symbol,
    pub(crate) import_meta: Symbol,
    pub(crate) import_meta_env: Symbol,
    pub(crate) jetpm_output_directory: String,
    pub(crate) jetpm_constants: SharedMap<String, String>,

    pub(crate) env_cache: RefCell<Option<Rc<HashMap<String, String>>>>,

    pub(crate) function_types: RefCell<HashMap<usize, Vec<Symbol>>>,
    pub(crate) tuple_types: RefCell<HashMap<usize, Vec<Symbol>>>,
    pub(crate) nullable_types: RefCell<HashMap<Symbol, Symbol>>,

    /// Types after explicit type substitution.
    pub(crate) taets: RefCell<HashMap<Symbol, Vec<Symbol>>>,
    /// Variable properties after indirect type substitution.
    pub(crate) vapaits: RefCell<HashMap<Symbol, HashMap<SharedArray<Symbol>, Vec<Symbol>>>>,
    /// Virtual properties after indirect type substitution.
    pub(crate) vipaits: RefCell<HashMap<Symbol, HashMap<SharedArray<Symbol>, Vec<Symbol>>>>,
    /// Functions after explicit or indirect type substitution.
    pub(crate) faeoits: RefCell<HashMap<Symbol, HashMap<SharedArray<Symbol>, Vec<Symbol>>>>,

    pub(crate) top_level_package: Symbol,
    pub(crate) jet_lang_package: RefCell<Option<Symbol>>,
    pub(crate) object_type: RefCell<Option<Symbol>>,
    pub(crate) boolean_type: RefCell<Option<Symbol>>,
    pub(crate) string_type: RefCell<Option<Symbol>>,
    pub(crate) char_type: RefCell<Option<Symbol>>,
    pub(crate) number_type: RefCell<Option<Symbol>>,
    pub(crate) single_type: RefCell<Option<Symbol>>,
    pub(crate) long_type: RefCell<Option<Symbol>>,
    pub(crate) big_int_type: RefCell<Option<Symbol>>,
    pub(crate) function_type: RefCell<Option<Symbol>>,
    pub(crate) xml_type: RefCell<Option<Symbol>>,
    pub(crate) xml_list_type: RefCell<Option<Symbol>>,
    pub(crate) class_type: RefCell<Option<Symbol>>,
    pub(crate) array_type: RefCell<Option<Symbol>>,
    pub(crate) map_type: RefCell<Option<Symbol>>,
    pub(crate) namespace_type: RefCell<Option<Symbol>>,
    pub(crate) qname_type: RefCell<Option<Symbol>>,
    pub(crate) byte_array_type: RefCell<Option<Symbol>>,
    pub(crate) reg_exp_type: RefCell<Option<Symbol>>,
    pub(crate) iterator_type: RefCell<Option<Symbol>>,

    pub(crate) infinity_constant: RefCell<Option<Symbol>>,
    pub(crate) nan_constant: RefCell<Option<Symbol>>,

    pub(crate) root_scope: RefCell<Option<Symbol>>,
}

impl SymbolHost {
    pub fn new(jetpm_output_directory: &str) -> Rc<Self> {
        let arena = Arena::new();
        let unresolved = Symbol(arena.allocate(SymbolKind::Unresolved));
        let any_type = Symbol(arena.allocate(SymbolKind::Type(TypeKind::AnyType)));
        let void_type = Symbol(arena.allocate(SymbolKind::Type(TypeKind::VoidType)));

        let top_level_package = Symbol(arena.allocate(SymbolKind::Package(Rc::new(PackageData {
            name: String::new(),
            parent_definition: RefCell::new(None),
            properties: SharedMap::new(),
            redirect_packages: SharedArray::new(),
            subpackages: SharedMap::new(),
            jetdoc: RefCell::new(None),
        }))));

        let import_meta = Symbol(arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(any_type.clone()),
        }, Some(Rc::new(ValueKind::ImportMeta)))));

        let import_meta_env = Symbol(arena.allocate(SymbolKind::Value(ValueData {
            static_type: RefCell::new(any_type.clone()),
        }, Some(Rc::new(ValueKind::ImportMetaEnv)))));

        Rc::new(Self {
            arena,
            unresolved,
            any_type,
            void_type,
            import_meta,
            import_meta_env,
            jetpm_output_directory: jetpm_output_directory.to_owned(),
            jetpm_constants: SharedMap::new(),

            env_cache: RefCell::new(None),

            function_types: RefCell::new(HashMap::new()),
            tuple_types: RefCell::new(HashMap::new()),
            nullable_types: RefCell::new(HashMap::new()),
            taets: RefCell::new(HashMap::new()),
            vapaits: RefCell::new(HashMap::new()),
            vipaits: RefCell::new(HashMap::new()),
            faeoits: RefCell::new(HashMap::new()),

            top_level_package,

            jet_lang_package: RefCell::new(None),

            object_type: RefCell::new(None),
            boolean_type: RefCell::new(None),
            string_type: RefCell::new(None),
            char_type: RefCell::new(None),
            number_type: RefCell::new(None),
            single_type: RefCell::new(None),
            long_type: RefCell::new(None),
            big_int_type: RefCell::new(None),
            function_type: RefCell::new(None),
            xml_type: RefCell::new(None),
            xml_list_type: RefCell::new(None),
            class_type: RefCell::new(None),
            array_type: RefCell::new(None),
            map_type: RefCell::new(None),
            namespace_type: RefCell::new(None),
            qname_type: RefCell::new(None),
            byte_array_type: RefCell::new(None),
            reg_exp_type: RefCell::new(None),
            iterator_type: RefCell::new(None),

            infinity_constant: RefCell::new(None),
            nan_constant: RefCell::new(None),

            root_scope: RefCell::new(None),
        })
    }

    pub fn factory(&self) -> SymbolFactory {
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

    /// The JetPM output directory path.
    pub fn jetpm_output_directory(&self) -> String {
        self.jetpm_output_directory.clone()
    }

    /// The JetPM constants.
    pub fn jetpm_constants(&self) -> SharedMap<String, String> {
        self.jetpm_constants.clone()
    }

    pub fn top_level_package(&self) -> Symbol {
        (self.top_level_package).clone()
    }

    /// The `jet.lang.*` package.
    pub fn jet_lang_package(&self) -> Symbol {
        if let Some(r) = self.jet_lang_package.borrow().as_ref() {
            return r.clone();
        }
        let r = self.factory().create_package(["jet", "lang"]);
        self.jet_lang_package.replace(Some(r.clone()));
        r
    }

    /// The `jet.lang.Object` class, possibly `Unresolved`.
    pub fn object_type(&self) -> Symbol {
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
    pub fn boolean_type(&self) -> Symbol {
        if let Some(r) = self.boolean_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Boolean") {
            self.boolean_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.String` class, possibly `Unresolved`.
    pub fn string_type(&self) -> Symbol {
        if let Some(r) = self.string_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("String") {
            self.string_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Char` class, possibly `Unresolved`.
    pub fn char_type(&self) -> Symbol {
        if let Some(r) = self.char_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Char") {
            self.char_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Number` class, possibly `Unresolved`.
    pub fn number_type(&self) -> Symbol {
        if let Some(r) = self.number_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Number") {
            self.number_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Single` class, possibly `Unresolved`.
    pub fn single_type(&self) -> Symbol {
        if let Some(r) = self.single_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Single") {
            self.single_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Long` class, possibly `Unresolved`.
    pub fn long_type(&self) -> Symbol {
        if let Some(r) = self.long_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Long") {
            self.long_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.BigInt` class, possibly `Unresolved`.
    pub fn big_int_type(&self) -> Symbol {
        if let Some(r) = self.big_int_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("BigInt") {
            self.big_int_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Function` class, possibly `Unresolved`.
    pub fn function_type(&self) -> Symbol {
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
    pub fn xml_type(&self) -> Symbol {
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
    pub fn xml_list_type(&self) -> Symbol {
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
    pub fn class_type(&self) -> Symbol {
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

    /// The `jet.lang.Array` class, possibly `Unresolved`.
    pub fn array_type(&self) -> Symbol {
        if let Some(r) = self.array_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Array") {
            self.array_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Array.<*>` class, possibly `Unresolved`.
    pub fn array_type_of_any(&self) -> Symbol {
        let t = self.array_type();
        if t.is_unresolved() {
            t
        } else {
            self.factory().create_type_after_explicit_type_substitution(&t, &shared_array![self.any_type()])
        }
    }

    /// The `jet.lang.Map` class, possibly `Unresolved`.
    pub fn map_type(&self) -> Symbol {
        if let Some(r) = self.map_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Map") {
            self.map_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Map.<*, *>` class, possibly `Unresolved`.
    pub fn map_type_of_any_any(&self) -> Symbol {
        let t = self.map_type();
        if t.is_unresolved() {
            t
        } else {
            self.factory().create_type_after_explicit_type_substitution(&t, &shared_array![self.any_type(), self.any_type()])
        }
    }

    /// The `jet.lang.Namespace` class, possibly `Unresolved`.
    pub fn namespace_type(&self) -> Symbol {
        if let Some(r) = self.namespace_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Namespace") {
            self.namespace_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.QName` class, possibly `Unresolved`.
    pub fn qname_type(&self) -> Symbol {
        if let Some(r) = self.qname_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("QName") {
            self.qname_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.ByteArray` class, possibly `Unresolved`.
    pub fn byte_array_type(&self) -> Symbol {
        if let Some(r) = self.byte_array_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("ByteArray") {
            self.byte_array_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.RegExp` class, possibly `Unresolved`.
    pub fn reg_exp_type(&self) -> Symbol {
        if let Some(r) = self.reg_exp_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("RegExp") {
            self.reg_exp_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Iterator` class, possibly `Unresolved`.
    pub fn iterator_type(&self) -> Symbol {
        if let Some(r) = self.iterator_type.borrow().as_ref() {
            return r.clone();
        }
        if let Some(r) = self.lookup_at_jet_lang("Iterator") {
            self.iterator_type.replace(Some(r.clone()));
            r
        } else {
            self.unresolved()
        }
    }

    /// The `jet.lang.Infinity` constant, possibly `Unresolved`.
    pub fn infinity_constant(&self) -> Symbol {
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
    pub fn nan_constant(&self) -> Symbol {
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

    fn lookup_at_jet_lang(&self, name: &str) -> Option<Symbol> {
        self.jet_lang_package().properties(self).get(&name.to_owned())
    }

    pub fn is_numeric_type(&self, symbol: &Symbol) -> bool {
        [
            self.number_type(),
            self.single_type(),
            self.long_type(),
            self.big_int_type(),
        ].contains(symbol)
    }

    pub fn is_integer_type(&self, symbol: &Symbol) -> bool {
        [
            self.long_type(),
            self.big_int_type(),
        ].contains(symbol)
    }

    pub fn is_floating_point_type(&self, symbol: &Symbol) -> bool {
        [
            self.number_type(),
            self.single_type(),
        ].contains(symbol)
    }

    /// The root scope that imports the top-level package and `jet.lang.*`.
    pub fn root_scope(&self) -> Symbol {
        if let Some(r) = self.root_scope.borrow().as_ref() {
            return r.clone();
        }
        let r = self.factory().create_scope();
        r.open_packages().push(self.top_level_package());
        r.open_packages().push(self.jet_lang_package());
        r
    }

    pub(crate) fn preload_environment_variables(&self) -> Rc<HashMap<String, String>> {
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