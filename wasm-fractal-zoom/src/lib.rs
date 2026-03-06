use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub fn init() {
    console_error_panic_hook::set_once();
}

#[derive(Clone, Copy)]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}

const PALETTE0: [Color; 5] = [
    Color { r: 0, g: 255, b: 255 },   // Cyan
    Color { r: 255, g: 0, b: 255 },   // Magenta
    Color { r: 128, g: 0, b: 255 },  // Purple
    Color { r: 0, g: 128, b: 255 },  // Blue
    Color { r: 255, g: 255, b: 0 },   // Yellow
];

const PALETTE1: [Color; 5] = [
    Color { r: 255, g: 0, b: 128 },  // Hot Pink
    Color { r: 0, g: 255, b: 128 },  // Electric Green
    Color { r: 128, g: 0, b: 255 },  // Deep Purple
    Color { r: 0, g: 128, b: 255 },  // Tech Blue
    Color { r: 255, g: 128, b: 0 },  // Neon Orange
];

const PALETTE2: [Color; 4] = [
    Color { r: 206, g: 10, b: 0 },  // Dark Red
    Color { r: 206, g: 97, b: 0 },  // Amber
    Color { r: 1, g: 115, b: 125 }, // Cyan
    Color { r: 0, g: 160, b: 20 },  // Dark Green
];

pub fn get_color(iterations: f64, max_iterations: f64, palette_id: u32) -> (u8, u8, u8) {
    if iterations >= max_iterations {
        return (0, 0, 0);
    }

    let palette = if palette_id == 0 { &PALETTE0 } else { &PALETTE1 };
    let n = palette.len() as f64;
    let normalized = iterations / max_iterations;
    let scaled = normalized * (n - 1.0);
    
    let idx1 = scaled.floor() as usize;
    let idx2 = (idx1 + 1).min(palette.len() - 1);
    let t = scaled - scaled.floor();

    let c1 = &palette[idx1];
    let c2 = &palette[idx2];


pub fn get_color(iterations: f64, max_iterations: f64, palette_id: u32) -> (u8, u8, u8) {
    if iterations >= max_iterations {
        return (0, 0, 0);
    }

    // Select palette based on `palette_id`
    let palette = match palette_id {
        0 => PALETTE0.as_slice(), //&[Color]
        1 => PALETTE1.as_slice(), //&[Color]
        2 => PALETTE2.as_slice(), //&[Color]
        _ => PALETTE0.as_slice(), //fallback
    };

    // Smooth color interpolation within the palette
    let n = palette.len() as f64;
    let normalized = iterations / max_iterations;
    let scaled = normalized * (n - 1.0);
    
    let idx1 = scaled.floor() as usize;
    let idx2 = (idx1 + 1).min(palette.len() - 1);
    let t = scaled - scaled.floor();

    let c1 = &palette[idx1];
    let c2 = &palette[idx2];

    // Linear interpolation for RGB components
    let r = (c1.0 as f64 + t * (c2.0 as f64 - c1.0 as f64)) as u8;
    let g = (c1.1 as f64 + t * (c2.1 as f64 - c1.1 as f64)) as u8;
    let b = (c1.2 as f64 + t * (c2.2 as f64 - c1.2 as f64)) as u8;

    (r, g, b)
}

    (
        (c1.r as f64 * (1.0 - t) + c2.r as f64 * t) as u8,
        (c1.g as f64 * (1.0 - t) + c2.g as f64 * t) as u8,
        (c1.b as f64 * (1.0 - t) + c2.b as f64 * t) as u8,
    )
}

#[wasm_bindgen(js_name = generate_fractal)]
pub fn generate_fractal(
    width: u32,
    height: u32,
    center_x: f64,
    center_y: f64,
    zoom: f64,
    max_iters: u32,
    palette_id: u32,
) -> Vec<u8> {
    let mut image_data = vec![0u8; (width * height * 4) as usize];
    let aspect_ratio = width as f64 / height as f64;

    for y in 0..height {
        for x in 0..width {
            let cx = (x as f64 / width as f64 - 0.5) * 4.0 * aspect_ratio / zoom + center_x;
            let cy = (y as f64 / height as f64 - 0.5) * 4.0 / zoom + center_y;

            let mut zx = 0.0;
            let mut zy = 0.0;
            let mut iterations = 0;

            while zx * zx + zy * zy < 4.0 && iterations < max_iters {
                let tmp = zx * zx - zy * zy + cx;
                zy = 2.0 * zx * zy + cy;
                zx = tmp;
                iterations += 1;
            }

            let idx = ((y * width + x) * 4) as usize;
            if iterations >= max_iters {
                image_data[idx] = 0;
                image_data[idx + 1] = 0;
                image_data[idx + 2] = 0;
                image_data[idx + 3] = 255;
            } else {
                // Smooth coloring
                let z_mag_sq = zx * zx + zy * zy;
                let smooth_iter = iterations as f64 + 1.0 - (z_mag_sq.ln().ln() / 2.0_f64.ln());
                
                let (r, g, b) = get_color(smooth_iter, max_iters as f64, palette_id);
                image_data[idx] = r;
                image_data[idx + 1] = g;
                image_data[idx + 2] = b;
                image_data[idx + 3] = 255;
            }
        }
    }

    image_data
}
