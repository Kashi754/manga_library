use crate::App;
use egui::Ui;
use egui_extras::{Column, TableBuilder};

#[derive(serde::Serialize, serde::Deserialize, Default, Debug)]
pub struct InventoryManga {
    title: String,
    num_volumes: u8,
    num_read: u8,
    is_favorite: bool,
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
                title: "2.5 Dimensional Seduction".to_string(),
                num_volumes: 5,
                num_read: 1,
                is_favorite: false,
            },
            InventoryManga {
                title: "Ai Yori Aoshi".to_string(),
                num_volumes: 17,
                num_read: 17,
                is_favorite: true,
            },
            InventoryManga {
                title: "After God".to_string(),
                num_volumes: 1,
                num_read: 0,
                is_favorite: false,
            },
            InventoryManga {
                title: "Fly Me to the Moon".to_string(),
                num_volumes: 19,
                num_read: 17,
                is_favorite: true,
            },
        ];

        Self {
            reversed: false,
            manga_list: manga_vec,
        }
    }
}

impl MangaTable {
    pub fn show_inventory(&mut self, ui: &mut Ui) {
        let text_height = egui::TextStyle::Body
            .resolve(ui.style())
            .size
            .max(ui.spacing().interact_size.y);

        let available_height = ui.available_height();

        TableBuilder::new(ui)
            .striped(true)
            .resizable(true)
            .cell_layout(egui::Layout::left_to_right(egui::Align::LEFT))
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
            .header(25.0, |mut header| {
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
                body.rows(text_height, self.manga_list.len(), |mut row| {
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

                    row.col(|ui| {
                        ui.label(&*manga.title);
                    });
                    row.col(|ui| {
                        ui.label(manga.num_volumes.to_string());
                    });
                    row.col(|ui| {
                        let value = if manga.num_read == manga.num_volumes {
                            "Read"
                        } else {
                            &format!("{} Unread", manga.num_volumes - manga.num_read)
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
