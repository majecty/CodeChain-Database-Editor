use imgui::*;

mod db;
mod support;

extern crate kvdb_rocksdb;

use kvdb_rocksdb::Database;

struct UIGlobal {
    file_name: ImString,
    kvdb: Option<Database>,
    kv_explorer: KVExplorerDialog,
}

fn main() {
    // let kvdb = db::open("/Users/juhyeong/code/kodebox/codechain/db").expect("main");
    // db::print_all(&kvdb);
    // db::print_tendermint_backup(&kvdb);

    let mut ui_global = UIGlobal {
        file_name: ImString::with_capacity(1024),
        kvdb: None,
        kv_explorer: KVExplorerDialog::new(),
    };

    let system = support::init(file!());
    let mut file_dialog = FileDialog::new();
    system.main_loop(|_, ui| {
        // Window::new(im_str!("Hello world"))
        //     .position([50.0, 50.0], Condition::FirstUseEver)
        //     .size([300.0, 100.0], Condition::FirstUseEver)
        //     .build(ui, || {
        //         ui.text(im_str!("Hello world!"));
        //         ui.text(im_str!("こんにちは世界！"));
        //         ui.text(im_str!("This...is...imgui-rs!"));
        //         ui.separator();
        //         let mouse_pos = ui.io().mouse_pos;
        //         ui.text(format!(
        //             "Mouse Position: ({:.1},{:.1})",
        //             mouse_pos[0], mouse_pos[1]
        //         ));
        //     });

        // Window::new(im_str!("Open file"))
        //     .position([400.0, 50.0], Condition::FirstUseEver)
        //     .size([300.0, 100.0], Condition::FirstUseEver)
        //     .build(ui, || {
        //         ui.text(im_str!("Open a file"));
        //         if ui.button(im_str!("Open"), [100.0, 20.0]) {
        //             println!("Open a file UI");
        //             file_dialog.open();
        //         }
        //         file_dialog.display(ui);
        //     });

        Window::new(im_str!("Open DB directory"))
            .position([400.0, 50.0], Condition::FirstUseEver)
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.input_text(im_str!("Path"), &mut ui_global.file_name)
                    .build();
                if ui.button(im_str!("Open"), [0.0, 0.0]) {
                    let kvdb = db::open(ui_global.file_name.to_str())
                        .map_err(|err| {
                            println!(
                                "open db at path: {}, {:?}",
                                ui_global.file_name.to_str(),
                                err
                            );
                            err
                        })
                        .ok();
                    ui_global.kvdb = kvdb;
                    if ui_global.kvdb.is_some() {
                        ui_global.kv_explorer.open();
                    }
                }
                ui_global.kv_explorer.dsiplay(ui, &mut ui_global.kvdb);
            });
    });
}

struct KVExplorerDialog {
    open_flag: bool,
    column: i32,
    key: ImString,
    value: String,
}

impl KVExplorerDialog {
    pub fn new() -> KVExplorerDialog {
        KVExplorerDialog {
            open_flag: false,
            column: 0,
            key: ImString::with_capacity(1024),
            value: String::new(),
        }
    }

    pub fn open(&mut self) {
        self.open_flag = true;
    }

    pub fn dsiplay(&mut self, ui: &Ui, kvdb: &mut Option<Database>) {
        if self.open_flag {
            self.open_flag = false;
            ui.open_popup(im_str!("KVExplorer"));
        }
        if kvdb.is_none() {
            return;
        }
        let kvdb = kvdb.as_mut().unwrap();
        ui.popup_modal(im_str!("KVExplorer")).build(|| {
            ui.input_int(im_str!("column"), &mut self.column).build();
            ui.input_text(im_str!("Key"), &mut self.key).build();
            if ui.button(im_str!("Get Value"), [0.0, 0.0]) {
                let v = kvdb.get(Some(self.column as u32), self.key.to_str().as_bytes());
                self.value = format!("{:?}", v);
            }
            ui.text(&self.value);

            if ui.button(im_str!("Quit"), [0.0, 0.0]) {
                ui.close_current_popup();
            }
        })
    }
}

struct FileDialog {
    open_flag: bool,
    file_name: ImString,
}

impl FileDialog {
    pub fn new() -> FileDialog {
        FileDialog {
            open_flag: false,
            file_name: ImString::with_capacity(1024),
        }
    }

    pub fn open(&mut self) {
        self.open_flag = true;
    }

    pub fn display(&mut self, ui: &Ui) {
        if self.open_flag {
            self.open_flag = false;
            ui.open_popup(im_str!("File Open"));
        }
        ui.popup_modal(im_str!("File Open")).build(|| {
            ui.text(im_str!("Absolute Path"));
            if ui.input_text(im_str!("Path"), &mut self.file_name).build() {
                println!("Input text true {}", self.file_name.to_str());
            }
            if ui.button(im_str!("OK"), [0.0, 0.0]) {
                ui.close_current_popup();
            }
        })
    }
}
