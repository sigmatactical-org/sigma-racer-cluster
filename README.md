# Sigma Racer Cluster

[![CI](https://github.com/sigmatactical-org/sigma-racer-cluster/actions/workflows/ci.yml/badge.svg)](https://github.com/sigmatactical-org/sigma-racer-cluster/actions/workflows/ci.yml)
[![License](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)
[![MSRV](https://img.shields.io/badge/MSRV-1.97.0-blue.svg)](https://www.rust-lang.org)

Production instrument cluster application for the **Sigma Racer Wingman** motorcycle
display — ships as the `sigma-racer-cluster` binary on i.MX 8M Plus / i.MX 95 class hardware.

| Binary | Role |
|--------|------|
| `sigma-racer-cluster` | Full-screen Slint cluster UI (CAN-FD telemetry from M7 safety core) |

## Quick start

Requires sibling checkouts under `embedded/`:

```
embedded/
├── sigma-instrumentation/   # UI library + telemetry crate
└── sigma-racer-cluster/     # this repo
```

```bash
# Production binary (idle telemetry — same as embedded target)
cargo run --bin sigma-racer-cluster

# Panel-accurate local testing (800×480, matches imx8mp / QEMU virt)
cd ../sigma-instrumentation && cargo virt
```

## Telemetry / CAN test sources

The live source is chosen at runtime by `CLUSTER_TELEMETRY_SOURCE`. All sources
decode into `VehicleState`, map to `ClusterTelemetry`, and call
`sigma_instrumentation::apply_telemetry` — the UI never sees raw CAN.

| `CLUSTER_TELEMETRY_SOURCE` | Source |
|----------------------------|--------|
| `ipc` *(default)* | Subscribe to `sigma-racer-vehicle` over its Unix socket (production behaviour). |
| `replay` | Replay a `candump -L` log through the DBC decoder with original timing, looping. Uses the baked-in [`testdata/sample-ride.log`](testdata/sample-ride.log) unless `CLUSTER_REPLAY_LOG` points elsewhere. |
| `can` / `socketcan` | Read live frames off a SocketCAN interface (`CLUSTER_CAN_IFACE`, default `vcan0`). Requires `--features can-socket`. |

```bash
# Replay the baked sample ride — no hardware, no daemon
CLUSTER_TELEMETRY_SOURCE=replay cargo run

# Replay your own capture
CLUSTER_REPLAY_LOG=/path/to/ride.log CLUSTER_TELEMETRY_SOURCE=replay cargo run

# Live SocketCAN off vcan0 (bring the bus up first)
sudo scripts/vcan-up.sh vcan0
CLUSTER_TELEMETRY_SOURCE=can cargo run --features can-socket
# …then feed it, e.g.:  canplayer -I testdata/sample-ride.log vcan0=can1
```

Regenerate the baked sample from the scripted ride:

```bash
cargo run --example gen_sample_log > testdata/sample-ride.log
```

> The `can` source consumes whatever populates the bus. On the vehicle the M7
> safety core owns the CAN traffic and gateways it onto the Linux-visible bus;
> for the bench, `vcan0` + `canplayer`/`cansend` (or the sample replay) stand in.

## Embedded build (Wingman)

The Yocto recipe builds **`sigma-racer-cluster`** from this crate:

```bash
bitbake sigma-racer-cluster
```

| Item | Value |
|------|-------|
| Binary | `/usr/bin/sigma-racer-cluster` |
| systemd | `cluster-ui.service` |
| Environment | `/etc/sigma-racer-wingman/ui.env` |

Full distribution docs: [`sigma-racer-wingman`](../sigma-racer-wingman/README.md).

## Requirements

- Rust 1.86+ (Yocto meta-rust scarthgap)
- Slint 1.13.1 (pinned for Yocto Rust 1.86)
- [`sigma-instrumentation`](../sigma-instrumentation/) workspace (library + telemetry)

## Brand & artwork

© Sigma Tactical Group. **All rights reserved.**

The Sigma Tactical Group name, logos, marks, artwork, and visual identity are **proprietary**. They are not covered by this repository's source-code license. See [BRANDING.md](BRANDING.md).

## License

MIT OR Apache-2.0 — see `LICENSE-MIT` and `LICENSE-APACHE`.
