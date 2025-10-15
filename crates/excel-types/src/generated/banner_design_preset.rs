/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BannerDesignPreset {
    pub row_id: u32,
    pub background_id: u32,
    pub background: RowRef<BannerBg>,
    pub frame_id: u32,
    pub frame: RowRef<BannerFrame>,
    pub decoration_id: u32,
    pub decoration: RowRef<BannerDecoration>,
    pub sort_key: u16,
    pub name: String,
}

impl Sheet for BannerDesignPreset {
    const SHEET_NAME: &'static str = "BannerDesignPreset";
}

impl FromExcelRow for BannerDesignPreset {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            background_id: single_row.columns.get(0).to_u32(),
            background: RowRef::<BannerBg>::from(single_row.columns.get(0).to_u32()),
            frame_id: single_row.columns.get(1).to_u32(),
            frame: RowRef::<BannerFrame>::from(single_row.columns.get(1).to_u32()),
            decoration_id: single_row.columns.get(2).to_u32(),
            decoration: RowRef::<BannerDecoration>::from(single_row.columns.get(2).to_u32()),
            sort_key: single_row.columns.get(3).to_u16(),
            name: single_row.columns.get(4).to_owned_string(),
        })
    }
}

