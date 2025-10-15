/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InclusionShop {
    pub row_id: u32,
}

impl Sheet for InclusionShop {
    const SHEET_NAME: &'static str = "InclusionShop";
}

impl FromExcelRow for InclusionShop {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
        })
    }
}

