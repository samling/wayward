use futures::stream::{self, BoxStream};
use futures::{StreamExt, select};
use relm4::Sender;
use std::sync::Arc;
use wayle_systray::SystemTrayService;
use wayle_systray::core::item::TrayItem;
use wayle_systray::types::Coordinates;

use super::view_model::SystrayItemSummary;
use crate::bar::state::{BarItemState, SystrayState};
use crate::bar::widget::{SystrayAction, WidgetAction, WidgetEvent};
use crate::shell::ShellMsg;

pub fn start(
    sender: relm4::Sender<ShellMsg>,
    service: Option<Arc<SystemTrayService>>,
) -> relm4::tokio::task::JoinHandle<()> {
    relm4::spawn(async move {
        run_systray_watcher(sender, service).await;
    })
}

pub async fn run_systray_watcher(
    sender: Sender<ShellMsg>,
    service: Option<Arc<SystemTrayService>>,
) {
    let Some(service) = service else {
        let _ = sender.send(systray_message(SystrayState::Unavailable));
        return;
    };

    send_systray_snapshot(&sender, service.as_ref());

    let mut item_updates = service.items.watch().fuse();
    let mut property_updates = systray_item_update_stream(service.items.get()).fuse();

    loop {
        select! {
            update = item_updates.next() => {
                if update.is_none() {
                    break;
                }

                property_updates = systray_item_update_stream(service.items.get()).fuse();
                send_systray_snapshot(&sender, service.as_ref());
            }
            update = property_updates.next() => {
                if update.is_none() {
                    property_updates = systray_item_update_stream(service.items.get()).fuse();
                }

                send_systray_snapshot(&sender, service.as_ref());
            }
        }
    }

    let _ = sender.send(systray_message(SystrayState::Unavailable));
}

pub(crate) fn handle_event(event: WidgetEvent, service: Option<Arc<SystemTrayService>>) {
    match event.action {
        WidgetAction::Systray(SystrayAction::Clicked {
            item_id,
            button,
            x,
            y,
        }) => {
            let bus_name = item_id;
            handle_click(bus_name, button, x, y, service);
        }
        _ => {}
    }
}

fn handle_click(
    bus_name: String,
    button: u32,
    x: i32,
    y: i32,
    service: Option<Arc<SystemTrayService>>,
) {
    let Some(service) = service else {
        tracing::warn!("Ignoring systray click before service is ready");
        return;
    };

    relm4::spawn(async move {
        let Some(item) = service
            .items
            .get()
            .into_iter()
            .find(|item| item.bus_name.get() == bus_name)
        else {
            tracing::warn!("Systray item disappeared before click could be handled: {bus_name}");
            return;
        };

        let coords = Coordinates::new(x, y);

        let result = match button {
            1 if item.item_is_menu.get() => item.context_menu(coords).await,
            1 => item.activate(coords).await,
            2 => item.secondary_activate(coords).await,
            3 => item.context_menu(coords).await,
            _ => {
                return;
            }
        };

        if let Err(error) = result {
            tracing::error!("Failed to handle systray click for {bus_name}: {error}");
        }
    });
}

fn send_systray_snapshot(sender: &Sender<ShellMsg>, service: &SystemTrayService) {
    let items = service
        .items
        .get()
        .iter()
        .map(|item| SystrayItemSummary::from_wayle_item(item))
        .collect();

    let _ = sender.send(systray_message(SystrayState::Ready(items)));
}

fn systray_item_update_stream(items: Vec<Arc<TrayItem>>) -> BoxStream<'static, ()> {
    let streams = items
        .iter()
        .flat_map(|item| {
            [
                item.id.watch().skip(1).map(|_| ()).boxed(),
                item.title.watch().skip(1).map(|_| ()).boxed(),
                item.status.watch().skip(1).map(|_| ()).boxed(),
                item.icon_name.watch().skip(1).map(|_| ()).boxed(),
                item.icon_pixmap.watch().skip(1).map(|_| ()).boxed(),
                item.icon_theme_path.watch().skip(1).map(|_| ()).boxed(),
                item.tooltip.watch().skip(1).map(|_| ()).boxed(),
                item.bus_name.watch().skip(1).map(|_| ()).boxed(),
            ]
        })
        .collect();

    merged_update_stream(streams)
}

fn merged_update_stream(streams: Vec<BoxStream<'static, ()>>) -> BoxStream<'static, ()> {
    if streams.is_empty() {
        return stream::pending().boxed();
    }

    stream::select_all(streams).boxed()
}

fn systray_message(state: SystrayState) -> ShellMsg {
    ShellMsg::ItemStateChanged(BarItemState::Systray(state))
}

pub(crate) fn item_by_bus_name(
    service: &SystemTrayService,
    bus_name: &str,
) -> Option<std::sync::Arc<wayle_systray::core::item::TrayItem>> {
    service
        .items
        .get()
        .into_iter()
        .find(|item| item.bus_name.get() == bus_name)
}

#[cfg(test)]
#[path = "service_test.rs"]
mod tests;
