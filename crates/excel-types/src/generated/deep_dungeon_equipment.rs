/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DeepDungeonEquipment {
    pub row_id: u32,
    pub icon_id: u32,
    pub singular: String,
    pub adjective: i8,
    pub plural: String,
    pub possessive_pronoun: i8,
    pub starts_with_vowel: i8,
    pub pronoun: i8,
    pub article: i8,
    pub name: String,
    pub description: String,
}

impl Sheet for DeepDungeonEquipment {
    const SHEET_NAME: &'static str = "DeepDungeonEquipment";
}

impl FromExcelRow for DeepDungeonEquipment {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
            singular: single_row.columns.get(1).to_owned_string(),
            adjective: single_row.columns.get(2).to_i8(),
            plural: single_row.columns.get(3).to_owned_string(),
            possessive_pronoun: single_row.columns.get(4).to_i8(),
            starts_with_vowel: single_row.columns.get(5).to_i8(),
            pronoun: single_row.columns.get(7).to_i8(),
            article: single_row.columns.get(8).to_i8(),
            name: single_row.columns.get(9).to_owned_string(),
            description: single_row.columns.get(10).to_owned_string(),
        })
    }
}

