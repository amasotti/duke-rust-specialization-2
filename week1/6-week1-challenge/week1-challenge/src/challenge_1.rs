use csv::{DeserializeRecordsIter, Reader};
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;

/// This struct represents a record in the CSV file
/// The file itself is just a random one I googled
#[derive(Debug, Deserialize)]
struct Record {
    #[serde(rename = "Series_reference")]
    reference: String,
    #[serde(rename = "Period")]
    period: String,
    #[serde(rename = "Value")]
    value: f64,
    #[serde(rename = "STATUS")]
    status: String,
    #[serde(rename = "UNITS")]
    units: String,
}

pub fn get_data_from_csv(path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
    // Initialize CSV reader
    let mut reader = Reader::from_path(path).expect("Could not read file");
    // Initialize vector to store records
    let mut records = Vec::new();

    // Deserialize the CSV records into the vector
    let mut iter: DeserializeRecordsIter<File, Record> = reader.deserialize();

    for record in iter {
        if let Ok(Record { value, .. }) = record {
            records.push(value);
        }
    }

    Ok(records)
}

pub struct Statistics {
    min: f64,
    max: f64,
    mean: f64,
    variance: f64,
    std_dev: f64,
}

impl Display for Statistics {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "\n| {:<10} | {:<20} |", "Field", "Value")?;
        writeln!(f, "|------------|----------------------|")?;
        writeln!(f, "| {:<10} | {:<20} |", "Min", self.min)?;
        writeln!(f, "| {:<10} | {:<20} |", "Max", self.max)?;
        writeln!(f, "| {:<10} | {:<20} |", "Mean", self.mean)?;
        writeln!(f, "| {:<10} | {:<20} |", "Variance", self.variance)?;
        writeln!(f, "| {:<10} | {:<20} |", "Std Dev", self.std_dev);
        writeln!(f, "|------------|----------------------|")
    }
}

pub fn calc_statistics(data: Vec<f64>) -> Statistics {
    let n = data.len() as f64;
    let min = calc_min(&data);
    let max = calc_max(&data);
    let mean = calc_mean(&data);
    let variance = calc_variance(&data);
    let std_dev = calc_std_dev(&data);

    Statistics {
        min,
        max,
        mean,
        variance,
        std_dev,
    }
}

fn calc_min(data: &Vec<f64>) -> f64 {
    *data
        .iter()
        .min_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn calc_max(data: &Vec<f64>) -> f64 {
    *data
        .iter()
        .max_by(|a, b| a.partial_cmp(b).unwrap())
        .unwrap()
}

fn calc_mean(data: &Vec<f64>) -> f64 {
    data.iter().sum::<f64>() / data.len() as f64
}

fn calc_variance(data: &Vec<f64>) -> f64 {
    let n = data.len() as f64;
    let mean = calc_mean(data);
    data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n
}

fn calc_std_dev(data: &Vec<f64>) -> f64 {
    calc_variance(data).sqrt()
}
