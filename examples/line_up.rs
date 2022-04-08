#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, epaint, epi};
use egui::{pos2, Color32, Pos2, Stroke};
use rand::Rng;

// Line Length
const LINE_LENGTH_RANGE_MIN: f32 = 100.0;
const LINE_LENGTH_RANGE_MAX: f32 = 400.0;
// Speed
const SPEED_RANGE_MIN: f32 = 100.0;
const SPEED_RANGE_MAX: f32 = 200.0;
// Stroke Width
const STROKE_WIDTH_RANGE_MIN: f32 = 0.5;
const STROKE_WIDTH_RANGE_MAX: f32 = 3.0;
//
const PROBABILITY_GENERATION: f64 = 0.05;

const COLORS: [Color32; 5] = [
    Color32::WHITE,
    Color32::BLUE,
    Color32::YELLOW,
    Color32::RED,
    Color32::GREEN,
];

struct Range {
    min: f32,
    max: f32,
}

struct LineUpApp {
    lines: Vec<Line>,
    line_length_range: Range,
    speed_range: Range,
    stroke_width_range: Range,
    probability_generation: f64,
}

impl Default for LineUpApp {
    fn default() -> Self {
        Self {
            lines: Vec::new(),
            line_length_range: Range {
                min: LINE_LENGTH_RANGE_MIN,
                max: LINE_LENGTH_RANGE_MAX,
            },
            speed_range: Range {
                min: SPEED_RANGE_MIN,
                max: SPEED_RANGE_MAX,
            },
            stroke_width_range: Range {
                min: STROKE_WIDTH_RANGE_MIN,
                max: STROKE_WIDTH_RANGE_MAX,
            },
            probability_generation: PROBABILITY_GENERATION,
        }
    }
}

impl epi::App for LineUpApp {
    fn name(&self) -> &str {
        "LineUp animation"
    }

    fn update(&mut self, ctx: &egui::Context, _frame: &epi::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ctx.request_repaint();

            let time = ui.input().time;
            let window_size = ctx.available_rect().max;

            // ラインの生成
            if rand::thread_rng().gen_bool(self.probability_generation) {
                // if self.lines.is_empty() {
                let x = rand::thread_rng().gen_range(0.0..window_size.x);
                let line_length = rand::thread_rng()
                    .gen_range(self.line_length_range.min..self.line_length_range.max);
                let speed =
                    rand::thread_rng().gen_range(self.speed_range.min..self.speed_range.max);
                let stroke_width = rand::thread_rng()
                    .gen_range(self.stroke_width_range.min..self.stroke_width_range.max);
                let color_index = rand::thread_rng().gen_range(0..COLORS.len());

                let stroke = egui::Stroke::new(stroke_width, COLORS[color_index]);

                let line = Line::new(pos2(x, window_size.x), time, line_length, stroke, speed);
                self.lines.push(line);
            }

            let mut remove_index: Option<usize> = None;
            for (index, line) in self.lines.iter_mut().enumerate() {
                if !line.show(ui, time) {
                    remove_index = Some(index);
                }
            }

            if let Some(index) = remove_index {
                self.lines.remove(index);
            }
        });
    }
}

#[derive(Debug)]
struct Line {
    start_pos: Pos2,
    start_time: f64,
    line_length: f32,
    stroke: Stroke,
    speed: f32,
}

impl Line {
    fn new(start_pos: Pos2, start_time: f64, line_length: f32, stroke: Stroke, speed: f32) -> Self {
        Self {
            start_pos,
            start_time,
            line_length,
            stroke,
            speed,
        }
    }

    // true:
    // false: destory
    fn show(&mut self, ui: &egui::Ui, now_time: f64) -> bool {
        let time = (now_time - self.start_time) as f32;

        let p: Vec<Pos2> = vec![
            pos2(self.start_pos.x, self.start_pos.y - time * self.speed),
            pos2(
                self.start_pos.x,
                self.start_pos.y + self.line_length - time as f32 * self.speed,
            ),
        ];

        let p_last = p.last().unwrap().clone();

        ui.painter().add(epaint::PathShape::line(p, self.stroke));

        if p_last.y < 0.0 {
            false
        } else {
            true
        }
    }
}

fn main() {
    let app = LineUpApp::default();
    let options = eframe::NativeOptions {
        initial_window_pos: Some(egui::pos2(920., 0.)),
        initial_window_size: Some(egui::vec2(500., 500.)),
        ..Default::default()
    };

    eframe::run_native(Box::new(app), options);
}
