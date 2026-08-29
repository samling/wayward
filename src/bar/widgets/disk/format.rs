use wayle_sysinfo::types::DiskData;

pub(super) const DISK_ICON: &str = "drive-harddisk-symbolic";

const BYTE_UNITS: [&str; 5] = ["B", "K", "M", "G", "T"];

pub(super) fn initial_text() -> String {
    "NaN".to_string()
}

pub(super) fn disk_bytes_text(bytes: u64) -> String {
    let mut value = bytes as f64;
    let mut unit = 0;

    while value >= 1024.0 && unit < BYTE_UNITS.len() - 1 {
        value /= 1024.0;
        unit += 1;
    }

    let suffix = BYTE_UNITS[unit];

    if unit == 0 || value >= 100.0 {
        format!("{value:.0}{suffix}")
    } else {
        format!("{value:.1}{suffix}")
    }
}

pub(super) fn disk_usage_percent_text(percent: f32) -> String {
    format!("{percent:.0}%")
}

pub(super) fn disk_tooltip_text(disks: &[&DiskData]) -> Option<String> {
    if disks.is_empty() {
        return None;
    }

    let lines = disks
        .iter()
        .map(|disk| {
            format!(
                "{}: {} free of {}",
                disk.mount_point.display(),
                disk_bytes_text(disk.available_bytes),
                disk_bytes_text(disk.total_bytes)
            )
        })
        .collect::<Vec<_>>();

    Some(lines.join("\n"))
}
