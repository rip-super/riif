use eframe::egui::{self, IconData, SliderClamping, load::SizedTexture};
use image::RgbaImage;
use std::cell::RefCell;
use std::path::Path;

fn human_readable_file_size(bytes: u64) -> String {
    const KB: f64 = 1024.0;
    const MB: f64 = KB * 1024.0;
    const GB: f64 = MB * 1024.0;

    let b = bytes as f64;
    if b >= GB {
        format!("{:.2} GB", b / GB)
    } else if b >= MB {
        format!("{:.2} MB", b / MB)
    } else if b >= KB {
        format!("{:.2} KB", b / KB)
    } else {
        format!("{} B", bytes)
    }
}

fn load_icon() -> IconData {
    let icon_bytes = include_bytes!("../../icon.png");
    let image = image::load_from_memory(icon_bytes)
        .expect("Failed to load icon")
        .to_rgba8();
    let (width, height) = image.dimensions();

    IconData {
        rgba: image.into_raw(),
        width,
        height,
    }
}

pub fn view(img: RgbaImage, file_name: String) -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder {
            icon: Some(std::sync::Arc::new(load_icon())),
            ..Default::default()
        }
        .with_position(egui::pos2(f32::NAN, f32::NAN)),
        centered: true,
        ..Default::default()
    };

    let texture = RefCell::new(None);
    let zoom = RefCell::new(None::<f32>);

    eframe::run_simple_native("RIIF Image Viewer", options, move |ctx, _frame| {
        let mut scroll_delta = 0.0;
        let events: Vec<_> = ctx.input(|i| i.raw.events.clone());
        for event in events.iter() {
            if let egui::Event::MouseWheel { delta, .. } = event {
                scroll_delta += delta.y;
            }
        }

        if scroll_delta.abs() > 0.0 {
            let mut z = zoom.borrow_mut();
            if let Some(current) = z.as_mut() {
                let sensitivity = 0.1;
                *current = (*current * (1.0 + sensitivity * scroll_delta.signum())).clamp(0.1, 3.0);
            }
        }

        egui::CentralPanel::default().show(ctx, |ui| {
            let path = Path::new(&file_name);
            let filename = path.file_name().unwrap().to_string_lossy();
            let file_size = std::fs::metadata(path).map(|m| m.len()).unwrap_or(0);

            let (orig_width, orig_height) = img.dimensions();

            let avail_size = ui.available_size();

            let max_fit_scale = {
                let scale_x = avail_size.x / orig_width as f32;
                let scale_y = avail_size.y / orig_height as f32;
                scale_x.min(scale_y).min(1.0)
            };

            if zoom.borrow().is_none() {
                *zoom.borrow_mut() = Some(max_fit_scale);
            }

            let zoom_val = zoom.borrow().unwrap();
            *zoom.borrow_mut() = Some(zoom_val);

            let display_width = (orig_width as f32 * zoom_val) as usize;
            let display_height = (orig_height as f32 * zoom_val) as usize;

            if texture.borrow().is_none() {
                let color_img = egui::ColorImage::from_rgba_unmultiplied(
                    [orig_width as usize, orig_height as usize],
                    img.as_flat_samples().as_slice(),
                );
                *texture.borrow_mut() =
                    Some(ctx.load_texture("image", color_img, Default::default()));
            }

            if let Some(texture) = &*texture.borrow() {
                ui.vertical_centered(|ui| {
                    ui.heading(filename.to_string());
                    ui.add_space(8.0);

                    egui::ScrollArea::both()
                        .auto_shrink([false; 2])
                        .max_height(ui.available_height() - 100.0)
                        .show(ui, |ui| {
                            let available_size = ui.available_size();

                            let remaining_width = available_size.x - display_width as f32;
                            let remaining_height = available_size.y - display_height as f32;

                            let offset_x = (remaining_width / 2.0).max(0.0);
                            let offset_y = (remaining_height / 2.0).max(0.0);

                            ui.add_space(offset_y);
                            ui.horizontal(|ui| {
                                ui.add_space(offset_x);
                                ui.image(SizedTexture::new(
                                    texture,
                                    [display_width as f32, display_height as f32],
                                ));
                            });
                        });

                    ui.separator();

                    ui.vertical_centered(|ui| {
                        ui.label(format!(
                            "Dimensions: {}x{}     •     File Size: {}",
                            orig_width,
                            orig_height,
                            human_readable_file_size(file_size),
                        ));
                        ui.add_space(16.0);
                        ui.horizontal_centered(|ui| {
                            ui.add_space(200.0);
                            ui.label(format!("Zoom: {:.0}%", zoom_val * 100.0));
                            ui.add(
                                egui::Slider::new(zoom.borrow_mut().as_mut().unwrap(), 0.1..=3.0)
                                    .clamping(SliderClamping::Always)
                                    .show_value(false),
                            );
                            ui.add_space(15.0);
                            if ui.button("Reset").clicked() {
                                *zoom.borrow_mut() = Some(max_fit_scale.clamp(0.1, 3.0));
                            }
                        });
                    });
                });
            }
        });
    })
}
