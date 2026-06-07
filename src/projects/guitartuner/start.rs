use minifb::{Key, Window, WindowOptions};
use nokhwa::Camera;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::{CameraIndex, RequestedFormat, RequestedFormatType};

pub fn init() {
    let index = CameraIndex::Index(0);
    let requested =
        RequestedFormat::new::<RgbFormat>(RequestedFormatType::AbsoluteHighestFrameRate);

    // Use match or if-let instead of .unwrap()
    let mut camera = match Camera::new(index, requested) {
        Ok(cam) => cam,
        Err(e) => {
            eprintln!("❌ Failed to open camera: {:?}", e);
            return; // Exit gracefully
        }
    };

    // You also need to open the stream before grabbing a frame in newer nokhwa versions!
    if let Err(e) = camera.open_stream() {
        eprintln!("❌ Failed to start camera stream: {:?}", e);
        return;
    }

    match camera.frame() {
        Ok(frame) => {
            println!("Captured Single Frame of {}", frame.buffer().len());
            if let Ok(decoded) = frame.decode_image::<RgbFormat>() {
                println!("Decoded Frame of {}", decoded.len());
            }
        }
        Err(e) => eprintln!("❌ Failed to capture frame: {:?}", e),
    }
}

