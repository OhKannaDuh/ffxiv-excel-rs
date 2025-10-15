/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BNpcParts {
    pub row_id: u32,
    pub b_npc_base_1_id: u32,
    pub b_npc_base_1: RowRef<BNpcBase>,
    pub part_slot_1: u16,
    pub x_1: bool,
    pub y_1: f32,
    pub z_1: f32,
    pub scale_1: i16,
    pub b_npc_base_2_id: u32,
    pub b_npc_base_2: RowRef<BNpcBase>,
    pub part_slot_2: u16,
    pub x_2: bool,
    pub y_2: f32,
    pub z_2: f32,
    pub scale_2: i16,
    pub b_npc_base_3_id: u32,
    pub b_npc_base_3: RowRef<BNpcBase>,
    pub part_slot_3: u16,
    pub x_3: bool,
    pub y_3: f32,
    pub z_3: f32,
    pub scale_3: f32,
    pub b_npc_base_4_id: u32,
    pub b_npc_base_4: RowRef<BNpcBase>,
    pub part_slot_4: u16,
    pub x_4: bool,
    pub y_4: f32,
    pub z_4: f32,
    pub scale_4: i16,
    pub b_npc_base_5_id: u32,
    pub b_npc_base_5: RowRef<BNpcBase>,
    pub part_slot_5: u16,
    pub x_5: bool,
    pub y_5: f32,
    pub z_5: f32,
    pub scale_5: i16,
}

impl Sheet for BNpcParts {
    const SHEET_NAME: &'static str = "BNpcParts";
}

impl FromExcelRow for BNpcParts {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            b_npc_base_1_id: single_row.columns.get(0).to_u32(),
            b_npc_base_1: RowRef::<BNpcBase>::from(single_row.columns.get(0).to_u32()),
            part_slot_1: single_row.columns.get(1).to_u16(),
            x_1: single_row.columns.get(6).to_bool(),
            y_1: single_row.columns.get(7).to_f32(),
            z_1: single_row.columns.get(8).to_f32(),
            scale_1: single_row.columns.get(10).to_i16(),
            b_npc_base_2_id: single_row.columns.get(11).to_u32(),
            b_npc_base_2: RowRef::<BNpcBase>::from(single_row.columns.get(11).to_u32()),
            part_slot_2: single_row.columns.get(12).to_u16(),
            x_2: single_row.columns.get(17).to_bool(),
            y_2: single_row.columns.get(18).to_f32(),
            z_2: single_row.columns.get(19).to_f32(),
            scale_2: single_row.columns.get(21).to_i16(),
            b_npc_base_3_id: single_row.columns.get(22).to_u32(),
            b_npc_base_3: RowRef::<BNpcBase>::from(single_row.columns.get(22).to_u32()),
            part_slot_3: single_row.columns.get(23).to_u16(),
            x_3: single_row.columns.get(28).to_bool(),
            y_3: single_row.columns.get(29).to_f32(),
            z_3: single_row.columns.get(30).to_f32(),
            scale_3: single_row.columns.get(31).to_f32(),
            b_npc_base_4_id: single_row.columns.get(33).to_u32(),
            b_npc_base_4: RowRef::<BNpcBase>::from(single_row.columns.get(33).to_u32()),
            part_slot_4: single_row.columns.get(34).to_u16(),
            x_4: single_row.columns.get(39).to_bool(),
            y_4: single_row.columns.get(40).to_f32(),
            z_4: single_row.columns.get(41).to_f32(),
            scale_4: single_row.columns.get(43).to_i16(),
            b_npc_base_5_id: single_row.columns.get(44).to_u32(),
            b_npc_base_5: RowRef::<BNpcBase>::from(single_row.columns.get(44).to_u32()),
            part_slot_5: single_row.columns.get(45).to_u16(),
            x_5: single_row.columns.get(50).to_bool(),
            y_5: single_row.columns.get(51).to_f32(),
            z_5: single_row.columns.get(52).to_f32(),
            scale_5: single_row.columns.get(54).to_i16(),
        })
    }
}

