use crate::requestsender::Request;
use egui::{self, Ui};

pub struct Entry<'a> {
    left: &'a LeftSide<'a>,
}

pub struct LeftSide<'a> {
    topbar: &'a TopBar<'a>,
    collections: &'a Collections<'a>,
}

pub struct TopBar<'a> {
    ui: &'a mut Ui,
}

pub struct Collections<'a> {
    ui: &'a mut Ui,
    collections: Vec<Collection>,
}

pub struct Collection {
    id: i64,
    name: String,
    requests: Vec<Request>,
}

impl<'a> TopBar<'a> {
    pub fn new(ui: &'a mut Ui) -> Self {
        Self { ui: ui }
    }

    pub fn add_new_button(&mut self) {
        if self.ui.button("New").clicked() {
            // show a new tab in the right side
            println!("should show a new tab in the right side");
        }
    }
    pub fn add_import_button(&mut self) {
        if self.ui.button("Import").clicked() {
            // show import dialog
            println!("should show a import dialog");
        }
    }
}

// impl<'a> Entry<'a> {
//     pub fn new(ui: &'a mut Ui) -> Self {
//         Self {
//             ui:
//         }
//     }
// }
