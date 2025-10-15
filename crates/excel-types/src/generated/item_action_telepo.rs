/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ItemActionTelepo {
    pub row_id: u32,
    pub requirement_id: u32,
    pub requirement_quest: RowRef<Quest>,
    pub requirement_grand_company: RowRef<GrandCompany>,
    pub deny_message_id: u32,
    pub deny_message: RowRef<LogMessage>,
}

impl Sheet for ItemActionTelepo {
    const SHEET_NAME: &'static str = "ItemActionTelepo";
}

impl FromExcelRow for ItemActionTelepo {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            requirement_id: single_row.columns.get(0).to_u32(),
            requirement_quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
            requirement_grand_company: RowRef::<GrandCompany>::from(single_row.columns.get(0).to_u32()),
            deny_message_id: single_row.columns.get(1).to_u32(),
            deny_message: RowRef::<LogMessage>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

