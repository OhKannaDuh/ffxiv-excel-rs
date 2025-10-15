/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BannerFacial {
    pub row_id: u32,
    pub emote_id: u32,
    pub emote: RowRef<Emote>,
    pub unlock_condition_id: u32,
    pub unlock_condition: RowRef<BannerCondition>,
    pub sort_key: u16,
}

impl Sheet for BannerFacial {
    const SHEET_NAME: &'static str = "BannerFacial";
}

impl FromExcelRow for BannerFacial {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            emote_id: single_row.columns.get(0).to_u32(),
            emote: RowRef::<Emote>::from(single_row.columns.get(0).to_u32()),
            unlock_condition_id: single_row.columns.get(1).to_u32(),
            unlock_condition: RowRef::<BannerCondition>::from(single_row.columns.get(1).to_u32()),
            sort_key: single_row.columns.get(4).to_u16(),
        })
    }
}

