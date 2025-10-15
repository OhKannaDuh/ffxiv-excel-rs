/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RetainerTask {
    pub row_id: u32,
    pub is_random: bool,
    pub class_job_category_id: u32,
    pub class_job_category: RowRef<ClassJobCategory>,
    pub retainer_level: u8,
    pub retainer_task_parameter_id: u32,
    pub retainer_task_parameter: RowRef<RetainerTaskParameter>,
    pub venture_cost: u16,
    pub max_time_min: u16,
    pub experience: i32,
    pub required_item_level: u16,
    pub condition_param_0: u8,
    pub condition_param_1: u8,
    pub required_gathering: u16,
    pub task_id: u32,
    pub task_retainer_task_normal: RowRef<RetainerTaskNormal>,
    pub task_retainer_task_random: RowRef<RetainerTaskRandom>,
}

impl Sheet for RetainerTask {
    const SHEET_NAME: &'static str = "RetainerTask";
}

impl FromExcelRow for RetainerTask {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            is_random: single_row.columns.get(0).to_bit(0),
            class_job_category_id: single_row.columns.get(1).to_u32(),
            class_job_category: RowRef::<ClassJobCategory>::from(single_row.columns.get(1).to_u32()),
            retainer_level: single_row.columns.get(2).to_u8(),
            retainer_task_parameter_id: single_row.columns.get(4).to_u32(),
            retainer_task_parameter: RowRef::<RetainerTaskParameter>::from(single_row.columns.get(4).to_u32()),
            venture_cost: single_row.columns.get(5).to_u16(),
            max_time_min: single_row.columns.get(6).to_u16(),
            experience: single_row.columns.get(7).to_i32(),
            required_item_level: single_row.columns.get(8).to_u16(),
            condition_param_0: single_row.columns.get(9).to_u8(),
            condition_param_1: single_row.columns.get(10).to_u8(),
            required_gathering: single_row.columns.get(11).to_u16(),
            task_id: single_row.columns.get(13).to_u32(),
            task_retainer_task_normal: RowRef::<RetainerTaskNormal>::from(single_row.columns.get(13).to_u32()),
            task_retainer_task_random: RowRef::<RetainerTaskRandom>::from(single_row.columns.get(13).to_u32()),
        })
    }
}

