/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GroupPoseFrame {
    pub row_id: u32,
    pub image_id: u32,
    pub grid_text: String,
    pub text: String,
}

impl Sheet for GroupPoseFrame {
    const SHEET_NAME: &'static str = "GroupPoseFrame";
}

impl FromExcelRow for GroupPoseFrame {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(1).to_u32(),
            grid_text: single_row.columns.get(2).to_owned_string(),
            text: single_row.columns.get(7).to_owned_string(),
        })
    }
}

