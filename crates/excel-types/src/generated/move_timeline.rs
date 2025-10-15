/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MoveTimeline {
    pub row_id: u32,
    pub idle_id: u32,
    pub idle: RowRef<ActionTimeline>,
    pub move_forward_id: u32,
    pub move_forward: RowRef<ActionTimeline>,
    pub move_back_id: u32,
    pub move_back: RowRef<ActionTimeline>,
    pub move_left_id: u32,
    pub move_left: RowRef<ActionTimeline>,
    pub move_right_id: u32,
    pub move_right: RowRef<ActionTimeline>,
    pub move_up_id: u32,
    pub move_up: RowRef<ActionTimeline>,
    pub move_down_id: u32,
    pub move_down: RowRef<ActionTimeline>,
    pub move_turn_left_id: u32,
    pub move_turn_left: RowRef<ActionTimeline>,
    pub move_turn_right_id: u32,
    pub move_turn_right: RowRef<ActionTimeline>,
    pub extra_id: u32,
    pub extra: RowRef<ActionTimeline>,
}

impl Sheet for MoveTimeline {
    const SHEET_NAME: &'static str = "MoveTimeline";
}

impl FromExcelRow for MoveTimeline {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            idle_id: single_row.columns.get(0).to_u32(),
            idle: RowRef::<ActionTimeline>::from(single_row.columns.get(0).to_u32()),
            move_forward_id: single_row.columns.get(1).to_u32(),
            move_forward: RowRef::<ActionTimeline>::from(single_row.columns.get(1).to_u32()),
            move_back_id: single_row.columns.get(2).to_u32(),
            move_back: RowRef::<ActionTimeline>::from(single_row.columns.get(2).to_u32()),
            move_left_id: single_row.columns.get(3).to_u32(),
            move_left: RowRef::<ActionTimeline>::from(single_row.columns.get(3).to_u32()),
            move_right_id: single_row.columns.get(4).to_u32(),
            move_right: RowRef::<ActionTimeline>::from(single_row.columns.get(4).to_u32()),
            move_up_id: single_row.columns.get(5).to_u32(),
            move_up: RowRef::<ActionTimeline>::from(single_row.columns.get(5).to_u32()),
            move_down_id: single_row.columns.get(6).to_u32(),
            move_down: RowRef::<ActionTimeline>::from(single_row.columns.get(6).to_u32()),
            move_turn_left_id: single_row.columns.get(7).to_u32(),
            move_turn_left: RowRef::<ActionTimeline>::from(single_row.columns.get(7).to_u32()),
            move_turn_right_id: single_row.columns.get(8).to_u32(),
            move_turn_right: RowRef::<ActionTimeline>::from(single_row.columns.get(8).to_u32()),
            extra_id: single_row.columns.get(9).to_u32(),
            extra: RowRef::<ActionTimeline>::from(single_row.columns.get(9).to_u32()),
        })
    }
}

