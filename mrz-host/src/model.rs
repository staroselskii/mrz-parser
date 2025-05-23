use time::Date;

#[derive(Debug)]
pub struct MrzIcaoUnified {
    document_number: String,
    name: String,
    birth_date: Option<Date>,
    expiry_date: Option<Date>,
    sex: char,
    optional_data1: String,
    optional_data2: String,
    final_check: Option<bool>,
    nationality: String,
    issuing_state: String,
    document_code: String,
    format: String,
}

impl MrzIcaoUnified {
    pub fn new(
        document_number: String,
        name: String,
        birth_date: Option<Date>,
        expiry_date: Option<Date>,
        sex: char,
        optional_data1: String,
        optional_data2: String,
        final_check: Option<bool>,
        nationality: String,
        issuing_state: String,
        document_code: String,
        format: String,
    ) -> Self {
        Self {
            document_number,
            name,
            birth_date,
            expiry_date,
            sex,
            optional_data1,
            optional_data2,
            final_check,
            nationality,
            issuing_state,
            document_code,
            format,
        }
    }

    pub fn document_number(&self) -> &str {
        &self.document_number
    }
    pub fn name(&self) -> &str {
        &self.name
    }
    pub fn birth_date(&self) -> Option<Date> {
        self.birth_date
    }
    pub fn expiry_date(&self) -> Option<Date> {
        self.expiry_date
    }
    pub fn sex(&self) -> char {
        self.sex
    }
    pub fn optional_data1(&self) -> &str {
        &self.optional_data1
    }
    pub fn optional_data2(&self) -> &str {
        &self.optional_data2
    }
    pub fn final_check(&self) -> Option<bool> {
        self.final_check
    }
    pub fn nationality(&self) -> &str {
        &self.nationality
    }
    pub fn issuing_state(&self) -> &str {
        &self.issuing_state
    }
    pub fn document_code(&self) -> &str {
        &self.document_code
    }
    pub fn format(&self) -> &str {
        &self.format
    }
}

#[derive(Debug)]
pub enum MRZ {
    Icao(MrzIcaoUnified),
    Unknown,
}
