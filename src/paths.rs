use crate::ui;
use crate::util::dist;
use macroquad::prelude::*;
use macroquad::ui::{root_ui};
use macroquad::hash;
use macroquad::ui::widgets;
use std::fs;

impl ui::Ui {
    pub fn paths(&mut self, mode: ui::CreateState) {
        let leftButton = is_mouse_button_pressed(MouseButton::Left);
        let mut createValid = true;
        let mut prev;

        if self.path.len() > 0 {
            prev = self.path[0];
            for i in 0..self.path.len() {
                draw_line(prev.0, prev.1, self.path[i].0, self.path[i].1, 3.0, BLACK);
                prev = self.path[i];

                if dist(mouse_position(), self.path[i]) < 8.0 || self.editing == i as i8 {
                    draw_circle(self.path[i].0, self.path[i].1, 8.0, RED);
                    createValid = false;

                    if is_mouse_button_down(MouseButton::Left) {
                        self.path[i] = mouse_position().clone();
                        self.editing = i as i8;
                    }

                    else {
                        self.editing = -1;
                    }

                    if is_key_pressed(KeyCode::Backspace) {
                        self.path.remove(i);
                        break;
                    }

                }
                else {draw_circle(self.path[i].0, self.path[i].1, 4.0, RED)}
            }
        }

        if mode == ui::CreateState::Save {
            createValid = false;
            println!("{:?}", self.path);
            self.set(ui::State::Create(ui::CreateState::Draw));
        }

        if ! (mode == (ui::CreateState::Draw)) {
            if root_ui().button(Vec2{x: 130.0, y: 8.0}, "New Path") {
                self.set(ui::State::Create(ui::CreateState::Draw));
            }

            else if leftButton && createValid{
                self.path.push(mouse_position());
            }
        }

        if ! (mode == ui::CreateState::Edit) {
            if root_ui().button(Vec2{x: 193.0, y: 8.0}, "Edit Path") {
                self.set(ui::State::Create(ui::CreateState::Edit));
            }
        }

        if !(mode == ui::CreateState::Load) {
            if root_ui().button(Vec2{x: 333.0, y: 8.0}, "Load Path") {
                self.set(ui::State::Create(ui::CreateState::Load));
            }
        }

        if !(mode == ui::CreateState::Save) {
            if root_ui().button(Vec2{x: 263.0, y: 8.0}, "Save Path") {
                self.set(ui::State::Create(ui::CreateState::Save));
            }
        }
    }
}

pub struct Bezier {
    pub points: (f32, f32, f32, f32)
}

impl Bezier {
    pub fn length(&self) -> f32 {
        0.0
    }

    pub fn curvature(&self, t: f32) -> f32 {
        0.0
    }
}