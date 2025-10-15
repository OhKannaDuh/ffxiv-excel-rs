/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ItemAction {
    pub row_id: u32,
    pub cond_lv: u8,
    pub cond_battle: bool,
    pub cond_pvp: bool,
    pub cond_pvp_only: bool,
    pub _type: u16,
}

impl Sheet for ItemAction {
    const SHEET_NAME: &'static str = "ItemAction";
}

impl FromExcelRow for ItemAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            cond_lv: single_row.columns.get(0).to_u8(),
            cond_battle: single_row.columns.get(1).to_bit(0),
            cond_pvp: single_row.columns.get(2).to_bit(1),
            cond_pvp_only: single_row.columns.get(3).to_bit(2),
            _type: single_row.columns.get(4).to_u16(),
        })
    }
}

