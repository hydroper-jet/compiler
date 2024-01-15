use std::cell::RefCell;
use std::rc::Rc;
use crate::ns::*;

pub struct Tokenizer<'input> {
    compilation_unit: Rc<CompilationUnit>,
    line_number: usize,
    characters: CharacterReader<'input>,
}

impl<'input> Tokenizer<'input> {
    /// Constructs a tokenizer.
    pub fn new(compilation_unit: &'input Rc<CompilationUnit>) -> Self {
        let text: &'input str = compilation_unit.text.as_ref();
        let compilation_unit = compilation_unit.clone();
        assert!(!compilation_unit.already_tokenized.get(), "A CompilationUnit must be tokenized at most once.");
        compilation_unit.already_tokenized.set(true);
        Self {
            compilation_unit,
            line_number: 1,
            characters: CharacterReader::from(text),
        }
    }

    pub fn compilation_unit(&self) -> Rc<CompilationUnit> {
        self.compilation_unit.clone()
    }
}