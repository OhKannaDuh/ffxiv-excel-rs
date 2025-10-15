/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIStockyardManagementTable {
    pub row_id: u32,
    pub material_id: u32,
    pub material: RowRef<MJIItemPouch>,
}

impl Sheet for MJIStockyardManagementTable {
    const SHEET_NAME: &'static str = "MJIStockyardManagementTable";
}

impl FromExcelRow for MJIStockyardManagementTable {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            material_id: single_row.columns.get(0).to_u32(),
            material: RowRef::<MJIItemPouch>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

