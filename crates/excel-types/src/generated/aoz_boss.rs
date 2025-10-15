/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AOZBoss {
    pub row_id: u32,
    pub boss_id: u32,
    pub boss: RowRef<AOZContentBriefingBNpc>,
    pub position: u16,
}

impl Sheet for AOZBoss {
    const SHEET_NAME: &'static str = "AOZBoss";
}

impl FromExcelRow for AOZBoss {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            boss_id: single_row.columns.get(0).to_u32(),
            boss: RowRef::<AOZContentBriefingBNpc>::from(single_row.columns.get(0).to_u32()),
            position: single_row.columns.get(1).to_u16(),
        })
    }
}

