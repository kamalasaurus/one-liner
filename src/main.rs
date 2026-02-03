use std::env::args;
use std::io::{stdin, stdout, Write};
use bio::io::fasta;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = args().nth(1).unwrap_or_else(|| "-".to_string());

    let mut records = match input.as_str() {
        "-" => {
            Box::new(fasta::Reader::new(stdin().lock()).records()) as Box<dyn Iterator<Item = Result<fasta::Record, _>>>
        },
        _ => {
            Box::new(fasta::Reader::from_file(input)?.records()) as Box<dyn Iterator<Item = Result<fasta::Record, _>>>
        },
    };

    let mut writer = stdout();

    records.try_for_each(|record| -> Result<(), Box<dyn std::error::Error>> {
        let record = record.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        writeln!(writer, ">{}", record.id())?;
        writeln!(writer, "{}", String::from_utf8_lossy(record.seq()))?;
        Ok(())
    })?;

    Ok(())
}