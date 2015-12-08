use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

extern crate sdl2;

pub struct Graphics {
    sdl_context:     sdl2::Sdl,
    video_subsystem: sdl2::VideoSubsystem,
    renderer:        sdl2::render::Renderer<'static>,
    event_pump:      sdl2::EventPump
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
            .window("Chip-8 emulator", 512, 256)
            .position_centered()
            .opengl()
            .build()
            .unwrap();
        
        let mut rend = win
            .renderer()
            .build()
            .unwrap();

        rend.set_draw_color(Color::RGB(0,0,0));
        rend.set_scale(8.0,8.0); 
        rend.clear();
        rend.present();

        let event_p = sdl_con
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
    pub fn is_pressed(&mut self, key: u8) -> bool
    {
        let mut kc: Keycode;
        for event in self.event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode:
                                 Some(Keycode::Num0),.. }
                => { kc = Keycode::Num0 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num1),.. }
                => { kc = Keycode::Num1 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num2),.. }
                => { kc = Keycode::Num2 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num3),.. }
                => { kc = Keycode::Num3 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num4),.. }
                => { kc = Keycode::Num4 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num5),.. }
                => { kc = Keycode::Num5 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num6),.. }
                => { kc = Keycode::Num6 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num7),.. }
                => { kc = Keycode::Num7 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num8),.. }
                => { kc = Keycode::Num8 },
                Event::KeyDown { keycode:
                                 Some(Keycode::Num9),.. }
                => { kc = Keycode::Num9 },
                Event::KeyDown { keycode:
                                 Some(Keycode::A),.. }
                => { kc = Keycode::A },
                Event::KeyDown { keycode:
                                 Some(Keycode::B),.. }
                => { kc = Keycode::B },
                Event::KeyDown { keycode:
                                 Some(Keycode::C),.. }
                => { kc = Keycode::C },
                Event::KeyDown { keycode:
                                 Some(Keycode::D),.. }
                => { kc = Keycode::D },
                Event::KeyDown { keycode:
                                 Some(Keycode::E),.. }
                => { kc = Keycode::E },
                Event::KeyDown { keycode:
                                 Some(Keycode::F),.. }
                => { kc = Keycode::F },
                _ => { /* do nothing */
                kc = Keycode::G}

            }
            if kc == lookup_keycode(key)
            {   return true; }
            
        }
            false
    }


    pub fn draw_point(&mut self, x: usize, y:usize, on: bool) {
        self.renderer.set_draw_color(
            match on {
                true  => Color::RGB(255,255,255),
                false => Color::RGB(0,  0,  0  )
            });
        //DEBUG:
       // print!("DRAWING AT ({},{}) to {}\n", x, y, on);
        self.renderer
            .draw_point(sdl2::rect::Point::new(
                x as i32,
                y as i32));

    }

    pub fn draw_screen(&mut self) {
        self.renderer.present();
    }

    pub fn clear_screen(&mut self) {
        self.renderer.set_draw_color(
            Color::RGB(0,0,0));
        for x in 0..64 {
            for y in 0..32 {
                self.draw_point(x,y,false);
            }
        }
        self.renderer.clear();
        self.renderer.present();
    }
}

fn lookup_keycode(key: u8) -> Keycode {
    match key {
        0  => Keycode::Num0,
        1  => Keycode::Num1,
        2  => Keycode::Num2,
        3  => Keycode::Num3,
        4  => Keycode::Num4,
        5  => Keycode::Num5,
        6  => Keycode::Num6,
        7  => Keycode::Num7,
        8  => Keycode::Num8,
        9  => Keycode::Num9,
        10 => Keycode::A,
        11 => Keycode::B,
        12 => Keycode::C,
        13 => Keycode::D,
        14 => Keycode::E,
        15 => Keycode::F,
        _ => Keycode::H
            
    }
}
