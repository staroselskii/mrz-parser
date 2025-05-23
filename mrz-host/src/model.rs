use time::Date;

#[derive(Debug)]
pub struct MrzIcaoUnified {
    pub document_number: String,
    pub name: String,
    pub birth_date: Option<Date>,
    pub expiry_date: Option<Date>,
    pub sex: char,
    pub optional_data1: String,
    pub optional_data2: String,
    pub final_check: Option<bool>,
    pub nationality: String,
    pub issuing_state: String,
    pub document_code: String,
    pub format: String,
}

#[derive(Debug)]
pub enum MRZ {
    Icao(MrzIcaoUnified),
    Unknown,
}
