/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CreditBackImage {
    pub row_id: u32,
    pub back_image_id: u32,
}

impl Sheet for CreditBackImage {
    const SHEET_NAME: &'static str = "CreditBackImage";
}

impl FromExcelRow for CreditBackImage {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            back_image_id: single_row.columns.get(5).to_u32(),
        })
    }
}

