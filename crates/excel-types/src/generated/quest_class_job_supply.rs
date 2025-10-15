/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestClassJobSupply {
    pub row_id: u32,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
    pub e_npc_resident_id: u32,
    pub e_npc_resident: RowRef<ENpcResident>,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub amount_required: u8,
    pub item_hq: bool,
}

impl Sheet for QuestClassJobSupply {
    const SHEET_NAME: &'static str = "QuestClassJobSupply";
}

impl FromExcelRow for QuestClassJobSupply {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            class_job_category_id: single_row.columns.get(0).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(0).to_u32()),
            e_npc_resident_id: single_row.columns.get(2).to_u32(),
            e_npc_resident: RowRef::<ENpcResident>::from(single_row.columns.get(2).to_u32()),
            item_id: single_row.columns.get(3).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(3).to_u32()),
            amount_required: single_row.columns.get(4).to_u8(),
            item_hq: single_row.columns.get(5).to_bit(0),
        })
    }
}

