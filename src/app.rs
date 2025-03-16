use std::sync::{Arc, Mutex};

#[cfg(not(target_arch = "wasm32"))]
use tokio::task;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen_futures::spawn_local;

use self::api::Manga;
use self::modals::MangaTable;

mod api;
mod modals;
use api::*;

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Debug)]
pub struct App {
    manga_table: Arc<Mutex<MangaTable>>,
}

impl App {
    /// Called once before the first frame.
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.

        let manga_table = if let Some(storage) = cc.storage {
            eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default()
        } else {
            MangaTable::default()
        };

        let manga_table = Arc::new(Mutex::new(manga_table));

        if !manga_table.lock().unwrap().manga_list.is_empty() {
            let manga_table_clone = manga_table.clone();
            #[cfg(not(target_arch = "wasm32"))]
            {
                let manga_list: Vec<_> = manga_table_clone
                    .lock()
                    .unwrap()
                    .manga_list
                    .iter()
                    .map(|manga| Arc::new(tokio::sync::Mutex::new(manga.clone())))
                    .collect();

                for manga_arc in manga_list {
                    let manga_clone = manga_arc.clone();
                    let manga_table_clone = manga_table.clone();

                    task::spawn(async move {
                        let mut manga = manga_clone.lock().await;

                        match Manga::fetch(&manga.id).await {
                            ResponseType::Ok(data) => {
                                manga.manga_data = data.first().cloned();
                            }
                            ResponseType::Err(err) => {
                                eprintln!(
                                    "Fetch record with id {} failed with status code {}: {}",
                                    manga.id, err.code, err.detail
                                );
                            }
                        }

                        let mut table_guard = manga_table_clone.lock().unwrap();

                        if let Some(original_manga) =
                            table_guard.manga_list.iter_mut().find(|m| m.id == manga.id)
                        {
                            *original_manga = manga.clone();
                        }
                    });
                }

                Self { manga_table }
            }

            #[cfg(target_arch = "wasm32")]
            {
                let manga_list: Vec<_> = manga_table_clone
                    .lock()
                    .unwrap()
                    .manga_list
                    .iter()
                    .map(|manga| Arc::new(Mutex::new(manga.clone())))
                    .collect();

                for manga_arc in manga_list {
                    let manga_clone = manga_arc.clone();
                    let manga_table_clone = manga_table.clone();

                    spawn_local(async move {
                        let mut manga = manga_clone.lock().unwrap().clone();

                        match Manga::fetch(&manga.id.clone()).await {
                            ResponseType::Ok(data) => {
                                manga.manga_data = data.first().cloned();
                            }
                            ResponseType::Err(err) => {
                                eprintln!(
                                    "Fetch record with id {} failed with status code {}: {}",
                                    manga.id, err.code, err.detail
                                );
                            }
                        }

                        let mut table_guard = manga_table_clone.lock().unwrap();

                        if let Some(original_manga) =
                            table_guard.manga_list.iter_mut().find(|m| m.id == manga.id)
                        {
                            *original_manga = manga.clone();
                        }
                    });
                }

                Self { manga_table }
            }
        } else {
            Self::default()
        }
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            manga_table: Arc::new(Mutex::new(MangaTable::default())),
        }
    }
}

impl eframe::App for App {
    /// Called by the frame work to save state before shutdown.
    fn save(&mut self, storage: &mut dyn eframe::Storage) {
        eframe::set_value(storage, eframe::APP_KEY, &self.manga_table);
    }

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        // Put your widgets into a `SidePanel`, `TopBottomPanel`, `CentralPanel`, `Window` or `Area`.
        // For inspiration and more examples, go to https://emilk.github.io/egui

        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_theme_preference_buttons(ui);
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.manga_table.lock().unwrap().show_inventory(ui);

            // Footer
            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
