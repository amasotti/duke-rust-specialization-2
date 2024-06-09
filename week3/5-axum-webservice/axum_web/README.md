# Axum web service

_exercise from [Rust Axum Greedy Coin Microservice](https://gitlab.com/dukeaiml/duke-coursera-labs/rust-axum-greedy-coin-microservice)_

~~~shell
docker build -t greedy_changer:latest .
docker run -dp 3000:3000 greedy_changer:latest #runs in background

docker ps
 
~~~

Cool exercise, because it builds the image using a distroless base image.
__Note__: It is necessary to build for `musl`, since the distroless images don't contain much and the builds 
need to be statically linked.