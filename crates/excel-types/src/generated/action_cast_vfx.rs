/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ActionCastVFX {
    pub row_id: u32,
    pub vfx_id: u32,
    pub vfx: RowRef<VFX>,
}

impl Sheet for ActionCastVFX {
    const SHEET_NAME: &'static str = "ActionCastVFX";
}

impl FromExcelRow for ActionCastVFX {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            vfx_id: single_row.columns.get(0).to_u32(),
            vfx: RowRef::<VFX>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

