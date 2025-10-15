/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BNpcName {
    pub row_id: u32,
    pub singular: String,
    pub adjective: i8,
    pub plural: String,
    pub possessive_pronoun: i8,
    pub starts_with_vowel: i8,
    pub pronoun: i8,
    pub article: i8,
}

impl Sheet for BNpcName {
    const SHEET_NAME: &'static str = "BNpcName";
}

impl FromExcelRow for BNpcName {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            singular: single_row.columns.get(0).to_owned_string(),
            adjective: single_row.columns.get(1).to_i8(),
            plural: single_row.columns.get(2).to_owned_string(),
            possessive_pronoun: single_row.columns.get(3).to_i8(),
            starts_with_vowel: single_row.columns.get(4).to_i8(),
            pronoun: single_row.columns.get(6).to_i8(),
            article: single_row.columns.get(7).to_i8(),
        })
    }
}

