/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct BNpcState {
    pub row_id: u32,
    pub slot: u8,
    pub over_ray: i8,
    pub idle: u16,
    pub attribute_0: u8,
    pub attribute_flag_0: bool,
    pub attribute_1: u8,
    pub attribute_flag_1: bool,
    pub attribute_2: u8,
    pub attribute_flag_2: bool,
    pub scale: f32,
    pub loop_timeline: i32,
}

impl Sheet for BNpcState {
    const SHEET_NAME: &'static str = "BNpcState";
}

impl FromExcelRow for BNpcState {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            slot: single_row.columns.get(0).to_u8(),
            over_ray: single_row.columns.get(1).to_i8(),
            idle: single_row.columns.get(4).to_u16(),
            attribute_0: single_row.columns.get(5).to_u8(),
            attribute_flag_0: single_row.columns.get(6).to_bool(),
            attribute_1: single_row.columns.get(7).to_u8(),
            attribute_flag_1: single_row.columns.get(8).to_bool(),
            attribute_2: single_row.columns.get(9).to_u8(),
            attribute_flag_2: single_row.columns.get(10).to_bool(),
            scale: single_row.columns.get(11).to_f32(),
            loop_timeline: single_row.columns.get(13).to_i32(),
        })
    }
}

