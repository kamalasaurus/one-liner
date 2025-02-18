use std::env::args;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use bio::io::fasta;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = args().nth(1).unwrap();
    let input_path = Path::new(&input);
    let basename = input_path
        .file_stem()
        .ok_or("Invalid input file name")?
        .to_str()
        .ok_or("Invalid UTF-8 in file name")?;
    let parent = input_path.parent().unwrap_or(Path::new("."));
    let output_file_name = format!("{}_oneline.fasta", basename);
    let output_path = parent.join(output_file_name);
    let mut writer = File::create(output_path)?;
    
    let reader = fasta::Reader::from_file(input)?;
    reader.records().try_for_each(|record| -> Result<(), Box<dyn std::error::Error>> {
        let record = record.map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;
        writeln!(writer, ">{}", record.id())?;
        writeln!(writer, "{}", String::from_utf8_lossy(record.seq()))?;
        Ok(())
    })?;
    
    Ok(())
}