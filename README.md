[![Docker CI/CD](https://github.com/vatger/datafeed-cache/actions/workflows/prod.docker.yml/badge.svg)](https://github.com/vatger/datafeed-cache/actions/workflows/prod.docker.yml)
[![Code-Style Check](https://github.com/vatger/datafeed-cache/actions/workflows/dev.prettier.yml/badge.svg)](https://github.com/vatger/datafeed-cache/actions/workflows/dev.prettier.yml)

# Datafeed Cache

The Datafeed Cache project is a wrapper around [VATSIM's Datafeed service](https://data.vatsim.net/v3/vatsim-data.json)
which provides basic caching functionality
as well as failure detection. Every response includes an extra flag `failed` which indicates whether the last update
made by the server was successful, or whether the datafeed failed to update. If `failed` is true, then the previously
cached datafeed is returned
as the response in `data`.

The API Documentation can be found in the [wiki](https://github.com/vatger/datafeed-cache/wiki).
> [!IMPORTANT]  
> The wiki still contains the older typescript type syntax. Semantically, the result types are identical though.
> To view the concrete implementation of the types used, visit [the datafeed type definitions](./src/vatsim/types.rs).


If you wish to contribute and/or make changes, please check out our contribution guide [here](CONTRIBUTING.md).

## Contact

|         Name         | Responsible for |      Contact       |
|:--------------------:|:---------------:|:------------------:|
| Nikolas G. - 1373921 |        *        | `git[at]vatger.de` |

## Prerequisites

- **Rust** (tested with version 1.85.1)
- **Cargo**
- **OpenSSL** (required by *reqwest*)

## Running the Application

Firstly clone the repository by running:

```shell
$ git clone https://github.com/vatger/datafeed-cache.git  
$ cd datafeed-cache  
```

You can build the application using `cargo build` or run it directly using `cargo run`, which will include the
compilation steps.

You can also specify the type of release you would like to build (e.g. release) by specifying `cargo build --release`.

### Using Docker

If you prefer to use Docker for production / development, you can use the provided `docker-compose.yml` file, or create
your own.
This compose-file will build the image and deploy it locally exposing port `8007` on the host machine.

```shell
$ docker compose up
```

To stop the deployed stack, run

```shell
$ docker compose down
```