/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct GcArmyExpeditionMemberBonus {
    pub row_id: u32,
    pub race_id: u32,
    pub race: RowRef<Race>,
    pub class_job_id: u32,
    pub class_job: RowRef<ClassJob>,
}

impl Sheet for GcArmyExpeditionMemberBonus {
    const SHEET_NAME: &'static str = "GcArmyExpeditionMemberBonus";
}

impl FromExcelRow for GcArmyExpeditionMemberBonus {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            race_id: single_row.columns.get(0).to_u32(),
            race: RowRef::<Race>::from(single_row.columns.get(0).to_u32()),
            class_job_id: single_row.columns.get(1).to_u32(),
            class_job: RowRef::<ClassJob>::from(single_row.columns.get(1).to_u32()),
        })
    }
}

