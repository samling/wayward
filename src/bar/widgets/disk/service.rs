use crate::bar::state::{BarItemState, DiskSnapshot, DiskState};
use crate::shell::ShellMsg;
use futures::{FutureExt, StreamExt, future, select};
use relm4::Sender;
use std::sync::Arc;
use wayle_sysinfo::SysinfoService;

pub(super) fn start(
    sender: Sender<ShellMsg>,
    service: Option<Arc<SysinfoService>>,
) -> relm4::tokio::task::JoinHandle<()> {
    relm4::spawn(async move {
        run_disk_watcher(sender, service).await;
    })
}

async fn run_disk_watcher(
    sender: Sender<ShellMsg>,
    service: Option<Arc<SysinfoService>>,
) {
    let Some(service) = service else {
        let _ = sender.send(disk_message(DiskState::Unavailable));
        return;
    };

    send_disk_snapshot(&sender, service.as_ref());
}

fn send_disk_snapshot(
    sender: &Sender<ShellMsg>,
    service: &SysinfoService,
) {
    let snapshot = snapshot_from_services(service);

    let _ = sender.send(disk_message(DiskState::Ready(snapshot)));
}

pub(super) fn snapshot_from_services(
    service: &SysinfoService,
) -> DiskSnapshot {
    let disks = service.disks.get();

    DiskSnapshot {
        disks,
    }
}

fn disk_message(state: DiskState) -> ShellMsg {
    ShellMsg::ItemStateChanged(BarItemState::Disk(state))
}