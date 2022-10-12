use std::{fs::File, path::PathBuf};

use byteorder::{LittleEndian, WriteBytesExt};
use catibo::{
    output::Builder,
    rle::{encode_rle15, Run12},
    Magic,
};
use png::{ColorType, Decoder};

pub fn ctb_from_custom() -> Builder {
    let mut builder = Builder::for_revision(Magic::CTB, 2); // catibo doesn't seem to support version 3
    let mt = "ELEGOO SATURN".to_ascii_uppercase().into_bytes();
    builder.machine_type(mt);

    builder.printer_out_mm([192.0, 120.0, 200.0]);
    builder.overall_height_mm(1.6);
    builder.layer_height_mm(1.6); // Arbitrary, maybe useful to specify different pcb thicknesses
    builder.exposure_s(42.0); // TODO optional parameter
    builder.bot_exposure_s(10.0);
    builder.light_off_time_s(5.0);
    builder.resolution([3840, 2400]);
    builder.pwm_level(255);
    builder.bot_pwm_level(255);
    builder.bot_lift_dist_mm(5.0);
    builder.bot_lift_speed_mmpm(1200.0);
    builder.lift_dist_mm(5.0);
    builder.lift_speed_mmpm(1200.0);
    builder.retract_speed_mmpm(120.0);
    builder.print_volume_ml(42.0);
    builder.bot_light_off_time_s(5.0);

    builder.bot_layer_count(1);
    builder.print_mass_g(42.0); // dummy value
    builder.print_price(42.0);
    builder.print_time_s(90);

    builder
}

pub fn add_small_preview(builder: &mut Builder, file: PathBuf, debug_level: u8) {
    if debug_level > 0 {
        println!("Generating small preview image");
    }
    if let Some(data) = generate_rle15_data(file, debug_level) {
        builder.small_preview(200, 125, data);
    }
}

pub fn add_large_preview(builder: &mut Builder, file: PathBuf, debug_level: u8) {
    if debug_level > 0 {
        println!("Generating large preview image");
    }
    if let Some(data) = generate_rle15_data(file, debug_level) {
        builder.large_preview(400, 300, data);
    }
}

fn generate_rle15_data(path: PathBuf, debug_level: u8) -> Option<Vec<u8>> {
    let file_name = path.file_name().unwrap().to_owned();
    let file = File::open(path).unwrap();
    let decoder = Decoder::new(file);
    let mut reader = decoder.read_info().unwrap();
    let mut buf = vec![0; reader.output_buffer_size()];
    // Read the next frame. An APNG might contain multiple frames.
    let frame_info = reader.next_frame(&mut buf).unwrap();
    // Grab the bytes of the image.
    let img_bytes = &buf[..frame_info.buffer_size()];

    if debug_level > 0 {
        println!("Preview info: {:?}", frame_info);
    }

    let mut rgb_tuples: Vec<(u8, u8, u8)> = Vec::new();
    match frame_info.color_type {
        ColorType::Grayscale => {
            rgb_tuples.reserve(frame_info.buffer_size());
            for v in img_bytes {
                rgb_tuples.push((*v, *v, *v));
            }
        }
        ColorType::Rgb => {
            rgb_tuples.append(&mut vec![(0, 0, 0); frame_info.buffer_size() / 3]);
            for i in 0..frame_info.buffer_size() / 3 {
                rgb_tuples[i] = (
                    img_bytes[i * 3 + 0],
                    img_bytes[i * 3 + 1],
                    img_bytes[i * 3 + 2],
                );
            }
        }
        _ => {
            println!("Warning: Unsupported image format, skipping preview.");
            println!("{} has an unsupported format.", file_name.to_string_lossy());
            println!("Only grayscale or rgb supported at the moment.");
            return None;
        }
    }

    let mut pixels = Vec::new();
    let mut rgb_tuples = rgb_tuples.into_iter().peekable();
    while rgb_tuples.peek() != None {
        let run = encode_rle15(&mut rgb_tuples).unwrap();
        match run {
            Run12::Single(x) => {
                pixels.write_u16::<LittleEndian>(x).unwrap();
            }
            Run12::Double(x, n) => {
                pixels.write_u16::<LittleEndian>(x).unwrap();
                pixels.write_u16::<LittleEndian>(n).unwrap();
            }
        }
    }
    Some(pixels)
}
