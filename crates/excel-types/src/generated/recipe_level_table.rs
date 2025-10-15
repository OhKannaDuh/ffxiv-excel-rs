/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RecipeLevelTable {
    pub row_id: u32,
    pub class_job_level: u8,
    pub stars: u8,
    pub suggested_craftsmanship: u16,
    pub difficulty: u16,
    pub quality: u32,
    pub progress_divider: u8,
    pub quality_divider: u8,
    pub progress_modifier: u8,
    pub quality_modifier: u8,
    pub durability: u16,
    pub conditions_flag: u16,
}

impl Sheet for RecipeLevelTable {
    const SHEET_NAME: &'static str = "RecipeLevelTable";
}

impl FromExcelRow for RecipeLevelTable {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            class_job_level: single_row.columns.get(0).to_u8(),
            stars: single_row.columns.get(1).to_u8(),
            suggested_craftsmanship: single_row.columns.get(2).to_u16(),
            difficulty: single_row.columns.get(3).to_u16(),
            quality: single_row.columns.get(4).to_u32(),
            progress_divider: single_row.columns.get(5).to_u8(),
            quality_divider: single_row.columns.get(6).to_u8(),
            progress_modifier: single_row.columns.get(7).to_u8(),
            quality_modifier: single_row.columns.get(8).to_u8(),
            durability: single_row.columns.get(9).to_u16(),
            conditions_flag: single_row.columns.get(10).to_u16(),
        })
    }
}

