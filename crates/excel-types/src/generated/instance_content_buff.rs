/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct InstanceContentBuff {
    pub row_id: u32,
    pub echo_start: u16,
    pub echo_death: u16,
}

impl Sheet for InstanceContentBuff {
    const SHEET_NAME: &'static str = "InstanceContentBuff";
}

impl FromExcelRow for InstanceContentBuff {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            echo_start: single_row.columns.get(0).to_u16(),
            echo_death: single_row.columns.get(1).to_u16(),
        })
    }
}

