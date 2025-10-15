/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct CraftAction {
    pub row_id: u32,
    pub name: String,
    pub description: String,
    pub animation_start_id: u32,
    pub animation_start: RowRef<ActionTimeline>,
    pub animation_end_id: u32,
    pub animation_end: RowRef<ActionTimeline>,
    pub icon_id: u32,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
    pub class_job_level: u8,
    pub quest_requirement_id: u32,
    pub quest_requirement: RowRef<Quest>,
    pub specialist: bool,
    pub cost: u8,
    pub crp_id: u32,
    pub crp: RowRef<CraftAction>,
    pub bsm_id: u32,
    pub bsm: RowRef<CraftAction>,
    pub arm_id: u32,
    pub arm: RowRef<CraftAction>,
    pub gsm_id: u32,
    pub gsm: RowRef<CraftAction>,
    pub ltw_id: u32,
    pub ltw: RowRef<CraftAction>,
    pub wvr_id: u32,
    pub wvr: RowRef<CraftAction>,
    pub alc_id: u32,
    pub alc: RowRef<CraftAction>,
    pub cul_id: u32,
    pub cul: RowRef<CraftAction>,
}

impl Sheet for CraftAction {
    const SHEET_NAME: &'static str = "CraftAction";
}

impl FromExcelRow for CraftAction {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            description: single_row.columns.get(1).to_owned_string(),
            animation_start_id: single_row.columns.get(2).to_u32(),
            animation_start: RowRef::<ActionTimeline>::from(single_row.columns.get(2).to_u32()),
            animation_end_id: single_row.columns.get(3).to_u32(),
            animation_end: RowRef::<ActionTimeline>::from(single_row.columns.get(3).to_u32()),
            icon_id: single_row.columns.get(4).to_u32(),
            class_job_id: single_row.columns.get(5).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(5).to_u32()),
            class_job_category_id: single_row.columns.get(6).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(6).to_u32()),
            class_job_level: single_row.columns.get(7).to_u8(),
            quest_requirement_id: single_row.columns.get(8).to_u32(),
            quest_requirement: RowRef::<Quest>::from(single_row.columns.get(8).to_u32()),
            specialist: single_row.columns.get(9).to_bit(0),
            cost: single_row.columns.get(11).to_u8(),
            crp_id: single_row.columns.get(12).to_u32(),
            crp: RowRef::<CraftAction>::from(single_row.columns.get(12).to_u32()),
            bsm_id: single_row.columns.get(13).to_u32(),
            bsm: RowRef::<CraftAction>::from(single_row.columns.get(13).to_u32()),
            arm_id: single_row.columns.get(14).to_u32(),
            arm: RowRef::<CraftAction>::from(single_row.columns.get(14).to_u32()),
            gsm_id: single_row.columns.get(15).to_u32(),
            gsm: RowRef::<CraftAction>::from(single_row.columns.get(15).to_u32()),
            ltw_id: single_row.columns.get(16).to_u32(),
            ltw: RowRef::<CraftAction>::from(single_row.columns.get(16).to_u32()),
            wvr_id: single_row.columns.get(17).to_u32(),
            wvr: RowRef::<CraftAction>::from(single_row.columns.get(17).to_u32()),
            alc_id: single_row.columns.get(18).to_u32(),
            alc: RowRef::<CraftAction>::from(single_row.columns.get(18).to_u32()),
            cul_id: single_row.columns.get(19).to_u32(),
            cul: RowRef::<CraftAction>::from(single_row.columns.get(19).to_u32()),
        })
    }
}

