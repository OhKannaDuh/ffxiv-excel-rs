/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PresetCameraAdjust {
    pub row_id: u32,
    pub hyur_m: f32,
    pub hyur_f: f32,
    pub elezen_m: f32,
    pub elezen_f: f32,
    pub lalafell_m: f32,
    pub lalafell_f: f32,
    pub miqote_m: f32,
    pub miqote_f: f32,
    pub roe_m: f32,
    pub roe_f: f32,
    pub hrothgar_m: f32,
    pub hrothgar_f: f32,
    pub viera_m: f32,
    pub viera_f: f32,
}

impl Sheet for PresetCameraAdjust {
    const SHEET_NAME: &'static str = "PresetCameraAdjust";
}

impl FromExcelRow for PresetCameraAdjust {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            hyur_m: single_row.columns.get(0).to_f32(),
            hyur_f: single_row.columns.get(1).to_f32(),
            elezen_m: single_row.columns.get(2).to_f32(),
            elezen_f: single_row.columns.get(3).to_f32(),
            lalafell_m: single_row.columns.get(4).to_f32(),
            lalafell_f: single_row.columns.get(5).to_f32(),
            miqote_m: single_row.columns.get(6).to_f32(),
            miqote_f: single_row.columns.get(7).to_f32(),
            roe_m: single_row.columns.get(8).to_f32(),
            roe_f: single_row.columns.get(9).to_f32(),
            hrothgar_m: single_row.columns.get(10).to_f32(),
            hrothgar_f: single_row.columns.get(11).to_f32(),
            viera_m: single_row.columns.get(12).to_f32(),
            viera_f: single_row.columns.get(13).to_f32(),
        })
    }
}

