/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RPParameter {
    pub row_id: u32,
    pub b_npc_name_id: u32,
    pub b_npc_name: RowRef<BNpcName>,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
}

impl Sheet for RPParameter {
    const SHEET_NAME: &'static str = "RPParameter";
}

impl FromExcelRow for RPParameter {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            b_npc_name_id: single_row.columns.get(0).to_u32(),
            b_npc_name: RowRef::<BNpcName>::from(single_row.columns.get(0).to_u32()),
            class_job_id: single_row.columns.get(1).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

