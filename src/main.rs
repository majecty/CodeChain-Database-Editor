use imgui::*;

mod support;

fn main() {
    let system = support::init(file!());
    let mut file_dialog = FileDialog::new();
    system.main_loop(|_, ui| {
        Window::new(im_str!("Hello world"))
            .position([50.0, 50.0], Condition::FirstUseEver)
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Hello world!"));
                ui.text(im_str!("こんにちは世界！"));
                ui.text(im_str!("This...is...imgui-rs!"));
                ui.separator();
                let mouse_pos = ui.io().mouse_pos;
                ui.text(format!(
                    "Mouse Position: ({:.1},{:.1})",
                    mouse_pos[0], mouse_pos[1]
                ));
            });

        Window::new(im_str!("Open file"))
            .position([400.0, 50.0], Condition::FirstUseEver)
            .size([300.0, 100.0], Condition::FirstUseEver)
            .build(ui, || {
                ui.text(im_str!("Open a file"));
                if ui.button(im_str!("Open"), [100.0, 20.0]) {
                    println!("Open a file UI");
                    file_dialog.open();
                }
                file_dialog.display(ui);
            })
    });
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
