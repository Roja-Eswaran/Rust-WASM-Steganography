mod utils;
use std::io::Cursor;
use std::io::Read;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
    fn alert(s: &str);
}

#[derive(Debug)]
pub struct PPM {
    pub header: PPMHeader,
    pub pixels: Vec<u8>,
}
#[derive(Debug)]
pub enum StegError {
    BadDecode(String),
    BadEncode(String),
}

#[derive(Debug)]
pub struct PPMHeader {
    pub magic_number: [u8; 2],
    pub width: u32,
    pub height: u32,
    pub max_color_value: u32,
}

#[derive(Debug)]
pub enum PPMError {
    BadHeader(String),
    BadFile(String),
    // IOError(io::Error),
}

#[wasm_bindgen]
pub fn decode_from_file(data: &mut [u8]) -> String {
    let mut header_error = 0;
    let mut vec_data = Vec::new();
    vec_data.extend_from_slice(&data);
    let mut reader_bytes = Cursor::new(vec_data);
    let magic_number_bytes = parse_magic_number(&mut reader_bytes);
    let mut magic_number_struct = [0u8, 2];
    match magic_number_bytes {
        Ok(x) => {
            magic_number_struct[0] = x[0];
            magic_number_struct[1] = x[1];
        }
        Err(_) => {
            header_error = 1;
        }
    }

    // PARSE WIDTH
    let mut error = vec![0, 0];
    let width_bytes = parse_width_from_file(&mut reader_bytes);
    let mut width_struct = 0 as u32;
    match width_bytes {
        Ok(x) => {
            let mut valid_ascii_string = String::new();
            let ascii_value = bytes_to_ascii_string(x);
            match ascii_value {
                Ok(a) => valid_ascii_string = a.trim().to_string(),
                Err(_) => header_error = 1,
            }
            let su32 = string_to_u32(valid_ascii_string);
            match su32 {
                Ok(a) => width_struct = a,
                Err(_) => header_error = 1,
            }
        }
        Err(_) => {
            header_error = 1;
        }
    }

    //PARSE HEIGHT
    let height_bytes = parse_height_from_file(&mut reader_bytes);
    let mut height_struct = 0 as u32;
    match height_bytes {
        Ok(x) => {
            let mut valid_ascii_string = String::new();
            let ascii_value = bytes_to_ascii_string(x);
            match ascii_value {
                Ok(a) => valid_ascii_string = a.trim().to_string(),
                Err(_) => header_error = 1,
            }
            let su32 = string_to_u32(valid_ascii_string);
            match su32 {
                Ok(a) => height_struct = a,
                Err(_) => header_error = 1,
            }
        }
        Err(_) => {
            header_error = 1;
        }
    }
    //PARSE MAXIMUM HEIGHT
    let maximum_bytes = parse_maximum_color_value(&mut reader_bytes);
    let mut maximum_struct = 0 as u32;
    match maximum_bytes {
        Ok(x) => {
            let mut valid_ascii_string = String::new();
            let ascii_value = bytes_to_ascii_string(x);
            match ascii_value {
                Ok(a) => valid_ascii_string = a.trim().to_string(),
                Err(_) => header_error = 1,
            }
            let su32 = string_to_u32(valid_ascii_string);
            match su32 {
                Ok(a) => maximum_struct = a,
                Err(_) => header_error = 1,
            }
        }
        Err(_) => {
            header_error = 1;
        }
    }

    if (header_error == 1) {
        return "error".to_string();
    }
    //REST OF PIXELS
    let header = PPMHeader {
        magic_number: magic_number_struct,
        width: width_struct,
        height: height_struct,
        max_color_value: maximum_struct,
    };

    let mut pixels: Vec<u8> = Vec::new();
    reader_bytes.read_to_end(&mut pixels);
    //pixels.as_ptr()
    //let final_modified_bytes=encode_message(&message, &pixels);
    let ppm = PPM {
        header: header,
        pixels: pixels.clone(),
    };
    match decode_message(&ppm.pixels) {
        Ok(message) => {return message;},
        Err(err) => {return err;},
    }
}



