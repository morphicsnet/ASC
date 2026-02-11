use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Verdict {
    Allow,
    Clamp,
    Hold,
    Override,
    Shutdown,
}

impl Verdict {
    pub fn precedence(self) -> u8 {
        match self {
            Self::Allow => 0,
            Self::Clamp => 1,
            Self::Hold => 2,
            Self::Override => 3,
            Self::Shutdown => 4,
        }
    }
}
