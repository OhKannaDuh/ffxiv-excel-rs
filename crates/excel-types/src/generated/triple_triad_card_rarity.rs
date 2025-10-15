/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TripleTriadCardRarity {
    pub row_id: u32,
    pub stars: u8,
}

impl Sheet for TripleTriadCardRarity {
    const SHEET_NAME: &'static str = "TripleTriadCardRarity";
}

impl FromExcelRow for TripleTriadCardRarity {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            stars: single_row.columns.get(0).to_u8(),
        })
    }
}

