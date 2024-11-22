use arithmetic::arithmetic::{add, div, mult, sub};
use bitwise::bitwise::{and, bitshift, or, xor, BitshiftDirection};
use clap::{builder::styling::RgbColor, ArgAction, Parser, Subcommand};
use image::{codecs::png::PngEncoder, ImageEncoder, ImageReader};
use std::io::{self, BufWriter, Cursor, Read, Write};
use utils::utils::hex_to_rgb;

mod arithmetic;
mod bitwise;
mod utils;

#[derive(Subcommand)]
enum SubCommands {
    OR { color: String },
    AND { color: String },
    XOR { color: String },
    LEFT { bits: String },
    RIGHT { bits: String },
    ADD { color: String },
    SUB { color: String },
    MULT { color: String },
    DIV { color: String },
}

#[derive(Parser)]
#[command(name = "img-mod")]
#[command(version = "1.0.1")]
#[command(about = "Bitwise operations and other stuff to images.", long_about = None)]
struct Args {
    #[command(subcommand)]
    cmd: SubCommands,

    /// path/to/input/image
    #[arg(short, long, global = true)]
    input: Option<String>,

    /// path/to/output/image
    #[arg(long, default_value = ".")]
    output: Option<String>,

    /// Specify the left hand side operands for the function. E.g. --lhs b g r
    #[arg(long, num_args(1..), global = true)]
    lhs: Option<Vec<String>>,

    /// Specify the right hand side operands for the function. E.g. --rhs b r b
    #[arg(long, num_args(1..), global = true)]
    rhs: Option<Vec<String>>,

    /// If function is 'left' or 'right', how many bits to shift by.
    #[arg(short, long)]
    bit_shift: Option<u8>,

    /// Negate the logical operator
    #[arg(short, long, action=ArgAction::SetTrue, global = true)]
    negate: bool,
}

fn main() {
    let args = Args::parse();

    let mut color_arg: Option<&str> = None;
    let mut bit_shift = "";

    let in_path = args.input;
    let lhs = args.lhs;
    let rhs = args.rhs;
    let negate = args.negate;

    let img = match in_path {
        Some(path) => image::open(path).expect("Failed to open image."),
        None => {
            let mut buffer = Vec::new();

            io::stdin()
                .read_to_end(&mut buffer)
                .expect("Failed to read from stdin");

            ImageReader::new(Cursor::new(buffer))
                .with_guessed_format()
                .expect("Failed to guess image format")
                .decode()
                .expect("Failed to decode image from stdin")
        }
    };

    let output = match args.cmd {
        SubCommands::OR { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            or(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), negate)
        }
        SubCommands::AND { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            and(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), negate)
        }
        SubCommands::XOR { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            xor(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2), negate)
        }
        SubCommands::ADD { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            add(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::SUB { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            sub(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::MULT { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            mult(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::DIV { color } => {
            let rgb = hex_to_rgb(&color).expect("Could not convert color to rgb");
            div(img, lhs, rhs, RgbColor(rgb.0, rgb.1, rgb.2))
        }
        SubCommands::LEFT { bits } => {
            bit_shift = &bits;
            bitshift(
                img,
                BitshiftDirection::LEFT,
                bit_shift
                    .parse::<u8>()
                    .expect("Could not parse bits arg to u8"),
            )
        }
        SubCommands::RIGHT { bits } => {
            bit_shift = &bits;
            bitshift(
                img,
                BitshiftDirection::RIGHT,
                bit_shift
                    .parse::<u8>()
                    .expect("Could not parse bits arg to u8"),
            )
        }
    };

    // Epic fast buffer writing
    let stdout = io::stdout();
    let handle = stdout.lock();
    let mut writer = BufWriter::new(handle);

    let encoder = PngEncoder::new_with_quality(
        &mut writer,
        image::codecs::png::CompressionType::Default,
        image::codecs::png::FilterType::Adaptive,
    );

    encoder
        .write_image(
            &output,
            output.width(),
            output.height(),
            image::ExtendedColorType::Rgba8,
        )
        .expect("Error while encoding buffer");
    //output.write_to(&mut writer, image::ImageFormat::Png);

    writer.flush().expect("error flushing writer");
}
