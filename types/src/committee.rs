use num_enum::TryFromPrimitive;
use serde::{Deserialize, Serialize};

#[repr(u32)]
#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, TryFromPrimitive, Serialize, Deserialize,
)]
pub enum CommitteeVersionId {
    Version0 = 0,
    Version1,
    Version2,
}

impl CommitteeVersionId {
    pub fn latest() -> Self {
        Self::Version0
    }

    pub fn next() -> Self {
        Self::Version1
    }
}
