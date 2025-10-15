/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Race {
    pub row_id: u32,
    pub masculine: String,
    pub feminine: String,
    pub rse_m_body_id: u32,
    pub rse_m_body: RowRef<Item>,
    pub rse_m_hands_id: u32,
    pub rse_m_hands: RowRef<Item>,
    pub rse_m_legs_id: u32,
    pub rse_m_legs: RowRef<Item>,
    pub rse_m_feet_id: u32,
    pub rse_m_feet: RowRef<Item>,
    pub rse_f_body_id: u32,
    pub rse_f_body: RowRef<Item>,
    pub rse_f_hands_id: u32,
    pub rse_f_hands: RowRef<Item>,
    pub rse_f_legs_id: u32,
    pub rse_f_legs: RowRef<Item>,
    pub rse_f_feet_id: u32,
    pub rse_f_feet: RowRef<Item>,
    pub ex_pac_id: u32,
    pub ex_pac: RowRef<ExVersion>,
}

impl Sheet for Race {
    const SHEET_NAME: &'static str = "Race";
}

impl FromExcelRow for Race {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            masculine: single_row.columns.get(0).to_owned_string(),
            feminine: single_row.columns.get(1).to_owned_string(),
            rse_m_body_id: single_row.columns.get(2).to_u32(),
            rse_m_body: RowRef::<Item>::from(single_row.columns.get(2).to_u32()),
            rse_m_hands_id: single_row.columns.get(3).to_u32(),
            rse_m_hands: RowRef::<Item>::from(single_row.columns.get(3).to_u32()),
            rse_m_legs_id: single_row.columns.get(4).to_u32(),
            rse_m_legs: RowRef::<Item>::from(single_row.columns.get(4).to_u32()),
            rse_m_feet_id: single_row.columns.get(5).to_u32(),
            rse_m_feet: RowRef::<Item>::from(single_row.columns.get(5).to_u32()),
            rse_f_body_id: single_row.columns.get(6).to_u32(),
            rse_f_body: RowRef::<Item>::from(single_row.columns.get(6).to_u32()),
            rse_f_hands_id: single_row.columns.get(7).to_u32(),
            rse_f_hands: RowRef::<Item>::from(single_row.columns.get(7).to_u32()),
            rse_f_legs_id: single_row.columns.get(8).to_u32(),
            rse_f_legs: RowRef::<Item>::from(single_row.columns.get(8).to_u32()),
            rse_f_feet_id: single_row.columns.get(9).to_u32(),
            rse_f_feet: RowRef::<Item>::from(single_row.columns.get(9).to_u32()),
            ex_pac_id: single_row.columns.get(11).to_u32(),
            ex_pac: RowRef::<ExVersion>::from(single_row.columns.get(11).to_u32()),
        })
    }
}

