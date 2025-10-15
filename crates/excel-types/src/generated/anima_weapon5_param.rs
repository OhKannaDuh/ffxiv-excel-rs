/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AnimaWeapon5Param {
    pub row_id: u32,
    pub base_param_id: u32,
    pub base_param: RowRef<BaseParam>,
    pub name: String,
}

impl Sheet for AnimaWeapon5Param {
    const SHEET_NAME: &'static str = "AnimaWeapon5Param";
}

impl FromExcelRow for AnimaWeapon5Param {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            base_param_id: single_row.columns.get(0).to_u32(),
            base_param: RowRef::<BaseParam>::from(single_row.columns.get(0).to_u32()),
            name: single_row.columns.get(1).to_owned_string(),
        })
    }
}

