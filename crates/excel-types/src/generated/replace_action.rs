/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ReplaceAction {
    pub row_id: u32,
    pub action_id: u32,
    pub action: RowRef<Action>,
    pub type1: i8,
    pub param1_id: u32,
    pub replace_action1_id: u32,
    pub replace_action1: RowRef<Action>,
    pub type2: i8,
    pub param2_id: u32,
    pub replace_action2_id: u32,
    pub replace_action2: RowRef<Action>,
    pub type3: i8,
    pub param3_id: u32,
    pub replace_action3_id: u32,
    pub replace_action3: RowRef<Action>,
    pub replace_settable: i8,
}

impl Sheet for ReplaceAction {
    const SHEET_NAME: &'static str = "ReplaceAction";
}

impl FromExcelRow for ReplaceAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            action_id: single_row.columns.get(0).to_u32(),
            action: RowRef::<Action>::from(single_row.columns.get(0).to_u32()),
            type1: single_row.columns.get(1).to_i8(),
            param1_id: single_row.columns.get(2).to_u32(),
            replace_action1_id: single_row.columns.get(3).to_u32(),
            replace_action1: RowRef::<Action>::from(single_row.columns.get(3).to_u32()),
            type2: single_row.columns.get(4).to_i8(),
            param2_id: single_row.columns.get(5).to_u32(),
            replace_action2_id: single_row.columns.get(6).to_u32(),
            replace_action2: RowRef::<Action>::from(single_row.columns.get(6).to_u32()),
            type3: single_row.columns.get(7).to_i8(),
            param3_id: single_row.columns.get(8).to_u32(),
            replace_action3_id: single_row.columns.get(9).to_u32(),
            replace_action3: RowRef::<Action>::from(single_row.columns.get(9).to_u32()),
            replace_settable: single_row.columns.get(10).to_i8(),
        })
    }
}

