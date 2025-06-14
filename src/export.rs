use crate::models::OrderBookL1;
use csv::Writer;
use std::error::Error;

pub fn export_to_csv(data: Vec<OrderBookL1>, file_path: &str) -> Result<(), Box<dyn Error>> {
    let mut wtr = Writer::from_path(file_path)?;
    for record in data {
        wtr.serialize(record)?;
    }
    wtr.flush()?;
    Ok(())
}