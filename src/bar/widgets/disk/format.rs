use wayle_sysinfo::types::DiskData;

pub(super) fn initial_text() -> String {
    "Unavailable".to_string()
}

pub(super) fn disk_free_text(bytes: f64) -> String {
    format!("{bytes:.0}B")
}

pub(super) fn disk_used_text(bytes: f64) -> String {
    format!("{bytes:.0}B")
}

pub(super) fn disk_total_text(bytes: f64) -> String {
    format!("{bytes:.0}B")
}