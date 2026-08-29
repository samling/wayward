use std::path::Path;

use crate::bar::state::DiskSnapshot;
use wayle_sysinfo::types::DiskData;

use super::format::{
    DISK_ICON, disk_bytes_text, disk_tooltip_text, disk_usage_percent_text, initial_text,
};

const PRIMARY_MOUNT: &str = "/";

const HIDDEN_MOUNT_PREFIXES: &[&str] = &[
    "/boot",
    "/dev",
    "/proc",
    "/run",
    "/snap",
    "/sys",
    "/tmp",
    "/var/lib/docker",
];

const HIDDEN_FILESYSTEMS: &[&str] = &["devtmpfs", "overlay", "squashfs", "tmpfs"];

fn visible_disks(disks: &[DiskData]) -> Vec<&DiskData> {
    disks.iter().filter(|disk| is_visible(disk)).collect()
}

fn is_visible(disk: &DiskData) -> bool {
    !HIDDEN_FILESYSTEMS.contains(&disk.filesystem.as_str())
        && !HIDDEN_MOUNT_PREFIXES
            .iter()
            .any(|prefix| disk.mount_point.starts_with(prefix))
}

fn primary_disk<'a>(disks: &[&'a DiskData]) -> Option<&'a DiskData> {
    disks
        .iter()
        .copied()
        .find(|disk| disk.mount_point == Path::new(PRIMARY_MOUNT))
        .or_else(|| disks.iter().copied().max_by_key(|disk| disk.total_bytes))
}

#[derive(Clone, Debug)]
pub(super) struct DiskViewModel {
    pub(super) icon_name: &'static str,
    pub(super) used_text: String,
    pub(super) tooltip_text: Option<String>,
    pub(super) rows: Vec<DiskRowViewModel>,
}

impl DiskViewModel {
    pub(super) fn unavailable() -> Self {
        Self {
            icon_name: DISK_ICON,
            used_text: initial_text(),
            tooltip_text: None,
            rows: Vec::new(),
        }
    }

    pub(super) fn from_snapshot(snapshot: &DiskSnapshot) -> Self {
        let disks = visible_disks(&snapshot.disks);

        let used_text = primary_disk(&disks)
            .map(|disk| disk_usage_percent_text(disk.usage_percent))
            .unwrap_or_else(initial_text);

        Self {
            icon_name: DISK_ICON,
            used_text,
            tooltip_text: disk_tooltip_text(&disks),
            rows: disks
                .iter()
                .map(|disk| DiskRowViewModel::from_disk(disk))
                .collect(),
        }
    }
}

#[derive(Clone, Debug)]
pub(super) struct DiskRowViewModel {
    pub(super) mount_text: String,
    pub(super) used_text: String,
    pub(super) free_text: String,
    pub(super) total_text: String,
    pub(super) meter_value: f64,
}

impl DiskRowViewModel {
    fn from_disk(disk: &DiskData) -> Self {
        Self {
            mount_text: disk.mount_point.display().to_string(),
            used_text: disk_bytes_text(disk.used_bytes),
            free_text: disk_bytes_text(disk.available_bytes),
            total_text: disk_bytes_text(disk.total_bytes),
            meter_value: f64::from(disk.usage_percent).clamp(0.0, 100.0),
        }
    }
}
