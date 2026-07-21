# Seismiq Command Line Interface

Download the [latest release](https://github.com/seismiq-net/sqcli/releases) suitable for you operating system.

The `sensors` and `action` commands talk to the SeismiQ cloud API and need
credentials. Set them via two environment variables (a local `.env` file is
also picked up):

```shell
export SEISMIQ_USERNAME="your username" 
export SEISMIQ_PASSWORD="your password"
```

The `detect` command talks directly to devices on your LAN and needs no
authentication.

# Usage

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

### `sensors`

List the sensors registered to your SeismiQ account, fetched from the cloud
API. Only sensors seen **within the last hour** are shown; stale/offline ones
are omitted. Each row shows an icon for the hardware type, the sensor UID, its
software version, and how long ago it was last seen. Requires authentication.

```shell
sqcli sensors
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
