/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Materia {
    pub row_id: u32,
    pub base_param_id: u32,
    pub base_param: RowRef<BaseParam>,
}

impl Sheet for Materia {
    const SHEET_NAME: &'static str = "Materia";
}

impl FromExcelRow for Materia {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            base_param_id: single_row.columns.get(16).to_u32(),
            base_param: RowRef::<BaseParam>::from(single_row.columns.get(16).to_u32()),
        })
    }
}

