# Course 2 - Data Engineering with Rust

Part of the Coursera
specialization [Rust Programming Specialization](https://www.coursera.org/specializations/rust-programming) (Duke
University).

- Part 1: [Rust Programming](https://github.com/amasotti/duke-rust-specialization-1)
- Part 2: this repo
- Part 3: [Rust for DevOps](https://github.com/amasotti/duke-rust-specialization-3-devops)
- Part 4: [Rust with Linux Command Line CLI Tools](https://www.coursera.org/learn/python-rust-linux)
- Part 5: [Rust for ML Ops](https://www.coursera.org/learn/rust-llmops)


This is the second course of the specialization, which focuses on data engineering with Rust. It is all about performant
handling of date, ETL pipelines and MLOps.

__Instructor__: [Noah Gift](https://noahgift.com/)

### Structure

- [Week1](week1) - Introduction
    - Quite theoretical presentation of the course and the amazing power of AI as coding assistant
    - [Minimal Rust service](week1/0-simple-rust-api) (using actix-web for a simple API)
    - [Sequences in Rust](week1/1-rust-sequences) (vectors, vecDeque, linked lists, etc.)
    - Rust Graphs and Miscellanous Data Structures (graph algorithms, trees, pageRank, etc.)
- [Week2](week2) - Rust & Security
  - MFA, Network segmentation, Least privilege, Zero Trust, Encryption etc.
  - [Advanced operations with vecs](week2/1-advanced-vecs)
  - [Data Race and Mutex](week2/2-data-race)
  - Simple ciphers
  - SHA3 Hashing
  - [Concurrency Problems](week2/5-dining-philosophers)
  - [Crawling Webpages and analyzing them](week2/6-wikipedia-crawl)
  - [PyTorch Bindings for Rust](week2/7-gpu-stress)
- [Week3](week3) - Rust & Data Engineering
  - [Cargo Lambda, Serveless Rust](week3/1-cargo-lambda)
  - Rust & AWS
  - Rust & GCP
  - [Rust & Jupyter Notebooks via evcxr](week3/2-evcxr-jupyter)
  - [Pola.rs for Data Engineering](week3/3-eda-polars)
  - [Building a simple web microservice with Axum & Rust](week3/5-axum-webservice)
- [Week4](week4) - Data Storage and Processing
  - [Rust & SQLite](week4/rusqlite)
  - Introduction to LLMs, ONNX, Hugging Face and PyTorch Rust bindings
  - Final Assessment

## Run

I've chosen to organize my snippets in separate cargo projects, under the directories
named after the course week. They don't follow exactly the same structure as the course.

The [just](https://just.systems/man/en/chapter_1.html) file is used to confortably run the snippets
in the different subfolders without having to manually navigate to them.

```sh
# In the justfile I've given custom names to the subfolders (see below)
# simple-api -> week1/0-simple-rust-api/simple-api

# To run them, just type
just run simple-api 

# To test them
just test simple-api

# To build
just build simple-api

# To lint
just lint simple-api 

# To clean
just clean simple-api
```

Some aliases are defined (just lazyness):

- `just t` -> `just test`
- `just r` -> `just run`
- `just b` -> `just build`
- `just l` -> `just lint`
- `just c` -> `just clean`

## Resources

- [Rust Playground](https://play.rust-lang.org/?version=stable&mode=debug&edition=2021)

### Coursera Discussions

- [Week1 - Disappointment](https://www.coursera.org/learn/data-engineering-rust/discussions/forums/kUkT2yC8Ee6_Lgoc7ulpMw/threads/RROzRfxWEe66og4Z2poFYQ)
