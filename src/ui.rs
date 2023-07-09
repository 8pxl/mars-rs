use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};

#[derive(PartialEq)]
#[derive(Copy, Clone)]
pub enum State {
    Auton,
    Driver,
    Create
}

pub struct  Ui {
    state: State,
}

impl Ui{
    pub fn new() -> Ui {
        Ui {
            state: State::Driver
        }
    }

    pub fn render(&mut self) {
        draw_text("mode: ", 3.0, 20.0, 19.0, BLACK);
        draw_text(match self.state {
            State::Auton => "auton",
            State::Driver => "driver",
            State::Create => "create"
        }, 50.0, 20.0, 19.0, BLACK);

        if ! (self.state == State::Auton){
            if root_ui().button(Vec2{x: 3.0, y: 30.0}, "Auton") {
                self.state = State::Auton;
            }
        }

        if ! (self.state == State::Driver){
            if root_ui().button(Vec2{x: 3.0, y: 50.0}, "Driver") {
                self.state = State::Driver;
            }
        }

        if ! (self.state == State::Create){
            if root_ui().button(Vec2{x: 3.0, y: 70.0}, "Create") {
                self.state = State::Create;
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