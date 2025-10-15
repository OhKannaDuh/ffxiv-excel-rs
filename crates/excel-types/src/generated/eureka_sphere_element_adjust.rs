/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EurekaSphereElementAdjust {
    pub row_id: u32,
    pub power_modifier: u16,
}

impl Sheet for EurekaSphereElementAdjust {
    const SHEET_NAME: &'static str = "EurekaSphereElementAdjust";
}

impl FromExcelRow for EurekaSphereElementAdjust {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            power_modifier: single_row.columns.get(0).to_u16(),
        })
    }
}

