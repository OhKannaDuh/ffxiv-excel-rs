/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ContentsNote {
    pub row_id: u32,
    pub content_type_id: u32,
    pub icon_id: u32,
    pub menu_order: u8,
    pub required_amount: i32,
    pub reward_0: u8,
    pub exp_multiplier: i32,
    pub reward_1: u8,
    pub gil_rward: i32,
    pub level_unlock: u16,
    pub how_to_id: u32,
    pub how_to: RowRef<HowTo>,
    pub req_unlock_id: u32,
    pub req_unlock_quest: RowRef<Quest>,
    pub name: String,
    pub description: String,
    pub exp_cap: i32,
}

impl Sheet for ContentsNote {
    const SHEET_NAME: &'static str = "ContentsNote";
}

impl FromExcelRow for ContentsNote {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            content_type_id: single_row.columns.get(0).to_u32(),
            icon_id: single_row.columns.get(1).to_u32(),
            menu_order: single_row.columns.get(2).to_u8(),
            required_amount: single_row.columns.get(3).to_i32(),
            reward_0: single_row.columns.get(4).to_u8(),
            exp_multiplier: single_row.columns.get(5).to_i32(),
            reward_1: single_row.columns.get(6).to_u8(),
            gil_rward: single_row.columns.get(7).to_i32(),
            level_unlock: single_row.columns.get(8).to_u16(),
            how_to_id: single_row.columns.get(9).to_u32(),
            how_to: RowRef::<HowTo>::from(single_row.columns.get(9).to_u32()),
            req_unlock_id: single_row.columns.get(10).to_u32(),
            req_unlock_quest: RowRef::<Quest>::from(single_row.columns.get(10).to_u32()),
            name: single_row.columns.get(11).to_owned_string(),
            description: single_row.columns.get(12).to_owned_string(),
            exp_cap: single_row.columns.get(13).to_i32(),
        })
    }
}

