use std::fs::File;
use std::io;

use crate::ui::age_color;
use ratatui::style::Color;

fn color_to_rgb(color: Color) -> [u8; 3] {
    match color {
        Color::Rgb(r, g, b) => [r, g, b],
        _ => [0, 0, 0],
    }
}

pub fn encode_gif(
    frames: &[Vec<u16>],
    width: usize,
    height: usize,
    cell_size: u16,
    tick_ms: u16,
) -> io::Result<String> {
    let img_width = (width as u16) * cell_size;
    let img_height = (height as u16) * cell_size;

    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    let filename = format!("petri_{}.gif", timestamp);

    let file = File::create(&filename)?;
    let mut encoder = gif::Encoder::new(file, img_width, img_height, &[])
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;
    encoder
        .set_repeat(gif::Repeat::Infinite)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

    let delay = tick_ms / 10; // GIF delay is in centiseconds

    let mut prev_cells: Option<&Vec<u16>> = None;

    for frame_cells in frames {
        let mut pixels = vec![0u8; img_width as usize * img_height as usize * 4];

        for y in 0..height {
            for x in 0..width {
                let age = frame_cells[y * width + x];
                let changed = match prev_cells {
                    Some(prev) => prev[y * width + x] != age,
                    None => true,
                };

                if changed {
                    let rgb = color_to_rgb(age_color(age));
                    for dy in 0..cell_size as usize {
                        for dx in 0..cell_size as usize {
                            let px = x * cell_size as usize + dx;
                            let py = y * cell_size as usize + dy;
                            let offset = (py * img_width as usize + px) * 4;
                            pixels[offset] = rgb[0];
                            pixels[offset + 1] = rgb[1];
                            pixels[offset + 2] = rgb[2];
                            pixels[offset + 3] = 0xFF;
                        }
                    }
                }
                // unchanged pixels stay at [0,0,0,0] (transparent)
            }
        }

        let mut gif_frame = gif::Frame::from_rgba(img_width, img_height, &mut pixels);
        gif_frame.delay = delay;
        if prev_cells.is_some() {
            gif_frame.dispose = gif::DisposalMethod::Keep;
        }
        encoder
            .write_frame(&gif_frame)
            .map_err(|e| io::Error::new(io::ErrorKind::Other, e))?;

        prev_cells = Some(frame_cells);
    }

    Ok(filename)
}
