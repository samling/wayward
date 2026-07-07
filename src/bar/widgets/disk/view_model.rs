use crate::bar::state::DiskSnapshot;
use wayle_sysinfo::types::DiskData;

use super::format::{
    disk_free_text, disk_total_text, disk_used_text
};

#[derive(Clone, Debug)]
pub(super) struct DiskViewModel {
    pub(super) icon_name: &'static str,
    pub(super) used_text: Option<String>,
    pub(super) tooltip_text: Option<String>,
    pub(super) rows: Vec<DiskRowViewModel>,
}

impl DiskViewModel {
    pub(super) fn unavailable() -> Self {
        Self {
            icon_name: "disk-missing-symbolic",
            used_text: "NaN".to_string(),
            tooltip_text: None,
            rows: Vec::new(),
        }
    }

    pub(super) fn from_snapshot(snapshot: &DiskSnapshot) -> Self {
        Self {
            icon_name: "disk-missing-symbolic",
            used_text: disk_used_text(snapshot.disks)
                .unwrap_or_else(|| "Unavailable".to_string()),
            tooltip_text: disk_tooltip_text,
            rows: 
        }    
    }
}

#[derive(Clone, Debug)]
pub(super) struct DiskRowViewModel {
    pub(super) key: String,
    pub(super) mount_text: String,
    pub(super) used_text: String,
    pub(super) free_text: String,
    pub(super) total_text: String,
    pub(super) meter_value: f64,
}

impl DiskRowViewModel {
    fn from_disk(disk: &DiskData) -> Self {
        Self {
            key: disk.mount_point.display().to_string(),
            mount_text: disk.mount_point.display().to_string(),
            used_text: disk_used_text(disk.used_bytes as f64),
            free_text: disk_free_text(disk.free_bytes as f64),
            total_text: disk_total_text(disk.total_bytes as f64),

        }
    }
}