/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ResistanceWeaponAdjust {
    pub row_id: u32,
    pub max_total_stats: u16,
    pub max_each_stat: u16,
    pub image_id: u32,
}

impl Sheet for ResistanceWeaponAdjust {
    const SHEET_NAME: &'static str = "ResistanceWeaponAdjust";
}

impl FromExcelRow for ResistanceWeaponAdjust {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            max_total_stats: single_row.columns.get(0).to_u16(),
            max_each_stat: single_row.columns.get(1).to_u16(),
            image_id: single_row.columns.get(6).to_u32(),
        })
    }
}

