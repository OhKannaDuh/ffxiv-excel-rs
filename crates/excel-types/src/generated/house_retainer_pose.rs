/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HouseRetainerPose {
    pub row_id: u32,
    pub action_timeline_id: u32,
    pub action_timeline: RowRef<ActionTimeline>,
}

impl Sheet for HouseRetainerPose {
    const SHEET_NAME: &'static str = "HouseRetainerPose";
}

impl FromExcelRow for HouseRetainerPose {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            action_timeline_id: single_row.columns.get(0).to_u32(),
            action_timeline: RowRef::<ActionTimeline>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

