/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ZoneSharedGroup {
    pub row_id: u32,
    pub lgb_shared_group: u32,
    pub quest_0_id: u32,
    pub quest_0: RowRef<Quest>,
    pub seq_0: u32,
    pub quest_1_id: u32,
    pub quest_1: RowRef<Quest>,
    pub seq_1: u32,
    pub quest_2_id: u32,
    pub quest_2: RowRef<Quest>,
    pub seq_2: u32,
    pub quest_3_id: u32,
    pub quest_3: RowRef<Quest>,
    pub seq_3: u32,
    pub quest_4_id: u32,
    pub quest_4: RowRef<Quest>,
    pub seq_4: u32,
    pub quest_5_id: u32,
    pub quest_5: RowRef<Quest>,
    pub seq_5: u32,
}

impl Sheet for ZoneSharedGroup {
    const SHEET_NAME: &'static str = "ZoneSharedGroup";
}

impl FromExcelRow for ZoneSharedGroup {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            lgb_shared_group: single_row.columns.get(0).to_u32(),
            quest_0_id: single_row.columns.get(2).to_u32(),
            quest_0: RowRef::<Quest>::from(single_row.columns.get(2).to_u32()),
            seq_0: single_row.columns.get(3).to_u32(),
            quest_1_id: single_row.columns.get(6).to_u32(),
            quest_1: RowRef::<Quest>::from(single_row.columns.get(6).to_u32()),
            seq_1: single_row.columns.get(7).to_u32(),
            quest_2_id: single_row.columns.get(10).to_u32(),
            quest_2: RowRef::<Quest>::from(single_row.columns.get(10).to_u32()),
            seq_2: single_row.columns.get(11).to_u32(),
            quest_3_id: single_row.columns.get(14).to_u32(),
            quest_3: RowRef::<Quest>::from(single_row.columns.get(14).to_u32()),
            seq_3: single_row.columns.get(15).to_u32(),
            quest_4_id: single_row.columns.get(18).to_u32(),
            quest_4: RowRef::<Quest>::from(single_row.columns.get(18).to_u32()),
            seq_4: single_row.columns.get(19).to_u32(),
            quest_5_id: single_row.columns.get(22).to_u32(),
            quest_5: RowRef::<Quest>::from(single_row.columns.get(22).to_u32()),
            seq_5: single_row.columns.get(23).to_u32(),
        })
    }
}

