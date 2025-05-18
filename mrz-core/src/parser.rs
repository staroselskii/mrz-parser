use crate::{
    MRZFormat, MRZParseError, ParsedMRZ, MAX_DOC_NUM_LEN, MAX_FLIGHT_NUM_LEN, MAX_NAME_LEN,
    MAX_SEAT_LEN, MRZBCBP, MRZICAO,
};
use heapless::String;

pub fn detect_format(lines: &[&[u8]]) -> MRZFormat {
    if lines.len() == 2 && lines[0].starts_with(b"P<") {
        MRZFormat::ICAO
    } else if lines.len() == 1 && lines[0].starts_with(b"M1") {
        MRZFormat::BCBP
    } else {
        MRZFormat::Unknown
    }
}

pub fn parse_any(lines: &[&[u8]]) -> Result<ParsedMRZ, MRZParseError> {
    match detect_format(lines) {
        MRZFormat::ICAO => {
            if lines[0].len() < 44 || lines[1].len() < 44 {
                return Err(MRZParseError::InvalidLength);
            }
            Ok(parse_icao(lines[0], lines[1]))
        }
        MRZFormat::BCBP => {
            if lines[0].len() < 30 {
                return Err(MRZParseError::InvalidLength);
            }
            Ok(parse_bcbp(lines[0]))
        }
        MRZFormat::Unknown => Err(MRZParseError::UnknownFormat),
    }
}

fn parse_icao(line1: &[u8], line2: &[u8]) -> ParsedMRZ {
    let doc_num = &line2[0..9];
    let birth_date = &line2[13..19];
    let expiry_date = &line2[21..27];

    let mut document_number: String<MAX_DOC_NUM_LEN> = String::new();
    let mut name: String<MAX_NAME_LEN> = String::new();

    for &b in doc_num {
        let _ = document_number.push(char::from(b));
    }

    let name_field = &line1[5..44];
    for &b in name_field {
        if b == b'<' {
            let _ = name.push(' ');
        } else {
            let _ = name.push(b as char);
        }
    }

    ParsedMRZ::ICAO(MRZICAO {
        document_number,
        name,
        birth_date: birth_date.try_into().unwrap_or([b'0'; 6]),
        expiry_date: expiry_date.try_into().unwrap_or([b'0'; 6]),
    })
}

fn parse_bcbp(line: &[u8]) -> ParsedMRZ {
    let mut passenger_name = [b'<'; MAX_NAME_LEN];
    let mut flight_number = [b'<'; MAX_FLIGHT_NUM_LEN];
    let mut seat = [b'<'; MAX_SEAT_LEN];

    passenger_name[..18].copy_from_slice(&line[2..20]);
    flight_number[..6].copy_from_slice(&line[20..26]);
    seat[..4].copy_from_slice(&line[26..30]);

    ParsedMRZ::BCBP(MRZBCBP {
        passenger_name,
        flight_number,
        seat,
    })
}
