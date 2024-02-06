use std::collections::HashMap;
use maplit::hashmap;
use crate::ns::*;

#[path = "diagnostics_english_resources.rs"]
mod diagnostics_english_resources;

/// Represents a diagnostic originated from a compilation unit.
/// 
/// Arguments are formatted using integer keys counted from 1 (one).
#[derive(Clone)]
pub struct Diagnostic {
    pub(crate) location: Location,
    pub(crate) kind: DiagnosticKind,
    pub(crate) is_warning: bool,
    pub(crate) is_verify_error: bool,
    pub(crate) arguments: Vec<DiagnosticArgument>,
}

impl Eq for Diagnostic {}

impl PartialEq for Diagnostic {
    fn eq(&self, other: &Self) -> bool {
        self.location == other.location &&
        self.kind == other.kind
    }
}

impl Ord for Diagnostic {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.location.cmp(&other.location)
    }
}

impl PartialOrd for Diagnostic {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.location.partial_cmp(&other.location)
    }
}

impl Diagnostic {
    pub fn new_syntax_error(location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) -> Self {
        Self {
            location: location.clone(),
            kind,
            is_verify_error: false,
            is_warning: false,
            arguments,
        }
    }

    pub fn new_verify_error(location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) -> Self {
        Self {
            location: location.clone(),
            kind,
            is_verify_error: true,
            is_warning: false,
            arguments,
        }
    }

    pub fn new_warning(location: &Location, kind: DiagnosticKind, arguments: Vec<DiagnosticArgument>) -> Self {
        Self {
            location: location.clone(),
            kind,
            is_verify_error: false,
            is_warning: true,
            arguments,
        }
    }

    pub fn location(&self) -> Location {
        self.location.clone()
    }

    pub fn kind(&self) -> DiagnosticKind {
        self.kind.clone()
    }

    pub fn is_warning(&self) -> bool {
        self.is_warning
    }

    pub fn is_verify_error(&self) -> bool {
        self.is_verify_error
    }

    pub fn arguments(&self) -> Vec<DiagnosticArgument> {
        self.arguments.clone()
    }

    pub fn id(&self) -> i32 {
        self.kind.id()
    }

    /// Formats the diagnostic in English.
    pub fn format_english(&self) -> String {
        let category = (if self.is_verify_error {
            "Verify error"
        } else if self.is_warning {
            "Warning"
        } else {
            "Syntax error"
        }).to_owned();

        let file_path = self.location.compilation_unit.file_path.clone().map_or("".to_owned(), |s| format!("{s}:"));
        let line = self.location.first_line_number();
        let column = self.location.first_column() + 1;
        let message = self.format_message_english();
        let id = self.id().to_string();
        format!("{file_path}{line}:{column}: {category} #{id}: {message}")
    }

    pub fn format_message_english(&self) -> String {
        self.format_message(&diagnostics_english_resources::DATA)
    }

    pub fn format_message(&self, messages: &HashMap<i32, String>) -> String {
        let mut string_arguments: HashMap<String, String> = hashmap!{};
        let mut i = 1;
        for argument in &self.arguments {
            string_arguments.insert(i.to_string(), self.format_argument(argument.clone()));
            i += 1;
        }
        use late_format::LateFormat;
        let Some(msg) = messages.get(&self.id()) else {
            let id = self.id();
            panic!("Message resource is missing for ID {id}");
        };
        msg.late_format(string_arguments)
    }

    fn format_argument(&self, argument: DiagnosticArgument) -> String {
        match argument {
            DiagnosticArgument::String(s) => s.clone(),
            DiagnosticArgument::Token(t) => t.to_string(),
            DiagnosticArgument::Symbol(s) => s.to_string(),
        }
    }
}

pub macro diagnostic_arguments {
    ($($variant:ident($value:expr)),*) => { vec![ $(DiagnosticArgument::$variant($value)),* ] },
}

#[derive(Clone)]
pub enum DiagnosticArgument {
    String(String),
    Token(Token),
    Symbol(Symbol),
}