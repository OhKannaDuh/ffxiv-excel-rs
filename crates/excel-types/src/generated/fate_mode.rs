/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FateMode {
    pub row_id: u32,
    pub motivation_icon_id: u32,
    pub motivation_map_marker_id: u32,
    pub objective_icon_id: u32,
    pub objective_map_marker_id: u32,
}

impl Sheet for FateMode {
    const SHEET_NAME: &'static str = "FateMode";
}

impl FromExcelRow for FateMode {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            motivation_icon_id: single_row.columns.get(1).to_u32(),
            motivation_map_marker_id: single_row.columns.get(2).to_u32(),
            objective_icon_id: single_row.columns.get(3).to_u32(),
            objective_map_marker_id: single_row.columns.get(4).to_u32(),
        })
    }
}

