/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EquipRaceCategory {
    pub row_id: u32,
    pub hyur: bool,
    pub elezen: bool,
    pub lalafell: bool,
    pub miqote: bool,
    pub roegadyn: bool,
    pub au_ra: bool,
    pub male: bool,
    pub female: bool,
}

impl Sheet for EquipRaceCategory {
    const SHEET_NAME: &'static str = "EquipRaceCategory";
}

impl FromExcelRow for EquipRaceCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            hyur: single_row.columns.get(0).to_bool(),
            elezen: single_row.columns.get(1).to_bool(),
            lalafell: single_row.columns.get(2).to_bool(),
            miqote: single_row.columns.get(3).to_bool(),
            roegadyn: single_row.columns.get(4).to_bool(),
            au_ra: single_row.columns.get(5).to_bool(),
            male: single_row.columns.get(8).to_bit(0),
            female: single_row.columns.get(9).to_bit(1),
        })
    }
}

