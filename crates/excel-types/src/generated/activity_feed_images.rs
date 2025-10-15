/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct ActivityFeedImages {
    pub row_id: u32,
    pub expansion_image: String,
    pub activity_feed_ja: String,
    pub activity_feed_en: String,
    pub activity_feed_de: String,
    pub activity_feed_fr: String,
}

impl Sheet for ActivityFeedImages {
    const SHEET_NAME: &'static str = "ActivityFeedImages";
}

impl FromExcelRow for ActivityFeedImages {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            expansion_image: single_row.columns.get(0).to_owned_string(),
            activity_feed_ja: single_row.columns.get(1).to_owned_string(),
            activity_feed_en: single_row.columns.get(2).to_owned_string(),
            activity_feed_de: single_row.columns.get(3).to_owned_string(),
            activity_feed_fr: single_row.columns.get(4).to_owned_string(),
        })
    }
}

