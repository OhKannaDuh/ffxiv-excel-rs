/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct FieldMarker {
    pub row_id: u32,
    pub vfx_id: u32,
    pub vfx: RowRef<VFX>,
    pub ui_icon_id: u32,
    pub map_icon_id: u32,
    pub name: String,
}

impl Sheet for FieldMarker {
    const SHEET_NAME: &'static str = "FieldMarker";
}

impl FromExcelRow for FieldMarker {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            vfx_id: single_row.columns.get(0).to_u32(),
            vfx: RowRef::<VFX>::from(single_row.columns.get(0).to_u32()),
            ui_icon_id: single_row.columns.get(1).to_u32(),
            map_icon_id: single_row.columns.get(2).to_u32(),
            name: single_row.columns.get(3).to_owned_string(),
        })
    }
}

