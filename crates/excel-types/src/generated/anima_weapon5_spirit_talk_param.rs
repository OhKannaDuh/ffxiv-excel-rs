/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnimaWeapon5SpiritTalkParam {
    pub row_id: u32,
    pub prologue: String,
    pub epilogue: String,
}

impl Sheet for AnimaWeapon5SpiritTalkParam {
    const SHEET_NAME: &'static str = "AnimaWeapon5SpiritTalkParam";
}

impl FromExcelRow for AnimaWeapon5SpiritTalkParam {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            prologue: single_row.columns.get(0).to_owned_string(),
            epilogue: single_row.columns.get(1).to_owned_string(),
        })
    }
}

