# minparse
A minimal argument parser
## Usage

```rust
use minparse::minparse;

minparse::process_name();
// => Get process name <String>
minparse::switches();
// => Get options that do not require a value (e.g --help) Vec<String>
minparse::subcommands();
// => Subcommands (do not have prefixed `--`. Includes process name) Vec<String> example: vec!["cargo", "run"] 
minparse::fields()
// => Fields with a value (e.g. --port 8080) HashMap<String, String> 
```
## Donate
```toml
# Wallets to which you can donate :)

  BTC = "3NC14JNuzdLkxJTdNa6bnFXaYzMKMc1Uwt"
  ETH = "0xc41c0f847d58121f552c683e454ddafeb415f25e"
  XMR = "875smxvwbP64MFZnHrHwHcGahoEB4a5ARGCBidr95LqL4GEPiB4T8J74UB5TzrXK3wbTZ1iidfYoV37KZq1vqWCQSNztDAF"  
```
