use iced::window;
use iced::window::Icon;

pub fn load_icon(path: &str) -> Icon {
    let result = window::icon::from_file(path);

    match result {
        Ok(icon) => {
            icon.into()
        }
        Err(_e) => {
            println!("Load icon error");
            let rgba: Vec<u8> = create_rgba_image(16, 16);
            let width = 16;
            let height = 16;
            window::icon::from_rgba(rgba, width, height).expect("Failed to create icon")
        }
    }
}


pub fn create_rgba_image(width: u32, height: u32) -> Vec<u8> {
    let mut image = vec![0; (width * height * 4) as usize];

    for y in 0..height {
        for x in 0..width {
            let index = (y * width + x) as usize * 4;
            image[index] = 135;
            image[index + 1] = 206;
            image[index + 2] = 250;
            image[index + 3] = 255;
        }
    }

    image
}