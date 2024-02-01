# Introduction

This project is trying to provide a basic upload and sharing system.
The detail technical spec is described in the google docs provided below
https://docs.google.com/document/d/1g6BvaeYmA4Yb3sNFfgfrOfCE0oVkSdi_xuBVTdLImI4/edit#heading=h.km65ay44yuru


# Getting Started

## Prerequirement
You need to ensure the following tools is properly installed in your system
- docker
- docker-compose
- rust (https://rustup.rs/)

## Build the container
- issue `sudo docker build -t thundershare/thundershare-backend:v0.1.4 .`

## Run the infra
- issue `sudo docker-compose up`

The docker compose currently use v0.1.4, so if the tag version is changed, please remember to update the docker-compose as well

## Perform Local Build
- issue `cargo build`

## Perform Local Server
- issue `sudo docker-compose up db`
- Create a .env file like the following example
- issue `TMPDIR=./ cargo run`

```.env
SERVER_HOST=0.0.0.0
SERVER_PORT=8080
DB_HOST=localhost
DB_PORT=5432
DB_NAME=thundershare
DB_USER=pgsql
DB_PASS=password
```
  
## Perform Unit Test
- issue `cargo test`

# Future works

- Provide the Frontend to access the RESTFUL API with ease.
- Provide a cache layer to improve the performance
- Provide integration test so that we do not need to constanly perfrom curl to test
