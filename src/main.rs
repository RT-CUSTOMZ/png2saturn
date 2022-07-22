use std::{fs::File, path::PathBuf};

use catibo::output::encode_rle7_slice;
use clap::Parser;
use png::Decoder;

mod ctb_generator;
use ctb_generator::*;

const RESOLUTION_X: usize = 3840;
const RESOLUTION_Y: usize = 2400;

#[derive(clap::ValueEnum, Clone)]
enum Corner {
    NorthWest,
    NorthEast,
    SouthWest,
    SouthEast,
}

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(value_parser, value_name = "PNG_INPUT")]
    png: PathBuf,

    #[clap(value_parser, value_name = "CTB_OUTPUT")]
    output: PathBuf,

    #[clap(short, long, value_enum, default_value_t = Corner::NorthWest)]
    corner: Corner,

    #[clap(short, long, value_parser, default_value_t = 0)]
    x_padding: usize,

    #[clap(short, long, value_parser, default_value_t = 0)]
    y_padding: usize,

    #[clap(short, long, value_parser, default_value_t = 90.0)]
    exposure: f32,

    /// Turn debugging information on
    #[clap(short, long, action = clap::ArgAction::Count)]
    debug: u8,
}
fn main() {
    let cli = Cli::parse();
    let mut rle7_slice: Vec<u8> = Vec::new();

    {
        let file = File::open(cli.png).unwrap();
        let decoder = Decoder::new(file);
        let mut reader = decoder.read_info().unwrap();
        let mut buf = vec![0; reader.output_buffer_size()];
        // Read the next frame. An APNG might contain multiple frames.
        let info = reader.next_frame(&mut buf).unwrap();

        if cli.debug > 0 {
            println!("PNG info: {:?}", info);
        }
        if info.width > RESOLUTION_X as u32 && info.height > RESOLUTION_Y as u32 {
            panic!("PCB to large!");
        }

        // Grab the bytes of the image.
        let img_bytes = &buf[..info.buffer_size()];

        let mut img_bw = vec![0; info.buffer_size() / 3];

        for i in 0..info.buffer_size() / 3 {
            img_bw[i] = img_bytes[i * 3];
        }
        let layer = position_bw_image(
            img_bw,
            info.width.try_into().unwrap(),
            info.height.try_into().unwrap(),
            cli.x_padding,
            cli.y_padding,
            cli.corner,
        );
        encode_rle7_slice(layer.into_iter().peekable(), 1741386203, 0, &mut rle7_slice);
    }

    let mut builder = ctb_from_custom();
    insert_sample_previews(&mut builder, cli.debug);
    builder.encryption_key(1741386203);
    builder.encryption_mode(0xF);
    builder.layer(1.6, cli.exposure, 0.0, rle7_slice);
    builder.exposure_s(cli.exposure);
    builder.bot_exposure_s(cli.exposure);

    let out = File::create(cli.output).unwrap();
    builder.write(out).unwrap();
}

fn position_bw_image(
    bw_img: Vec<u8>,
    orig_x: usize,
    orig_y: usize,
    padding_x: usize,
    padding_y: usize,
    corner: Corner,
) -> Vec<u8> {
    let mut bw_img = bw_img.into_iter();
    match corner {
        Corner::NorthWest => {
            let mut full_image = vec![0u8; RESOLUTION_X * RESOLUTION_Y];
            for i in 0..orig_y {
                let line = (&mut bw_img).take(orig_x);
                let start = padding_y * RESOLUTION_X + i * RESOLUTION_X + padding_x; // y padding lines + lines already advanced + px
                let stop = padding_y * RESOLUTION_X + i * RESOLUTION_X + padding_x + orig_x;
                full_image.splice(start..stop, line);
            }
            full_image
        }
        Corner::NorthEast => {
            let mut full_image = vec![0u8; RESOLUTION_X * RESOLUTION_Y];
            for i in 0..orig_y {
                let line = (&mut bw_img).take(orig_x);
                let start =
                    padding_y * RESOLUTION_X + (i + 1) * RESOLUTION_X - (padding_x + orig_x);
                let stop = padding_y * RESOLUTION_X + (i + 1) * RESOLUTION_X - padding_x;
                full_image.splice(start..stop, line);
            }
            full_image
        }
        Corner::SouthWest => {
            let mut full_image = vec![0u8; RESOLUTION_X * RESOLUTION_Y];
            let offset_from_top =
                RESOLUTION_X * RESOLUTION_Y - padding_y * RESOLUTION_X - orig_y * RESOLUTION_X;
            for i in 0..orig_y {
                let line = (&mut bw_img).take(orig_x);
                let start = offset_from_top + i * RESOLUTION_X + padding_x;
                let stop = offset_from_top + i * RESOLUTION_X + padding_x + orig_x;
                full_image.splice(start..stop, line);
            }
            full_image
        }
        Corner::SouthEast => {
            let mut full_image = vec![0u8; RESOLUTION_X * RESOLUTION_Y];
            let offset_from_top =
                RESOLUTION_X * RESOLUTION_Y - padding_y * RESOLUTION_X - orig_y * RESOLUTION_X;
            for i in 0..orig_y {
                let line = (&mut bw_img).take(orig_x);
                let start = offset_from_top + (i + 1) * RESOLUTION_X - (padding_x + orig_x);
                let stop = offset_from_top + (i + 1) * RESOLUTION_X - padding_x;
                full_image.splice(start..stop, line);
            }
            full_image
        }
    }
}