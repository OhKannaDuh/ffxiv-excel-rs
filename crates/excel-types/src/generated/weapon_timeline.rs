/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WeaponTimeline {
    pub row_id: u32,
    pub file: String,
    pub next_weapon_timeline: i16,
}

impl Sheet for WeaponTimeline {
    const SHEET_NAME: &'static str = "WeaponTimeline";
}

impl FromExcelRow for WeaponTimeline {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            file: single_row.columns.get(0).to_owned_string(),
            next_weapon_timeline: single_row.columns.get(1).to_i16(),
        })
    }
}

