use std::cell::{RefCell, Cell, RefMut};
use std::rc::Rc;
use crate::ns::*;

/// `CompilationUnit` identifies a Jet compilation unit and contains
/// a source text.
pub struct CompilationUnit {
    pub(crate) file_path: Option<String>,
    pub(crate) text: String,
    pub(crate) line_number_offsets: RefCell<Vec<usize>>,
    pub(crate) already_tokenized: Cell<bool>,
    diagnostics: RefCell<Vec<Diagnostic>>,
    pub(crate) error_count: Cell<u32>,
    pub(crate) warning_count: Cell<u32>,
    pub(crate) invalidated: Cell<bool>,
    pub(crate) compiler_options: Rc<CompilerOptions>,
    pub(crate) comments: RefCell<Vec<Rc<Comment>>>,
}

impl Default for CompilationUnit {
    fn default() -> Self {
        Self {
            file_path: None,
            text: "".into(),
            line_number_offsets: RefCell::new(vec![0, 0]),
            already_tokenized: Cell::new(false),
            diagnostics: RefCell::new(vec![]),
            invalidated: Cell::new(false),
            error_count: Cell::new(0),
            warning_count: Cell::new(0),
            compiler_options: CompilerOptions::new(),
            comments: RefCell::new(vec![]),
        }
    }
}

impl CompilationUnit {
    /// Constructs a source file in unparsed and non verified state.
    pub fn new(file_path: Option<String>, text: String, compiler_options: &Rc<CompilerOptions>) -> Rc<Self> {
        Rc::new(Self {
            file_path,
            text,
            line_number_offsets: RefCell::new(vec![0, 0]),
            already_tokenized: Cell::new(false),
            diagnostics: RefCell::new(vec![]),
            invalidated: Cell::new(false),
            error_count: Cell::new(0),
            warning_count: Cell::new(0),
            compiler_options: compiler_options.clone(),
            comments: RefCell::new(vec![]),
        })
    }

    /// File path of the source or `None` if not a file.
    pub fn file_path(&self) -> Option<String> {
        self.file_path.clone()
    }

    /// Source text.
    pub fn text(&self) -> &String {
        &self.text
    }

    /// Whether the source contains any errors after parsing
    /// and/or verification.
    pub fn invalidated(&self) -> bool {
        self.invalidated.get()
    }

    /// The comments present in the source file. To get mutable access to the
    /// collection of comments, use the `comments_mut()` method instead.
    pub fn comments(&self) -> Vec<Rc<Comment>> {
        let mut collection = vec![];
        for c in self.comments.borrow().iter() {
            collection.push(c.clone());
        }
        collection
    }

    /// The comments present in the source file, as a mutable collection.
    pub fn comments_mut(&self) -> RefMut<Vec<Rc<Comment>>> {
        self.comments.borrow_mut()
    }

    /// Diagnostics of the source file after parsing and/or
    /// verification.
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.diagnostics.borrow().clone()
    }

    /// Sort diagnostics from the compilation unit.
    pub fn sort_diagnostics(&self) {
        self.diagnostics.borrow_mut().sort();
    }

    pub fn add_diagnostic(&self, diagnostic: Diagnostic) {
        if diagnostic.is_warning() {
            self.warning_count.set(self.warning_count.get() + 1);
        } else {
            self.error_count.set(self.error_count.get() + 1);
            self.invalidated.set(true);
        }
        self.diagnostics.borrow_mut().push(diagnostic);
    }

    pub fn error_count(&self) -> u32 {
        self.error_count.get()
    }

    pub fn warning_count(&self) -> u32 {
        self.warning_count.get()
    }
    
    /// Gets offset from line number (counted from one).
    pub fn get_line_offset(&self, line: usize) -> Option<usize> {
        self.line_number_offsets.borrow().get(line).map(|v| *v)
    }

    pub fn get_line_indent(&self, line: usize) -> usize {
        let line_offset = self.get_line_offset(line).unwrap_or(*self.line_number_offsets.borrow().last().unwrap());
        let indent = CharacterValidator::indent_count(&self.text[line_offset..]);
        indent - line_offset
    }
}