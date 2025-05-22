use crate::{
    MRZFormat, MRZParseError, MrzIcaoTd3, ParsedMRZ, ICAO_TD1_DOC_NUM_MAX_LEN,
    ICAO_TD3_DOC_NUM_MAX_LEN, ICAO_TD3_NAME_MAX_LEN,
};
use heapless::String;

fn compute_checksum(data: &[u8]) -> u8 {
    fn char_value(c: u8) -> u8 {
        match c {
            b'0'..=b'9' => c - b'0',
            b'A'..=b'Z' => c - b'A' + 10,
            b'<' => 0,
            _ => panic!("Unexpected character in MRZ for checksum: {:?}", c),
        }
    }
    let weights = [7, 3, 1];
    let sum = data
        .iter()
        .enumerate()
        .map(|(i, &b)| {
            let val = char_value(b);
            let weight = weights[i % 3];
            let product = val as u32 * weight as u32;
            product
        })
        .sum::<u32>();

    (sum % 10) as u8
}

fn verify_checksum(data: &[u8], check_digit: u8) -> bool {
    compute_checksum(data) == (check_digit - b'0')
}

fn decode_mrz_filler<const N: usize>(slice: &[u8]) -> String<N> {
    let mut out = String::new();
    for &b in slice {
        let _ = out.push(if b == b'<' { ' ' } else { b as char });
    }
    out
}

pub fn detect_format(lines: &[&[u8]]) -> MRZFormat {
    if lines.len() == 2
        && lines[0].starts_with(b"P")
        && lines[0].len() >= 40
        && lines[1].len() >= 40
    {
        MRZFormat::MrzIcaoTd3 {}
    } else if lines.len() == 3
        && lines[0].len() == 30
        && lines[1].len() == 30
        && lines[2].len() == 30
    {
        MRZFormat::MrzIcaoTd1
    } else if lines.len() == 1 && lines[0].starts_with(b"M1") {
        MRZFormat::BCBP
    } else {
        MRZFormat::Unknown
    }
}

pub fn parse_any(lines: &[&[u8]]) -> Result<ParsedMRZ, MRZParseError> {
    match detect_format(lines) {
        MRZFormat::MrzIcaoTd3 => {
            if lines.len() != 2 || lines[0].len() < 44 || lines[1].len() < 44 {
                return Err(MRZParseError::InvalidLength);
            }
            Ok(parse_td3(lines[0], lines[1]))
        }
        MRZFormat::MrzIcaoTd1 => {
            if lines.len() != 3
                || lines[0].len() != 30
                || lines[1].len() != 30
                || lines[2].len() != 30
            {
                return Err(MRZParseError::InvalidLength);
            }
            Ok(parse_td1(lines[0], lines[1], lines[2]))
        }
        MRZFormat::BCBP => {
            if lines[0].len() < 30 {
                return Err(MRZParseError::InvalidLength);
            }
            return Err(MRZParseError::UnsupportedFormat);
        }
        MRZFormat::Unknown => Err(MRZParseError::UnknownFormat),
    }
}

