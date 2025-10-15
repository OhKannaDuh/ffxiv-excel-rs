/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RideShootingTextData {
    pub row_id: u32,
    pub string: String,
}

impl Sheet for RideShootingTextData {
    const SHEET_NAME: &'static str = "RideShootingTextData";
}

impl FromExcelRow for RideShootingTextData {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            string: single_row.columns.get(0).to_owned_string(),
        })
    }
}

