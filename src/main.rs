mod program_args;
use std::{fs::File, io};
use std::io::prelude::*;

use image::{ImageBuffer, EncodableLayout};
use program_args::ProgramArgs; // Bring Args into scope
use clap::Parser; // Make sure Parser is in scope


enum Depth {
    B2,
    B4,
    B8,
}

impl Depth {
    pub fn as_u8(&self) -> u8 {
        match self {
            Depth::B2 => 2,
            Depth::B4 => 4,
            Depth::B8 => 8,
        }
    }
}

enum DynamicImageBuffer {
    IM2(ImageBuffer<image::Rgb<u8>, Vec<u8>>),
    IM4(ImageBuffer<image::Rgb<u8>, Vec<u8>>),
    IM8(ImageBuffer<image::Rgb<u8>, Vec<u8>>),
}

impl DynamicImageBuffer {
    pub fn as_bytes(&self) -> &[u8] {
        match self {
            DynamicImageBuffer::IM2(buffer) | DynamicImageBuffer::IM4(buffer) | DynamicImageBuffer::IM8(buffer) => {
                buffer.as_bytes()
            }
        }   
    }

    pub fn depth(&self) -> Depth {
        match self {
            DynamicImageBuffer::IM2(_) => Depth::B2,
            DynamicImageBuffer::IM4(_) => Depth::B4,
            DynamicImageBuffer::IM8(_) => Depth::B8
        }
    }
}

impl Depth {
    pub fn from_u8(depth: u8) -> Result<Depth, String> {
        return match depth  {
            2 => Ok(Depth::B2),
            4 => Ok(Depth::B4),
            8 => Ok(Depth::B8),
            _ => Err("Invalid depth value".to_string())
        }
    }
}

fn main() {
    let args = ProgramArgs::parse();
    let depth = Depth::from_u8(args.depth).expect("Failed to decode depth. Valid values include (8, 16, 24, 32)");
    let buffer = to_buffer(args.file, depth);
    let outfile = args.output;
    let mut file = File::create(outfile.clone()).expect(format!("Could not create file {}", outfile).as_str());
    write_bytes_to_file(&buffer, &mut file).expect("Failed to write bytes to file");
}

fn to_buffer(filename: String, depth: Depth) -> DynamicImageBuffer {
    // check the file type
    let res = image::open(filename).expect("Failed to read image. Check that the pathname is correct and the image is not corrupted.");
    match depth {
        Depth::B2 => {
            DynamicImageBuffer::IM2(down_sample(res.as_rgb8().expect("Could not convert image to rgb2").clone(), depth))
        },
        Depth::B4 => {
            DynamicImageBuffer::IM4(down_sample(res.as_rgb8().expect("Could not convert image to rgb2").clone(), depth))
        },
        Depth::B8 => {
            DynamicImageBuffer::IM8((res.as_rgb8().expect("Could not convert image to rgb8")).clone())
        }
    }

}

fn down_sample(mut buffer: ImageBuffer<image::Rgb<u8>, Vec<u8>>, depth: Depth) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
    for pixel in buffer.pixels_mut() {
        let (r, g, b) = (pixel[0], pixel[1], pixel[2]);
        // Downsample each color based on the specified depth
        match depth {
            Depth::B2 => {
                pixel[0] = (r / 64) * 64 + 32; // Map to nearest of the 4 levels in 2-bit
                pixel[1] = (g / 64) * 64 + 32; // Adding 32 to center the value in the range
                pixel[2] = (b / 64) * 64 + 32;
            }
            Depth::B4 => {
                pixel[0] = (r / 16) * 16 + 8; // Map to nearest of the 16 levels in 4-bit
                pixel[1] = (g / 16) * 16 + 8; // Adding 8 to center the value in the range
                pixel[2] = (b / 16) * 16 + 8;
            }

            _ => {}
        }
        
    }
    
    buffer
}

fn write_bytes_to_file(img: &DynamicImageBuffer, file: &mut File) -> io::Result<()> {
    let hex_chars_per_pixel = img.depth().as_u8(); // 2, 4, or 8

    // Iterate through each pixel's worth of data based on depth
    let bytes_per_pixel = 3; // Assuming 3 bytes per RGB pixel
    for pixel_data in img.as_bytes().chunks(bytes_per_pixel) {
        // Accumulate the pixel value based on depth
        let pixel_value = pixel_data.iter().fold(0u32, |acc, &val| (acc << hex_chars_per_pixel) | (u32::from(val) >> (8 - hex_chars_per_pixel)));

        // Write the accumulated pixel value as hex, padded to the nearest byte
        writeln!(file, "{:0width$x}", pixel_value, width = hex_chars_per_pixel as usize)?;
    }

    Ok(())
}