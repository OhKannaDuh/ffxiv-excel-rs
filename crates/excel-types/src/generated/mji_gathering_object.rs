/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MJIGatheringObject {
    pub row_id: u32,
    pub sgb_id: u32,
    pub sgb: RowRef<ExportedSG>,
    pub map_icon_id: u32,
    pub name_id: u32,
    pub name: RowRef<EObjName>,
}

impl Sheet for MJIGatheringObject {
    const SHEET_NAME: &'static str = "MJIGatheringObject";
}

impl FromExcelRow for MJIGatheringObject {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            sgb_id: single_row.columns.get(0).to_u32(),
            sgb: RowRef::<ExportedSG>::from(single_row.columns.get(0).to_u32()),
            map_icon_id: single_row.columns.get(1).to_u32(),
            name_id: single_row.columns.get(3).to_u32(),
            name: RowRef::<EObjName>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

