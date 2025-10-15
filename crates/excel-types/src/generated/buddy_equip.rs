/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BuddyEquip {
    pub row_id: u32,
    pub singular: String,
    pub adjective: i8,
    pub plural: String,
    pub possessive_pronoun: i8,
    pub starts_with_vowel: i8,
    pub pronoun: i8,
    pub article: i8,
    pub name: String,
    pub model_top: i32,
    pub model_body: i32,
    pub model_legs: i32,
    pub grand_company_id: u32,
    pub grand_company: RowRef<GrandCompany>,
    pub icon_head_id: u32,
    pub icon_body_id: u32,
    pub icon_legs_id: u32,
    pub order: u8,
}

impl Sheet for BuddyEquip {
    const SHEET_NAME: &'static str = "BuddyEquip";
}

impl FromExcelRow for BuddyEquip {
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
            name: single_row.columns.get(8).to_owned_string(),
            model_top: single_row.columns.get(9).to_i32(),
            model_body: single_row.columns.get(10).to_i32(),
            model_legs: single_row.columns.get(11).to_i32(),
            grand_company_id: single_row.columns.get(12).to_u32(),
            grand_company: RowRef::<GrandCompany>::from(single_row.columns.get(12).to_u32()),
            icon_head_id: single_row.columns.get(13).to_u32(),
            icon_body_id: single_row.columns.get(14).to_u32(),
            icon_legs_id: single_row.columns.get(15).to_u32(),
            order: single_row.columns.get(16).to_u8(),
        })
    }
}

