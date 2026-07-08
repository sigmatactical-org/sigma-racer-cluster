# Sigma Racer Cluster

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

## License

MIT OR Apache-2.0 — see `LICENSE-MIT` and `LICENSE-APACHE`.
