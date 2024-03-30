use apache_avro::{Error, Schema};
use env_logger;
use std::fs::File;
use std::io::Read;

fn main() -> Result<(), Error> {
    // Enable logging
    env_logger::init();
    // Path to your schema.json file

    let mut schema_strings = Vec::new();

    let files = vec![
        "User.json",
        "Host.json",
        // "CompanyOffice.json",
        // "schema.json",
    ];

    for file in files {
        let mut temp_content = String::new();
        // Read the schema file content
        let mut main_schema = File::open(file).expect("Error opening schema file");
        main_schema
            .read_to_string(&mut temp_content)
            .expect("Error reading schema file");
        schema_strings.push(temp_content.clone());
        temp_content.clear();
    }

    for line in &schema_strings {
        println!("{:?}", line);
        println!("-----------------------------------------");
    }

    // Handle parsing and validation results
    let newvec: Vec<&str> = schema_strings.iter().map(AsRef::as_ref).collect();
    println!("======================================");
    println!("{:?}", newvec);
    println!("======================================");
    let _schema = Schema::parse_list(&newvec)?;
    println!("{:?}", _schema);

    Ok(())
}