#[wasm_bindgen]
pub fn encode_from_file(message: &str, data: &mut [u8]) -> *const u8 {
    let mut header_error = 0;
    let mut content_error = 0;
    let mut vec_data = Vec::new();
    vec_data.extend_from_slice(&data);
    let mut reader_bytes = Cursor::new(vec_data);
    let mut final_encoded_image = Vec::new();
    // MAGIC NUMBER PARSING

    let magic_number_bytes = parse_magic_number(&mut reader_bytes);
    let mut magic_number_struct = [0u8, 2];
    match magic_number_bytes {
        Ok(x) => {
            magic_number_struct[0] = x[0];
            magic_number_struct[1] = x[1];
        }
        Err(_) => {
            header_error = 1;
        }
    }

    // PARSE WIDTH
    let mut error = vec![0, 0];
    let width_bytes = parse_width_from_file(&mut reader_bytes);
    let mut width_struct = 0 as u32;
    match width_bytes {
        Ok(x) => {
            let mut valid_ascii_string = String::new();
            let ascii_value = bytes_to_ascii_string(x);
            match ascii_value {
                Ok(a) => valid_ascii_string = a.trim().to_string(),
                Err(_) => header_error = 1,
            }
            let su32 = string_to_u32(valid_ascii_string);
            match su32 {
                Ok(a) => width_struct = a,
                Err(_) => header_error = 1,
            }
        }
        Err(_) => {
            header_error = 1;
        }
    }

    //PARSE HEIGHT
    let height_bytes = parse_height_from_file(&mut reader_bytes);
    let mut height_struct = 0 as u32;
    match height_bytes {
        Ok(x) => {
            let mut valid_ascii_string = String::new();
            let ascii_value = bytes_to_ascii_string(x);
            match ascii_value {
                Ok(a) => valid_ascii_string = a.trim().to_string(),
                Err(_) => header_error = 1,
            }
            let su32 = string_to_u32(valid_ascii_string);
            match su32 {
                Ok(a) => height_struct = a,
                Err(_) => header_error = 1,
            }
        }
        Err(_) => {
            header_error = 1;
        }
    }
    //PARSE MAXIMUM HEIGHT
    let maximum_bytes = parse_maximum_color_value(&mut reader_bytes);
    let mut maximum_struct = 0 as u32;
    match maximum_bytes {
        Ok(x) => {
            let mut valid_ascii_string = String::new();
            let ascii_value = bytes_to_ascii_string(x);
            match ascii_value {
                Ok(a) => valid_ascii_string = a.trim().to_string(),
                Err(_) => header_error = 1,
            }
            let su32 = string_to_u32(valid_ascii_string);
            match su32 {
                Ok(a) => maximum_struct = a,
                Err(_) => header_error = 1,
            }
        }
        Err(_) => {
            header_error = 1;
        }
    }
    //REST OF PIXELS
    let header = PPMHeader {
        magic_number: magic_number_struct,
        width: width_struct,
        height: height_struct,
        max_color_value: maximum_struct,
    };

    let mut pixels: Vec<u8> = Vec::new();
    reader_bytes.read_to_end(&mut pixels);
    //pixels.as_ptr()
    //let final_modified_bytes=encode_message(&message, &pixels);
    let ppm = PPM {
        header: header,
        pixels: pixels.clone(),
    };
     

    //WRITING - ENCODING AND RETURNING PIXELS

    if (header_error == 1) {
        final_encoded_image.push(0);
        final_encoded_image.as_ptr()
    } else {
        match encode_message(&message, &ppm) {
            Ok(bytes) => {
                // we got some bytes
                // need to write ppm header first
                // TODO move this to library

                // first write magic number
                final_encoded_image.extend_from_slice(&ppm.header.magic_number);
                final_encoded_image.extend_from_slice(&"\n".to_string().into_bytes());

                final_encoded_image.extend_from_slice(&ppm.header.width.to_string().as_bytes());
                final_encoded_image.extend_from_slice(&" ".to_string().into_bytes());
                final_encoded_image.extend_from_slice(&ppm.header.height.to_string().as_bytes());
                final_encoded_image.extend_from_slice(&"\n".to_string().into_bytes());
                final_encoded_image
                    .extend_from_slice(&ppm.header.max_color_value.to_string().as_bytes());
                final_encoded_image.extend_from_slice(&"\n".to_string().into_bytes());
                final_encoded_image.extend_from_slice(&bytes);
            }
            Err(err) => match err {
                _ => {
                    content_error = 1;
                }
            },
        }

        if (content_error == 1) {
            final_encoded_image.push(1);
            //return final_encoded_image.as_ptr();
        }
        final_encoded_image.as_ptr()
    }
}

fn encode_message(message: &str, ppm: &PPM) -> Result<Vec<u8>, String> {
    let mut encoded = vec![0u8; 0];
    let mut return_encode: Result<Vec<u8>, String>;

    // loop through each character in the message
    // for each character, pull 8 bytes out of the file
    // encode those 8 bytes to hide the character in the message
    // add those 8 bytes to the enocded return value
    // add a trailing \0 after all character encoded
    // output the remainder of the original file
    let mut possible_character = (&ppm.pixels.len() / 8)  ;
    // alert(&possible_character.to_string());
    //alert(&message.chars().count().to_string());
    let mut ppm_length = &ppm.pixels.len();
    let mut start_index = 0;
    if (message.chars().count() > possible_character - 1) {
        return_encode = Err("Parsing_error".to_string());
        return return_encode;
    }
    for c in message.chars() {
        encoded.extend(&encode_character(
            c,
            &ppm.pixels[start_index..start_index + 8],
        ));
        start_index += 8;
    }
    
    encoded.extend(&encode_character(
        '\0',
        &ppm.pixels[start_index..start_index + 8],
    ));

    start_index += 8;

    

    // spit out remainder of ppm pixel data.

    if (start_index < *ppm_length) {
        encoded.extend(&ppm.pixels[start_index..]);
        
    } 
    return_encode = Ok(encoded);
    return return_encode;
}

