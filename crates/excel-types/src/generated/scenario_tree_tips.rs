/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ScenarioTreeTips {
    pub row_id: u32,
    pub tips1: u32,
    pub tips2_id: u32,
    pub tips2: RowRef<ScenarioTree>,
}

impl Sheet for ScenarioTreeTips {
    const SHEET_NAME: &'static str = "ScenarioTreeTips";
}

impl FromExcelRow for ScenarioTreeTips {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            tips1: single_row.columns.get(1).to_u32(),
            tips2_id: single_row.columns.get(3).to_u32(),
            tips2: RowRef::<ScenarioTree>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

