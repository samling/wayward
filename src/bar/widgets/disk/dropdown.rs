use relm4::factory::FactoryVecDeque;
use relm4::gtk;
use relm4::gtk::prelude::*;
use relm4::prelude::*;

use crate::bar::{dropdown, layout::BarEdge, widget::BarRegion};

use super::row::{DiskRow, DiskRowInput};
use super::view_model::{DiskRowViewModel, DiskViewModel};

pub(super) struct DiskDropdown {
    edge: BarEdge,
    region: BarRegion,
    view_model: DiskViewModel,
    rows: FactoryVecDeque<DiskRow>,
    shell: Option<dropdown::DropdownPopover>,
}

pub(super) struct DiskDropdownInit {
    pub(super) edge: BarEdge,
    pub(super) region: BarRegion,
}

#[derive(Debug)]
pub(super) enum DiskDropdownInput {
    SetPlacement { edge: BarEdge, region: BarRegion },
    SetViewModel(DiskViewModel),
}

#[relm4::component(pub(super))]
impl SimpleComponent for DiskDropdown {
    type Init = DiskDropdownInit;
    type Input = DiskDropdownInput;
    type Output = ();

    view! {
        #[root]
        #[template]
        #[name = "shell"]
        dropdown::DropdownPopover(dropdown::DropdownPopoverInit {
            root_css_class: "disk-dropdown",
            content_css_class: "disk-dropdown-content",
            content_spacing: 8,
        }) {
            #[template_child]
            content {

                    gtk::Box {
                        add_css_class: "dropdown-header",
                        add_css_class: "disk-dropdown-header",
                        set_orientation: gtk::Orientation::Horizontal,
                        set_spacing: 8,
                        set_hexpand: true,

                        gtk::Label {
                            add_css_class: "dropdown-title",
                            add_css_class: "disk-dropdown-title",
                            set_halign: gtk::Align::Start,
                            set_hexpand: true,
                            set_text: "Disks",
                        },

                        gtk::Label {
                            add_css_class: "disk-dropdown-usage",
                            set_halign: gtk::Align::End,

                            #[watch]
                            set_text: &model.view_model.used_text,
                        },
                    },

                    #[local_ref]
                    list -> gtk::ListBox {
                        add_css_class: "disk-list",
                        set_hexpand: true,
                        set_selection_mode: gtk::SelectionMode::None,
                    },
            },
        },
    }

    fn init(
        init: Self::Init,
        root: Self::Root,
        _sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let list = gtk::ListBox::default();
        let rows = FactoryVecDeque::builder().launch(list.clone()).detach();

        let mut model = Self {
            edge: init.edge,
            region: init.region,
            view_model: DiskViewModel::unavailable(),
            rows,
            shell: None,
        };

        let widgets = view_output!();

        root.set_placement(init.edge, init.region);
        root.connect_revealer();
        model.shell = Some(root);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, msg: Self::Input, _sender: ComponentSender<Self>) {
        match msg {
            DiskDropdownInput::SetPlacement { edge, region } => {
                self.edge = edge;
                self.region = region;
                if let Some(shell) = &self.shell {
                    shell.set_placement(edge, region);
                }
            }
            DiskDropdownInput::SetViewModel(view_model) => {
                self.sync_rows(view_model.rows.clone());
                self.view_model = view_model;
            }
        }
    }
}

impl DiskDropdown {
    fn sync_rows(&mut self, rows: Vec<DiskRowViewModel>) {
        if !self.mounts_match(&rows) {
            let mut guard = self.rows.guard();
            guard.clear();

            for row in &rows {
                guard.push_back(row.clone());
            }
        }

        for (index, row) in rows.into_iter().enumerate() {
            self.rows.send(index, DiskRowInput::SetViewModel(row));
        }
    }

    fn mounts_match(&self, rows: &[DiskRowViewModel]) -> bool {
        self.rows.len() == rows.len()
            && self
                .rows
                .iter()
                .zip(rows)
                .all(|(existing, row)| existing.mount() == row.mount_text)
    }
}
