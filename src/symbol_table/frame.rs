use std::collections::HashMap;

use crate::symbol_table::symbol::Symbol;

pub(crate) type Frame = HashMap<String, Symbol>;