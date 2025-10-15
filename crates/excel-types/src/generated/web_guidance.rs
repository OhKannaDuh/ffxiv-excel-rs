/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct WebGuidance {
    pub row_id: u32,
    pub image_id: u32,
    pub url_id: u32,
    pub url: RowRef<WebURL>,
    pub name: String,
    pub description: String,
}

impl Sheet for WebGuidance {
    const SHEET_NAME: &'static str = "WebGuidance";
}

impl FromExcelRow for WebGuidance {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            image_id: single_row.columns.get(0).to_u32(),
            url_id: single_row.columns.get(1).to_u32(),
            url: RowRef::<WebURL>::from(single_row.columns.get(1).to_u32()),
            name: single_row.columns.get(2).to_owned_string(),
            description: single_row.columns.get(4).to_owned_string(),
        })
    }
}

