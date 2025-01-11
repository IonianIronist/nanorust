extern crate sdl2;

use rand::prelude::*;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use std::f32::consts::PI;
use std::fs::File;
use std::io::prelude::*;
use std::time::Duration;

pub struct InformationMolecule {
    speed: f32,
    direction: f32,
    rect: Rect,
    residence_time: u32,
    stopped: bool,
    released: bool,
    stimulus: u32,
    tumble_timer: i32,
}

impl InformationMolecule {
    pub fn new(x: i32, y: i32, w: u32, h: u32, speed: f32) -> Self {
        let direction = thread_rng().gen_range(0.0..2.0 * PI);
        InformationMolecule {
            speed,
            direction,
            rect: Rect::new(x, y, w, h),
            residence_time: 60,
            stopped: false,
            released: false,
            stimulus: 0,
            tumble_timer: 0,
        }
    }

    pub fn tumble(&mut self, current_stimulus: u32) {
        if current_stimulus > self.stimulus {
            self.tumble_timer = 15;
        }
        if self.tumble_timer <= 0 {
            self.direction += thread_rng().gen_range(0.0..2.0 * PI);
        }
        self.tumble_timer -= 1;
        self.stimulus = current_stimulus;
    }

    pub fn run(&mut self) {
        if !self.stopped {
            let dx = (self.speed as f32 * self.direction.cos()) as i32;
            let dy = (self.speed as f32 * self.direction.sin()) as i32;
            self.rect.set_x(self.rect.x() + dx);
            self.rect.set_y(self.rect.y() + dy);
            if self.rect.x > 2000
                || self.rect.y > 2000
                || self.rect.x < -2000
                || self.rect.y < -2000
            {
                self.released = true;
            }
        } else {
            self.residence_time -= 1;
            if self.residence_time == 0 {
                self.released = true;
            }
        }
    }
}

pub struct ChemotacticMolecule {
    speed: f32,
    direction: f32,
    rect: Rect,
}

impl ChemotacticMolecule {
    pub fn new(x: i32, y: i32, w: u32, h: u32, speed: f32) -> Self {
        let direction = thread_rng().gen_range(0.0..2.0 * PI);
        ChemotacticMolecule {
            speed,
            direction,
            rect: Rect::new(x, y, w, h),
        }
    }

    pub fn tumble(&mut self) {
        self.direction += thread_rng().gen_range(0.0..2.0 * PI);
    }

    pub fn run(&mut self) {
        let dx = (self.speed as f32 * self.direction.cos()) as i32;
        let dy = (self.speed as f32 * self.direction.sin()) as i32;
        self.rect.set_x(self.rect.x() + dx);
        self.rect.set_y(self.rect.y() + dy);
    }
}

pub struct Transmitter {
    transmission_size: usize,
    rect: Rect,
}

impl Transmitter {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Transmitter {
            transmission_size: 800,
            rect: Rect::new(x, y, w, h),
        }
    }

    pub fn transmit(&self) -> Vec<InformationMolecule> {
        let mut vec = Vec::with_capacity(self.transmission_size);
        for _ in 0..self.transmission_size {
            vec.push(InformationMolecule::new(
                self.rect.x(),
                self.rect.y(),
                5,
                5,
                12.2,
            ));
        }
        vec
    }
}

pub struct Receiver {
    chemotatic_molecules_count: usize,
    max_receptors: usize,
    free_receptors: usize,
    rect: Rect,
    timer: u128,
}

impl Receiver {
    pub fn new(x: i32, y: i32, w: u32, h: u32) -> Self {
        Receiver {
            chemotatic_molecules_count: 1000,
            max_receptors: 20,
            free_receptors: 20,
            rect: Rect::new(x, y, w, h),
            timer: 0,
        }
    }

    pub fn receive(&mut self) -> bool {
        if self.free_receptors > 0 {
            self.free_receptors -= 1;
            return true;
        }
        false
    }

    pub fn release(&mut self) -> bool {
        if self.free_receptors < self.max_receptors {
            self.free_receptors += 1;
            return true;
        }
        false
    }

    pub fn set_timer(&mut self, time: u128) {
        self.timer = time;
    }

