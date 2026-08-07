use crate::bar::state::{BarItemState, DiskSnapshot, DiskState};
use crate::shell::ShellMsg;
use futures::StreamExt;
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

    let mut disk_updates = service.disks.watch();

    while let Some(disks) = disk_updates.next().await {
        let _ = sender.send(disk_message(DiskState::Ready(DiskSnapshot { disks })));
    }

    let _ = sender.send(disk_message(DiskState::Unavailable));
}

fn disk_message(state: DiskState) -> ShellMsg {
    ShellMsg::ItemStateChanged(BarItemState::Disk(state))
}