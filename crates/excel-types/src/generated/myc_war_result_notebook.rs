/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct MYCWarResultNotebook {
    pub row_id: u32,
    pub number: u8,
    pub link: u8,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub icon_id: u32,
    pub image_id: u32,
    pub rarity: u8,
    pub name_jp: String,
    pub name: String,
    pub description: String,
}

impl Sheet for MYCWarResultNotebook {
    const SHEET_NAME: &'static str = "MYCWarResultNotebook";
}

impl FromExcelRow for MYCWarResultNotebook {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            number: single_row.columns.get(0).to_u8(),
            link: single_row.columns.get(2).to_u8(),
            quest_id: single_row.columns.get(3).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(3).to_u32()),
            icon_id: single_row.columns.get(5).to_u32(),
            image_id: single_row.columns.get(6).to_u32(),
            rarity: single_row.columns.get(7).to_u8(),
            name_jp: single_row.columns.get(8).to_owned_string(),
            name: single_row.columns.get(9).to_owned_string(),
            description: single_row.columns.get(10).to_owned_string(),
        })
    }
}

