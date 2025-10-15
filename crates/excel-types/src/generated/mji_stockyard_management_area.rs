/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIStockyardManagementArea {
    pub row_id: u32,
    pub rare_material_id: u32,
    pub rare_material: RowRef<MJIItemPouch>,
    pub area_id: u32,
    pub area: RowRef<MJIText>,
}

impl Sheet for MJIStockyardManagementArea {
    const SHEET_NAME: &'static str = "MJIStockyardManagementArea";
}

impl FromExcelRow for MJIStockyardManagementArea {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            rare_material_id: single_row.columns.get(0).to_u32(),
            rare_material: RowRef::<MJIItemPouch>::from(single_row.columns.get(0).to_u32()),
            area_id: single_row.columns.get(2).to_u32(),
            area: RowRef::<MJIText>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

