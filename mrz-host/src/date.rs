use time::{Date, Month};

pub fn parse_mrz_date_with_reference(date: &[u8; 6], reference: Option<&[u8; 6]>) -> Option<Date> {
    let year = core::str::from_utf8(&date[0..2])
        .ok()?
        .parse::<u16>()
        .ok()?;
    let month = core::str::from_utf8(&date[2..4]).ok()?.parse::<u8>().ok()?;
    let day = core::str::from_utf8(&date[4..6]).ok()?.parse::<u8>().ok()?;

    let full_year = if let Some(ref_date) = reference {
        let ref_year = core::str::from_utf8(&ref_date[0..2])
            .ok()?
            .parse::<u16>()
            .ok()?;
        let ref_century = if ref_year >= 50 { 1900 } else { 2000 };
        let ref_full_year = ref_century + ref_year;
        let candidate = 1900 + year;
        if candidate > ref_full_year {
            2000 + year
        } else {
            candidate
        }
    } else {
        if year >= 50 {
            1900 + year
        } else {
            2000 + year
        }
    };

    let month = Month::try_from(month).ok()?;
    Date::from_calendar_date(full_year as i32, month, day).ok()
}
