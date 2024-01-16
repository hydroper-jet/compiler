use crate::ns::*;
use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

/// Context used to control the parsing of an expression.
#[derive(Clone)]
pub struct ExpressionContext {
    pub min_precedence: OperatorPrecedence,
    pub allow_in: bool,
    pub allow_assignment: bool,
    pub with_type_annotation: bool,
}

impl Default for ExpressionContext {
    fn default() -> Self {
        Self {
            min_precedence: OperatorPrecedence::List,
            allow_in: true,
            allow_assignment: true,
            with_type_annotation: true,
        }
    }
}

#[derive(Clone)]
struct ArrowFunctionContext {
    left: Option<Rc<Expression>>,
    right_context: ExpressionContext,
}

impl Default for ArrowFunctionContext {
    fn default() -> Self {
        Self {
            left: None,
            right_context: default(),
        }
    }
}

#[derive(Clone)]
pub enum DirectiveContext {
    Default,
    TopLevel,
    PackageBlock,
    ClassBlock {
        name: String,
    },
    InterfaceBlock,
    EnumBlock,
    ConstructorBlock {
        super_statement_found: Cell<bool>,
    },
    WithControl {
        to_be_labeled: Option<String>,
        control_context: ControlContext,
        labels: HashMap<String, ControlContext>,
    },
}

impl DirectiveContext {
    fn is_type_block(&self) -> bool {
        match self {
            Self::ClassBlock { .. } |
            Self::InterfaceBlock |
            Self::EnumBlock => true,
            _ => false,
        }
    }

    fn clone_control(&self) -> Self {
        match self {
            Self::WithControl { .. } => self.clone(),
            _ => Self::Default,
        }
    }

    fn override_control_context(&self, label_only: bool, mut context: ControlContext) -> Self {
        let mut prev_context = None;
        let mut label = None;
        let mut labels = match self {
            Self::WithControl { control_context, labels, to_be_labeled: label1 } => {
                prev_context = Some(control_context.clone());
                label = label1.clone();
                labels.clone()
            },
            _ => HashMap::new(),
        };
        if let Some(label) = label.clone() {
            labels.insert(label, context.clone());
        }
        if label_only {
            context = prev_context.unwrap_or(ControlContext {
                breakable: false,
                iteration: false,
            });
        }
        Self::WithControl { control_context: context, labels, to_be_labeled: None }
    }

    fn put_label(&self, label: String) -> Self {
        match self {
            Self::WithControl { control_context, labels, to_be_labeled: _ } => Self::WithControl {
                to_be_labeled: Some(label),
                control_context: control_context.clone(),
                labels: labels.clone(),
            },
            _ => Self::WithControl {
                to_be_labeled: Some(label),
                control_context: ControlContext {
                    breakable: false,
                    iteration: false,
                },
                labels: HashMap::new(),
            },
        }
    }

    fn is_label_defined(&self, label: String) -> bool {
        self.resolve_label(label).is_some()
    }

    fn resolve_label(&self, label: String) -> Option<ControlContext> {
        if let Self::WithControl { labels, .. } = &self { labels.get(&label).map(|c| c.clone()) } else { None }
    }

    fn is_break_allowed(&self, label: Option<String>) -> bool {
        if let Some(label) = label {
            let context = self.resolve_label(label);
            if let Some(context) = context { context.breakable } else { false }
        } else {
            if let Self::WithControl { control_context, .. } = &self { control_context.breakable } else { false }
        }
    }

    fn is_continue_allowed(&self, label: Option<String>) -> bool {
        if let Some(label) = label {
            let context = self.resolve_label(label);
            if let Some(context) = context { context.iteration } else { false }
        } else {
            if let Self::WithControl { control_context, .. } = &self { control_context.iteration } else { false }
        }
    }
}

#[derive(Clone)]
pub struct ControlContext {
    breakable: bool,
    iteration: bool,
}

#[derive(Clone)]
struct AnnotatableContext {
    start: Location,
    metadata_exp: Vec<Rc<Expression>>,
    asdoc: Option<Rc<JetDoc>>,
    first_modifier: Option<Rc<Expression>>,
    previous_token_is_definition_keyword: bool,
    context: DirectiveContext,
}