use core::convert::AsRef;
use sp_std::vec::Vec;

use crate::StellarSdkError;

const ALPHABET: &'static [u8; 32] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ234567";

fn ascii_to_value_5bit(char: u8) -> Option<u8> {
    match char as char {
        'a'..='z' => Some(char - ('a' as u8)),
        'A'..='Z' => Some(char - ('A' as u8)),
        '2'..='7' => Some(char - ('2' as u8) + 26),
        '0' => Some(14),
        '1' => Some(8),
        _ => None,
    }
}

pub fn encode<T: AsRef<[u8]>>(binary: T) -> Vec<u8> {
    let mut buffer = Vec::with_capacity(binary.as_ref().len() * 2);
    let mut shift = 3;
    let mut carry = 0;

    for byte in binary.as_ref().iter() {
        let value_5bit = if shift == 8 { carry } else { carry | ((*byte) >> shift) };
        buffer.push(ALPHABET[(value_5bit & 0x1f) as usize]);

        if shift > 5 {
            shift -= 5;
            let value_5bit = (*byte) >> shift;
            buffer.push(ALPHABET[(value_5bit & 0x1f) as usize]);
        }

        shift = 5 - shift;
        carry = *byte << shift;
        shift = 8 - shift;
    }

    if shift != 3 {
        buffer.push(ALPHABET[(carry & 0x1f) as usize]);
    }

    buffer
}

pub fn decode<T: AsRef<[u8]>>(string: T) -> Result<Vec<u8>, StellarSdkError> {
    let mut result = Vec::with_capacity(string.as_ref().len());
    let mut shift: i8 = 8;
    let mut carry: u8 = 0;

    for (position, ascii) in string.as_ref().iter().enumerate() {
        if *ascii as char == '=' {
            break
        }

        let value_5bit = ascii_to_value_5bit(*ascii);
        if let Some(value_5bit) = value_5bit {
            shift -= 5;
            if shift > 0 {
                carry |= value_5bit << shift;
            } else if shift < 0 {
                result.push(carry | (value_5bit >> -shift));
                shift += 8;
                carry = value_5bit << shift;
            } else {
                result.push(carry | value_5bit);
                shift = 8;
                carry = 0;
            }
        } else {
            return Err(StellarSdkError::InvalidBase32Character { at_position: position })
        }
    }

    if shift != 8 && carry != 0 {
        result.push(carry);
    }

    Ok(result)
}
