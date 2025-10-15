/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TripleTriadCard {
    pub row_id: u32,
    pub name: String,
    pub starts_with_vowel: i8,
    pub description: String,
}

impl Sheet for TripleTriadCard {
    const SHEET_NAME: &'static str = "TripleTriadCard";
}

impl FromExcelRow for TripleTriadCard {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            starts_with_vowel: single_row.columns.get(4).to_i8(),
            description: single_row.columns.get(8).to_owned_string(),
        })
    }
}

