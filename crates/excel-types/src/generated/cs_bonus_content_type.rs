/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CSBonusContentType {
    pub row_id: u32,
    pub content_type_id: u32,
    pub content_type: RowRef<ContentType>,
    pub image_id: u32,
    pub unlock_quest_id: u32,
    pub unlock_quest: RowRef<Quest>,
}

impl Sheet for CSBonusContentType {
    const SHEET_NAME: &'static str = "CSBonusContentType";
}

impl FromExcelRow for CSBonusContentType {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            content_type_id: single_row.columns.get(0).to_u32(),
            content_type: RowRef::<ContentType>::from(single_row.columns.get(0).to_u32()),
            image_id: single_row.columns.get(5).to_u32(),
            unlock_quest_id: single_row.columns.get(7).to_u32(),
            unlock_quest: RowRef::<Quest>::from(single_row.columns.get(7).to_u32()),
        })
    }
}

