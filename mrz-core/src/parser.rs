use crate::checksum::compute_checksum;
use crate::field_correction::correct_checked_field;
use crate::{
    CheckedField, MRZChecksumError, MRZFormat, MRZParseError, MrzIcaoTd3, ParsedMRZ,
    ICAO_COMMON_COUNTRY_CODE_LEN, ICAO_COMMON_DATE_LEN, ICAO_COMMON_DOC_NUM_MAX_LEN,
    ICAO_TD1_OPTIONAL1_MAX_LEN, ICAO_TD1_OPTIONAL2_MAX_LEN, ICAO_TD3_NAME_MAX_LEN,
    ICAO_TD3_OPTIONAL1_MAX_LEN, ICAO_TD3_OPTIONAL2_MAX_LEN,
};
use core::fmt::Write;
use heapless::String;

// Maximum number of permutations to try for field correction.
const MAX_FIELD_PERMUTATIONS: usize = 8;

fn parse_document_number<const N: usize>(
    line: &[u8],
    start: usize,
    end: usize,
    check: usize,
) -> Result<CheckedField<String<N>>, MRZParseError> {
    let field_str =
        core::str::from_utf8(&line[start..end]).map_err(|_| MRZParseError::Utf8Error)?;
    let check_char = line[check] as char;

    // Try checksum directly first
    let raw_bytes = field_str.as_bytes();
    if verify_checksum(raw_bytes, check_char as u8) {
        let mut buf = String::new();
        for &b in raw_bytes {
            let _ = buf.push(b as char);
        }
        return Ok(CheckedField::new(buf, None));
    }

    // If checksum is invalid, try correction
    correct_checked_field::<N, MAX_FIELD_PERMUTATIONS, String<N>>(
        field_str,
        check_char,
        MAX_FIELD_PERMUTATIONS,
        MRZChecksumError::DocumentNumber,
    )
}

fn parse_date<const N: usize>(
    line: &[u8],
    start: usize,
    end: usize,
    check: usize,
    kind: MRZChecksumError,
) -> Result<CheckedField<[u8; N]>, MRZParseError> {
    let (date, valid) = checked_field::<N>(&line[start..end], line[check]);
    if !valid {
        Err(MRZParseError::InvalidChecksumField(kind))
    } else {
        Ok(CheckedField::new(date, None))
    }
}

fn validate_final_check(
    segments: &[&[u8]],
    check_char: u8,
    kind: MRZChecksumError,
) -> Result<Option<bool>, MRZParseError> {
    if check_char == b'<' {
        Ok(None)
    } else {
        match compute_composite_checksum(segments, check_char) {
            Some(true) => Ok(Some(true)),
            Some(false) => Err(MRZParseError::InvalidChecksumField(kind)),
            None => Ok(None),
        }
    }
}
fn parse_checked_field<const N: usize>(
    field: &[u8],
    check: u8,
    kind: MRZChecksumError,
) -> Result<CheckedField<[u8; N]>, MRZParseError> {
    let (data, valid) = checked_field::<N>(field, check);
    if !valid {
        Err(MRZParseError::InvalidChecksumField(kind))
    } else {
        Ok(CheckedField::new(data, None))
    }
}

fn verify_checksum(data: &[u8], check_digit: u8) -> bool {
    match compute_checksum(data) {
        Some(csum) => csum == (check_digit - b'0'),
        None => false,
    }
}

fn compute_composite_checksum<'a>(segments: &[&'a [u8]], check_digit: u8) -> Option<bool> {
    if (b'0'..=b'9').contains(&check_digit) {
        let mut final_check_data: heapless::Vec<u8, 64> = heapless::Vec::new();
        for segment in segments {
            final_check_data.extend_from_slice(segment).ok()?;
        }
        let checksum = compute_checksum(&final_check_data);
        return checksum.map(|csum| csum == (check_digit - b'0'));
    } else {
        None
    }
}

fn checked_field<const N: usize>(field: &[u8], check_digit: u8) -> ([u8; N], bool) {
    let array = field.try_into().unwrap_or([b'0'; N]);
    let valid = verify_checksum(&array, check_digit);
    (array, valid)
}

fn fixed_slice<const N: usize>(slice: &[u8]) -> [u8; N] {
    slice.try_into().unwrap_or([b' '; N])
}

fn decode_range<const N: usize>(slice: &[u8]) -> String<N> {
    let mut out = String::new();
    for &b in slice {
        let _ = out.push(b as char);
    }
    out
}

