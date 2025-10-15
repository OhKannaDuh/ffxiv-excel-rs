/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Credit {
    pub row_id: u32,
    pub roles_1_id: u32,
    pub roles_1: RowRef<CreditCast>,
    pub japanese_cast_1_id: u32,
    pub japanese_cast_1: RowRef<CreditCast>,
    pub english_cast_1_id: u32,
    pub english_cast_1: RowRef<CreditCast>,
    pub french_cast_1_id: u32,
    pub french_cast_1: RowRef<CreditCast>,
    pub german_cast_1_id: u32,
    pub german_cast_1: RowRef<CreditCast>,
    pub roles_2_id: u32,
    pub roles_2: RowRef<CreditCast>,
    pub japanese_cast_2_id: u32,
    pub japanese_cast_2: RowRef<CreditCast>,
    pub english_cast_2_id: u32,
    pub english_cast_2: RowRef<CreditCast>,
    pub french_cast_2_id: u32,
    pub french_cast_2: RowRef<CreditCast>,
    pub german_cast_2_id: u32,
    pub german_cast_2: RowRef<CreditCast>,
}

impl Sheet for Credit {
    const SHEET_NAME: &'static str = "Credit";
}

impl FromExcelRow for Credit {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            roles_1_id: single_row.columns.get(1).to_u32(),
            roles_1: RowRef::<CreditCast>::from(single_row.columns.get(1).to_u32()),
            japanese_cast_1_id: single_row.columns.get(2).to_u32(),
            japanese_cast_1: RowRef::<CreditCast>::from(single_row.columns.get(2).to_u32()),
            english_cast_1_id: single_row.columns.get(3).to_u32(),
            english_cast_1: RowRef::<CreditCast>::from(single_row.columns.get(3).to_u32()),
            french_cast_1_id: single_row.columns.get(4).to_u32(),
            french_cast_1: RowRef::<CreditCast>::from(single_row.columns.get(4).to_u32()),
            german_cast_1_id: single_row.columns.get(5).to_u32(),
            german_cast_1: RowRef::<CreditCast>::from(single_row.columns.get(5).to_u32()),
            roles_2_id: single_row.columns.get(6).to_u32(),
            roles_2: RowRef::<CreditCast>::from(single_row.columns.get(6).to_u32()),
            japanese_cast_2_id: single_row.columns.get(7).to_u32(),
            japanese_cast_2: RowRef::<CreditCast>::from(single_row.columns.get(7).to_u32()),
            english_cast_2_id: single_row.columns.get(8).to_u32(),
            english_cast_2: RowRef::<CreditCast>::from(single_row.columns.get(8).to_u32()),
            french_cast_2_id: single_row.columns.get(9).to_u32(),
            french_cast_2: RowRef::<CreditCast>::from(single_row.columns.get(9).to_u32()),
            german_cast_2_id: single_row.columns.get(10).to_u32(),
            german_cast_2: RowRef::<CreditCast>::from(single_row.columns.get(10).to_u32()),
        })
    }
}

