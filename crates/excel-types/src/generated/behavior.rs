/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Behavior {
    pub row_id: u32,
    pub condition_0_target: u8,
    pub condition_0_type: u8,
    pub balloon_id: u32,
    pub balloon: RowRef<Balloon>,
    pub condition_1_target: u8,
    pub condition_1_type: u8,
    pub content_argument_0: u32,
    pub content_argument_1: u8,
}

impl Sheet for Behavior {
    const SHEET_NAME: &'static str = "Behavior";
}

impl FromExcelRow for Behavior {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            condition_0_target: single_row.columns.get(2).to_u8(),
            condition_0_type: single_row.columns.get(3).to_u8(),
            balloon_id: single_row.columns.get(8).to_u32(),
            balloon: RowRef::<Balloon>::from(single_row.columns.get(8).to_u32()),
            condition_1_target: single_row.columns.get(9).to_u8(),
            condition_1_type: single_row.columns.get(10).to_u8(),
            content_argument_0: single_row.columns.get(11).to_u32(),
            content_argument_1: single_row.columns.get(12).to_u8(),
        })
    }
}

