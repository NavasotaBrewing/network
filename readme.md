# BCS network
This crate contains all the necessary networking components for the Brewery Control System to operate.

> note: this package is called `NavasotaBrewing/network`, but because the name needs to be more specific for crates.io, the crate is known as `bcs_network` for Brewery Control System-network. I wish crates.io had namespacing.

# Usage
There are 2 places you'd want to use this package:
1. The master station
2. The RTUs

See the [architecture page](https://github.com/NavasotaBrewing/readme/blob/master/architecture.md) for more information about layout.

On the master station, you don't need the device drivers because there's no hardware connected to it. By default, the drivers are missing to keep it lighweight:

```
$ cargo install bcs_network
$ bcs_network master
```

On the RTUs, you do need the device drivers:
```
$ cargo install bcs_network --features=rtu
$ bcs_network rtu
```

Cargo should handle everything for you.

## Specifying the Serial Port
On the RTUs, the default serial port that this crates uses is `/dev/ttyAMA0`. If this is incorrect, set the environment variable `BREWDRIVERS_PORT`. I use `/dev/ttyUSB0` on my development machine, so I would run

```
$ BREWDRIVERS_PORT=/dev/ttyUSB0 bcs_network rtu
```
