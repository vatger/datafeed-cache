[![Docker CI/CD](https://github.com/vatger/datafeed-cache/actions/workflows/prod.docker.yml/badge.svg)](https://github.com/vatger/datafeed-cache/actions/workflows/prod.docker.yml)
[![Code-Style Check](https://github.com/vatger/datafeed-cache/actions/workflows/dev.prettier.yml/badge.svg)](https://github.com/vatger/datafeed-cache/actions/workflows/dev.prettier.yml)

# Datafeed Cache

The Datafeed Cache project is a wrapper around [VATSIM's Datafeed service](https://data.vatsim.net/v3/vatsim-data.json) which provides basic caching functionality
as well as failure detection. Every response includes an extra flag `failed` which indicates whether the last update 
made by the server was successful, or whether the datafeed failed to update. If `failed` is true, then the previously cached datafeed is returned
as the response in `data`. 

The API Documentation can be found in the [wiki](https://github.com/vatger/datafeed-cache/wiki).

If you wish to contribute and/or make changes, please check out our contribution guide [here](CONTRIBUTING.md).

## Contact

|         Name         | Responsible for |      Contact       |
|:--------------------:|:---------------:|:------------------:|
| Nikolas G. - 1373921 |        *        | `git[at]vatger.de` |

## Prerequisites
- **Rust** (tested with version 1.85.1)
- **Cargo**

### Using Docker

Using Docker is possibly the easiest way to deploy the application. Using the included `docker-compose.yml`, you should 
simply be able to build and run the application by running `docker compose up`.
The included compose file exposes port `8000` on the host by default, however can be configured at will to fit any 
deployment scheme.

## Running the Application
`cargo run`
