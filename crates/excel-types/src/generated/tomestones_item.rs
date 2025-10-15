/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct TomestonesItem {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub tomestones_id: u32,
    pub tomestones: RowRef<Tomestones>,
}

impl Sheet for TomestonesItem {
    const SHEET_NAME: &'static str = "TomestonesItem";
}

impl FromExcelRow for TomestonesItem {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            tomestones_id: single_row.columns.get(2).to_u32(),
            tomestones: RowRef::<Tomestones>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

