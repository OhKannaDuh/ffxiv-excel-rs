/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CSBonusContent {
    pub row_id: u32,
    pub content_type_id: u32,
    pub content_type: RowRef<CSBonusContentType>,
}

impl Sheet for CSBonusContent {
    const SHEET_NAME: &'static str = "CSBonusContent";
}

impl FromExcelRow for CSBonusContent {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            content_type_id: single_row.columns.get(0).to_u32(),
            content_type: RowRef::<CSBonusContentType>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

