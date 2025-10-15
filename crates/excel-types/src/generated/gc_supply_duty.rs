/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GCSupplyDuty {
    pub row_id: u32,
}

impl Sheet for GCSupplyDuty {
    const SHEET_NAME: &'static str = "GCSupplyDuty";
}

impl FromExcelRow for GCSupplyDuty {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
        })
    }
}

