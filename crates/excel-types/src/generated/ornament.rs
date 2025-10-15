/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Ornament {
    pub row_id: u32,
    pub model: u16,
    pub order: i16,
    pub icon_id: u32,
    pub transient: u16,
    pub singular: String,
    pub adjective: i8,
    pub plural: String,
    pub possessive_pronoun: i8,
    pub starts_with_vowel: i8,
    pub pronoun: i8,
    pub article: i8,
}

impl Sheet for Ornament {
    const SHEET_NAME: &'static str = "Ornament";
}

impl FromExcelRow for Ornament {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            model: single_row.columns.get(0).to_u16(),
            order: single_row.columns.get(5).to_i16(),
            icon_id: single_row.columns.get(6).to_u32(),
            transient: single_row.columns.get(7).to_u16(),
            singular: single_row.columns.get(8).to_owned_string(),
            adjective: single_row.columns.get(9).to_i8(),
            plural: single_row.columns.get(10).to_owned_string(),
            possessive_pronoun: single_row.columns.get(11).to_i8(),
            starts_with_vowel: single_row.columns.get(12).to_i8(),
            pronoun: single_row.columns.get(14).to_i8(),
            article: single_row.columns.get(15).to_i8(),
        })
    }
}

