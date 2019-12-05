use imgui::*;

mod db;
mod file;
mod support;

extern crate kvdb_rocksdb;

use kvdb_rocksdb::Database;

struct UIGlobal {
    file_name: ImString,
    kv_explorer: Vec<KVExplorer>,
}

fn main() {
    let mut ui_global = UIGlobal {
        file_name: ImString::with_capacity(1024),
        kv_explorer: Vec::new(),
    };

    let system = support::init(file!());
    system.main_loop(|_, ui| {
        Window::new(im_str!("Open DB directory"))
            .position([400.0, 50.0], Condition::FirstUseEver)
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.input_text(im_str!("Path"), &mut ui_global.file_name)
                    .build();
                if ui.button(im_str!("Open"), [0.0, 0.0]) {
                    let explorer = KVExplorer::new(ui_global.file_name.to_str());
                    ui_global.kv_explorer.push(explorer);
                }
            });
        for explorer in &mut ui_global.kv_explorer {
            explorer.display(ui);
        }
        ui_global.kv_explorer.retain(|v| !v.quit());
    });
}

struct KVExplorer {
    column: i32,
    key: ImString,
    value: String,
    kvdb: Option<Database>,
    db_path: String,
    quit: bool,
}

impl KVExplorer {
    pub fn new(file_name: &str) -> KVExplorer {
        let kvdb = db::open(file_name)
            .map_err(|err| {
                println!("open db at path: {}, {:?}", file_name, err);
                // show an error popup
                err
            })
            .ok();
        KVExplorer {
            column: 0,
            key: ImString::with_capacity(1024),
            value: String::new(),
            kvdb,
            db_path: file_name.to_owned(),
            quit: false,
        }
    }

    pub fn quit(&self) -> bool {
        self.quit
    }

    pub fn display(&mut self, ui: &mut imgui::Ui) {
        Window::new(im_str!("KVExplorer"))
            .position([50.0, 50.0], Condition::FirstUseEver)
            .size([500.0, 200.0], Condition::FirstUseEver)
            .build(ui, || {
                if self.kvdb.is_none() {
                    return;
                }
                let kvdb = self.kvdb.as_mut().unwrap();
                ui.text(&format!("DB: {}", self.db_path));
                ui.input_int(im_str!("column"), &mut self.column).build();
                ui.input_text(im_str!("Key"), &mut self.key).build();
                if ui.button(im_str!("Get Value"), [0.0, 0.0]) {
                    let v = kvdb.get(Some(self.column as u32), self.key.to_str().as_bytes());
                    self.value = format!("{:?}", v);
                }
                ui.text(&self.value);
                if ui.button(im_str!("Quit"), [0.0, 0.0]) {
                    self.quit = true;
                }
            });
    }
}
