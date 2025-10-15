/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIGathering {
    pub row_id: u32,
    pub gathering_object_id: u32,
    pub gathering_object: RowRef<MJIGatheringObject>,
}

impl Sheet for MJIGathering {
    const SHEET_NAME: &'static str = "MJIGathering";
}

impl FromExcelRow for MJIGathering {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gathering_object_id: single_row.columns.get(0).to_u32(),
            gathering_object: RowRef::<MJIGatheringObject>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

