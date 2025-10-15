/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EurekaDungeonPortal {
    pub row_id: u32,
    pub level_id_id: u32,
    pub level_id: RowRef<Level>,
}

impl Sheet for EurekaDungeonPortal {
    const SHEET_NAME: &'static str = "EurekaDungeonPortal";
}

impl FromExcelRow for EurekaDungeonPortal {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            level_id_id: single_row.columns.get(0).to_u32(),
            level_id: RowRef::<Level>::from(single_row.columns.get(0).to_u32()),
        })
    }
}

