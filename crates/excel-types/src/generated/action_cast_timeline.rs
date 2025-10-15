/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ActionCastTimeline {
    pub row_id: u32,
    pub name_id: u32,
    pub name: RowRef<ActionTimeline>,
    pub vfx_id: u32,
    pub vfx: RowRef<VFX>,
}

impl Sheet for ActionCastTimeline {
    const SHEET_NAME: &'static str = "ActionCastTimeline";
}

impl FromExcelRow for ActionCastTimeline {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name_id: single_row.columns.get(0).to_u32(),
            name: RowRef::<ActionTimeline>::from(single_row.columns.get(0).to_u32()),
            vfx_id: single_row.columns.get(1).to_u32(),
            vfx: RowRef::<VFX>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

