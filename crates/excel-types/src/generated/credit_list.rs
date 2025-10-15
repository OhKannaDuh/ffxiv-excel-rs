/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CreditList {
    pub row_id: u32,
    pub scale: u16,
    pub icon_id: u32,
    pub font_id: u32,
    pub cast_id: u32,
    pub cast: RowRef<CreditListText>,
}

impl Sheet for CreditList {
    const SHEET_NAME: &'static str = "CreditList";
}

impl FromExcelRow for CreditList {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            scale: single_row.columns.get(0).to_u16(),
            icon_id: single_row.columns.get(1).to_u32(),
            font_id: single_row.columns.get(2).to_u32(),
            cast_id: single_row.columns.get(5).to_u32(),
            cast: RowRef::<CreditListText>::from(single_row.columns.get(5).to_u32()),
        })
    }
}

