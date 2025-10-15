/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GimmickJump {
    pub row_id: u32,
    pub fall_damage: u16,
    pub height: u16,
    pub loop_motion_id: u32,
    pub loop_motion: RowRef<ActionTimeline>,
    pub end_motion_id: u32,
    pub end_motion: RowRef<ActionTimeline>,
    pub start_client: bool,
}

impl Sheet for GimmickJump {
    const SHEET_NAME: &'static str = "GimmickJump";
}

impl FromExcelRow for GimmickJump {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            fall_damage: single_row.columns.get(0).to_u16(),
            height: single_row.columns.get(1).to_u16(),
            loop_motion_id: single_row.columns.get(2).to_u32(),
            loop_motion: RowRef::<ActionTimeline>::from(single_row.columns.get(2).to_u32()),
            end_motion_id: single_row.columns.get(3).to_u32(),
            end_motion: RowRef::<ActionTimeline>::from(single_row.columns.get(3).to_u32()),
            start_client: single_row.columns.get(4).to_bit(0),
        })
    }
}

