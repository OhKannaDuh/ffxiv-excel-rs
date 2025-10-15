/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct EurekaLogosMixerProbability {
    pub row_id: u32,
    pub probability_percent: u8,
}

impl Sheet for EurekaLogosMixerProbability {
    const SHEET_NAME: &'static str = "EurekaLogosMixerProbability";
}

impl FromExcelRow for EurekaLogosMixerProbability {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            probability_percent: single_row.columns.get(0).to_u8(),
        })
    }
}

