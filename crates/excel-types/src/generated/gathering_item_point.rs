/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringItemPoint {
    pub row_id: u32,
    pub gathering_point_id: u32,
    pub gathering_point: RowRef<GatheringPoint>,
}

impl Sheet for GatheringItemPoint {
    const SHEET_NAME: &'static str = "GatheringItemPoint";
}

impl FromExcelRow for GatheringItemPoint {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gathering_point_id: single_row.columns.get(0).to_u32(),
            gathering_point: RowRef::<GatheringPoint>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

