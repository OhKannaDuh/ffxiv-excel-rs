/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ScenarioTreeTipsClassQuest {
    pub row_id: u32,
    pub quest_id: u32,
    pub quest: RowRef<Quest>,
    pub required_level: u16,
    pub required_expansion_id: u32,
    pub required_expansion: RowRef<ExVersion>,
    pub required_quest_id: u32,
    pub required_quest: RowRef<Quest>,
}

impl Sheet for ScenarioTreeTipsClassQuest {
    const SHEET_NAME: &'static str = "ScenarioTreeTipsClassQuest";
}

impl FromExcelRow for ScenarioTreeTipsClassQuest {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            quest_id: single_row.columns.get(0).to_u32(),
            quest: RowRef::<Quest>::from(single_row.columns.get(0).to_u32()),
            required_level: single_row.columns.get(1).to_u16(),
            required_expansion_id: single_row.columns.get(2).to_u32(),
            required_expansion: RowRef::<ExVersion>::from(single_row.columns.get(2).to_u32()),
            required_quest_id: single_row.columns.get(3).to_u32(),
            required_quest: RowRef::<Quest>::from(single_row.columns.get(3).to_u32()),
        })
    }
}

