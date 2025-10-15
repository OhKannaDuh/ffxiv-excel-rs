pub mod core;
pub use core::{FromExcelRow, MultiRef2, RowRef, Sheet};

mod generated {
    include!("generated/mod.rs");
}

pub mod prelude {
    pub use crate::core::*;
    pub use crate::generated::*;
    pub(crate) use physis::exd::{ExcelRow, ExcelRowKind};
}
