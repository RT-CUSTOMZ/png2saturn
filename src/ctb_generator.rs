use std::fs::File;

use byteorder::{LittleEndian, WriteBytesExt};
use catibo::{
    output::Builder,
    rle::{encode_rle15, Run12},
    Magic,
};
use png::Decoder;

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

pub fn insert_sample_previews(builder: &mut Builder, debug_level: u8) {
    if debug_level > 0 {
        println!("Generating preview images");
    }
    {
        let file = File::open("small-preview.png").unwrap();
        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let img_bytes = &buf[..info.buffer_size()];

        if debug_level > 0 {
            println!("Small preview info: {:?}", info);
        }

        let mut rgb_tuples = vec![(0, 0, 0); info.buffer_size() / 3];

        for i in 0..info.buffer_size() / 3 {
            rgb_tuples[i] = (
                img_bytes[i * 3 + 0],
                img_bytes[i * 3 + 1],
                img_bytes[i * 3 + 2],
            );
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
        builder.small_preview(200, 125, pixels);
    }

    {
        let file = File::open("large-preview.png").unwrap();
        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();
        // Grab the bytes of the image.
        let img_bytes = &buf[..info.buffer_size()];

        if debug_level > 0 {
            println!("Large preview info: {:?}", info);
        }

        let mut rgb_tuples = vec![(0, 0, 0); info.buffer_size() / 3];

        for i in 0..info.buffer_size() / 3 {
            rgb_tuples[i] = (
                img_bytes[i * 3 + 0],
                img_bytes[i * 3 + 1],
                img_bytes[i * 3 + 2],
            );
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
        builder.large_preview(400, 300, pixels);
    }
}
