/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FCActivity {
    pub row_id: u32,
    pub text: String,
    pub self_kind: u8,
    pub target_kind: u8,
    pub num_param: u8,
    pub fc_activity_category_id: u32,
    pub fc_activity_category: RowRef<FCActivityCategory>,
    pub icon_type: i8,
}

impl Sheet for FCActivity {
    const SHEET_NAME: &'static str = "FCActivity";
}

impl FromExcelRow for FCActivity {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            text: single_row.columns.get(0).to_owned_string(),
            self_kind: single_row.columns.get(1).to_u8(),
            target_kind: single_row.columns.get(2).to_u8(),
            num_param: single_row.columns.get(3).to_u8(),
            fc_activity_category_id: single_row.columns.get(4).to_u32(),
            fc_activity_category: RowRef::<FCActivityCategory>::from(single_row.columns.get(4).to_u32()),
            icon_type: single_row.columns.get(5).to_i8(),
        })
    }
}

