/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Snipe {
    pub row_id: u32,
    pub lgb_target_marker: u32,
    pub vfx_fire: String,
    pub vfx_hit: String,
    pub vfx_miss: String,
    pub vfx_additional: String,
    pub objective_0: String,
    pub hint_0: String,
    pub objective_1: String,
    pub hint_1: String,
    pub action_text: String,
}

impl Sheet for Snipe {
    const SHEET_NAME: &'static str = "Snipe";
}

impl FromExcelRow for Snipe {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            lgb_target_marker: single_row.columns.get(0).to_u32(),
            vfx_fire: single_row.columns.get(11).to_owned_string(),
            vfx_hit: single_row.columns.get(12).to_owned_string(),
            vfx_miss: single_row.columns.get(13).to_owned_string(),
            vfx_additional: single_row.columns.get(14).to_owned_string(),
            objective_0: single_row.columns.get(93).to_owned_string(),
            hint_0: single_row.columns.get(94).to_owned_string(),
            objective_1: single_row.columns.get(95).to_owned_string(),
            hint_1: single_row.columns.get(96).to_owned_string(),
            action_text: single_row.columns.get(104).to_owned_string(),
        })
    }
}

