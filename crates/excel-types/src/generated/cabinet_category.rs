/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CabinetCategory {
    pub row_id: u32,
    pub menu_order: u8,
    pub hide_order: u8,
    pub icon_id: u32,
    pub category_id: u32,
    pub category: RowRef<Addon>,
}

impl Sheet for CabinetCategory {
    const SHEET_NAME: &'static str = "CabinetCategory";
}

impl FromExcelRow for CabinetCategory {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            menu_order: single_row.columns.get(0).to_u8(),
            hide_order: single_row.columns.get(1).to_u8(),
            icon_id: single_row.columns.get(2).to_u32(),
            category_id: single_row.columns.get(3).to_u32(),
            category: RowRef::<Addon>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

