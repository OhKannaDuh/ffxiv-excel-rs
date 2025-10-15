/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SkyIsland2MissionType {
    pub row_id: u32,
    pub _type: bool,
}

impl Sheet for SkyIsland2MissionType {
    const SHEET_NAME: &'static str = "SkyIsland2MissionType";
}

impl FromExcelRow for SkyIsland2MissionType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type: single_row.columns.get(0).to_bit(0),
        })
    }
}

