/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct Warp {
    pub row_id: u32,
    pub pop_range_id: u32,
    pub pop_range: RowRef<Level>,
    pub territory_type_id: u32,
    pub territory_type: RowRef<TerritoryType>,
    pub condition_success_event_id: u32,
    pub condition_success_event: RowRef<DefaultTalk>,
    pub condition_fail_event_id: u32,
    pub condition_fail_event: RowRef<DefaultTalk>,
    pub confirm_event_id: u32,
    pub confirm_event: RowRef<DefaultTalk>,
    pub warp_condition_id: u32,
    pub warp_condition: RowRef<WarpCondition>,
    pub warp_logic_id: u32,
    pub warp_logic: RowRef<WarpLogic>,
    pub start_cutscene_id: u32,
    pub start_cutscene: RowRef<Cutscene>,
    pub end_cutscene_id: u32,
    pub end_cutscene: RowRef<Cutscene>,
    pub can_skip_cutscene: bool,
    pub name: String,
    pub question: String,
}

impl Sheet for Warp {
    const SHEET_NAME: &'static str = "Warp";
}

impl FromExcelRow for Warp {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            pop_range_id: single_row.columns.get(0).to_u32(),
            pop_range: RowRef::<Level>::from(single_row.columns.get(0).to_u32()),
            territory_type_id: single_row.columns.get(1).to_u32(),
            territory_type: RowRef::<TerritoryType>::from(single_row.columns.get(1).to_u32()),
            condition_success_event_id: single_row.columns.get(2).to_u32(),
            condition_success_event: RowRef::<DefaultTalk>::from(single_row.columns.get(2).to_u32()),
            condition_fail_event_id: single_row.columns.get(3).to_u32(),
            condition_fail_event: RowRef::<DefaultTalk>::from(single_row.columns.get(3).to_u32()),
            confirm_event_id: single_row.columns.get(4).to_u32(),
            confirm_event: RowRef::<DefaultTalk>::from(single_row.columns.get(4).to_u32()),
            warp_condition_id: single_row.columns.get(5).to_u32(),
            warp_condition: RowRef::<WarpCondition>::from(single_row.columns.get(5).to_u32()),
            warp_logic_id: single_row.columns.get(6).to_u32(),
            warp_logic: RowRef::<WarpLogic>::from(single_row.columns.get(6).to_u32()),
            start_cutscene_id: single_row.columns.get(7).to_u32(),
            start_cutscene: RowRef::<Cutscene>::from(single_row.columns.get(7).to_u32()),
            end_cutscene_id: single_row.columns.get(8).to_u32(),
            end_cutscene: RowRef::<Cutscene>::from(single_row.columns.get(8).to_u32()),
            can_skip_cutscene: single_row.columns.get(9).to_bit(0),
            name: single_row.columns.get(10).to_owned_string(),
            question: single_row.columns.get(11).to_owned_string(),
        })
    }
}

