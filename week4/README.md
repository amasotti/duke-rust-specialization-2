# Week 4 - Rust for Data Engineering

Notes and Resources for Week 4 of the Rust for Data Engineering course.

## Content

### Part 1

- Intro in very basic Data Engineering Concepts (ETL Pipeline, Data Ingestion etc...)
- Demo with Rust and AWS Lambda (see previous experiments of mine: [SecurityHub to SNS via Cargo Lambda](https://github.com/amasotti/aws-securityhub-summary-rust))
  - Interesting fact: [Firecracker](https://firecracker-microvm.github.io/) is written in Rust! 
- Intro to [Distroless](https://github.com/GoogleContainerTools/distroless) containers (already seen in [Week3](../week3/5-axum-webservice))
- Demo with Rust and GCP (CloudRun)

__Resources__:

- [AWS Whitepaper - Build E2E Data Driven Applications on AWS](https://docs.aws.amazon.com/whitepapers/latest/build-e2e-data-driven-applications/build-e2e-data-driven-applications.html)
- [Google Whitepaper - Data Preparaation and Feature Engineering in ML](https://developers.google.com/machine-learning/data-prep)


### Part 2 - NLP with Rust

__Key Terms__;
- Hugging Face Hub: Model repository, the Github for ML models
- [Rustbert](https://github.com/guillaume-be/rust-bert) - NLP Transformer Models in Rust
- ONNX: Open standard for AI models
- Fine-tuning: process of training a pre-trained model on a new dataset
- Transfer Learning: process of using a pre-trained model on a new task


__Resources__:

- [Tract](https://github.com/sonos/tract) - Sonos Rust ML Inference Library
- [ONNX Models - Introduction](https://onnx.ai/onnx/intro/index.html)
- [HuggingFace Course Documentation](https://huggingface.co/learn/nlp-course/chapter1/1)
    - [HuggingFace Fine-Tuning](https://huggingface.co/learn/nlp-course/chapter3/1?fw=pt)


### Part 3 - SQL Solutions with Rust

__Key Terms__:
- SQLite: Embedded SQL Database
- Google Cloud SQL : Fully managed SQL Database
- Bigtable: NoSQL Database
- Spanner: Horizontally scalable SQL Database, good for global applications
- MemoryStore: In-memory data store
- Firestore: NoSQL Database (Document Database)

This section was mostly focused on GCP, especially on the SQL solutions available with some demos
running with Colab and BigQuery.