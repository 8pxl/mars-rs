use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

use crate::hex;

#[derive(PartialEq)]
#[derive(Copy, Clone, Debug)]
pub enum CreateState {
    Save,
    Draw,
    Edit,
    Load,
}

#[derive(PartialEq)]
#[derive(Copy, Clone, Debug)]
pub enum State {
    Auton,
    Driver,
    Create(CreateState)
}

pub struct  Ui {
    state: State,
    pub path: Vec<(f32, f32)>,
    pub editing: i8,
    pub filePath: String,
}

impl Ui{
    pub fn new() -> Ui {
        Ui {
            state: State::Driver,
            path: Vec::new(),
            editing: -1,
            filePath: String::new(),
        }
    }

    pub fn render(&mut self) {
        draw_text("mode: ", 3.0, 20.0, 19.0, Color::from_hex(0x6F2232));
        draw_text(match self.state {
            State::Auton => "auton",
            State::Driver => "driver",
            State::Create(_) => "create"
        }, 50.0, 20.0, 19.0, hex!(0x6F2232));
        draw_line(3.0, 30.0, 90.0, 30.0, 3.0, hex!(0x221D2F));

        if ! (self.state == State::Auton){
            if root_ui().button(Vec2{x: 3.0, y: 40.0}, "Auton") {
                self.state = State::Auton;
            }
        }

        if ! (self.state == State::Driver){
            if root_ui().button(Vec2{x: 3.0, y: 60.0}, "Driver") {
                self.state = State::Driver;
            }
        }
        match self.state {
            State::Create(mode) => {
                self.paths(mode);
            },
            _ => {
                if root_ui().button(Vec2{x: 3.0, y: 80.0}, "Create") {
                    self.state = State::Create(CreateState::Draw);
                }
            }
        }
    }

    pub fn state(&self) -> State{
        self.state
    }
    
    pub fn set(&mut self, new: State) {
        self.state = new;
    }
}