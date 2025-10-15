/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Glasses {
    pub row_id: u32,
    pub glasses_style_id: u32,
    pub glasses_style: RowRef<GlassesStyle>,
    pub icon_id: u32,
    pub singular: String,
    pub plural: String,
    pub description: String,
    pub name: String,
}

impl Sheet for Glasses {
    const SHEET_NAME: &'static str = "Glasses";
}

impl FromExcelRow for Glasses {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            glasses_style_id: single_row.columns.get(1).to_u32(),
            glasses_style: RowRef::<GlassesStyle>::from(single_row.columns.get(1).to_u32()),
            icon_id: single_row.columns.get(2).to_u32(),
            singular: single_row.columns.get(4).to_owned_string(),
            plural: single_row.columns.get(6).to_owned_string(),
            description: single_row.columns.get(12).to_owned_string(),
            name: single_row.columns.get(13).to_owned_string(),
        })
    }
}

