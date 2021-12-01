use std::fs::File;
use std::io;
use std::io::Read;
use std::io::Seek;
use std::io::Write;
use byteorder::{LittleEndian, WriteBytesExt};

fn read_file(filename: &str) -> io::Result<()> {
    let padding = 800;
    let block_size = 1177;

    let mut _position = 0;
    let mut _block = 0;

    let mut _shift_left = 0;
    let mut _shift_right = 0;
    let mut _channel_id = 0;

    let mut file = match File::open(&filename) {
        Err(e) => return Err(e),
        Ok(f) => f,
    };

    let mut output_file = match File::create("output.wav") {
        Err(e) => return Err(e),
        Ok(f) => f,
    };

    match file.seek(std::io::SeekFrom::Start(padding)) {
        Err(e) => return Err(e),
        _ => (),
    };

    let mut wav_header: Vec<u8> = Vec::new();

    wav_header.write_all(b"RIFF")?;
    wav_header.write_u32::<LittleEndian>(579123588)?;
    wav_header.write_all(b"WAVE")?;
    wav_header.write_all(b"fmt ")?;
    wav_header.write_u32::<LittleEndian>(16)?;
    wav_header.write_u16::<LittleEndian>(1)?;
    wav_header.write_u16::<LittleEndian>(2)?;
    wav_header.write_u32::<LittleEndian>(44100)?;
    wav_header.write_u32::<LittleEndian>(176400)?;
    wav_header.write_u16::<LittleEndian>(4)?;
    wav_header.write_u16::<LittleEndian>(16)?;
    wav_header.write_all(b"data")?;
    wav_header.write_u32::<LittleEndian>(579123552)?;

    output_file.write_all(&wav_header).unwrap();

    for byte in file.bytes() {
        if _position % block_size == 0 {
            let shift_val = byte.unwrap();
            _shift_left = shift_val >> 4;
            _shift_right = shift_val & 0x0F;
            _position += 1;
            _block += 1;
            continue;
        }

        _channel_id = match _position {
            p if p % 2 == 0 && _block % 2 == 0 => 0,
            p if p % 2 == 1 && _block % 2 == 1 => 0,
            _ => 1,
        };

        if _channel_id == 0 {
            let eight_bit_sample = byte.unwrap() as i8;
            let sixteen_bit_value = (eight_bit_sample as i16) << _shift_left;
            match output_file.write_i16::<LittleEndian>(sixteen_bit_value) {
                Err(e) => return Err(e),
                _ => (),
            }
        } else {
            let eight_bit_sample = byte.unwrap() as i8;
            let sixteen_bit_value = (eight_bit_sample as i16) << _shift_right;
            match output_file.write_i16::<LittleEndian>(sixteen_bit_value) {
                Err(e) => return Err(e),
                _ => ()
            }
        }

        _position += 1;

        if _block % 200 == 0 {
            print!("Block {:06} of {}\r", _block, 246227);
        }
    }

    Ok(())
}

fn main() {
    let _result = read_file("CDDA.SOU");
    println!("");
}
