/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDAnnounce {
    pub row_id: u32,
    pub name: String,
    pub enpc_id: u32,
    pub enpc: RowRef<ENpcResident>,
}

impl Sheet for HWDAnnounce {
    const SHEET_NAME: &'static str = "HWDAnnounce";
}

impl FromExcelRow for HWDAnnounce {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            enpc_id: single_row.columns.get(1).to_u32(),
            enpc: RowRef::<ENpcResident>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

