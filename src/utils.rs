use anyhow::Result;

pub fn parse_u16(input: &str) -> Result<u16> {
    if let Some(hex) = input.strip_prefix("0x") {
        Ok(u16::from_str_radix(hex, 16)?)
    } else if let Some(bin) = input.strip_prefix("0b") {
        Ok(u16::from_str_radix(bin, 2)?)
    } else {
        Ok(input.parse::<u16>()?)
    }
}
