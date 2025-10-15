/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BattleLeve {
    pub row_id: u32,
    pub rule_id: u32,
    pub rule: RowRef<BattleLeveRule>,
    pub varient: u8,
    pub objective_0_id: u32,
    pub objective_0: RowRef<LeveString>,
    pub objective_1_id: u32,
    pub objective_1: RowRef<LeveString>,
    pub objective_2_id: u32,
    pub objective_2: RowRef<LeveString>,
    pub help_0: u16,
    pub help_1: u16,
}

impl Sheet for BattleLeve {
    const SHEET_NAME: &'static str = "BattleLeve";
}

impl FromExcelRow for BattleLeve {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            rule_id: single_row.columns.get(176).to_u32(),
            rule: RowRef::<BattleLeveRule>::from(single_row.columns.get(176).to_u32()),
            varient: single_row.columns.get(177).to_u8(),
            objective_0_id: single_row.columns.get(178).to_u32(),
            objective_0: RowRef::<LeveString>::from(single_row.columns.get(178).to_u32()),
            objective_1_id: single_row.columns.get(179).to_u32(),
            objective_1: RowRef::<LeveString>::from(single_row.columns.get(179).to_u32()),
            objective_2_id: single_row.columns.get(180).to_u32(),
            objective_2: RowRef::<LeveString>::from(single_row.columns.get(180).to_u32()),
            help_0: single_row.columns.get(181).to_u16(),
            help_1: single_row.columns.get(182).to_u16(),
        })
    }
}

