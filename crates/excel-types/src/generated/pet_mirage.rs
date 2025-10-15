/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PetMirage {
    pub row_id: u32,
    pub name: String,
}

impl Sheet for PetMirage {
    const SHEET_NAME: &'static str = "PetMirage";
}

impl FromExcelRow for PetMirage {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(2).to_owned_string(),
        })
    }
}

