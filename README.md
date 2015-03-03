# Yahoo Quote

A small command line program that gets stock/currency prices from Yahoo.

This is just an experiment so I can get my hands dirty with Rust.

## Usage

    yahoo_quote AAPL GOOG MSFT AUDUSD=X

Prints...

    Apple Inc. (AAPL) 129.025 @ 3/3/2015 9:36am
    Google Inc. (GOOG) 570.878 @ 3/3/2015 9:36am
    Microsoft Corpora (MSFT) 43.80 @ 3/3/2015 9:36am
    AUD to USD (AUDUSD=X) 0.7834 @ 3/3/2015 9:51am

## Rust Vesion

    rustc 1.0.0-nightly (b4c965ee8 2015-03-02) (built 2015-03-02)

## Tests

    cargo test

## Build

    cargo build

## License

The MIT License (MIT).

See [LICENSE](LICENSE)
