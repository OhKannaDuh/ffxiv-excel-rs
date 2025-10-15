/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TripleTriadResident {
    pub row_id: u32,
    pub order: u16,
}

impl Sheet for TripleTriadResident {
    const SHEET_NAME: &'static str = "TripleTriadResident";
}

impl FromExcelRow for TripleTriadResident {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            order: single_row.columns.get(0).to_u16(),
        })
    }
}

