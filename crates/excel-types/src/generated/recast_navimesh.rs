/// This file is auto-generated. Do not edit manually.

use crate::prelude::*;

#[derive(Debug, Clone)]
pub struct RecastNavimesh {
    pub row_id: u32,
    pub tile_size: f32,
    pub cell_size: f32,
    pub cell_height: f32,
    pub agent_height: f32,
    pub agent_radius: f32,
    pub agent_max_climb: f32,
    pub agent_max_slope: f32,
    pub region_min_size: f32,
    pub region_merged_size: f32,
    pub max_edge_length: f32,
    pub max_edge_error: f32,
    pub verts_per_poly: f32,
    pub detail_mesh_sample_distance: f32,
    pub detail_mesh_max_sample_error: f32,
}

impl Sheet for RecastNavimesh {
    const SHEET_NAME: &'static str = "RecastNavimesh";
}

impl FromExcelRow for RecastNavimesh {
    fn from_row(row: &ExcelRow) -> Option<Self> {
        let single_row = match &row.kind {
            ExcelRowKind::SingleRow(s) => s,
            _ => return None,
        };

        Some(Self {
            row_id: row.row_id,
            tile_size: single_row.columns.get(1).to_f32(),
            cell_size: single_row.columns.get(2).to_f32(),
            cell_height: single_row.columns.get(3).to_f32(),
            agent_height: single_row.columns.get(4).to_f32(),
            agent_radius: single_row.columns.get(5).to_f32(),
            agent_max_climb: single_row.columns.get(6).to_f32(),
            agent_max_slope: single_row.columns.get(7).to_f32(),
            region_min_size: single_row.columns.get(9).to_f32(),
            region_merged_size: single_row.columns.get(10).to_f32(),
            max_edge_length: single_row.columns.get(12).to_f32(),
            max_edge_error: single_row.columns.get(13).to_f32(),
            verts_per_poly: single_row.columns.get(14).to_f32(),
            detail_mesh_sample_distance: single_row.columns.get(15).to_f32(),
            detail_mesh_max_sample_error: single_row.columns.get(16).to_f32(),
        })
    }
}

