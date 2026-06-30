mod component;
mod service;

use crate::bar::state::{BarItemState, DiskState};
use crate::bar::widget::{
    BarContext, BarWidget, BarWidgetRuntime, WidgetBuildContext, WidgetInstance,
};
use crate::shell::ShellMsg;
use relm4::Controller;
use relm4::Sender;
use relm4::gtk;
use relm4::gtk::glib::object::Cast;
use relm4::prelude::*;

struct DiskRuntime {
    controller: Controller<DiskComponent>,
}

impl BarWidgetRuntime for DiskRuntime {
    fn root(&self) -> gtk::Widget {
        self.controller.widget().clone().upcast()
    }

    fn update(&mut self, state: &BarItemState, context: &BarContext) {
        self.controller.emit(DiskInput::SetPlacement {
            edge: context.edge,
            region: context.region,
        });

        match state {
            BarItemState::Disk(DiskState::Ready(snapshot)) => {
                self.controller
                    .emit(DiskInput::SetSnapshot(snapshot.clone()));
            }
            BarItemState::Disk(DiskState::Unavailable) => {
                self.controller.emit(DiskInput::SetUnavailable);
            }
            _ => {}
        }
    }
}

pub(crate) struct DiskWidget;

pub(crate) static WIDGET: DiskWidget = DiskWidget;

impl BarWidget for DiskWidget {
    fn id(&self) -> &'static str {
        "disk"
    }

    fn build(
        &self,
        _instance: &WidgetInstance,
        context: &WidgetBuildContext,
    ) -> Box<dyn BarWidgetRuntime> {
        let controller = DiskComponent::build()
            .launch(DiskInit {
                edge: context.bar.edge,
                region: context.bar.region,
            })
            .detach();

        Box::new(DiskRuntime { controller })
    }

    fn initial_state(&self) -> Option<BarItemState> {
        Some(BarItemState::Disk(DiskState::Unavailable))
    }

    fn start(
        &self,
        sender: Sender<ShellMsg>,
        services: &crate::services::ShellServices,
    ) -> Option<relm4::JoinHandle<()>> {
        Some(service::start(
            sender,
            services.disk.clone(),
        ))
    }
}