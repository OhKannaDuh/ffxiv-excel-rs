/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct SkyIsland2MissionDetail {
    pub row_id: u32,
    pub _type_id: u32,
    pub _type: RowRef<SkyIsland2MissionType>,
    pub range_id: u32,
    pub range: RowRef<SkyIsland2RangeType>,
    pub e_obj_id: u32,
    pub e_obj: RowRef<EObjName>,
    pub objective: String,
}

impl Sheet for SkyIsland2MissionDetail {
    const SHEET_NAME: &'static str = "SkyIsland2MissionDetail";
}

impl FromExcelRow for SkyIsland2MissionDetail {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            _type_id: single_row.columns.get(0).to_u32(),
            _type: RowRef::<SkyIsland2MissionType>::from(single_row.columns.get(0).to_u32()),
            range_id: single_row.columns.get(2).to_u32(),
            range: RowRef::<SkyIsland2RangeType>::from(single_row.columns.get(2).to_u32()),
            e_obj_id: single_row.columns.get(4).to_u32(),
            e_obj: RowRef::<EObjName>::from(single_row.columns.get(4).to_u32()),
            objective: single_row.columns.get(7).to_owned_string(),
        })
    }
}

