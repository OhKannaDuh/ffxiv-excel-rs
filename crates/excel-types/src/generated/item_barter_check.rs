/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ItemBarterCheck {
    pub row_id: u32,
    pub category_id: u32,
    pub question_id: u32,
    pub question_log_message: RowRef<LogMessage>,
    pub question_addon: RowRef<Addon>,
    pub confirm_id: u32,
    pub confirm: RowRef<Addon>,
}

impl Sheet for ItemBarterCheck {
    const SHEET_NAME: &'static str = "ItemBarterCheck";
}

impl FromExcelRow for ItemBarterCheck {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            category_id: single_row.columns.get(0).to_u32(),
            question_id: single_row.columns.get(1).to_u32(),
            question_log_message: RowRef::<LogMessage>::from(single_row.columns.get(1).to_u32()),
            question_addon: RowRef::<Addon>::from(single_row.columns.get(1).to_u32()),
            confirm_id: single_row.columns.get(2).to_u32(),
            confirm: RowRef::<Addon>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

