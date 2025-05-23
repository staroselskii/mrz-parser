use mrz_host::parse_lines;
use mrz_host::MRZ;

fn main() {
    let td3 = [
        "P<UTOERIKSSON<<ANNA<MARIA<<<<<<<<<<<<<<<<<<<",
        "L898902C36UTO7408122F1204159ZE184226B<<<<<<<",
    ];

    let td1 = [
        "I<UTOD231458907<<<<<<<<<<<<<<<",
        "7408122F1204159UTO<<<<<<<<<<<6",
        "ERIKSSON<<ANNA<MARIA<<<<<<<<<<",
    ];

    println!("--- TD3 ---");
    handle_parse(&td3);

    println!("--- TD1 ---");
    handle_parse(&td1);
}

fn handle_parse(lines: &[&str]) {
    match parse_lines(lines) {
        Ok(MRZ::Icao(data)) => {
            println!("Document: {}", data.document_number);
            println!("Name:     {}", data.name);
            println!("Nationality:     {}", data.nationality);
            println!("Issuer:          {}", data.issuing_state);
            println!(
                "Birth:    {}",
                data.birth_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "Invalid".into())
            );
            println!(
                "Expiry:   {}",
                data.expiry_date
                    .map(|d| d.to_string())
                    .unwrap_or_else(|| "Invalid".into())
            );
        }
        Ok(_) => println!("Parsed, but not ICAO."),
        Err(e) => println!("Parse error: {:?}", e),
    }
}
