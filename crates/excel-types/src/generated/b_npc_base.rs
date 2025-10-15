/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BNpcBase {
    pub row_id: u32,
    pub behavior_id: u32,
    pub behavior: RowRef<Behavior>,
    pub battalion_id: u32,
    pub link_race_id: u32,
    pub rank: u8,
    pub scale: f32,
    pub model_chara_id: u32,
    pub model_chara: RowRef<ModelChara>,
    pub b_npc_customize_id: u32,
    pub b_npc_customize: RowRef<BNpcCustomize>,
    pub npc_equip_id: u32,
    pub npc_equip: RowRef<NpcEquip>,
    pub special: u16,
    pub se_pack: u8,
    pub array_event_handler_id: u32,
    pub array_event_handler: RowRef<ArrayEventHandler>,
    pub b_npc_parts_id: u32,
    pub b_npc_parts: RowRef<BNpcParts>,
    pub is_target_line: bool,
    pub is_display_level: bool,
}

impl Sheet for BNpcBase {
    const SHEET_NAME: &'static str = "BNpcBase";
}

impl FromExcelRow for BNpcBase {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            behavior_id: single_row.columns.get(0).to_u32(),
            behavior: RowRef::<Behavior>::from(single_row.columns.get(0).to_u32()),
            battalion_id: single_row.columns.get(1).to_u32(),
            link_race_id: single_row.columns.get(2).to_u32(),
            rank: single_row.columns.get(3).to_u8(),
            scale: single_row.columns.get(4).to_f32(),
            model_chara_id: single_row.columns.get(5).to_u32(),
            model_chara: RowRef::<ModelChara>::from(single_row.columns.get(5).to_u32()),
            b_npc_customize_id: single_row.columns.get(6).to_u32(),
            b_npc_customize: RowRef::<BNpcCustomize>::from(single_row.columns.get(6).to_u32()),
            npc_equip_id: single_row.columns.get(7).to_u32(),
            npc_equip: RowRef::<NpcEquip>::from(single_row.columns.get(7).to_u32()),
            special: single_row.columns.get(8).to_u16(),
            se_pack: single_row.columns.get(9).to_u8(),
            array_event_handler_id: single_row.columns.get(11).to_u32(),
            array_event_handler: RowRef::<ArrayEventHandler>::from(single_row.columns.get(11).to_u32()),
            b_npc_parts_id: single_row.columns.get(13).to_u32(),
            b_npc_parts: RowRef::<BNpcParts>::from(single_row.columns.get(13).to_u32()),
            is_target_line: single_row.columns.get(16).to_bit(2),
            is_display_level: single_row.columns.get(17).to_bit(3),
        })
    }
}

