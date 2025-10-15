/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CycleTime {
    pub row_id: u32,
    pub first_cycle: u32,
    pub cycle: u32,
}

impl Sheet for CycleTime {
    const SHEET_NAME: &'static str = "CycleTime";
}

impl FromExcelRow for CycleTime {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            first_cycle: single_row.columns.get(0).to_u32(),
            cycle: single_row.columns.get(1).to_u32(),
        })
    }
}