fn decode_mrz_td_name<const N: usize>(raw: &str) -> String<N> {
    let mut given_names_buf: String<N> = String::new();
    let mut parts = raw.split("<<");
    let surname = parts.next().unwrap_or("").trim_end();
    let given = parts.next().unwrap_or("");

    for c in given.chars() {
        let ch = if c == '<' { ' ' } else { c };
        let _ = given_names_buf.push(ch);
    }
    while given_names_buf.ends_with(' ') {
        given_names_buf.pop();
    }

    let mut full_name: String<N> = String::new();
    let _ = write!(full_name, "{}<<{}", surname, given_names_buf);
    full_name
}

/// Detects the MRZ format (e.g., TD1, TD3) based on the provided lines.
/// Returns `MRZFormat::Unknown` if the format cannot be determined.
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

/// Parses any supported MRZ format from the provided lines.
/// Returns an error if the format is unknown or the lines are malformed.
pub fn parse_any(lines: &[&[u8]]) -> Result<ParsedMRZ, MRZParseError> {
    match detect_format(lines) {
        MRZFormat::MrzIcaoTd3 => {
            if lines.len() != 2 || lines[0].len() < 44 || lines[1].len() < 44 {
                return Err(MRZParseError::InvalidLength);
            }
            parse_td3(lines[0], lines[1])
        }
        MRZFormat::MrzIcaoTd1 => {
            if lines.len() != 3
                || lines[0].len() != 30
                || lines[1].len() != 30
                || lines[2].len() != 30
            {
                return Err(MRZParseError::InvalidLength);
            }
            parse_td1(lines[0], lines[1], lines[2])
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

fn parse_td3(line1: &[u8], line2: &[u8]) -> Result<ParsedMRZ, MRZParseError> {
    const DOC_NUM_START: usize = 0;
    const DOC_NUM_END: usize = 9;
    const DOC_NUM_CHECK: usize = 9;

    const BIRTH_DATE_START: usize = 13;
    const BIRTH_DATE_END: usize = 19;
    const BIRTH_DATE_CHECK: usize = 19;

    const EXPIRY_DATE_START: usize = 21;
    const EXPIRY_DATE_END: usize = 27;
    const EXPIRY_DATE_CHECK: usize = 27;

    const FINAL_CHECK_POS: usize = 43;

    const NAME_START: usize = 5;
    const NAME_END: usize = 44;

    let document_number = parse_document_number::<ICAO_COMMON_DOC_NUM_MAX_LEN>(
        line2,
        DOC_NUM_START,
        DOC_NUM_END,
        DOC_NUM_CHECK,
    )?;

    let birth_date = parse_date::<ICAO_COMMON_DATE_LEN>(
        line2,
        BIRTH_DATE_START,
        BIRTH_DATE_END,
        BIRTH_DATE_CHECK,
        MRZChecksumError::BirthDate,
    )?;

    let expiry_date = parse_date::<ICAO_COMMON_DATE_LEN>(
        line2,
        EXPIRY_DATE_START,
        EXPIRY_DATE_END,
        EXPIRY_DATE_CHECK,
        MRZChecksumError::ExpiryDate,
    )?;

    let final_check_char = line2.get(FINAL_CHECK_POS).copied().unwrap_or(b'<');
    let final_check = if final_check_char == b'<' {
        None
    } else {
        let mut docnum_with_check = [0u8; 10];
        docnum_with_check[..ICAO_COMMON_DOC_NUM_MAX_LEN]
            .copy_from_slice(document_number.value().as_bytes());
        docnum_with_check[ICAO_COMMON_DOC_NUM_MAX_LEN] = line2[DOC_NUM_CHECK];

        let segments = &[
            &docnum_with_check,
            &line2[BIRTH_DATE_START..=BIRTH_DATE_CHECK],
            &line2[EXPIRY_DATE_START..=EXPIRY_DATE_CHECK],
            &line2[EXPIRY_DATE_CHECK + 1..=FINAL_CHECK_POS - 1],
        ];
        Some(CheckedField::new(
            (),
            match validate_final_check(segments, final_check_char, MRZChecksumError::Final) {
                Ok(Some(true)) => None,
                Ok(Some(false)) => Some(MRZChecksumError::Final),
                Ok(None) => None,
                Err(e) => return Err(e),
            },
        ))
    };

    let raw_name = decode_range::<ICAO_TD3_NAME_MAX_LEN>(&line1[NAME_START..NAME_END]);
    let name = decode_mrz_td_name::<ICAO_TD3_NAME_MAX_LEN>(&raw_name);

    let optional_data1 =
        decode_range::<ICAO_TD3_OPTIONAL1_MAX_LEN>(&line2[28..43.min(line2.len())]);
    let optional_data2 =
        decode_range::<ICAO_TD3_OPTIONAL2_MAX_LEN>(&line1[28..43.min(line1.len())]);
    let sex = line2.get(20).copied().unwrap_or(b'<');

    Ok(ParsedMRZ::MrzIcaoTd3(MrzIcaoTd3 {
        document_code: fixed_slice::<2>(&line1[0..2]),
        issuing_state: fixed_slice::<3>(&line1[2..5]),
        nationality: fixed_slice::<3>(&line2[15..18]),
        name,
        document_number,
        birth_date,
        expiry_date,
        final_check,
        sex,
        optional_data1: optional_data1.clone(),
        optional_data2: optional_data2.clone(),
    }))
}

// Parse ICAO TD1 MRZ format
fn parse_td1(line1: &[u8], line2: &[u8], line3: &[u8]) -> Result<ParsedMRZ, MRZParseError> {
    use crate::{MrzIcaoTd1, ICAO_TD1_NAME_MAX_LEN};

    const DOC_CODE_START: usize = 0;
    const DOC_CODE_END: usize = 2;
    const DOC_CODE_LEN: usize = 2;

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

    let document_code = fixed_slice::<DOC_CODE_LEN>(&line1[DOC_CODE_START..DOC_CODE_END]);
    let issuing_state =
        fixed_slice::<ICAO_COMMON_COUNTRY_CODE_LEN>(&line1[ISSUER_START..ISSUER_END]);

    let document_number = parse_document_number::<ICAO_COMMON_DOC_NUM_MAX_LEN>(
        line1,
        DOC_NUM_START,
        DOC_NUM_END,
        DOC_NUM_CHECK,
    )?;

    let optional_data1 =
        decode_range::<ICAO_TD1_OPTIONAL1_MAX_LEN>(&line1[OPTIONAL1_START..OPTIONAL1_END]);

    let nationality =
        fixed_slice::<ICAO_COMMON_COUNTRY_CODE_LEN>(&line2[NATIONALITY_START..NATIONALITY_END]);

    let birth_date = parse_date::<ICAO_COMMON_DATE_LEN>(
        line2,
        BIRTH_DATE_START,
        BIRTH_DATE_END,
        BIRTH_DATE_CHECK,
        MRZChecksumError::BirthDate,
    )?;

    let sex = line2[SEX_POS];

    let expiry_date = parse_date::<ICAO_COMMON_DATE_LEN>(
        line2,
        EXPIRY_DATE_START,
        EXPIRY_DATE_END,
        EXPIRY_DATE_CHECK,
        MRZChecksumError::ExpiryDate,
    )?;

    let optional_data2 =
        decode_range::<ICAO_TD1_OPTIONAL2_MAX_LEN>(&line2[OPTIONAL2_START..OPTIONAL2_END]);

    let final_check_char = line2.get(FINAL_CHECK_POS).copied().unwrap_or(b'<');
    let final_check = if final_check_char == b'<' {
        None
    } else {
        let mut docnum_with_check = [0u8; 10];
        docnum_with_check[..ICAO_COMMON_DOC_NUM_MAX_LEN]
            .copy_from_slice(document_number.value().as_bytes());
        docnum_with_check[ICAO_COMMON_DOC_NUM_MAX_LEN] = line1[DOC_NUM_CHECK];

        let segments = &[
            &docnum_with_check,
            &line2[BIRTH_DATE_START..=BIRTH_DATE_CHECK],
            &line2[EXPIRY_DATE_START..=EXPIRY_DATE_CHECK],
            &line2[OPTIONAL2_START..FINAL_CHECK_POS],
        ];
        Some(CheckedField::new(
            (),
            match validate_final_check(segments, final_check_char, MRZChecksumError::Final) {
                Ok(Some(true)) => None,
                Ok(Some(false)) => Some(MRZChecksumError::Final),
                Ok(None) => None,
                Err(e) => return Err(e),
            },
        ))
    };

    let raw_name = decode_range::<ICAO_TD1_NAME_MAX_LEN>(&line3[NAME_START..NAME_END]);
    let full_name = decode_mrz_td_name::<ICAO_TD1_NAME_MAX_LEN>(&raw_name);

    Ok(ParsedMRZ::MrzIcaoTd1(MrzIcaoTd1 {
        document_code,
        issuing_state,
        name: full_name,
        nationality,
        optional_data1: optional_data1.clone(),
        optional_data2: optional_data2.clone(),
        document_number,
        birth_date,
        expiry_date,
        final_check,
        sex,
    }))
}
