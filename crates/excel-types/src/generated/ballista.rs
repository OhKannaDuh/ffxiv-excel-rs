/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Ballista {
    pub row_id: u32,
    pub bnpc_id: u32,
    pub bnpc: RowRef<BNpcBase>,
    pub near: i8,
    pub far: i8,
    pub angle: u16,
    pub bullet: u8,
    pub action_0_id: u32,
    pub action_0: RowRef<Action>,
    pub action_1_id: u32,
    pub action_1: RowRef<Action>,
    pub action_2_id: u32,
    pub action_2: RowRef<Action>,
    pub action_3_id: u32,
    pub action_3: RowRef<Action>,
}

impl Sheet for Ballista {
    const SHEET_NAME: &'static str = "Ballista";
}

impl FromExcelRow for Ballista {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            bnpc_id: single_row.columns.get(0).to_u32(),
            bnpc: RowRef::<BNpcBase>::from(single_row.columns.get(0).to_u32()),
            near: single_row.columns.get(1).to_i8(),
            far: single_row.columns.get(2).to_i8(),
            angle: single_row.columns.get(3).to_u16(),
            bullet: single_row.columns.get(4).to_u8(),
            action_0_id: single_row.columns.get(7).to_u32(),
            action_0: RowRef::<Action>::from(single_row.columns.get(7).to_u32()),
            action_1_id: single_row.columns.get(8).to_u32(),
            action_1: RowRef::<Action>::from(single_row.columns.get(8).to_u32()),
            action_2_id: single_row.columns.get(9).to_u32(),
            action_2: RowRef::<Action>::from(single_row.columns.get(9).to_u32()),
            action_3_id: single_row.columns.get(10).to_u32(),
            action_3: RowRef::<Action>::from(single_row.columns.get(10).to_u32()),
        })
    }
}

