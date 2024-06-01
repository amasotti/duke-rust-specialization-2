use csv::{DeserializeRecordsIter, Reader};
use serde::Deserialize;
use std::error::Error;
use std::fmt::Display;
use std::fs::File;


pub fn challenge_1() {
    let data = get_data_from_csv("./data/sample-data.csv").unwrap();
    let stats = calc_statistics(data);

    println!("Data Statistics: {}", stats);
}

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

fn get_data_from_csv(path: &str) -> Result<Vec<f64>, Box<dyn Error>> {
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

/// This struct holds the statistics of the data
///
/// It has the following fields:
/// - min: The minimum value in the data
/// - max: The maximum value in the data
/// - mean: The mean of the data
/// - variance: The variance of the data
/// - std_dev: The standard deviation of the data
struct Statistics {
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

fn calc_statistics(data: Vec<f64>) -> Statistics {
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

/// Calculate the minimum value in the data
///
/// ## How it is done in Rust
///
/// The `calc_min` function takes a reference to a vector of f64 values
/// and returns the minimum value in the vector.
///
/// It uses the `iter` method to get an iterator over the vector,
/// and the `min_by` method to get the minimum value.
/// the `unwrap` method is used to get the value from the `Option` returned by `min_by`.
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

/// Calculate the variance of the data
///
/// The variance of a set of data is the average of the squared differences of each value from the mean.
/// or in formula form:
///
/// `variance = Σ(x - mean)^2 / n`
///
/// ## How it is done in Rust
///
/// The `calc_variance` function takes a reference to a vector of f64 values
/// and returns the variance of the data.
///
/// It calculates the mean of the data using the `calc_mean` function,
/// then uses the `map` method to get the squared difference of each value from the mean.
/// The `sum` method is used to get the sum of the squared differences.
/// Finally, the sum is divided by the length of the data to get the variance.
///
/// ## See
/// [calc_mean](fn.calc_mean.html)
fn calc_variance(data: &Vec<f64>) -> f64 {
    let n = data.len() as f64;
    let mean = calc_mean(data);
    data.iter().map(|x| (x - mean).powi(2)).sum::<f64>() / n
}

/// Calculate the standard deviation of the data
fn calc_std_dev(data: &Vec<f64>) -> f64 {
    calc_variance(data).sqrt()
}
