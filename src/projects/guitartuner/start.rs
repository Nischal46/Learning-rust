use minifb::{Key, Window, WindowOptions};
use nokhwa::Camera;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};

pub fn init() -> Result<(), Box<dyn std::error::Error>> {
    // 🎥 Open camera
    let mut camera = Camera::new(
        CameraIndex::Index(0),
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate),
    )?;

    let frame = camera.frame()?;
    let img = frame.decode_image::<RgbFormat>()?;

    let width = img.width() as usize;
    let height = img.height() as usize;

    println!("Camera opened: {}x{}", width, height);

    // 🪟 Create window
    let mut window = Window::new(
        "Rust Webcam (ESC to exit)",
        width,
        height,
        WindowOptions::default(),
    )?;

    window.limit_update_rate(Some(std::time::Duration::from_millis(16)));

    // 🧠 Frame buffer for minifb (ARGB / RGB packed)
    let mut buffer: Vec<u32> = vec![0; width * height];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let frame = camera.frame()?;
        let img = frame.decode_image::<RgbFormat>()?;

        let pixels = img.as_raw();

        // Convert RGB -> minifb u32 (0x00RRGGBB)
        for i in 0..width * height {
            let r = pixels[i * 3] as u32;
            let g = pixels[i * 3 + 1] as u32;
            let b = pixels[i * 3 + 2] as u32;

            buffer[i] = (r << 16) | (g << 8) | b;
        }

        window.update_with_buffer(&buffer, width, height)?;
    }

    Ok(())
}

