use relm4::factory::FactoryComponent;
use relm4::gtk;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use super::view_model::DiskRowViewModel;

pub(super) struct DiskRow {
    view_model: DiskRowViewModel,
}

#[derive(Debug)]
pub(super) enum DiskRowInput {
    SetViewModel(DiskRowViewModel),
}

#[relm4::factory(pub(super))]
impl FactoryComponent for DiskRow {
    type Init = DiskRowViewModel;
    type Input = DiskRowInput;
    type Output = ();
    type CommandOutput = ();
    type ParentWidget = gtk::ListBox;

    view! {
        #[root]
        gtk::Box {
            add_css_class: "disk-row",
            set_orientation: gtk::Orientation::Vertical,
            set_spacing: 4,
            set_hexpand: true,

            #[watch]
            set_tooltip_text: Some(&format!("{} free", self.view_model.free_text)),

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_spacing: 8,
                set_hexpand: true,

                gtk::Label {
                    add_css_class: "disk-row-mount",
                    set_halign: gtk::Align::Start,
                    set_hexpand: true,

                    #[watch]
                    set_text: &self.view_model.mount_text,
                },

                gtk::Label {
                    add_css_class: "disk-row-usage",
                    set_halign: gtk::Align::End,

                    #[watch]
                    set_text: &usage_text(&self.view_model),
                },
            },

            gtk::LevelBar {
                add_css_class: "disk-row-meter",
                set_hexpand: true,
                set_min_value: 0.0,
                set_max_value: 100.0,

                #[watch]
                set_value: self.view_model.meter_value,
            },
        }
    }

    fn init_model(
        view_model: Self::Init,
        _index: &DynamicIndex,
        _sender: FactorySender<Self>,
    ) -> Self {
        Self { view_model }
    }

    fn update(&mut self, msg: Self::Input, _sender: FactorySender<Self>) {
        match msg {
            DiskRowInput::SetViewModel(view_model) => self.view_model = view_model,
        }
    }
}

impl DiskRow {
    pub(super) fn mount(&self) -> &str {
        &self.view_model.mount_text
    }
}

fn usage_text(view_model: &DiskRowViewModel) -> String {
    format!("{} / {}", view_model.used_text, view_model.total_text)
}