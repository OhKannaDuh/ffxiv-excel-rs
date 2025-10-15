/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ExportedGatheringPoint {
    pub row_id: u32,
    pub x: f32,
    pub y: f32,
    pub gathering_type_id: u32,
    pub gathering_type: RowRef<GatheringType>,
    pub gathering_point_type: u8,
    pub radius: u16,
}

impl Sheet for ExportedGatheringPoint {
    const SHEET_NAME: &'static str = "ExportedGatheringPoint";
}

impl FromExcelRow for ExportedGatheringPoint {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            x: single_row.columns.get(0).to_f32(),
            y: single_row.columns.get(1).to_f32(),
            gathering_type_id: single_row.columns.get(2).to_u32(),
            gathering_type: RowRef::<GatheringType>::from(single_row.columns.get(2).to_u32()),
            gathering_point_type: single_row.columns.get(3).to_u8(),
            radius: single_row.columns.get(4).to_u16(),
        })
    }
}

