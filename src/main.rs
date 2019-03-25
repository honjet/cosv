use colored::*;
use csv::Reader;
use std::io::{BufWriter, Write};
use Color::*;

const COLORS: &[Color] = &[Blue, Magenta, Cyan, Yellow, Green, Red];

fn main() -> Result<(), Box<std::error::Error>> {
    let mut reader = Reader::from_reader(std::io::stdin());
    let out = std::io::stdout();
    let mut writer = BufWriter::new(out.lock());

    for record in reader.records() {
        let mut colors = COLORS.iter().cycle();
        for item in record?.iter() {
            let color = colors.next().ok_or("Color is none.")?;
            let item = item.color(*color);
            writer.write(format!("{} ", item).as_bytes())?;
        }
        writer.write(b"\n")?;
    }
    Ok(())
}