    pub fn transmit(&self) -> Vec<ChemotacticMolecule> {
        let mut vec = Vec::with_capacity(self.max_receptors);
        for _ in 0..self.chemotatic_molecules_count {
            vec.push(ChemotacticMolecule::new(
                self.rect.x(),
                self.rect.y(),
                5,
                5,
                10.2,
            ));
        }
        vec
    }
}

fn create_output_file() -> File {
    let file = File::create("output.txt").unwrap();
    file
}

fn count_molecules_in_range(
    molecule: &InformationMolecule,
    chemotactic_molecules: &Vec<ChemotacticMolecule>,
    range: f32,
) -> u32 {
    let mut count: u32 = 0;
    let x1 = molecule.rect.x();
    let y1 = molecule.rect.y();

    let max_x_distance = x1 + range as i32;
    let min_x_distance = x1 - range as i32;
    let max_y_distance = y1 + range as i32;
    let min_y_distance = y1 - range as i32;

    for i in 0..chemotactic_molecules.len() {
        let x2 = chemotactic_molecules[i].rect.x();
        let y2 = chemotactic_molecules[i].rect.y();
        if x2 >= min_x_distance
            && x2 <= max_x_distance
            && y2 >= min_y_distance
            && y2 <= max_y_distance
        {
            count += 1;
        }
    }
    count
}

pub fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_accelerated_visual(true);
    gl_attr.set_double_buffer(true);
    gl_attr.set_context_version(3, 2);
    // Enable anti-aliasing
    gl_attr.set_multisample_buffers(1);
    gl_attr.set_multisample_samples(4);

    let mut output_file = create_output_file();

    //let window = video_subsystem
    //    .window("chemotaxis", 800, 600)
    //    .position_centered()
    //    .opengl()
    //    .build()
    //    .unwrap();

    //let mut canvas = window.into_canvas().build().unwrap();

    let transmitter = Transmitter::new(200, 200, 40, 40);
    let mut information_molecules = transmitter.transmit();

    let mut receiver = Receiver::new(650, 500, 40, 40);
    let mut chemotactic_molecules = receiver.transmit();

    //canvas.set_draw_color(Color::RGB(0, 255, 255));
    //canvas.clear();
    //canvas.present();

    let mut event_pump = sdl_context.event_pump().unwrap();

    let mut timer: u128 = 0;

    'running: loop {
        //canvas.set_draw_color(Color::RGB(200, 200, 200));
        //canvas.clear();
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        // The rest of the game loop goes here...
        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        //let _ = canvas.fill_rect(transmitter.rect);
        //let _ = canvas.fill_rect(receiver.rect);

        if timer % 100 == 0 {
            println!("{}", timer);
        }
        if timer % 3000 == 0 {
            let mut new_molecules = transmitter.transmit();
            information_molecules.append(&mut new_molecules);
            println!("Transmitting new molecules");
        }

        //canvas.set_draw_color(Color::RGB(255, 0, 0));
        for i in 0..chemotactic_molecules.len() {
            chemotactic_molecules[i].tumble();
            chemotactic_molecules[i].run();
            //let _ = canvas.draw_rect(chemotactic_molecules[i].rect);
        }

        //canvas.set_draw_color(Color::RGB(0, 0, 0));
        for i in 0..information_molecules.len() {
            let mut in_range = information_molecules[i].stimulus;
            if information_molecules[i].tumble_timer <= 0 {
                in_range = count_molecules_in_range(
                    &information_molecules[i],
                    &chemotactic_molecules,
                    20.0,
                );
            }
            information_molecules[i].tumble(in_range);
            information_molecules[i].run();
            //let _ = canvas.draw_rect(information_molecules[i].rect);
            if receiver
                .rect
                .has_intersection(information_molecules[i].rect)
                && !information_molecules[i].stopped
            {
                if receiver.receive() {
                    information_molecules[i].stopped = true;
                }
            }
        }
        //let _ = canvas.draw_rect(information_molecules[0].rect);

        information_molecules = information_molecules
            .into_iter()
            .filter(|m| {
                if m.released {
                    if m.stopped && receiver.release() {
                        return false;
                    }
                    return false;
                }
                true
            })
            .collect();

        //::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
        //canvas.present();
        timer += 1;
        //let datapoint = format!(
        //    "{},{};",
        //    receiver.max_receptors - receiver.free_receptors,
        //    timer
        //);
        //output_file.write(datapoint.as_bytes()).unwrap();
    }
}
