/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct IKDRoute {
    pub row_id: u32,
    pub image_id: u32,
    pub content_finder_condition_id: u32,
    pub content_finder_condition: RowRef<ContentFinderCondition>,
    pub name: String,
}

impl Sheet for IKDRoute {
    const SHEET_NAME: &'static str = "IKDRoute";
}

impl FromExcelRow for IKDRoute {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(6).to_u32(),
            content_finder_condition_id: single_row.columns.get(9).to_u32(),
            content_finder_condition: RowRef::<ContentFinderCondition>::from(single_row.columns.get(9).to_u32()),
            name: single_row.columns.get(11).to_owned_string(),
        })
    }
}

