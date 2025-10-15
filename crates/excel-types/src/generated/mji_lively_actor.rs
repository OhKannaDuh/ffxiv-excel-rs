/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJILivelyActor {
    pub row_id: u32,
    pub enpc_id: u32,
    pub enpc: RowRef<ENpcResident>,
    pub behavior_id: u32,
    pub behavior: RowRef<Behavior>,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub rot: f32,
}

impl Sheet for MJILivelyActor {
    const SHEET_NAME: &'static str = "MJILivelyActor";
}

impl FromExcelRow for MJILivelyActor {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            enpc_id: single_row.columns.get(0).to_u32(),
            enpc: RowRef::<ENpcResident>::from(single_row.columns.get(0).to_u32()),
            behavior_id: single_row.columns.get(1).to_u32(),
            behavior: RowRef::<Behavior>::from(single_row.columns.get(1).to_u32()),
            x: single_row.columns.get(2).to_f32(),
            y: single_row.columns.get(3).to_f32(),
            z: single_row.columns.get(4).to_f32(),
            rot: single_row.columns.get(5).to_f32(),
        })
    }
}

