/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RecipeLookup {
    pub row_id: u32,
    pub crp_id: u32,
    pub crp: RowRef<Recipe>,
    pub bsm_id: u32,
    pub bsm: RowRef<Recipe>,
    pub arm_id: u32,
    pub arm: RowRef<Recipe>,
    pub gsm_id: u32,
    pub gsm: RowRef<Recipe>,
    pub ltw_id: u32,
    pub ltw: RowRef<Recipe>,
    pub wvr_id: u32,
    pub wvr: RowRef<Recipe>,
    pub alc_id: u32,
    pub alc: RowRef<Recipe>,
    pub cul_id: u32,
    pub cul: RowRef<Recipe>,
}

impl Sheet for RecipeLookup {
    const SHEET_NAME: &'static str = "RecipeLookup";
}

impl FromExcelRow for RecipeLookup {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            crp_id: single_row.columns.get(0).to_u32(),
            crp: RowRef::<Recipe>::from(single_row.columns.get(0).to_u32()),
            bsm_id: single_row.columns.get(1).to_u32(),
            bsm: RowRef::<Recipe>::from(single_row.columns.get(1).to_u32()),
            arm_id: single_row.columns.get(2).to_u32(),
            arm: RowRef::<Recipe>::from(single_row.columns.get(2).to_u32()),
            gsm_id: single_row.columns.get(3).to_u32(),
            gsm: RowRef::<Recipe>::from(single_row.columns.get(3).to_u32()),
            ltw_id: single_row.columns.get(4).to_u32(),
            ltw: RowRef::<Recipe>::from(single_row.columns.get(4).to_u32()),
            wvr_id: single_row.columns.get(5).to_u32(),
            wvr: RowRef::<Recipe>::from(single_row.columns.get(5).to_u32()),
            alc_id: single_row.columns.get(6).to_u32(),
            alc: RowRef::<Recipe>::from(single_row.columns.get(6).to_u32()),
            cul_id: single_row.columns.get(7).to_u32(),
            cul: RowRef::<Recipe>::from(single_row.columns.get(7).to_u32()),
        })
    }
}

