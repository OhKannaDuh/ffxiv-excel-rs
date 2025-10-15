/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GatheringPointBonus {
    pub row_id: u32,
    pub condition_id: u32,
    pub condition: RowRef<GatheringCondition>,
    pub condition_value: u32,
    pub bonus_type_id: u32,
    pub bonus_type: RowRef<GatheringPointBonusType>,
    pub bonus_value: u16,
}

impl Sheet for GatheringPointBonus {
    const SHEET_NAME: &'static str = "GatheringPointBonus";
}

impl FromExcelRow for GatheringPointBonus {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            condition_id: single_row.columns.get(0).to_u32(),
            condition: RowRef::<GatheringCondition>::from(single_row.columns.get(0).to_u32()),
            condition_value: single_row.columns.get(1).to_u32(),
            bonus_type_id: single_row.columns.get(3).to_u32(),
            bonus_type: RowRef::<GatheringPointBonusType>::from(single_row.columns.get(3).to_u32()),
            bonus_value: single_row.columns.get(4).to_u16(),
        })
    }
}

