# BCS network
This crate contains all the necessary networking components for the BCS to operate.

> note: this package is called `NavasotaBrewing/network`, but because the name needs to be more specific for crates.io, the crate is known as `bcs_network` for Brewery Control System-network. I wish crates.io had namespacing.

[`brewdrivers`](https://github.com/NavasotaBrewing/brewdrivers) is all the device drivers and a CLI to use them; [`brewkit`](https://github.com/NavasotaBrewing/brewkit) is the web interface for the brewer to use; this crate is the networking components that link these two together.

Originally, everything here was part of `brewdrivers`, which was convenient but cumbersome. Compile times were very long on the RPi, and the executable had grown to 100+ MB for debug. This pulls about 85 MB debug out of `brewdrivers` so it can remain thinner.

# Usage
There are 2 places you'd want to use this package:
1. The master station
2. The RTUs

See the [architecture page](https://github.com/NavasotaBrewing/readme/blob/master/architecture.md) for more information about layout.

On the master station, you don't need the device drivers because there's no hardware connected to it:

```
$ cargo install bcs_network
$ bcs_network master
```

On the RTUs, you do need `brewdrivers`:
```
$ cargo install bcs_network --features=rtu
$ bcs_network rtu
```

Cargo should handle everything for you.
