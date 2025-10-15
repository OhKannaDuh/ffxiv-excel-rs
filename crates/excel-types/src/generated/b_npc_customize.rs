/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BNpcCustomize {
    pub row_id: u32,
    pub race_id: u32,
    pub race: RowRef<Race>,
    pub gender: u8,
    pub body_type: u8,
    pub height: u8,
    pub tribe_id: u32,
    pub tribe: RowRef<Tribe>,
    pub face: u8,
    pub hair_style: u8,
    pub hair_highlight: u8,
    pub skin_color: u8,
    pub eye_heterochromia: u8,
    pub hair_color: u8,
    pub hair_highlight_color: u8,
    pub facial_feature: u8,
    pub facial_feature_color: u8,
    pub eyebrows: u8,
    pub eye_color: u8,
    pub eye_shape: u8,
    pub nose: u8,
    pub jaw: u8,
    pub mouth: u8,
    pub lip_color: u8,
    pub bust_or_tone1: u8,
    pub extra_feature1: u8,
    pub extra_feature2_or_bust: u8,
    pub face_paint: u8,
    pub face_paint_color: u8,
}

impl Sheet for BNpcCustomize {
    const SHEET_NAME: &'static str = "BNpcCustomize";
}

impl FromExcelRow for BNpcCustomize {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            race_id: single_row.columns.get(0).to_u32(),
            race: RowRef::<Race>::from(single_row.columns.get(0).to_u32()),
            gender: single_row.columns.get(1).to_u8(),
            body_type: single_row.columns.get(2).to_u8(),
            height: single_row.columns.get(3).to_u8(),
            tribe_id: single_row.columns.get(4).to_u32(),
            tribe: RowRef::<Tribe>::from(single_row.columns.get(4).to_u32()),
            face: single_row.columns.get(5).to_u8(),
            hair_style: single_row.columns.get(6).to_u8(),
            hair_highlight: single_row.columns.get(7).to_u8(),
            skin_color: single_row.columns.get(8).to_u8(),
            eye_heterochromia: single_row.columns.get(9).to_u8(),
            hair_color: single_row.columns.get(10).to_u8(),
            hair_highlight_color: single_row.columns.get(11).to_u8(),
            facial_feature: single_row.columns.get(12).to_u8(),
            facial_feature_color: single_row.columns.get(13).to_u8(),
            eyebrows: single_row.columns.get(14).to_u8(),
            eye_color: single_row.columns.get(15).to_u8(),
            eye_shape: single_row.columns.get(16).to_u8(),
            nose: single_row.columns.get(17).to_u8(),
            jaw: single_row.columns.get(18).to_u8(),
            mouth: single_row.columns.get(19).to_u8(),
            lip_color: single_row.columns.get(20).to_u8(),
            bust_or_tone1: single_row.columns.get(21).to_u8(),
            extra_feature1: single_row.columns.get(22).to_u8(),
            extra_feature2_or_bust: single_row.columns.get(23).to_u8(),
            face_paint: single_row.columns.get(24).to_u8(),
            face_paint_color: single_row.columns.get(25).to_u8(),
        })
    }
}

