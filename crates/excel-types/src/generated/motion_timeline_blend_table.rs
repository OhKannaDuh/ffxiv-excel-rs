/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MotionTimelineBlendTable {
    pub row_id: u32,
    pub dest_blend_group: u8,
    pub src_blend_group: u8,
    pub blend_frame_pc: u8,
    pub blend_fram_type_a: u8,
    pub blend_fram_type_b: u8,
    pub blend_fram_type_c: u8,
}

impl Sheet for MotionTimelineBlendTable {
    const SHEET_NAME: &'static str = "MotionTimelineBlendTable";
}

impl FromExcelRow for MotionTimelineBlendTable {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            dest_blend_group: single_row.columns.get(0).to_u8(),
            src_blend_group: single_row.columns.get(1).to_u8(),
            blend_frame_pc: single_row.columns.get(2).to_u8(),
            blend_fram_type_a: single_row.columns.get(3).to_u8(),
            blend_fram_type_b: single_row.columns.get(4).to_u8(),
            blend_fram_type_c: single_row.columns.get(5).to_u8(),
        })
    }
}

