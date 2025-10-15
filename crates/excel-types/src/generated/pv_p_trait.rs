/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PvPTrait {
    pub row_id: u32,
    pub trait_1_id: u32,
    pub trait_1: RowRef<Trait>,
    pub trait_2_id: u32,
    pub trait_2: RowRef<Trait>,
    pub trait_3_id: u32,
    pub trait_3: RowRef<Trait>,
}

impl Sheet for PvPTrait {
    const SHEET_NAME: &'static str = "PvPTrait";
}

impl FromExcelRow for PvPTrait {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            trait_1_id: single_row.columns.get(0).to_u32(),
            trait_1: RowRef::<Trait>::from(single_row.columns.get(0).to_u32()),
            trait_2_id: single_row.columns.get(1).to_u32(),
            trait_2: RowRef::<Trait>::from(single_row.columns.get(1).to_u32()),
            trait_3_id: single_row.columns.get(2).to_u32(),
            trait_3: RowRef::<Trait>::from(single_row.columns.get(2).to_u32()),
        })
    }
}

