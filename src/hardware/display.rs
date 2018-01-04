use sdl2;
use sdl2::pixels;
use sdl2::rect::Rect;
use sdl2::render::Canvas;
use sdl2::video::Window;

pub struct Display {
    canvas: Canvas<Window>,
    scale: u32
}

impl Display {
    pub fn new(context: &sdl2::Sdl, scale: u32) -> Display {
        let video_subsys = context.video().unwrap();
        let window = video_subsys.window("CHIP-8", 64*scale, 32*scale)
            .position_centered()
            .opengl()
            .resizable()
            .build()
            .unwrap();

        let mut canvas = window.into_canvas().build().unwrap();

        canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
        canvas.clear();
        canvas.present();

        Display {
            canvas: canvas,
            scale: scale
        }
    }

    pub fn draw(&mut self, vram:[[u8; 64]; 32]) {

        for y in 0..32 {
            for x in 0..64 {
                if vram[y][x] == 0 { 
                    self.canvas.set_draw_color(pixels::Color::RGB(0, 0, 0));
                } else {
                    self.canvas.set_draw_color(pixels::Color::RGB(255, 255, 255));
                }
                
                let _ = self.canvas.fill_rect(
                    Rect::new((x as i32) * (self.scale as i32), (y as i32) * (self.scale as i32), self.scale, self.scale)
                );
            }
        }
        self.canvas.present();
    }
}