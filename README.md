[![Docker CI/CD](https://github.com/vatger/datafeed-cache/actions/workflows/prod.docker.yml/badge.svg)](https://github.com/vatger/datafeed-cache/actions/workflows/prod.docker.yml)
[![Code-Style Check](https://github.com/vatger/datafeed-cache/actions/workflows/dev.prettier.yml/badge.svg?branch=dev)](https://github.com/vatger/datafeed-cache/actions/workflows/dev.prettier.yml)

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
- **Node.js** (https://nodejs.org/en)
- **Typescript** (Optional) 
  - Typescript is included in the project's dependencies, can however be installed globally using `npm install -g typescript`. 
  This may however not be necessary in your situation and is more of a recommendation rather than a requirement. 

## Running the Application

### Using Node.js

Assuming node.js is installed, running the application (locally) should be as simple as executing the following two commands. 

1. Run `npm install`
2. Run `npm run start:dev`

### Using Docker

Using the included `docker-compose.yml` you should simply be able to build and run the application. 
The included compose file exposes port `8000` on the host by default, however can be configured at will to fit any deployment scheme.
