# AlphaVantage_Rust
Rust API and full stack demo for Alpha Vantage API.


This is a work in progress. daily updates.





create the follwoing .env file in the root dir with the following contents
``` 
DATABASE_URL=USERNAME://postgres:PASSWORD@localhost/sec_master
OTHER_LISTED=[Path_to_this_file]/data/other-listed.csv
ALPHA_VANTAGE_API_KEY="YOUR_ALPHA_VANTAGE_API_KEY"
NASDAQ_LISTED=[Path_to_this_file]/data/nasdaq-listed_csv.csv
```

Current Status:
- [x] Security loader 
- [x] Security overview loader
- [ ] Intraday price loader
- [ ] Open Close price loader
- [ ] Refactoring of security loader to async and multi-threading 