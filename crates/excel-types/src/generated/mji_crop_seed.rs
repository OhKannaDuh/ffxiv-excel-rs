/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJICropSeed {
    pub row_id: u32,
    pub item_id: u32,
    pub item: RowRef<Item>,
    pub sgb_id: u32,
    pub sgb: RowRef<ExportedSG>,
    pub name_id: u32,
    pub name: RowRef<EObjName>,
}

impl Sheet for MJICropSeed {
    const SHEET_NAME: &'static str = "MJICropSeed";
}

impl FromExcelRow for MJICropSeed {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            item_id: single_row.columns.get(0).to_u32(),
            item: RowRef::<Item>::from(single_row.columns.get(0).to_u32()),
            sgb_id: single_row.columns.get(1).to_u32(),
            sgb: RowRef::<ExportedSG>::from(single_row.columns.get(1).to_u32()),
            name_id: single_row.columns.get(2).to_u32(),
            name: RowRef::<EObjName>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