fn encode_character(c: char, bytes: &[u8]) -> [u8; 8] {
    let c = c as u8;

    let mut ret = [0u8; 8];

    for i in 0..bytes.len() {
        if bit_set_at(c, i) {
            ret[i] = bytes[i] | 00000_0001;
        } else {
            ret[i] = bytes[i] & 0b1111_1110;
        }
    }

    ret
}

fn bit_set_at(c: u8, position: usize) -> bool {
    bit_at(c, position) == 1
}

fn bit_at(c: u8, position: usize) -> u8 {
    (c >> (7 - position)) & 0b0000_0001
}
fn decode_message(pixels: &Vec<u8>) -> Result<String, String> {
    let mut message = String::from("");
    let mut final_decode: Result<String, String>;
    for bytes in pixels.chunks(8) {
        // eprintln!("chunk!");
        if bytes.len() < 8 {
            //error
            final_decode = Err("error".to_string());
            return final_decode; 
        }

        let character = decode_character(bytes);

        if character > 127 {
            //error
            final_decode = Err("no".to_string());
           return final_decode;
        }

        message.push(char::from(character));

        if char::from(character) == '\0' {
            // eprintln!("Found terminating null!");
            final_decode= Ok(message.clone());
            return final_decode
        }
    }
    final_decode = Err("no".to_string());
    
    return final_decode;
}

fn decode_character(bytes: &[u8]) -> u8 {
    if bytes.len() != 8 {
        panic!("Tried to decode from less than 8 bytes!");
    }

    let mut character: u8 = 0b0000_0000;

    for (i, &byte) in bytes.iter().enumerate() {
        if lsb(byte) {
            match i {
                0 => character ^= 0b1000_0000,
                1 => character ^= 0b0100_0000,
                2 => character ^= 0b0010_0000,
                3 => character ^= 0b0001_0000,
                4 => character ^= 0b0000_1000,
                5 => character ^= 0b0000_0100,
                6 => character ^= 0b0000_0010,
                7 => character ^= 0b0000_0001,
                _ => panic!("uh oh!"),
            }
        }
    }

    character
}

fn lsb(byte: u8) -> bool {
    (0b0000_0001 & byte) == 1
}

fn parse_maximum_color_value(mut f: &mut impl Read) -> Result<Vec<u8>, PPMError> {
    let mut digit_start_found = false;

    let mut digit_start_index = 0;

    let mut ret = vec![0u8; 0];

    let mut b = [0u8; 1];

    loop {
        // this feels done poorly...
        match f.read(&mut b) {
            Ok(1) if !digit_start_found => {
                if is_white_space(b[0]) {
                    ret.extend(&b);
                    digit_start_index += 1;
                } else if is_digit(b[0]) {
                    ret.extend(&b);
                    digit_start_found = true;
                } else {
                    return Err(PPMError::BadHeader(
                        "Found a non digit when parsing max color value!".to_string(),
                    ));
                }
            }
            Ok(1) => {
                if is_digit(b[0]) {
                    ret.extend(&b);
                } else if is_white_space(b[0]) {
                    ret.extend(&b);
                    // ok, we've reached the end of searching for our digits.
                    // let's now let's make sure that it's legit
                    // eprintln!("ret.len() = {}", ret.len());
                    // eprintln!("ret: {:?}", ret);
                    let digits = &ret[digit_start_index..ret.len() - 1];
                    // eprintln!("digits.len() = {}", digits.len());
                    // eprintln!("digits: {:?}", digits);
                    match digits.len() {
                        1..=2 => {
                            // good
                            return Ok(ret);
                        }
                        3 if (digits[0] <= 50 && digits[1] <= 53 && digits[2] <= 53) => {
                            // eprintln!("ret.len() = {}", ret.len());
                            // eprintln!("ret: {:?}", ret);
                            return Ok(ret);
                        }
                        _ => {
                            return Err(PPMError::BadHeader(
                                "Max color value bigger than 255!".to_string(),
                            ))
                        }
                    }
                } else {
                    return Err(PPMError::BadHeader(
                        "Found a non digit when parsing max color value!".to_string(),
                    ));
                }
            }
            Ok(_) => {
                return Err(PPMError::BadHeader(
                    "Reached end EOF while parsing max color value".to_string(),
                ));
            }
            Err(err) => {
                return Err(PPMError::BadHeader(err.to_string()));
            }
        }
    }
}

