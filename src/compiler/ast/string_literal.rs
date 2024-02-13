use crate::ns::*;
use serde::{Serialize, Deserialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct StringLiteral {
    pub location: Location,
    pub value: String,
}

impl StringLiteral {
    pub(crate) fn verify(&self, verifier: &mut VerifierVerifier, context: &ExpressionVerifyContext) -> Result<Option<Symbol>, DeferVerificationError> {
        if let Some(t) = context.context_type.as_ref() {
            let t_non_null = t.non_null_type();
            if t_non_null == verifier.host.char_type() {
                let ch = self.value.chars().collect::<Vec<char>>();
                if ch.len() != 1 {
                    verifier.add_verify_error(&self.location, DiagnosticKind::StringLiteralMustBeASingleCharacter, diagnostic_arguments![]);
                    return Ok(None);
                }
                return Ok(Some(verifier.host.factory().create_char_constant(ch[0], &t)));
            }
            if t_non_null.is_enum_type() {
                let et = t_non_null;
                let m = et.enumeration_members().get(&self.value);
                if let Some(m) = m {
                    return Ok(Some(verifier.host.factory().create_enum_constant(m, &t)));
                } else {
                    verifier.add_verify_error(&self.location, DiagnosticKind::EnumerationHasNoMember, diagnostic_arguments![Symbol(et), String(self.value.clone())]);
                    return Ok(None);
                }
            }
        }
        return Ok(Some(verifier.host.factory().create_string_constant(self.value.clone(), &verifier.host.string_type())));
    }
}