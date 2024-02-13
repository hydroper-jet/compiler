use crate::ns::*;
use serde::{Serialize, Deserialize};
use std::rc::Rc;

/// The `embed {...}` expression.
/// 
/// It is semantically assigned an `EmbedValue` symbol.
#[derive(Clone, Serialize, Deserialize)]
pub struct EmbedExpression {
    pub location: Location,
    pub description: ObjectInitializer,
}

impl EmbedExpression {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier, context: &ExpressionVerifyContext) -> Result<Option<Symbol>, DeferVerificationError> {
        let EmbedExpression { description, .. } = self;

        let mut source: Option<String> = None;
        let mut result_type: Option<Symbol> = None;

        for field in &description.fields {
            if let InitializerField::Field { name, value, .. } = field.as_ref() {
                if name.0.id_equals("source") {
                    if let Some(source_1) = self.verify_source_field_value(verifier, value) {
                        source = Some(source_1);
                    } else {
                        verifier.add_verify_error(&field.location(), DiagnosticKind::UnrecognizedEmbedExpressionField, diagnostic_arguments![]);
                    }
                } else if name.0.id_equals("type") {
                    if let Some(value) = value {
                        result_type = verifier.verify_type_expression(value)?;
                    } else {
                        verifier.add_verify_error(&field.location(), DiagnosticKind::UnrecognizedEmbedExpressionField, diagnostic_arguments![]);
                    }
                } else {
                    verifier.add_verify_error(&field.location(), DiagnosticKind::UnrecognizedEmbedExpressionField, diagnostic_arguments![]);
                }
            } else {
                verifier.add_verify_error(&field.location(), DiagnosticKind::UnrecognizedEmbedExpressionField, diagnostic_arguments![]);
            }
        }

        result_type = result_type.or(context.context_type.clone()).map(|t| t.non_null_type());

        if source.is_none() || result_type.is_none() {
            verifier.add_verify_error(&self.location, DiagnosticKind::EmbedSourceOrTypeNotSpecified, diagnostic_arguments![]);
            return Ok(None);
        }

        let source = source.unwrap();
        let result_type = result_type.unwrap();

        // String
        if result_type == verifier.host.string_type() {
            if let Ok(data) = std::fs::read_to_string(&source) {
                let v = verifier.host.factory().create_embed_value(EmbedValueDataContent::String(data));
                Ok(Some(v))
            } else {
                verifier.add_verify_error(&self.location, DiagnosticKind::FailedLoadingEmbeddedFile, diagnostic_arguments![String(source)]);
                Ok(None)
            }
        // ByteArray
        } else if result_type == verifier.host.byte_array_type() {
            if let Ok(data) = std::fs::read(&source) {
                let v = verifier.host.factory().create_embed_value(EmbedValueDataContent::ByteArray(data));
                Ok(Some(v))
            } else {
                verifier.add_verify_error(&self.location, DiagnosticKind::FailedLoadingEmbeddedFile, diagnostic_arguments![String(source)]);
                Ok(None)
            }
        // Unsupported data type
        } else {
            verifier.add_verify_error(&self.location, DiagnosticKind::EmbedUnsupportedType, diagnostic_arguments![Symbol(result_type)]);
            Ok(None)
        }
    }

    fn verify_source_field_value(&self, verifier: &mut VerifierVerifier, value: &Option<Rc<Expression>>) -> Option<String> {
        use file_paths::FlexPath;
        if value.is_none() {
            return None;
        }
        let value = value.as_ref().unwrap();
        match value.as_ref() {
            Expression::StringLiteral(StringLiteral { value, .. }) => {
                Some(FlexPath::new_native(&self.location.compilation_unit().file_path().unwrap_or(String::new())).resolve("..").resolve(value).to_string_with_flex_separator())
            },
            Expression::Binary(BinaryExpression { left, operator, right, .. }) => {
                if *operator != Operator::Add {
                    return None;
                }
                let Some(left_id) = left.to_identifier_name_or_asterisk() else {
                    return None;
                };
                if left_id.0 != "output" {
                    return None;
                }
                let Expression::StringLiteral(StringLiteral { value: right_val, .. }) = right.as_ref() else {
                    return None;
                };
                Some(FlexPath::from_n_native([verifier.host.jetpm_output_directory().as_ref(), right_val.as_ref()]).to_string_with_flex_separator())
            },
            _ => None,
        }
    }
}