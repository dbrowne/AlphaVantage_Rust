# alpha_vantage_rust

![Maintenance](https://img.shields.io/badge/Maintained%3F-yes-green.svg)
![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)
![Build Status:](https://github.com/dbrowne/alpha_vantage_rust/actions/workflows/rust.yml/badge.svg)

A Rust API client and demonstration for the [Alpha Vantage](https://www.alphavantage.co/) API.

> 🚧 **Project Status**: Currently under active development with frequent updates.

## Table of Contents
- [Getting Started](#getting-started)
- [Usage](#usage)
- [Development Progress](#development-progress)
- [Contributing](#contributing)
- [License](#license)

## Getting Started

1. **Clone the repository:**
    ```sh
    git clone https://github.com/your_username/alpha_vantage_rust.git
    cd alpha_vantage_rust
    ```

2. **Set up the environment:**
   Create a `.env` file in the root directory and configure the following:

    ```dotenv
    DATABASE_URL=USERNAME://postgres:PASSWORD@localhost/sec_master
    OTHER_LISTED=[PATH_TO_THIS_FILE]/data/other-listed.csv
    ALPHA_VANTAGE_API_KEY="YOUR_ALPHA_VANTAGE_API_KEY"
    NASDAQ_LISTED=[PATH_TO_THIS_FILE]/data/nasdaq-listed_csv.csv
    ```

   Replace the placeholders (`USERNAME`, `PASSWORD`, etc.) with your actual values.

## Usage

This section will provide guidance on how to utilize the API client. (Expand upon this with examples, commands, etc.)

## Status Update:
### Finally getting back to this will have more updates and improved code coverage. 
### Current code coverage state: 8.92% coverage, 127/1424 lines covered.

## Development Progress:
- [x] Security loader
- [x] Security overview loader
- [x] Intraday price loader
- [x] open/close price loader
- [x] Symbol activity loader
- [x] News loader
- [x] Warning cleanup 
- [ ] Add crypto
- [ ] Error type handling ----- in progress
- [ ] db refactoring
- [ ] improve code coverage
- [ ] Add logging and enhance error handling
- [ ] Refactor code for production use 
- [ ] Front end

## DB Schema
![plot](schema.png)

## Contributing

1. Fork the repository.
2. Create a new branch (`git checkout -b feature/NewFeatureName`).
3. Commit your changes (`git commit -am 'Add a new feature'`).
4. Push to the branch (`git push origin feature/NewFeatureName`).
5. Open a Pull Request.

Your contributions are always welcome!

## License

This project is licensed under the MIT License. See the [LICENSE.md](LICENSE.md) file for details.
