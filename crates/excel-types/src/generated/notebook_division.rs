/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct NotebookDivision {
    pub row_id: u32,
    pub name: String,
    pub notebook_division_category_id: u32,
    pub notebook_division_category: RowRef<NotebookDivisionCategory>,
    pub craft_opening_level: u8,
    pub gathering_opening_level: u8,
    pub quest_unlock_id: u32,
    pub quest_unlock: RowRef<Quest>,
    pub crp_craft: u8,
    pub bsm_craft: u8,
    pub arm_craft: bool,
    pub gsm_craft: bool,
    pub ltw_craft: bool,
    pub wvr_craft: bool,
    pub alc_craft: bool,
    pub cul_craft: bool,
}

impl Sheet for NotebookDivision {
    const SHEET_NAME: &'static str = "NotebookDivision";
}

impl FromExcelRow for NotebookDivision {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            name: single_row.columns.get(0).to_owned_string(),
            notebook_division_category_id: single_row.columns.get(1).to_u32(),
            notebook_division_category: RowRef::<NotebookDivisionCategory>::from(single_row.columns.get(1).to_u32()),
            craft_opening_level: single_row.columns.get(2).to_u8(),
            gathering_opening_level: single_row.columns.get(3).to_u8(),
            quest_unlock_id: single_row.columns.get(4).to_u32(),
            quest_unlock: RowRef::<Quest>::from(single_row.columns.get(4).to_u32()),
            crp_craft: single_row.columns.get(7).to_u8(),
            bsm_craft: single_row.columns.get(8).to_u8(),
            arm_craft: single_row.columns.get(9).to_bool(),
            gsm_craft: single_row.columns.get(10).to_bool(),
            ltw_craft: single_row.columns.get(11).to_bool(),
            wvr_craft: single_row.columns.get(12).to_bool(),
            alc_craft: single_row.columns.get(13).to_bool(),
            cul_craft: single_row.columns.get(14).to_bool(),
        })
    }
}

