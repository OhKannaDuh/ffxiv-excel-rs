/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GilShop {
    pub row_id: u32,
    pub name: String,
    pub icon_id: u32,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub accept_talk_id: u32,
    pub accept_talk: RowRef<DefaultTalk>,
    pub fail_talk_id: u32,
    pub fail_talk: RowRef<DefaultTalk>,
}

impl Sheet for GilShop {
    const SHEET_NAME: &'static str = "GilShop";
}

impl FromExcelRow for GilShop {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            icon_id: single_row.columns.get(1).to_u32(),
            quest_id: single_row.columns.get(2).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(2).to_u32()),
            accept_talk_id: single_row.columns.get(3).to_u32(),
            accept_talk: RowRef::<DefaultTalk>::from(single_row.columns.get(3).to_u32()),
            fail_talk_id: single_row.columns.get(4).to_u32(),
            fail_talk: RowRef::<DefaultTalk>::from(single_row.columns.get(4).to_u32()),
        })
    }
}

