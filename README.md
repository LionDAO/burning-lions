# Burning Lions NFT Collection
Currently a standard CW721 collection with room for future expansions. The collection is minted over the LionDAO's Burn Festival of 12 months.

## Building
The contracts can be built with `build.sh` on systems with bash. The script will need to be ported for other shells, such as zsh (MacOS) or powershell (Windows). The script compiles the unoptimized WASM with `cargo wasm` and then minifies and optimizes it with the standard docker image.

# License
Licensed under Apache 2.0
