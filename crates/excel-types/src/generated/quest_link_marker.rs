/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct QuestLinkMarker {
    pub row_id: u32,
    pub source_map_id: u32,
    pub source_map: RowRef<Map>,
    pub level_id: u32,
    pub level: RowRef<Level>,
    pub target_map_id: u32,
    pub target_map: RowRef<Map>,
}

impl Sheet for QuestLinkMarker {
    const SHEET_NAME: &'static str = "QuestLinkMarker";
}

impl FromExcelRow for QuestLinkMarker {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            source_map_id: single_row.columns.get(0).to_u32(),
            source_map: RowRef::<Map>::from(single_row.columns.get(0).to_u32()),
            level_id: single_row.columns.get(1).to_u32(),
            level: RowRef::<Level>::from(single_row.columns.get(1).to_u32()),
            target_map_id: single_row.columns.get(2).to_u32(),
            target_map: RowRef::<Map>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

