/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct VVDNotebookContents {
    pub row_id: u32,
    pub icon_id: u32,
    pub image_id: u32,
    pub name: String,
    pub description: String,
}

impl Sheet for VVDNotebookContents {
    const SHEET_NAME: &'static str = "VVDNotebookContents";
}

impl FromExcelRow for VVDNotebookContents {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            icon_id: single_row.columns.get(0).to_u32(),
            image_id: single_row.columns.get(1).to_u32(),
            name: single_row.columns.get(2).to_owned_string(),
            description: single_row.columns.get(3).to_owned_string(),
        })
    }
}

