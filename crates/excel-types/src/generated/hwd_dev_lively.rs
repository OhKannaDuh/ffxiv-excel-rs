/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct HWDDevLively {
    pub row_id: u32,
    pub enpc_id: u32,
    pub enpc: RowRef<ENpcBase>,
}

impl Sheet for HWDDevLively {
    const SHEET_NAME: &'static str = "HWDDevLively";
}

impl FromExcelRow for HWDDevLively {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            enpc_id: single_row.columns.get(0).to_u32(),
            enpc: RowRef::<ENpcBase>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

