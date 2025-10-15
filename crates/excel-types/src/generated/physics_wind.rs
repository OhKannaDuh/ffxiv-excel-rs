/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct PhysicsWind {
    pub row_id: u32,
    pub threshold: f32,
    pub amplitude: f32,
    pub amplitude_frequency: f32,
    pub power_min: f32,
    pub power_max: f32,
    pub power_frequency: f32,
}

impl Sheet for PhysicsWind {
    const SHEET_NAME: &'static str = "PhysicsWind";
}

impl FromExcelRow for PhysicsWind {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            threshold: single_row.columns.get(0).to_f32(),
            amplitude: single_row.columns.get(1).to_f32(),
            amplitude_frequency: single_row.columns.get(2).to_f32(),
            power_min: single_row.columns.get(3).to_f32(),
            power_max: single_row.columns.get(4).to_f32(),
            power_frequency: single_row.columns.get(5).to_f32(),
        })
    }
}

