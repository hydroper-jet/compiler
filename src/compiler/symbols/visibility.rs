#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Visibility {
    Public,
    Private,
    Protected,
    Internal,
}

impl ToString for Visibility {
    fn to_string(&self) -> String {
        match self {
            Self::Public => "public".into(),
            Self::Private => "private".into(),
            Self::Protected => "protected".into(),
            Self::Internal => "internal".into(),
        }
    }
}