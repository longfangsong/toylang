use sum_type::sum_type;

mod alloca;
mod branch;
mod calculate;
mod call;
mod jump;
mod load;
pub mod phi;
mod store;

use alloca::Alloca;
use branch::Branch;
use calculate::Calculate;
use jump::Jump;
use load::Load;
use phi::Phi;
use store::Store;

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum IRStatement {
        Alloca,
        Branch,
        Calculate,
        Jump,
        Load,
        Store,
        Phi,
    }
}

sum_type! {
    #[derive(Debug, Eq, PartialEq, Clone)]
    pub enum Terminator {
        Branch,
        Jump,
    }
}
