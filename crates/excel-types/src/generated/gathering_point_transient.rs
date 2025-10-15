/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringPointTransient {
    pub row_id: u32,
    pub ephemeral_start_time: u16,
    pub ephemeral_end_time: u16,
    pub gathering_rare_pop_time_table_id: u32,
    pub gathering_rare_pop_time_table: RowRef<GatheringRarePopTimeTable>,
}

impl Sheet for GatheringPointTransient {
    const SHEET_NAME: &'static str = "GatheringPointTransient";
}

impl FromExcelRow for GatheringPointTransient {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            ephemeral_start_time: single_row.columns.get(0).to_u16(),
            ephemeral_end_time: single_row.columns.get(1).to_u16(),
            gathering_rare_pop_time_table_id: single_row.columns.get(2).to_u32(),
            gathering_rare_pop_time_table: RowRef::<GatheringRarePopTimeTable>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

