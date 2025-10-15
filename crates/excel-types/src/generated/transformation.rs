/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Transformation {
    pub row_id: u32,
    pub model_id: u32,
    pub model: RowRef<ModelChara>,
    pub b_npc_name_id: u32,
    pub b_npc_name: RowRef<BNpcName>,
    pub b_npc_customize_id: u32,
    pub b_npc_customize: RowRef<BNpcCustomize>,
    pub npc_equip_id: u32,
    pub npc_equip: RowRef<NpcEquip>,
    pub ex_hotbar_enable_config: bool,
    pub action_0_id: u32,
    pub action_0: RowRef<Action>,
    pub action_1_id: u32,
    pub action_1: RowRef<Action>,
    pub action_2_id: u32,
    pub action_2: RowRef<Action>,
    pub action_3_id: u32,
    pub action_3: RowRef<Action>,
    pub action_4_id: u32,
    pub action_4: RowRef<Action>,
    pub action_5_id: u32,
    pub action_5: RowRef<Action>,
    pub rp_parameter_id: u32,
    pub rp_parameter: RowRef<RPParameter>,
    pub remove_action_id: u32,
    pub remove_action: RowRef<Action>,
    pub speed: f32,
    pub scale: f32,
    pub is_pv_p: bool,
    pub is_event: bool,
    pub player_camera: bool,
    pub start_vfx_id: u32,
    pub start_vfx: RowRef<VFX>,
    pub end_vfx_id: u32,
    pub end_vfx: RowRef<VFX>,
    pub action_6_id: u32,
    pub action_6: RowRef<Action>,
    pub action_7_id: u32,
    pub action_7: RowRef<Action>,
}

impl Sheet for Transformation {
    const SHEET_NAME: &'static str = "Transformation";
}

impl FromExcelRow for Transformation {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            model_id: single_row.columns.get(1).to_u32(),
            model: RowRef::<ModelChara>::from(single_row.columns.get(1).to_u32()),
            b_npc_name_id: single_row.columns.get(2).to_u32(),
            b_npc_name: RowRef::<BNpcName>::from(single_row.columns.get(2).to_u32()),
            b_npc_customize_id: single_row.columns.get(3).to_u32(),
            b_npc_customize: RowRef::<BNpcCustomize>::from(single_row.columns.get(3).to_u32()),
            npc_equip_id: single_row.columns.get(4).to_u32(),
            npc_equip: RowRef::<NpcEquip>::from(single_row.columns.get(4).to_u32()),
            ex_hotbar_enable_config: single_row.columns.get(5).to_bool(),
            action_0_id: single_row.columns.get(6).to_u32(),
            action_0: RowRef::<Action>::from(single_row.columns.get(6).to_u32()),
            action_1_id: single_row.columns.get(8).to_u32(),
            action_1: RowRef::<Action>::from(single_row.columns.get(8).to_u32()),
            action_2_id: single_row.columns.get(10).to_u32(),
            action_2: RowRef::<Action>::from(single_row.columns.get(10).to_u32()),
            action_3_id: single_row.columns.get(12).to_u32(),
            action_3: RowRef::<Action>::from(single_row.columns.get(12).to_u32()),
            action_4_id: single_row.columns.get(14).to_u32(),
            action_4: RowRef::<Action>::from(single_row.columns.get(14).to_u32()),
            action_5_id: single_row.columns.get(16).to_u32(),
            action_5: RowRef::<Action>::from(single_row.columns.get(16).to_u32()),
            rp_parameter_id: single_row.columns.get(18).to_u32(),
            rp_parameter: RowRef::<RPParameter>::from(single_row.columns.get(18).to_u32()),
            remove_action_id: single_row.columns.get(19).to_u32(),
            remove_action: RowRef::<Action>::from(single_row.columns.get(19).to_u32()),
            speed: single_row.columns.get(24).to_f32(),
            scale: single_row.columns.get(25).to_f32(),
            is_pv_p: single_row.columns.get(26).to_bit(3),
            is_event: single_row.columns.get(27).to_bit(4),
            player_camera: single_row.columns.get(28).to_bit(5),
            start_vfx_id: single_row.columns.get(31).to_u32(),
            start_vfx: RowRef::<VFX>::from(single_row.columns.get(31).to_u32()),
            end_vfx_id: single_row.columns.get(32).to_u32(),
            end_vfx: RowRef::<VFX>::from(single_row.columns.get(32).to_u32()),
            action_6_id: single_row.columns.get(33).to_u32(),
            action_6: RowRef::<Action>::from(single_row.columns.get(33).to_u32()),
            action_7_id: single_row.columns.get(36).to_u32(),
            action_7: RowRef::<Action>::from(single_row.columns.get(36).to_u32()),
        })
    }
}

