/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PresetCamera {
    pub row_id: u32,
    pub eid: u16,
    pub pos_x: f32,
    pub pos_y: f32,
    pub pos_z: f32,
    pub elezen: f32,
    pub lalafell: f32,
    pub miqote: f32,
    pub roe: f32,
    pub hrothgar: f32,
    pub viera: f32,
    pub hyur_f: f32,
    pub elezen_f: f32,
    pub lalafell_f: f32,
    pub miqote_f: f32,
    pub roe_f: f32,
    pub hrothgar_f: f32,
    pub viera_f: f32,
}

impl Sheet for PresetCamera {
    const SHEET_NAME: &'static str = "PresetCamera";
}

impl FromExcelRow for PresetCamera {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            eid: single_row.columns.get(0).to_u16(),
            pos_x: single_row.columns.get(1).to_f32(),
            pos_y: single_row.columns.get(2).to_f32(),
            pos_z: single_row.columns.get(3).to_f32(),
            elezen: single_row.columns.get(4).to_f32(),
            lalafell: single_row.columns.get(5).to_f32(),
            miqote: single_row.columns.get(6).to_f32(),
            roe: single_row.columns.get(7).to_f32(),
            hrothgar: single_row.columns.get(8).to_f32(),
            viera: single_row.columns.get(9).to_f32(),
            hyur_f: single_row.columns.get(11).to_f32(),
            elezen_f: single_row.columns.get(12).to_f32(),
            lalafell_f: single_row.columns.get(13).to_f32(),
            miqote_f: single_row.columns.get(14).to_f32(),
            roe_f: single_row.columns.get(15).to_f32(),
            hrothgar_f: single_row.columns.get(16).to_f32(),
            viera_f: single_row.columns.get(18).to_f32(),
        })
    }
}

