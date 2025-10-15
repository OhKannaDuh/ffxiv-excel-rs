/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CharaCardDesignPreset {
    pub row_id: u32,
    pub base_plate_id: u32,
    pub base_plate: RowRef<CharaCardBase>,
    pub top_border_id: u32,
    pub top_border: RowRef<CharaCardHeader>,
    pub bottom_border_id: u32,
    pub bottom_border: RowRef<CharaCardHeader>,
    pub backing_id: u32,
    pub backing: RowRef<CharaCardDecoration>,
    pub pattern_overlay_id: u32,
    pub pattern_overlay: RowRef<CharaCardDecoration>,
    pub portrait_frame_id: u32,
    pub portrait_frame: RowRef<CharaCardDecoration>,
    pub plate_frame_id: u32,
    pub plate_frame: RowRef<CharaCardDecoration>,
    pub accent_id: u32,
    pub accent: RowRef<CharaCardDecoration>,
    pub sort_key: u16,
    pub name: String,
}

impl Sheet for CharaCardDesignPreset {
    const SHEET_NAME: &'static str = "CharaCardDesignPreset";
}

impl FromExcelRow for CharaCardDesignPreset {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            base_plate_id: single_row.columns.get(0).to_u32(),
            base_plate: RowRef::<CharaCardBase>::from(single_row.columns.get(0).to_u32()),
            top_border_id: single_row.columns.get(1).to_u32(),
            top_border: RowRef::<CharaCardHeader>::from(single_row.columns.get(1).to_u32()),
            bottom_border_id: single_row.columns.get(2).to_u32(),
            bottom_border: RowRef::<CharaCardHeader>::from(single_row.columns.get(2).to_u32()),
            backing_id: single_row.columns.get(3).to_u32(),
            backing: RowRef::<CharaCardDecoration>::from(single_row.columns.get(3).to_u32()),
            pattern_overlay_id: single_row.columns.get(4).to_u32(),
            pattern_overlay: RowRef::<CharaCardDecoration>::from(single_row.columns.get(4).to_u32()),
            portrait_frame_id: single_row.columns.get(5).to_u32(),
            portrait_frame: RowRef::<CharaCardDecoration>::from(single_row.columns.get(5).to_u32()),
            plate_frame_id: single_row.columns.get(6).to_u32(),
            plate_frame: RowRef::<CharaCardDecoration>::from(single_row.columns.get(6).to_u32()),
            accent_id: single_row.columns.get(7).to_u32(),
            accent: RowRef::<CharaCardDecoration>::from(single_row.columns.get(7).to_u32()),
            sort_key: single_row.columns.get(8).to_u16(),
            name: single_row.columns.get(9).to_owned_string(),
        })
    }
}

