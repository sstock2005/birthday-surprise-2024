#![windows_subsystem = "windows"] // no console

extern crate sdl2;
extern crate bsod;

use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::render::{WindowCanvas, Texture};
use sdl2::rect::{Point, Rect};
use sdl2::image::{self, LoadTexture, InitFlag};
use std::time::Duration;
use bsod::bsod;

fn render(
    canvas: &mut WindowCanvas,
    color: Color,
    texture: &Texture,
    position: Point,
    sprite: Rect,
    sprite_scale: u32,
    text_1_texture: &Texture,
    text_2_texture: &Texture
) -> Result<(), String> {
    canvas.set_draw_color(color);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = position + Point::new(width as i32 / 2, height as i32 / 2);

    let text_1_screen_position = position + Point::new(width as i32 / 2, height as i32 / 2 - 200);

    let text_2_screen_position = position + Point::new(width as i32 / 2, height as i32 / 2 + 200);

    let screen_rect = Rect::from_center(screen_position, sprite.width() / sprite_scale, sprite.height() / sprite_scale);

    let text_1_rect = Rect::from_center(text_1_screen_position, 256, 128);

    let text_2_rect = Rect::from_center(text_2_screen_position, 256, 128);

    // Copy Image
    canvas.copy(texture, sprite, screen_rect)?;

    // Copy Text #1
    canvas.copy(text_1_texture, None, text_1_rect)?;

    // Copy Text #2
    canvas.copy(text_2_texture, None, text_2_rect)?;

    canvas.present();

    Ok(())
}

fn main() -> Result<(), String>
{
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG)?;
    let ttf_context = sdl2::ttf::init().map_err(|e| e.to_string())?;

    let window = video_subsystem.window("Happy Birthday!", 800, 600)
        .position_centered()
        .build()
        .expect("[!] Could not Initialize Video Subsystem");

    let mut canvas = window.into_canvas().build()
        .expect("[!] Could not convert window to canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/heart.png")?;

    let mut font = ttf_context.load_font("assets/font.ttf", 64)?;
    font.set_style(sdl2::ttf::FontStyle::BOLD);

    let text_1_surface = font
        .render("Happy Birthday!")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;

    let text_1_texture = texture_creator
        .create_texture_from_surface(&text_1_surface)
        .map_err(|e| e.to_string())?;

    let text_2_surface = font
        .render("Press Enter")
        .blended(Color::RGBA(255, 0, 0, 255))
        .map_err(|e| e.to_string())?;

    let text_2_texture = texture_creator
        .create_texture_from_surface(&text_2_surface)
        .map_err(|e| e.to_string())?;

    let sprite  = Rect::new(0, 0, 2000, 2000);

    let mut event_pump = sdl_context.event_pump()?;
    
    let mut i = 20;
    let mut j = 0;
    let mut j_inverse: bool = false;

    'running: loop 
    {

        for event in event_pump.poll_iter()
        {
            match event
            {
                Event::Quit {..} |
                Event::KeyDown { keycode: Some(Keycode::Escape), .. } =>
                {
                    break 'running;
                },
                Event::KeyDown { keycode: Some(Keycode::RETURN), ..} =>
                {
                    bsod();
                    break 'running;
                },
                _ => {}
            }
        }

        if !j_inverse
        {
            j = j + 1;
            i = i + 1;
        }
        else
        {
            j = j - 1;
            i = i - 1;
        }

        if j > 30 && !j_inverse
        {
            j_inverse = true;
        }
        
        if j < -10 && j_inverse
        {
            j_inverse = false;
        }


        let _ = render(&mut canvas, Color::RGB(255 - i, 126, 0), &texture, Point::new(0, 10 + j), sprite, 5, &text_1_texture, &text_2_texture);

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60));
    }

    Ok(())
}
