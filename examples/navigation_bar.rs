#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::egui::{pos2, vec2, Color32, Id, Layout, Rect, Response, Sense, Ui};
use eframe::{egui, epi};
use egui_extras::RetainedImage;

struct SimpleApp {
    navigation_bar: Box<NavigationBar>,
}

impl Default for SimpleApp {
    fn default() -> Self {
        Self {
            navigation_bar: Box::new(NavigationBar::new(vec![
                RetainedImage::from_svg_bytes(
                    "settings_icon_218164.svg",
                    include_bytes!("../assets/images/settings_icon_218164.svg"),
                )
                .unwrap(),
                RetainedImage::from_svg_bytes(
                    "github_icon_217623.svg",
                    include_bytes!("../assets/images/github_icon_217623.svg"),
                )
                .unwrap(),
                RetainedImage::from_svg_bytes(
                    "door_simple_home_icon_217593.svg",
                    include_bytes!("../assets/images/door_simple_home_icon_217593.svg"),
                )
                .unwrap(),
                RetainedImage::from_svg_bytes(
                    "safari_icon_218169.svg",
                    include_bytes!("../assets/images/safari_icon_218169.svg"),
                )
                .unwrap(),
                RetainedImage::from_svg_bytes(
                    "instagram_icon_217588.svg",
                    include_bytes!("../assets/images/instagram_icon_217588.svg"),
                )
                .unwrap(),
            ])),
        }
    }
}

impl epi::App for SimpleApp {
    fn name(&self) -> &str {
        "SimpleApp"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();
            self.navigation_bar.show(ui);
        });
    }
}

pub struct NavigationBar {
    index: f32,
    circle_size: f32,
    icon_size: Option<f32>,
    circle_margin: f32,
    height: f32,
    background_color: Option<Color32>,
    front_color: Color32,
    animation_time: f32,
    is_animation: bool,
    sense: Sense,
    svgs: Vec<RetainedImage>,
}

impl Default for NavigationBar {
    fn default() -> Self {
        Self {
            index: 1.,
            circle_size: 24.,
            circle_margin: 6.,
            icon_size: None,
            height: 50.,
            front_color: Color32::WHITE,
            background_color: None,
            animation_time: 0.1,
            is_animation: true,
            sense: Sense::click(),
            svgs: vec![],
        }
    }
}

impl NavigationBar {
    pub fn new(svgs: Vec<RetainedImage>) -> Self {
        Self {
            svgs,
            ..Default::default()
        }
    }

    pub fn custom_new(
        init_index: usize,
        circle_size: f32,
        circle_margin: f32,
        icon_size: Option<f32>,
        height: f32,
        front_color: Color32,
        background_color: Option<Color32>,
        animation_time: f32,
        is_animation: bool,
        sense: Sense,
        svgs: Vec<RetainedImage>,
    ) -> Self {
        Self {
            index: init_index as f32,
            circle_size,
            circle_margin,
            icon_size,
            height,
            front_color,
            background_color,
            animation_time,
            is_animation,
            sense,
            svgs,
        }
    }

    pub fn show(&mut self, ui: &mut Ui) -> Vec<Response> {
        if self.is_animation {
            ui.ctx().request_repaint();
        }

        let window_rect = ui.ctx().available_rect();

        let min_y = window_rect.max.y - self.height;

        let navigation_bar_rect =
            Rect::from_min_max(pos2(window_rect.min.x, min_y), window_rect.max);

        // circle size
        let outer_radius = self.circle_size;
        let inner_radius = self.circle_size - self.circle_margin;
        let item_num = self.svgs.len();

        // move up circle
        let circle_center_x = if self.is_animation {
            ui.ctx().animate_value_with_time(
                Id::new("navigation_bar_circle"),
                window_rect.width() / (item_num + 1) as f32 * self.index,
                self.animation_time,
            )
        } else {
            window_rect.width() / (item_num + 1) as f32 * self.index
        };

        // Background Color
        let bg_color = if let Some(color) = self.background_color {
            color
        } else {
            ui.visuals().faint_bg_color
        };

        let circle_center = pos2(circle_center_x, min_y);

        // Painter -> bar, outer_circle, inner_circle
        let window_rounding = ui.visuals().window_rounding.ne;
        ui.painter()
            .rect_filled(navigation_bar_rect, window_rounding, self.front_color);

        ui.painter()
            .circle_filled(circle_center, outer_radius, bg_color);
        ui.painter()
            .circle_filled(circle_center, inner_radius, self.front_color);

        let mut some_response: Vec<Response> = Vec::new();

        // それぞれのレスポンス領域
        for i in 1..item_num + 1 {
            let i = i as f32;

            let icon_center_x = window_rect.width() / (item_num + 1) as f32 * i;

            let icon_center_y = if self.index == i {
                min_y
            } else {
                min_y + self.height / 2.
            };

            let icon_center_y = if self.is_animation {
                ui.ctx().animate_value_with_time(
                    Id::new(format!("icon{}", i)),
                    icon_center_y,
                    self.animation_time,
                )
            } else {
                icon_center_y
            };

            let icon_size = if let Some(size) = self.icon_size {
                size
            } else {
                self.circle_size
            };

            let icon_frame_rect = Rect::from_center_size(
                pos2(icon_center_x, icon_center_y),
                vec2(icon_size, icon_size),
            );

            let mut icon_ui = ui.child_ui(icon_frame_rect, Layout::default());
            self.svgs[(i - 1.) as usize].show_size(&mut icon_ui, vec2(icon_size, icon_size));

            // Icon response area
            let min = pos2(icon_center_x - (icon_size + 4.), min_y - icon_size);
            let max = pos2(icon_center_x + (icon_size + 4.), icon_frame_rect.max.y);

            let response_rect = Rect::from_min_max(min, max);
            let response = ui.allocate_rect(response_rect, self.sense);
            if response.hovered() {
                self.index = i;
            }

            some_response.push(response);
        }

        some_response
    }
}

fn main() {
    let app = SimpleApp::default();
    let options = eframe::NativeOptions {
        initial_window_pos: Some(egui::pos2(920., 0.)),
        initial_window_size: Some(egui::vec2(600., 800.)),
        ..Default::default()
    };
    eframe::run_native(Box::new(app), options);
}
