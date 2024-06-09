set shell := ["bash", "-c"]

alias l := lint
alias b := build
alias t := test
alias r := run
alias c := clean

# Simplified project path determination using a function-like approach
set-proj lesson:
    # Use a case-like structure for pattern matching
    @case "{{lesson}}" in \
        expl) echo "./scratchpad/exploration";; \
        img-gray) echo "./scratchpad/image-grayscaler";; \
        simple-api) echo "./week1/0-simple-rust-api/simple-api";; \
        sequences) echo "./week1/1-rust-sequences/sequences";; \
        fruit-cli) echo "./week1/2-fruit-salad/fruit-cli";; \
        count-freq) echo "./week1/3-count-freq/count-freq";; \
        graph-demo) echo "./week1/4-graph-demo/graph-demo";; \
        bin-heap) echo "./week1/5-binary-heap/bin-heap";; \
        week1-challenge) echo "./week1/6-week1-challenge/week1-challenge";; \
        advanced-vec) echo "./week2/1-advanced-vecs/mutable-salad";; \
        data-race) echo "./week2/2-data-race/data-race-lab";; \
        caeser-cipher) echo "./week2/3-caeser-cipher/caeser-cipher";; \
        duplicate-sha3) echo "./week2/4-duplicate-sha3/duplicate-sha3";; \
        dining-philosophers) echo "./week2/5-dining-philosophers/dining-philosophers";; \
        wikipedia-crawl) echo "./week2/6-wikipedia-crawl/wikipedia-crawl";; \
        eda-polars) echo "./week3/3-eda-polars/eda-polars";; \
        eda-polars-cli) echo "./week3/4-eda-polars-cli/eda_polars_cli";; \
        web-service) echo "./week3/5-axum-webservice/axum_web";; \
        *) echo "Error: Project '{{lesson}}' not found" >&2; exit 1;; \
    esac

build project:
    @echo "Building {{project}}"
    @cd $(just set-proj {{project}}) && cargo build

test project:
    @echo "Testing {{project}}"
    @cd $(just set-proj {{project}}) && cargo test

run project:
    @echo "Running {{project}}"
    @cd $(just set-proj {{project}}) && cargo run

lint project: install-fmt install-clippy
    @echo "Linting {{project}}"
    cd $(just set-proj {{project}}) && cargo fmt --all && cargo clippy

clean project:
    @echo "Cleaning {{project}}"
    @cd $(just set-proj {{project}}) && cargo clean

add project lib:
    @echo "Adding {{lib}} to {{project}}"
    @cd $(just set-proj {{project}}) && cargo add {{lib}}

install-fmt:
    rustup component add rustfmt 2>/dev/null

install-clippy:
    rustup component add clippy 2>/dev/null
