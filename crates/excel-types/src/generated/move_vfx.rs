/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MoveVfx {
    pub row_id: u32,
    pub vfx_normal_id: u32,
    pub vfx_normal: RowRef<VFX>,
    pub vfx_walking_id: u32,
    pub vfx_walking: RowRef<VFX>,
}

impl Sheet for MoveVfx {
    const SHEET_NAME: &'static str = "MoveVfx";
}

impl FromExcelRow for MoveVfx {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            vfx_normal_id: single_row.columns.get(0).to_u32(),
            vfx_normal: RowRef::<VFX>::from(single_row.columns.get(0).to_u32()),
            vfx_walking_id: single_row.columns.get(1).to_u32(),
            vfx_walking: RowRef::<VFX>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

