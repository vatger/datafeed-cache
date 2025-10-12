[![Docker CI/CD](https://github.com/vatger/datafeed-cache/actions/workflows/prod.build.yml/badge.svg)](https://github.com/vatger/datafeed-cache/actions/workflows/prod.docker.yml)
[![Code-Style Check](https://github.com/vatger/datafeed-cache/actions/workflows/dev.build.yml/badge.svg)](https://github.com/vatger/datafeed-cache/actions/workflows/dev.prettier.yml)

# Datafeed Cache

The Datafeed Cache project is a wrapper around [VATSIM's Datafeed service](https://data.vatsim.net/v3/vatsim-data.json)
which provides basic caching functionality
as well as failure detection. Every response includes an extra flag `failed` which indicates whether the last update
made by the server was successful, or whether the datafeed failed to update. If `failed` is true, then the previously
cached datafeed is returned
as the response in `data`.

The API Documentation can be found in the [wiki](https://github.com/vatger/datafeed-cache/wiki).

If you wish to contribute and/or make changes, please check out our contribution guide [here](CONTRIBUTING.md).

## Contact

|         Name         | Responsible for |      Contact       |
|:--------------------:|:---------------:|:------------------:|
| Nikolas G. - 1373921 |        *        | `git[at]vatger.de` |

## Prerequisites

- **Rust** (version 1.85.1+)
- **Cargo**
- **OpenSSL** (required by *reqwest*)

## Running the Application

Firstly clone the repository by running:

```shell
$ git clone https://github.com/vatger/datafeed-cache.git  
$ cd datafeed-cache  
```

You can build the application using `cargo build -p datafeed-cache-server` or run it directly using `cargo run -p datafeed-cache-server`, which will include the
compilation steps.

You can also specify the type of release you would like to build (e.g. release) by specifying `cargo build --release -p datafeed-cache-server`.

### Using Docker

If you prefer to use Docker for production / development, you can use the provided `datafeed-cache-server/docker-compose.yml` file, or create
your own.
This compose-file will build the image and deploy it locally exposing port `8007` on the host machine.

```shell
$ docker compose up
```

To stop the deployed stack, run

```shell
$ docker compose down
```
