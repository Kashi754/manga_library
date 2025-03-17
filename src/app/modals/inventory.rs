use std::borrow::Cow;

use egui::Ui;
use egui_extras::{Column, TableBuilder};

use crate::app::api::{load_texture_from_url, Manga};

#[derive(serde::Serialize, serde::Deserialize, Default, Debug, Clone)]
pub struct InventoryManga {
    pub id: String,
    pub num_read: u8,
    pub is_favorite: bool,
    #[serde(skip)]
    pub manga_data: Option<Manga>,
}

#[derive(serde::Serialize, serde::Deserialize, Debug)]
pub struct MangaTable {
    pub reversed: bool,
    pub manga_list: Vec<InventoryManga>,
}

impl Default for MangaTable {
    fn default() -> Self {
        let manga_vec = vec![
            InventoryManga {
                id: "66".to_string(),
                num_read: 1,
                is_favorite: false,
                manga_data: None,
            },
            InventoryManga {
                id: "78".to_string(),
                num_read: 17,
                is_favorite: true,
                manga_data: None,
            },
            InventoryManga {
                id: "88".to_string(),
                num_read: 0,
                is_favorite: false,
                manga_data: None,
            },
            InventoryManga {
                id: "99".to_string(),
                num_read: 17,
                is_favorite: true,
                manga_data: None,
            },
        ];

        Self {
            reversed: false,
            manga_list: manga_vec,
        }
    }
}

impl MangaTable {
    pub fn show_inventory(&mut self, ui: &mut Ui, ctx: &egui::Context) {
        let available_height = ui.available_height();

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
            .column(Column::auto().at_least(30.0).clip(false).resizable(true))
            .column(
                Column::remainder()
                    .at_least(40.0)
                    .clip(true)
                    .resizable(true),
            )
            .column(Column::auto())
            .column(Column::auto())
            .column(Column::auto())
            .min_scrolled_height(0.0)
            .max_scroll_height(available_height)
            .header(30.0, |mut header| {
                header.col(|ui| {
                    ui.strong("");
                });
                header.col(|ui| {
                    egui::Sides::new().show(
                        ui,
                        |ui| {
                            ui.strong("Title");
                        },
                        |ui| {
                            self.reversed ^=
                                ui.button(if self.reversed { "⬆" } else { "⬇" }).clicked();
                        },
                    );
                });
                header.col(|ui| {
                    ui.strong("# Volumes");
                });
                header.col(|ui| {
                    ui.strong("# Read");
                });
                header.col(|ui| {
                    ui.label("");
                    ui.strong("❤");
                });
            })
            .body(|body| {
                body.rows(100.0, self.manga_list.len(), |mut row| {
                    let row_index = if self.reversed {
                        self.manga_list.len() - 1 - row.index()
                    } else {
                        row.index()
                    };

                    let inventory_size = self.manga_list.len();
                    let manga = self.manga_list.get_mut(row_index).unwrap_or_else(|| {
                        panic!(
                            "Failed to get index {} from manga list with length {}",
                            row_index, inventory_size
                        )
                    });

                    let manga_data = if let Some(manga) = &manga.manga_data {
                        manga
                    } else {
                        return;
                    };

                    row.col(|ui| {
                        let image_src = Cow::from(&*manga_data.attributes.poster_image.original);

                        #[cfg(not(target_arch = "wasm32"))]
                        {
                            let image = egui::Image::new(egui::ImageSource::Uri(image_src))
                                .max_height(100.0)
                                .maintain_aspect_ratio(true)
                                .corner_radius(5);

                            ui.label("");
                            ui.add(image);
                        }

                        #[cfg(target_arch = "wasm32")]
                        {
                            if let Some(texture) = load_texture_from_url(ctx, &image_src) {
                                ui.image(&texture);
                            }
                        }
                    });
                    row.col(|ui| {
                        ui.label(&*manga_data.attributes.canonical_title);
                    });
                    row.col(|ui| {
                        ui.columns(3, |cols| {
                            cols[0].centered_and_justified(|ui| ui.label(""));
                            cols[1].centered_and_justified(|ui| {
                                if let Some(num_volumes) = manga_data.attributes.volume_count {
                                    ui.label(num_volumes.to_string());
                                } else {
                                    ui.label("unk");
                                }
                            });
                            cols[0].centered_and_justified(|ui| ui.label(""));
                        });
                    });
                    row.col(|ui| {
                        let value = if let Some(num_volumes) = manga_data.attributes.volume_count {
                            let num_read = manga.num_read;
                            if num_read == num_volumes {
                                "Read".to_string()
                            } else if let Some(unread) = num_volumes.checked_sub(num_read) {
                                format!("{} Unread", unread)
                            } else {
                                "Read".to_string()
                            }
                        } else {
                            "unk".to_string()
                        };

                        ui.label(value);
                    });
                    row.col(|ui| {
                        ui.label("");
                        ui.checkbox(&mut manga.is_favorite, "");
                    });
                })
            });

        /*
        ui.horizontal(|ui| {
            ui.label("Write something: ");
            ui.text_edit_singleline(&mut app.label);
        });

        ui.add(egui::Slider::new(&mut app.value, 0.0..=10.0).text("value"));
        if ui.button("Increment").clicked() {
            app.value += 1.0;
        }

        ui.separator();

        ui.add(egui::github_link_file!(
            "https://github.com/emilk/eframe_template/blob/main/",
            "Source code."
        ));
        */
    }
}
