/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct DeepDungeonBan {
    pub row_id: u32,
    pub screen_image_id: u32,
    pub screen_image: RowRef<ScreenImage>,
    pub log_message_id: u32,
    pub log_message: RowRef<LogMessage>,
    pub name_id: u32,
    pub name: RowRef<DeepDungeonFloorEffectUI>,
}

impl Sheet for DeepDungeonBan {
    const SHEET_NAME: &'static str = "DeepDungeonBan";
}

impl FromExcelRow for DeepDungeonBan {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            screen_image_id: single_row.columns.get(0).to_u32(),
            screen_image: RowRef::<ScreenImage>::from(single_row.columns.get(0).to_u32()),
            log_message_id: single_row.columns.get(1).to_u32(),
            log_message: RowRef::<LogMessage>::from(single_row.columns.get(1).to_u32()),
            name_id: single_row.columns.get(2).to_u32(),
            name: RowRef::<DeepDungeonFloorEffectUI>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

