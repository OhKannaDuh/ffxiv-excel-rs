/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Perform {
    pub row_id: u32,
    pub name: String,
    pub model_key: u64,
    pub animation_start_id: u32,
    pub animation_start: RowRef<ActionTimeline>,
    pub animation_end_id: u32,
    pub animation_end: RowRef<ActionTimeline>,
    pub animation_idle_id: u32,
    pub animation_idle: RowRef<ActionTimeline>,
    pub animation_play01_id: u32,
    pub animation_play01: RowRef<ActionTimeline>,
    pub animation_play02_id: u32,
    pub animation_play02: RowRef<ActionTimeline>,
    pub stop_animation_id: u32,
    pub stop_animation: RowRef<ActionTimeline>,
    pub instrument: String,
    pub order: i32,
    pub transient_id: u32,
    pub transient: RowRef<PerformTransient>,
}

impl Sheet for Perform {
    const SHEET_NAME: &'static str = "Perform";
}

impl FromExcelRow for Perform {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            model_key: single_row.columns.get(2).to_u64(),
            animation_start_id: single_row.columns.get(3).to_u32(),
            animation_start: RowRef::<ActionTimeline>::from(single_row.columns.get(3).to_u32()),
            animation_end_id: single_row.columns.get(4).to_u32(),
            animation_end: RowRef::<ActionTimeline>::from(single_row.columns.get(4).to_u32()),
            animation_idle_id: single_row.columns.get(5).to_u32(),
            animation_idle: RowRef::<ActionTimeline>::from(single_row.columns.get(5).to_u32()),
            animation_play01_id: single_row.columns.get(6).to_u32(),
            animation_play01: RowRef::<ActionTimeline>::from(single_row.columns.get(6).to_u32()),
            animation_play02_id: single_row.columns.get(7).to_u32(),
            animation_play02: RowRef::<ActionTimeline>::from(single_row.columns.get(7).to_u32()),
            stop_animation_id: single_row.columns.get(8).to_u32(),
            stop_animation: RowRef::<ActionTimeline>::from(single_row.columns.get(8).to_u32()),
            instrument: single_row.columns.get(9).to_owned_string(),
            order: single_row.columns.get(10).to_i32(),
            transient_id: single_row.columns.get(11).to_u32(),
            transient: RowRef::<PerformTransient>::from(single_row.columns.get(11).to_u32()),
        })
    }
}

