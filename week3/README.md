# Week 3 - Rust for Data Engineering

__Focus__: I/O Operations and File Handling

## Key concepts and Resources

### Part 1

- How to use the [`csv` library](https://docs.rs/csv/latest/csv/)
    - <small>Not reproduced here, since I've already used it many times</small>
- [Cargo Lambda](https://www.cargo-lambda.info/) -> **AMAZING!**
    - Short guide-through about *AWS SDK for Rust* (stable since Nov 2023)
- [Parquet crate](https://crates.io/crates/parquet) for processing Parquet files
    - [Parquet file format](https://parquet.apache.org/)

### Part 2

- Key concepts in Data Analysis (with Rust)
    - Data Frames
    - EDA (Exploratory Data Analysis)
    - Notebooks
    - ML Flow
    - [Polars](https://pola.rs) - Rust Dataframe Library
    - [evcxr - Evaluation Context for Rust](https://github.com/evcxr/evcxr/blob/main/evcxr_jupyter/README.md) - Allows
      you to run Rust code in Jupyter Notebooks

### EDA with Pola.rs 

__Note__: You will need `libfontconfig1-dev` installed to be able to compile the `week3/3-eda-polars/eda-polars` example.

```bash
# On Debian based systems
sudo apt-get install libfontconfig1-dev

# On fedora and other rpm based systems
sudo dnf install fontconfig-devel
```

Also mentioned in the lesson: [Data Bricks](https://www.databricks.com/), a
collaborative data science platform that allows you to run ML Workloads in the cloud.
