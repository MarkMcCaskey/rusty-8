use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::keyboard::Scancode;
use std::collections::HashSet;

//mod graphics;

extern crate sdl2;

pub struct Graphics {
    sdl_context: sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
//    window: sdl2::video::Window,
    renderer: sdl2::render::Renderer<'static>,
    event_pump: sdl2::EventPump
}

impl Default for Graphics {
    #[inline]
    fn default() -> Graphics {
        let sdl_con = sdl2::init()
            .unwrap();

        let video_sub = sdl_con
            .video()
            .unwrap();
        
        let win = video_sub
            .window("Chip-8 emulator", 800, 600)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        
        let mut rend = win
            .renderer()
            .build()
            .unwrap();

        rend.set_draw_color(Color::RGB(0,0,0));
        rend.clear();
        rend.present();

        let mut event_p = sdl_con
            .event_pump()
            .unwrap();

        Graphics {
            sdl_context:     sdl_con,
            video_subsystem: video_sub,
            renderer:        rend,
            event_pump:      event_p
        }
        
    }
}


impl Graphics {
    pub fn graphics(&mut self) {

        'running: loop {
            for event in self.event_pump.poll_iter() {
                match event {
                    Event::Quit {..}
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),..}
                    => {
                        break 'running
                    },
                    _ => {}
                }
            }
        }
    }

    // IO code needs a lot of review, commenting it out for now
    fn pressed_scancode_set(e: &sdl2::EventPump)
                            -> HashSet<Scancode> {
        e.keyboard_state().pressed_scancodes().collect()
    }

    /*fn pressed_keycode_set(e: &sdl2::EventPump) -> HashSet <Keycode> {
    e.keyboard_state().pressed_scancodes()
    .filter_map(Keycode::from_scancode())
    .collect()
}*/
    
    fn newly_pressed(old: &HashSet<Scancode>,
                     new: &HashSet<Scancode>)
                     -> HashSet<Scancode> {
        new - old
    }
    
}
