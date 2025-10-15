/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WeeklyBingoOrderData {
    pub row_id: u32,
    pub _type: u32,
    pub data_id: u32,
    pub text_id: u32,
    pub text: RowRef<WeeklyBingoText>,
    pub icon_id: u32,
}

impl Sheet for WeeklyBingoOrderData {
    const SHEET_NAME: &'static str = "WeeklyBingoOrderData";
}

impl FromExcelRow for WeeklyBingoOrderData {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_u32(),
            data_id: single_row.columns.get(1).to_u32(),
            text_id: single_row.columns.get(3).to_u32(),
            text: RowRef::<WeeklyBingoText>::from(single_row.columns.get(3).to_u32()),
            icon_id: single_row.columns.get(4).to_u32(),
        })
    }
}

