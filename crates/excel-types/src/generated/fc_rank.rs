/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FCRank {
    pub row_id: u32,
    pub next_point: u32,
    pub current_point: u32,
    pub rights: u16,
    pub fc_action_active_num: u8,
    pub fc_action_stock_num: u8,
    pub fc_chest_compartments: u8,
}

impl Sheet for FCRank {
    const SHEET_NAME: &'static str = "FCRank";
}

impl FromExcelRow for FCRank {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            next_point: single_row.columns.get(0).to_u32(),
            current_point: single_row.columns.get(1).to_u32(),
            rights: single_row.columns.get(2).to_u16(),
            fc_action_active_num: single_row.columns.get(5).to_u8(),
            fc_action_stock_num: single_row.columns.get(6).to_u8(),
            fc_chest_compartments: single_row.columns.get(7).to_u8(),
        })
    }
}

