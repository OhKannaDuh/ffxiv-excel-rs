/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringPointBase {
    pub row_id: u32,
    pub gathering_type_id: u32,
    pub gathering_type: RowRef<GatheringType>,
    pub gathering_level: u8,
}

impl Sheet for GatheringPointBase {
    const SHEET_NAME: &'static str = "GatheringPointBase";
}

impl FromExcelRow for GatheringPointBase {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            gathering_type_id: single_row.columns.get(0).to_u32(),
            gathering_type: RowRef::<GatheringType>::from(single_row.columns.get(0).to_u32()),
            gathering_level: single_row.columns.get(1).to_u8(),
        })
    }
}

