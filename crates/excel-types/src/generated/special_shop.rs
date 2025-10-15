/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SpecialShop {
    pub row_id: u32,
    pub name: String,
    pub use_currency_type: u8,
    pub quest_unlock_id: u32,
    pub quest_unlock: RowRef<Quest>,
    pub complete_text_id: u32,
    pub complete_text: RowRef<DefaultTalk>,
    pub not_complete_text_id: u32,
    pub not_complete_text: RowRef<DefaultTalk>,
}

impl Sheet for SpecialShop {
    const SHEET_NAME: &'static str = "SpecialShop";
}

impl FromExcelRow for SpecialShop {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            use_currency_type: single_row.columns.get(2041).to_u8(),
            quest_unlock_id: single_row.columns.get(2042).to_u32(),
            quest_unlock: RowRef::<Quest>::from(single_row.columns.get(2042).to_u32()),
            complete_text_id: single_row.columns.get(2043).to_u32(),
            complete_text: RowRef::<DefaultTalk>::from(single_row.columns.get(2043).to_u32()),
            not_complete_text_id: single_row.columns.get(2044).to_u32(),
            not_complete_text: RowRef::<DefaultTalk>::from(single_row.columns.get(2044).to_u32()),
        })
    }
}

