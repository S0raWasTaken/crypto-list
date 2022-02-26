# Crypto-list
For lack of a better name, crypto-list is a rofi-built UI for retrieving prices and additional info from cryptocurrencies.<br>

### Binary Dependencies
- `rofi`
### Build Dependencies
- `rustc`
- `cargo`
and probably `base-devel` (or `build-essentials` for debian-based systems)

## Installing
### Compiling from source
```bash
git clone https://github.com/S0raWasTaken/crypto-list
cd crypto-list
cargo build --release

# Alternatively, use 'sudo cp -iv target/release/crypto-list /usr/local/bin'
sudo install -Dm 755 target/release/crypto-list /usr/local/bin
```
### Making config file
```bash
mkdir -p ~/.config/crypto-list/
# Assuming you're in the clone dir
# Alternatively: 'cp -iv config.yml ~/.config/crypto-list/'
install -Dm 644 config.yml ~/.config/crypto-list/
```
Make sure to take a look at the config file, it's not hard to understand it.<br>
Remember to do not use tabs on the yaml document, use spaces instead.

## Automated install
### AUR
I'll post it someday I think

## TODOs:
- Write more TODOs
<br>
Yes, the description is a deadmau5 reference.
