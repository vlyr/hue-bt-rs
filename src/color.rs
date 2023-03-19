use std::fmt;

#[derive(Debug)]
pub enum ColorError {
    InvalidHex,
}

impl fmt::Display for ColorError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ColorError::InvalidHex => write!(f, "Invalid hex color code"),
        }
    }
}

impl ::std::error::Error for ColorError {}

pub fn convert(rgb: &[u8; 3]) -> Result<[u8; 4], ColorError> {
    let scale = 0xff;
    let mut adjusted = rgb.iter().map(|&chan| chan.max(1)).collect::<Vec<u8>>();
    let total: u32 = adjusted.iter().map(|&chan| chan as u32).sum();
    adjusted = adjusted
        .iter()
        .map(|&chan| ((chan as f32 / total as f32) * scale as f32 + 0.5) as u8)
        .collect::<Vec<u8>>();

    if adjusted.iter().sum::<u8>() > scale {
        println!("AIJAIJAIAJ");
        return Err(ColorError::InvalidHex);
    }

    Ok([0x1, adjusted[0], adjusted[2], adjusted[1]])
}

pub fn color(hexstr: &str) -> Result<[u8; 4], ColorError> {
    let hexstr_no_hash = if hexstr.len() == 7 {
        &hexstr[1..]
    } else {
        hexstr
    };

    let color_bits = [
        u8::from_str_radix(&hexstr_no_hash[..2], 16).map_err(|_| ColorError::InvalidHex)?,
        u8::from_str_radix(&hexstr_no_hash[2..4], 16).map_err(|_| ColorError::InvalidHex)?,
        u8::from_str_radix(&hexstr_no_hash[4..], 16).map_err(|_| ColorError::InvalidHex)?,
    ];

    convert(&color_bits)
}
