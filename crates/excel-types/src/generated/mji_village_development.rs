/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIVillageDevelopment {
    pub row_id: u32,
    pub enpc_id: u32,
    pub enpc: RowRef<ENpcResident>,
    pub behavior_0_id: u32,
    pub behavior_0: RowRef<Behavior>,
    pub behavior_1_id: u32,
    pub behavior_1: RowRef<Behavior>,
}

impl Sheet for MJIVillageDevelopment {
    const SHEET_NAME: &'static str = "MJIVillageDevelopment";
}

impl FromExcelRow for MJIVillageDevelopment {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            enpc_id: single_row.columns.get(0).to_u32(),
            enpc: RowRef::<ENpcResident>::from(single_row.columns.get(0).to_u32()),
            behavior_0_id: single_row.columns.get(9).to_u32(),
            behavior_0: RowRef::<Behavior>::from(single_row.columns.get(9).to_u32()),
            behavior_1_id: single_row.columns.get(11).to_u32(),
            behavior_1: RowRef::<Behavior>::from(single_row.columns.get(11).to_u32()),
        })
    }
}

