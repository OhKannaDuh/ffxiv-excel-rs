/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AirshipExplorationPart {
    pub row_id: u32,
    pub slot: u8,
    pub rank: u8,
    pub components: u8,
    pub surveillance: i16,
    pub retrieval: i16,
    pub speed: i16,
    pub range: i16,
    pub favor: i16,
    pub class: u16,
    pub repair_materials: u8,
}

impl Sheet for AirshipExplorationPart {
    const SHEET_NAME: &'static str = "AirshipExplorationPart";
}

impl FromExcelRow for AirshipExplorationPart {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            slot: single_row.columns.get(0).to_u8(),
            rank: single_row.columns.get(1).to_u8(),
            components: single_row.columns.get(2).to_u8(),
            surveillance: single_row.columns.get(3).to_i16(),
            retrieval: single_row.columns.get(4).to_i16(),
            speed: single_row.columns.get(5).to_i16(),
            range: single_row.columns.get(6).to_i16(),
            favor: single_row.columns.get(7).to_i16(),
            class: single_row.columns.get(8).to_u16(),
            repair_materials: single_row.columns.get(9).to_u8(),
        })
    }
}