fn parse_td3(line1: &[u8], line2: &[u8]) -> ParsedMRZ {
    let doc_num = &line2[0..9];
    let birth_date = &line2[13..19];
    let expiry_date = &line2[21..27];

    let birth_date_check = line2[19];
    let expiry_date_check = line2[27];
    let birth_valid = verify_checksum(birth_date, birth_date_check);
    let expiry_valid = verify_checksum(expiry_date, expiry_date_check);

    let (final_check, final_check_valid) = if let Some(&ch) = line2.get(43) {
        if (b'0'..=b'9').contains(&ch) {
            let mut final_check_data: heapless::Vec<u8, 64> = heapless::Vec::new();
            final_check_data.extend_from_slice(&line2[0..10]).unwrap();  // Document number + check
            final_check_data.extend_from_slice(&line2[13..20]).unwrap(); // Birth + check
            final_check_data.extend_from_slice(&line2[21..28]).unwrap(); // Expiry + check
            final_check_data.extend_from_slice(&line2[28..43]).unwrap(); // Optional + check
            let is_valid = compute_checksum(&final_check_data) == (ch - b'0');
            (Some(ch), Some(is_valid))
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let document_number = decode_mrz_filler::<ICAO_TD3_DOC_NUM_MAX_LEN>(doc_num);
    let name = decode_mrz_filler::<ICAO_TD3_NAME_MAX_LEN>(&line1[5..44]);

    ParsedMRZ::MrzIcaoTd3(MrzIcaoTd3 {
        document_number,
        name,
        birth_date: birth_date.try_into().unwrap_or([b'0'; 6]),
        expiry_date: expiry_date.try_into().unwrap_or([b'0'; 6]),
        birth_date_check,
        expiry_date_check,
        birth_date_check_valid: birth_valid,
        expiry_date_check_valid: expiry_valid,
        final_check,
        final_check_valid,
    })
}

// Parse ICAO TD1 MRZ format
fn parse_td1(line1: &[u8], line2: &[u8], line3: &[u8]) -> ParsedMRZ {
    use crate::{MrzIcaoTd1, ICAO_TD1_NAME_MAX_LEN};
    use heapless::String;

    const DOC_CODE_START: usize = 0;
    const DOC_CODE_END: usize = 2;

    const ISSUER_START: usize = 2;
    const ISSUER_END: usize = 5;

    const DOC_NUM_START: usize = 5;
    const DOC_NUM_END: usize = 14;
    const DOC_NUM_CHECK: usize = 14;

    const OPTIONAL1_START: usize = 15;
    const OPTIONAL1_END: usize = 30;

    const BIRTH_DATE_START: usize = 0;
    const BIRTH_DATE_END: usize = 6;
    const BIRTH_DATE_CHECK: usize = 6;

    const SEX_POS: usize = 7;

    const EXPIRY_DATE_START: usize = 8;
    const EXPIRY_DATE_END: usize = 14;
    const EXPIRY_DATE_CHECK: usize = 14;

    const NATIONALITY_START: usize = 15;
    const NATIONALITY_END: usize = 18;

    const OPTIONAL2_START: usize = 18;
    const OPTIONAL2_END: usize = 29;

    const FINAL_CHECK_POS: usize = 29;

    const NAME_START: usize = 0;
    const NAME_END: usize = 30;

    let document_code = line1[DOC_CODE_START..DOC_CODE_END]
        .try_into()
        .unwrap_or([b' '; 2]);
    let issuing_state = line1[ISSUER_START..ISSUER_END]
        .try_into()
        .unwrap_or([b' '; 3]);
    let mut document_number: String<ICAO_TD1_DOC_NUM_MAX_LEN> = String::new();
    for &b in &line1[DOC_NUM_START..DOC_NUM_END] {
        let _ = document_number.push(b as char);
    }
    let document_number_check = line1[DOC_NUM_CHECK];
    let doc_valid = verify_checksum(&line1[DOC_NUM_START..DOC_NUM_END], document_number_check);

    let mut optional_data1: String<15> = String::new();
    for &b in &line1[OPTIONAL1_START..OPTIONAL1_END] {
        let _ = optional_data1.push(b as char);
    }

    let nationality = line2[NATIONALITY_START..NATIONALITY_END]
        .try_into()
        .unwrap_or([b' '; 3]);

    let birth_date = line2[BIRTH_DATE_START..BIRTH_DATE_END]
        .try_into()
        .unwrap_or([b'0'; 6]);
    let birth_date_check = line2[BIRTH_DATE_CHECK];
    let birth_date_check_value = compute_checksum(&birth_date);
    let birth_valid = verify_checksum(&birth_date, birth_date_check);

    let sex = line2[SEX_POS];

    let expiry_date = line2[EXPIRY_DATE_START..EXPIRY_DATE_END]
        .try_into()
        .unwrap_or([b'0'; 6]);
    let expiry_date_check = line2[EXPIRY_DATE_CHECK];
    let expiry_date_check_value = compute_checksum(&expiry_date);
    let expiry_valid = verify_checksum(&expiry_date, expiry_date_check);

    let mut optional_data2: String<11> = String::new();
    for &b in &line2[OPTIONAL2_START..OPTIONAL2_END] {
        let _ = optional_data2.push(b as char);
    }

    let (final_check, final_check_valid) = if let Some(&ch) = line2.get(FINAL_CHECK_POS) {
        if (b'0'..=b'9').contains(&ch) {
            let mut final_check_data: heapless::Vec<u8, 50> = heapless::Vec::new();

            // Composite checksum per ICAO 9303:
            final_check_data
                .extend_from_slice(&line1[DOC_NUM_START..OPTIONAL1_END])
                .unwrap(); // Line 1: positions 6–30
            final_check_data
                .extend_from_slice(&line2[BIRTH_DATE_START..=BIRTH_DATE_CHECK])
                .unwrap(); // Line 2: 1–7 (include check digit)
            final_check_data
                .extend_from_slice(&line2[EXPIRY_DATE_START..=EXPIRY_DATE_CHECK])
                .unwrap(); // Line 2: 9–15 (include check digit)
            final_check_data
                .extend_from_slice(&line2[OPTIONAL2_START..FINAL_CHECK_POS])
                .unwrap(); // Line 2: 19–29

            let final_valid = compute_checksum(&final_check_data) == (ch - b'0');
            (Some(ch), Some(final_valid))
        } else {
            (None, None)
        }
    } else {
        (None, None)
    };

    let name = decode_mrz_filler::<ICAO_TD1_NAME_MAX_LEN>(&line3[NAME_START..NAME_END]);

    ParsedMRZ::MrzIcaoTd1(MrzIcaoTd1 {
        document_code,
        issuing_state,
        name,
        document_number,
        document_number_check,
        nationality,
        birth_date,
        birth_date_check,
        birth_date_check_valid: birth_valid,
        sex,
        expiry_date,
        expiry_date_check,
        expiry_date_check_valid: expiry_valid,
        optional_data1,
        optional_data2,
        final_check: final_check,
        document_number_check_valid: doc_valid,
        final_check_valid,
    })
}
