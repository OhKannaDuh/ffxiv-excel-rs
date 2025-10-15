/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct StatusLoopVFX {
    pub row_id: u32,
    pub friendly_vfx_id: u32,
    pub friendly_vfx: RowRef<VFX>,
    pub stack_vfx1_trigger: u8,
    pub stack_vfx1_id: u32,
    pub stack_vfx1: RowRef<VFX>,
    pub stack_vfx2_trigger: u8,
    pub stack_vfx2_id: u32,
    pub stack_vfx2: RowRef<VFX>,
    pub hostile_vfx_id: u32,
    pub hostile_vfx: RowRef<VFX>,
}

impl Sheet for StatusLoopVFX {
    const SHEET_NAME: &'static str = "StatusLoopVFX";
}

impl FromExcelRow for StatusLoopVFX {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            friendly_vfx_id: single_row.columns.get(0).to_u32(),
            friendly_vfx: RowRef::<VFX>::from(single_row.columns.get(0).to_u32()),
            stack_vfx1_trigger: single_row.columns.get(1).to_u8(),
            stack_vfx1_id: single_row.columns.get(2).to_u32(),
            stack_vfx1: RowRef::<VFX>::from(single_row.columns.get(2).to_u32()),
            stack_vfx2_trigger: single_row.columns.get(3).to_u8(),
            stack_vfx2_id: single_row.columns.get(4).to_u32(),
            stack_vfx2: RowRef::<VFX>::from(single_row.columns.get(4).to_u32()),
            hostile_vfx_id: single_row.columns.get(5).to_u32(),
            hostile_vfx: RowRef::<VFX>::from(single_row.columns.get(5).to_u32()),
        })
    }
}