pub fn parse_magic_number(f: &mut impl Read) -> Result<[u8; 2], PPMError> {
    let mut magic_number_bytes = [0u8; 2];
    match f.read(&mut magic_number_bytes) {
        Ok(2) => {
            // check to see if the magic number is correct!
            // if char::from(magic_number_bytes[0]) == 'P' && char::from
            let b1 = char::from(magic_number_bytes[0]);
            let b2 = char::from(magic_number_bytes[1]);

            match (b1, b2) {
                ('P', '6') => Ok(magic_number_bytes),
                _ => Err(PPMError::BadHeader(format!(
                    "Bad Magic Number: {}{}",
                    b1, b2
                ))),
            }
        }
        Ok(n) => Err(PPMError::BadHeader(format!(
            "Could not read two bytes for magic number parsing! Read {} bytes!",
            n
        ))),
        Err(err) => Err(PPMError::BadHeader(err.to_string())),
    }
}

fn parse_width_from_file(mut f: &mut impl Read) -> Result<Vec<u8>, PPMError> {
    let mut ret = vec![0u8; 0];

    ret.extend(parse_one_white_space(&mut f)?);

    //        ret.extend(parse_dimension(data)?);
    ret.extend(parse_dimension(&mut f)?);

    Ok(ret)
}

fn parse_one_white_space(f: &mut impl Read) -> Result<Vec<u8>, PPMError> {
    let mut ret = vec![0u8; 0];

    let mut b = [0u8; 1];

    match f.read(&mut b) {
        Ok(1) => {
            // we got one byte, if it is white space, we can stick
            // it into our result
            if is_white_space(b[0]) {
                ret.extend(&b);
                Ok(ret)
            } else {
                Err(PPMError::BadHeader(format!(
                    "Expected white space, got: {}",
                    b[0]
                )))
            }
        }
        Ok(_) => Err(PPMError::BadHeader(
            "Reached end EOF while looking for a single white space character!".to_string(),
        )),
        Err(err) => Err(PPMError::BadHeader(err.to_string())),
    }
}

fn is_white_space(b: u8) -> bool {
    match char::from(b) {
        '\n' | ' ' | '\t' | '\r' => true,
        _ => false,
    }
}

fn is_digit(b: u8) -> bool {
    (b >= 48) && (b <= 57)
}
fn parse_dimension(f: &mut impl Read) -> Result<Vec<u8>, PPMError> {
    // eprintln!("Parsing dimension");

    let mut ret = vec![0u8; 0];

    let mut b = [0u8; 1];

    // now we keep reading until we hit something that is not white space

    let mut digit_start_found = false;

    loop {
        match f.read(&mut b) {
            Ok(1) if !digit_start_found => {
                // probably need to put extra error checking
                // stuff here, although we should make it to
                // the EOF while parsing dimension Error
                // if we never find a digit.
                if is_white_space(b[0]) {
                    ret.extend(&b);
                } else if is_digit(b[0]) {
                    digit_start_found = true;
                    ret.extend(&b);
                }
            }
            Ok(1) => {
                // either we read a digit, in which case
                // we are good and keep looking for more digits
                // or we read a white space character, in which case
                // we have reached the end of our width field
                // or we read something else
                // in which case we have an error
                if is_digit(b[0]) {
                    ret.extend(&b);
                } else if is_white_space(b[0]) {
                    ret.extend(&b);
                    return Ok(ret);
                } else {
                    return Err(PPMError::BadHeader(
                        "Unexpected character when parsing dimensino in header".to_string(),
                    ));
                }
            }
            Ok(_) => {
                return Err(PPMError::BadHeader(
                    "EOF while parsing dimension".to_string(),
                ));
            }
            Err(error) => {
                return Err(PPMError::BadHeader(error.to_string()));
            }
        }
    }
}

fn bytes_to_ascii_string(bytes: Vec<u8>) -> Result<String, PPMError> {
    String::from_utf8(bytes).map_err(|_| PPMError::BadHeader("Bad String in header".to_string()))
}

pub fn string_to_u32(s: String) -> Result<u32, PPMError> {
    s.parse::<u32>()
        .map_err(|_| PPMError::BadHeader("Invalid number in header".to_string()))
}

fn parse_height_from_file(mut f: &mut impl Read) -> Result<Vec<u8>, PPMError> {
    // eprintln!("Parsing height");
    Ok(parse_dimension(&mut f)?)
}
