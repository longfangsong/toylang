use crate::shared::data_type::Type;
use std::fmt::{self, Display, Formatter};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct Alloca {
    pub to_register: String,
    pub alloc_type: Type,
}

impl Display for Alloca {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{} = alloca {}", self.to_register, self.alloc_type)
    }
}
