use crate::requestsender::Request;
use egui::{self, Ui};

pub struct Entry<'a> {
    left: &'a LeftSide,
}

pub struct LeftSide {
    topbar: TopBar,
    collections: Collections,
}

pub struct TopBar {}

pub struct Collections {
    collections: Vec<Collection>,
}

pub struct Collection {
    id: i64,
    name: String,
    requests: Vec<Request>,
}

impl LeftSide {
    pub fn new() -> Self {
        Self {
            topbar: TopBar::new(),
            collections: Collections::new(),
        }
    }
}

impl TopBar {
    pub fn new() -> Self {
        Self {}
    }

    pub fn display(&mut self) -> Box<dyn Fn(&mut Ui)> {
        Box::new(|ui: &mut Ui| {
            if ui.button("New").clicked() {
                // show a new tab in the right side
                println!("should show a new tab in the right side");
            }

            if ui.button("Import").clicked() {
                // show import dialog
                println!("should show a import dialog");
            }
        })
    }
}

impl Collections {
    pub fn new() -> Self {
        Self {
            collections: vec![],
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
