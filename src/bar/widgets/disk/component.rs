use crate::bar::layout::BarEdge;
use crate::bar::widget::BarRegion;
use relm4::Controller;
use relm4::gtk;
use relm4::gtk::prelude::{OrientableExt, WidgetExt};
use relm4::prelude::*;
use wayle_sysinfo::SysinfoService;

pub(super) struct DiskComponent {
    view_model: DiskViewModel,
    edge: BarEdge,
    region: BarRegion,
    dropdown: Controller<DiskDropdown>,
}

pub(super) struct DiskInit {
    pub(super) edge: BarEdge,
    pub(super) region: BarRegion,
}

#[derive(Debug)]
pub(super) enum DiskInput {
    SetPlacement { edge: BarEdge, region: BarRegion },
    SetSnapshot(DiskSnapshot),
    SetUnavailable,
}

#[relm4::component(pub(super))]
impl SimpleComponent for DiskComponent {
    type Init = DiskInit;
    type Input = DiskInput;
    type Output = ();

    view! {
        gtk::MenuButton {
            set_always_show_arrow: false,
            set_cursor_from_name: Some("pointer"),
            add_css_class: "bar-item",
            add_css_class: "disk",
            add_css_class: "flat",

            #[watch]
            set_tooltip_text: model.view_model.tooltip_text.as_deref(),

            #[wrap(Some)]
            #[name = "content"]
            set_child = &gtk::Box {
                #[watch]
                set_orientation: model.edge.orientation(),

                gtk::Image {
                    add_css_class: "disk-icon",

                    #[watch]
                    set_icon_name: Some(model.view_model.icon_name),
                },

                #[name = "used"]
                gtk::Label {
                    add_css_class: "disk-used",

                    #[watch]
                    set_text: &model.view_model.used_text,
                }
            }
        }
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let dropdown = DiskDropdown::builder()
            .launch(DiskDropdownInit {
                edge: init.edge,
                region: init.region,
            })
            .detach();
        let model = Self {
            view_model: DiskViewModel::unavailable(),
            edge: init.edge,
            region: init.region,
            dropdown,
        };

        let widgets = view_output!();
        crate::bar::style::add_bar_item_content_classes(&widgets.content, "disk-content");
        crate::bar::style::configure_bar_label(&widgets.used);

        root.set_popover(Some(model.dropdown.widget().as_ref()));

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DiskInput::SetPlacement { edge, region } => {
                self.edge = edge,
                self.region = region,
                self.dropdown
                    .emit(DiskDropdownInput::SetPlacement { edge, region });
            }
            DiskInput::SetSnapshot(snapshot) => {
                let view_model = DiskViewModel::from_snapshot(&snapshot);

                self.dropdown.emit(DiskDropdownInput::SetSnapshot {
                    view_model: view_model.clone(),
                });

                self.view_model = view_model;
            }
            DiskInput::SetUnavailable => {
                let view_model = DiskViewModel::unavailable();
                self.dropdown
                    .emit(DiskDropdownInput::SetViewModel(view_model.clone()));
                self.view_model = view_model;
            }
        }
    }
}