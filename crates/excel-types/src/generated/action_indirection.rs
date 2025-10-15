/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ActionIndirection {
    pub row_id: u32,
    pub name_id: u32,
    pub name: RowRef<Action>,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
    pub previous_combo_action_id: u32,
    pub previous_combo_action: RowRef<Action>,
}

impl Sheet for ActionIndirection {
    const SHEET_NAME: &'static str = "ActionIndirection";
}

impl FromExcelRow for ActionIndirection {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name_id: single_row.columns.get(0).to_u32(),
            name: RowRef::<Action>::from(single_row.columns.get(0).to_u32()),
            class_job_id: single_row.columns.get(1).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(1).to_u32()),
            previous_combo_action_id: single_row.columns.get(2).to_u32(),
            previous_combo_action: RowRef::<Action>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

