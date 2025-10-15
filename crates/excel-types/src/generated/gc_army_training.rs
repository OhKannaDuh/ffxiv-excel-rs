/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GcArmyTraining {
    pub row_id: u32,
    pub physical_bonus: i8,
    pub mental_bonus: i8,
    pub tactical_bonus: i8,
    pub experience: u32,
    pub name: String,
    pub description: String,
}

impl Sheet for GcArmyTraining {
    const SHEET_NAME: &'static str = "GcArmyTraining";
}

impl FromExcelRow for GcArmyTraining {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            physical_bonus: single_row.columns.get(0).to_i8(),
            mental_bonus: single_row.columns.get(1).to_i8(),
            tactical_bonus: single_row.columns.get(2).to_i8(),
            experience: single_row.columns.get(3).to_u32(),
            name: single_row.columns.get(4).to_owned_string(),
            description: single_row.columns.get(5).to_owned_string(),
        })
    }
}

