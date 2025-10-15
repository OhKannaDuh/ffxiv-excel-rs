/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ActivityFeedCaptions {
    pub row_id: u32,
    pub ja: String,
    pub en: String,
    pub de: String,
    pub fr: String,
}

impl Sheet for ActivityFeedCaptions {
    const SHEET_NAME: &'static str = "ActivityFeedCaptions";
}

impl FromExcelRow for ActivityFeedCaptions {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            ja: single_row.columns.get(0).to_owned_string(),
            en: single_row.columns.get(1).to_owned_string(),
            de: single_row.columns.get(2).to_owned_string(),
            fr: single_row.columns.get(3).to_owned_string(),
        })
    }
}

