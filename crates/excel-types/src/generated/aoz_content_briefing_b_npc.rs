/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct AOZContentBriefingBNpc {
    pub row_id: u32,
    pub b_npc_name_id: u32,
    pub b_npc_name: RowRef<BNpcName>,
    pub target_small_id: u32,
    pub target_large_id: u32,
    pub hide_stats: bool,
    pub endurance: u8,
    pub fire: u8,
    pub ice: u8,
    pub wind: u8,
    pub earth: u8,
    pub thunder: u8,
    pub water: u8,
    pub slashing: u8,
    pub piercing: u8,
    pub blunt: u8,
    pub magic: u8,
    pub slow_vuln: bool,
    pub petrification_vuln: bool,
    pub paralysis_vuln: bool,
    pub interruption_vuln: bool,
    pub blind_vuln: bool,
    pub stun_vuln: bool,
    pub sleep_vuln: bool,
    pub bind_vuln: bool,
    pub heavy_vuln: bool,
    pub flat_or_death_vuln: bool,
}

impl Sheet for AOZContentBriefingBNpc {
    const SHEET_NAME: &'static str = "AOZContentBriefingBNpc";
}

impl FromExcelRow for AOZContentBriefingBNpc {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            b_npc_name_id: single_row.columns.get(0).to_u32(),
            b_npc_name: RowRef::<BNpcName>::from(single_row.columns.get(0).to_u32()),
            target_small_id: single_row.columns.get(1).to_u32(),
            target_large_id: single_row.columns.get(2).to_u32(),
            hide_stats: single_row.columns.get(3).to_bit(0),
            endurance: single_row.columns.get(4).to_u8(),
            fire: single_row.columns.get(5).to_u8(),
            ice: single_row.columns.get(6).to_u8(),
            wind: single_row.columns.get(7).to_u8(),
            earth: single_row.columns.get(8).to_u8(),
            thunder: single_row.columns.get(9).to_u8(),
            water: single_row.columns.get(10).to_u8(),
            slashing: single_row.columns.get(11).to_u8(),
            piercing: single_row.columns.get(12).to_u8(),
            blunt: single_row.columns.get(13).to_u8(),
            magic: single_row.columns.get(14).to_u8(),
            slow_vuln: single_row.columns.get(15).to_bit(1),
            petrification_vuln: single_row.columns.get(16).to_bit(2),
            paralysis_vuln: single_row.columns.get(17).to_bit(3),
            interruption_vuln: single_row.columns.get(18).to_bit(4),
            blind_vuln: single_row.columns.get(19).to_bit(5),
            stun_vuln: single_row.columns.get(20).to_bit(6),
            sleep_vuln: single_row.columns.get(21).to_bit(7),
            bind_vuln: single_row.columns.get(22).to_bit(0),
            heavy_vuln: single_row.columns.get(23).to_bit(1),
            flat_or_death_vuln: single_row.columns.get(24).to_bit(2),
        })
    }
}

