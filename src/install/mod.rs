pub mod wizard_step1;
pub mod wizard_step2;
pub mod wizard_step3;
pub mod installer;
pub mod utils;

use anyhow::Result;

pub fn run_install_wizard() -> Result<()> {
    let native_options = eframe::NativeOptions::default();
    let _ = eframe::run_native(
        "図書システムをインストール",
        native_options,
        Box::new(|cc| Box::new(InstallWizardApp::new(cc))),
    );
    Ok(())
}

pub struct InstallWizardApp {
    current_step: i32,
    selected_path: String,
    use_auto_path: bool,
    installing: bool,
    installation_complete: bool,
}

impl InstallWizardApp {
    pub fn new(_cc: &eframe::CreationContext) -> Self {
        Self {
            current_step: 1,
            selected_path: String::new(),
            use_auto_path: true,
            installing: false,
            installation_complete: false,
        }
    }
}

impl eframe::App for InstallWizardApp {
    fn update(&mut self, ctx: &egui::Context, frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.current_step {
                1 => self.step1_ui(ui),
                2 => self.step2_ui(ui),
                3 => self.step3_ui(ui, frame),
                _ => {}
            }
        });
    }
}

impl InstallWizardApp {
    fn step1_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("図書システムをインストール");
            ui.separator();
            ui.label("お使いのコンピューターに図書システムをインストールします。");
            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() - 200.0);
                if ui.button("キャンセル [c]").clicked() {
                    std::process::exit(0);
                }
                if ui.button("次へ [n]").clicked() {
                    self.current_step = 2;
                }
            });
        });

        if ui.input(|i| i.key_pressed(egui::Key::N)) {
            self.current_step = 2;
        }
        if ui.input(|i| i.key_pressed(egui::Key::C)) {
            std::process::exit(0);
        }
    }

    fn step2_ui(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            ui.heading("図書システムをインストール");
            ui.separator();
            ui.label("インストール場所を指定します。");
            ui.add_space(10.0);

            ui.vertical(|ui| {
                if ui.radio(self.use_auto_path, "推奨：自動指定").clicked() {
                    self.use_auto_path = true;
                }
                if let Ok(path) = crate::config::get_install_path_auto() {
                    ui.label(format!("インストール先: {}", path.display()));
                }

                ui.separator();

                if ui.radio(!self.use_auto_path, "非推奨：自分で指定").clicked() {
                    self.use_auto_path = false;
                }

                if !self.use_auto_path {
                    ui.text_edit_singleline(&mut self.selected_path);
                    if ui.button("参照...").clicked() {
                    }
                }
            });

            ui.add_space(20.0);

            ui.horizontal(|ui| {
                ui.add_space(ui.available_width() - 200.0);
                if ui.button("戻る [b]").clicked() {
                    self.current_step = 1;
                }
                if ui.button("次へ [n]").clicked() {
                    self.current_step = 3;
                }
            });
        });

        if ui.input(|i| i.key_pressed(egui::Key::B)) {
            self.current_step = 1;
        }
        if ui.input(|i| i.key_pressed(egui::Key::N)) {
            self.current_step = 3;
        }
    }

    fn step3_ui(&mut self, ui: &mut egui::Ui, _frame: &mut eframe::Frame) {
        ui.vertical(|ui| {
            ui.heading("インストール中...");
            ui.separator();

            if !self.installing && !self.installation_complete {
                self.installing = true;
                self.perform_installation();
            }

            if self.installing {
                ui.label("インストール処理を実行中です...");
                ui.add_space(10.0);

                let loading_chars = ["⠋", "⠙", "⠹", "⠸", "⠼", "⠴", "⠦", "⠧", "⠇", "⠏"];
                let frame_count = (ui.ctx().frame_nr() / 10) as usize;
                let spinner = loading_chars[frame_count % loading_chars.len()];
                ui.label(format!("{} インストール中...", spinner));
                ui.ctx().request_repaint();
            }

            if self.installation_complete {
                ui.label("✓ インストールが完了しました！");
                ui.add_space(20.0);

                if ui.button("完了").clicked() {
                    std::process::exit(0);
                }
            }
        });
    }

    fn perform_installation(&mut self) {
        let install_path = if self.use_auto_path {
            crate::config::get_install_path_auto().unwrap()
        } else {
            std::path::PathBuf::from(self.selected_path.clone())
        };

        if let Err(e) = installer::perform_installation(&install_path) {
            log::error!("Installation failed: {}", e);
            self.installing = false;
            return;
        }

        if let Err(e) = create_desktop_shortcut(&install_path) {
            log::warn!("Failed to create desktop shortcut: {}", e);
        }

        self.installing = false;
        self.installation_complete = true;
    }
}

fn create_desktop_shortcut(install_path: &std::path::Path) -> Result<()> {
    let desktop = crate::config::get_desktop_path()?;
    let binary_path = crate::config::get_binary_path(install_path);

    #[cfg(target_os = "windows")]
    {
        use std::fs;
        let shortcut_path = desktop.join("Mylibrary.lnk");
        log::info!("Shortcut would be created at: {}", shortcut_path.display());
    }

    #[cfg(target_os = "linux")]
    {
        use std::fs;
        let desktop_file = desktop.join("my-library.desktop");
        let content = format!(
            "[Desktop Entry]\nType=Application\nName=Mylibrary\nExec=\"{}\"\nIcon=\n",
            binary_path.display()
        );
        fs::write(&desktop_file, content)?;
        log::info!("Desktop shortcut created at: {}", desktop_file.display());
    }

    Ok(())
}