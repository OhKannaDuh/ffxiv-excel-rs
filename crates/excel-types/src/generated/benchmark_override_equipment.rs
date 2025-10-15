/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BenchmarkOverrideEquipment {
    pub row_id: u32,
    pub model_main_hand: u64,
    pub dye_main_hand_id: u32,
    pub dye_main_hand: RowRef<Stain>,
    pub model_off_hand: u64,
    pub dye_off_hand_id: u32,
    pub dye_off_hand: RowRef<Stain>,
    pub model_head: u32,
    pub dye_head_id: u32,
    pub dye_head: RowRef<Stain>,
    pub model_body: u32,
    pub dye_body_id: u32,
    pub dye_body: RowRef<Stain>,
    pub model_hands: u32,
    pub dye_hands_id: u32,
    pub dye_hands: RowRef<Stain>,
    pub model_legs: u32,
    pub dye_legs_id: u32,
    pub dye_legs: RowRef<Stain>,
    pub model_feet: u32,
    pub dye_feet_id: u32,
    pub dye_feet: RowRef<Stain>,
    pub model_ears: u32,
    pub dye_ears_id: u32,
    pub dye_ears: RowRef<Stain>,
    pub model_neck: u32,
    pub dye_neck_id: u32,
    pub dye_neck: RowRef<Stain>,
    pub model_wrists: u32,
    pub dye_wrists_id: u32,
    pub dye_wrists: RowRef<Stain>,
    pub model_left_ring: u32,
    pub dye_left_ring_id: u32,
    pub dye_left_ring: RowRef<Stain>,
    pub model_right_ring: u32,
    pub dye_right_ring_id: u32,
    pub dye_right_ring: RowRef<Stain>,
}

impl Sheet for BenchmarkOverrideEquipment {
    const SHEET_NAME: &'static str = "BenchmarkOverrideEquipment";
}

impl FromExcelRow for BenchmarkOverrideEquipment {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            model_main_hand: single_row.columns.get(4).to_u64(),
            dye_main_hand_id: single_row.columns.get(5).to_u32(),
            dye_main_hand: RowRef::<Stain>::from(single_row.columns.get(5).to_u32()),
            model_off_hand: single_row.columns.get(6).to_u64(),
            dye_off_hand_id: single_row.columns.get(7).to_u32(),
            dye_off_hand: RowRef::<Stain>::from(single_row.columns.get(7).to_u32()),
            model_head: single_row.columns.get(10).to_u32(),
            dye_head_id: single_row.columns.get(11).to_u32(),
            dye_head: RowRef::<Stain>::from(single_row.columns.get(11).to_u32()),
            model_body: single_row.columns.get(12).to_u32(),
            dye_body_id: single_row.columns.get(13).to_u32(),
            dye_body: RowRef::<Stain>::from(single_row.columns.get(13).to_u32()),
            model_hands: single_row.columns.get(14).to_u32(),
            dye_hands_id: single_row.columns.get(15).to_u32(),
            dye_hands: RowRef::<Stain>::from(single_row.columns.get(15).to_u32()),
            model_legs: single_row.columns.get(16).to_u32(),
            dye_legs_id: single_row.columns.get(17).to_u32(),
            dye_legs: RowRef::<Stain>::from(single_row.columns.get(17).to_u32()),
            model_feet: single_row.columns.get(18).to_u32(),
            dye_feet_id: single_row.columns.get(19).to_u32(),
            dye_feet: RowRef::<Stain>::from(single_row.columns.get(19).to_u32()),
            model_ears: single_row.columns.get(20).to_u32(),
            dye_ears_id: single_row.columns.get(21).to_u32(),
            dye_ears: RowRef::<Stain>::from(single_row.columns.get(21).to_u32()),
            model_neck: single_row.columns.get(22).to_u32(),
            dye_neck_id: single_row.columns.get(23).to_u32(),
            dye_neck: RowRef::<Stain>::from(single_row.columns.get(23).to_u32()),
            model_wrists: single_row.columns.get(24).to_u32(),
            dye_wrists_id: single_row.columns.get(25).to_u32(),
            dye_wrists: RowRef::<Stain>::from(single_row.columns.get(25).to_u32()),
            model_left_ring: single_row.columns.get(26).to_u32(),
            dye_left_ring_id: single_row.columns.get(27).to_u32(),
            dye_left_ring: RowRef::<Stain>::from(single_row.columns.get(27).to_u32()),
            model_right_ring: single_row.columns.get(28).to_u32(),
            dye_right_ring_id: single_row.columns.get(29).to_u32(),
            dye_right_ring: RowRef::<Stain>::from(single_row.columns.get(29).to_u32()),
        })
    }
}

