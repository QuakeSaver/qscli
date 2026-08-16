# Seismiq Command Line Interface

# Installation

Download the artifact for your operating system from the
[latest release](https://github.com/seismiq-net/sqcli/releases), then follow
the steps below. Replace `<version>` (e.g. `1.1.0`) with the release you
downloaded.

## Debian / Ubuntu

Install the `.deb` package. It places `sqcli` on your `PATH` automatically:

```shell
sudo apt install ./sqcli-<version>.deb
# or: sudo dpkg -i sqcli-<version>.deb
```

## Linux (other distributions, x86_64)

Download the bare `sqcli` binary, make it executable, and move it onto your
`PATH`:

```shell
chmod +x sqcli
sudo mv sqcli /usr/local/bin/sqcli
```

## macOS (Apple Silicon)

Download `sqcli-osx`, make it executable, and move it onto your `PATH`. macOS
quarantines files downloaded from the browser, so clear that flag as well:

```shell
chmod +x sqcli-osx
xattr -d com.apple.quarantine sqcli-osx   # only needed for browser downloads
sudo mv sqcli-osx /usr/local/bin/sqcli
```

## Windows

Download `sqcli.exe` and place it in a folder that is on your `PATH` (or add its
folder to `PATH`). You can then run `sqcli` from PowerShell or Command Prompt.

# Usage

The `sensors` and `action` commands talk to the SeismiQ cloud API and need
credentials. Set them via two environment variables (a local `.env` file is
also picked up):

```shell
export SEISMIQ_USERNAME="your username" 
export SEISMIQ_PASSWORD="your password"
```

The `detect` command talks directly to devices on your LAN and needs no
authentication.

Get a list of available commands:

```shell
sqcli -h
```

## Commands

### `detect [interface]`

Discover SeismiQ sensors on your **local network**. It probes every host in
your machine's `/24` subnet on port `27740` and prints the ones that answer.
By default it uses your primary network interface; pass an interface name
(e.g. `en0`, `eth0`) to base the scan on a different one.

```shell
sqcli detect
```

```text
IP ADDRESS      UUID     VERSION  TYPE
192.168.178.55  A3B7K9Q2  dev      MEMS
192.168.178.89  unknown   dev      HiDRA
```

### `sensors [-f <filter>...] [-s <column>] [-r]`

List the sensors registered to your SeismiQ account, fetched from the cloud
API. Requires authentication. Every sensor is listed unless you filter the
list. Each row shows an icon for the hardware revision, the sensor UID, its
hardware family, whether it is online, its software version, how long ago it
was last seen, and its number of warnings.

```shell
sqcli sensors
```

```text
    UID         TYPE     STATUS   VERSION     LAST SEEN  WARN
▣   A3B7K9Q2    MEMS     online   1.0.0       5m 0s
🌀  XYZ12345    HiDRA    offline  1.12.0-rc1  3d 1h
🌋  LONGERUID1  MEMS     offline  1.0.0       1h 15m     3
```

#### Filtering

Pass `--filter`/`-f` to narrow the list. Repeat the flag or comma-separate the
values:

- `online` — seen within the last hour.
- `offline` — not seen within the last hour.
- `mems` — MEMS sensors (ADXL or BMA accelerometer).
- `hidra` — HiDRA sensors.
- `unknown` — sensors with a hardware revision `sqcli` does not recognise.
- `warnings` — sensors carrying at least one warning.

Filters of the same kind widen the selection, filters of different kinds narrow
it. So `-f mems -f hidra` lists both hardware families, while
`-f mems,hidra -f online` lists only the online ones among them:

```shell
sqcli sensors -f online              # what the bare command used to show
sqcli sensors -f offline -f hidra    # HiDRA units that dropped off
sqcli sensors -f warnings            # anything reporting a problem
```

#### Sorting

Pass `--sort`/`-s` to order the list by `uid` (the default), `last-seen`,
`first-seen`, `version`, `type`, or `warnings`. Add `--reverse`/`-r` to flip
the order. Sensors sharing a value are ordered by UID.

```shell
sqcli sensors -s last-seen           # freshest first
sqcli sensors -s last-seen -r        # longest silent first
sqcli sensors -f offline -s version  # stale units, grouped by firmware
```

### `action <action> <sensor-uid>`

Send a remote command to a single sensor (identified by its UID) through the
cloud API. Requires authentication. Available actions:

- `reboot` — reboot the sensor.
- `blink` — blink the sensor's LED, handy for physically locating a unit.
- `check-update` — tell the sensor to check for a firmware update.

```shell
sqcli action blink A3B7K9Q2
```
