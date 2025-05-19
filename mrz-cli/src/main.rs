use mrz_host::parse_lines;
use mrz_host::MRZ;

fn main() {
    let lines = [
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<",
        "L898902C36UTO7408122F1204159ZE184226B<<<<<<<",
    ];
    match parse_lines(&lines) {
        Ok(MRZ::IcaoTd3(data)) => {
            println!("Document: {}", data.document_number);
            println!("Name:     {}", data.name);
            println!(
                "Birth:    {}",
                data.birth_date
                    .map(|d| d.to_string())
                    .unwrap_or("Invalid".into())
            );
            println!(
                "Expiry:   {}",
                data.expiry_date
                    .map(|d| d.to_string())
                    .unwrap_or("Invalid".into())
            );
        }
        Ok(_) => println!("Parsed, but not ICAO."),
        Err(e) => println!("Parse error: {:?}", e),
    }

    let td1 = [
        "I<UTOD231458907<<<<<<<<<<<<<<<",
        "7408122F1204159UTO<<<<<<<<<<<6",
        "ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];

    match parse_lines(&td1) {
        Ok(MRZ::IcaoTd1(data)) => {
            println!("Document: {}", data.document_number);
            println!("Name:     {}", data.name);
            println!("Nationality:     {}", data.nationality);
            println!("Issuer:          {}", data.issuing_state);
            println!(
                "Birth:    {}",
                data.birth_date
                    .map(|d| d.to_string())
                    .unwrap_or("Invalid".into())
            );
            println!(
                "Expiry:   {}",
                data.expiry_date
                    .map(|d| d.to_string())
                    .unwrap_or("Invalid".into())
            );
        }
        Ok(_) => println!("Parsed, but not ICAO."),
        Err(e) => println!("Parse error: {:?}", e),
    }
}
