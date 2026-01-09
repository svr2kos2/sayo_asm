/// Address type for the assembler
pub type Address = u32;

/// Section identifier
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Section {
    Text,
    Data,
}

impl Section {
    pub fn from_directive_name(name: &str) -> Option<Self> {
        match name {
            "text" => Some(Self::Text),
            "data" => Some(Self::Data),
            _ => None,
        }
    }
}
