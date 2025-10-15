/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DynamicEventType {
    pub row_id: u32,
    pub icon_objective_0_id: u32,
    pub icon_objective_1_id: u32,
}

impl Sheet for DynamicEventType {
    const SHEET_NAME: &'static str = "DynamicEventType";
}

impl FromExcelRow for DynamicEventType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_objective_0_id: single_row.columns.get(0).to_u32(),
            icon_objective_1_id: single_row.columns.get(1).to_u32(),
        })
    }
}

