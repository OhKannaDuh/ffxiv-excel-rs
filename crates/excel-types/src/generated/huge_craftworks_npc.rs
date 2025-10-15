/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HugeCraftworksNpc {
    pub row_id: u32,
    pub e_npc_resident_id: u32,
    pub e_npc_resident: RowRef<ENpcResident>,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
    pub transient: u8,
}

impl Sheet for HugeCraftworksNpc {
    const SHEET_NAME: &'static str = "HugeCraftworksNpc";
}

impl FromExcelRow for HugeCraftworksNpc {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            e_npc_resident_id: single_row.columns.get(0).to_u32(),
            e_npc_resident: RowRef::<ENpcResident>::from(single_row.columns.get(0).to_u32()),
            class_job_category_id: single_row.columns.get(1).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(1).to_u32()),
            transient: single_row.columns.get(86).to_u8(),
        })
    }
}

