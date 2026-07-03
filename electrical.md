# Sigma — Electrical, Lighting & Harness

*Companion to `README.md`. The bike's nervous system below the ECU: the bespoke harness, power distribution/protection, the battery, road-legal lighting, and the switchgear that feeds the custom ECU. The ECU, connector standards and charging/power budget live in `electronics.md` (§3 connectors, §9 charging); hand-control ergonomics in `bodywork.md`. Flat-black finish and the sourcing rule come from the `README.md` hub.*

---

## 1 · Harness architecture

| Item | Spec | Status |
|---|---|---|
| Loom | **Bespoke** — replaces the donor Yamaha loom entirely; built to the ECU pinout. Connector standards (AMPSEAL 16, Deutsch DT/DTM, Yamaha OEM at the engine) are speced in `electronics.md` §3 | `[BESPOKE]` |
| Topology | Star/zoned from a central power-distribution point; keep sensor grounds clean and separate from load grounds (signal integrity — `electronics.md` §4) | `[BESPOKE]` |
| Sleeving / routing | Motorsport sleeve; routed off vibration paths; strain-relieved at every connector | `[PENDING]` |

## 2 · Power distribution & protection

| Item | Spec | Status |
|---|---|---|
| Fuse / relay box | Central sealed fuse + relay block (e.g., automotive micro-fuse + ISO relays, or a solid-state PDM). A **PDM** pairs well with the custom ECU for programmable load-shedding (`electronics.md` §9) | `[PENDING]` |
| Main / ignition | Ignition switch or keyless + main relay; interacts with the immobiliser/CAN handshake decision (`electronics.md` §7) | `[PENDING]` |
| Charging feed | Factory stator + **SH847-class series reg/rec** → battery (decision + power budget in `electronics.md` §9) | `[BUY]` (per §9) |
| Reverse / transient protection | Automotive-grade at the DC-DC and PDM inputs (the cockpit DC-DC front-end is speced in `electronics.md` §8) | `[BESPOKE]` |

## 3 · Battery

| Item | Spec | Status |
|---|---|---|
| Type | **LiFePO4** — the idle/transient buffer chosen in `electronics.md` §9; lighter than lead-acid | `[BUY]` |
| Sizing | To cold-crank the CP3 + hold the worst-case idle deficit (~200 W short-term, per the §9 load budget) | `[PENDING]` |
| Charging profile | Lithium-appropriate regulated voltage; confirm the reg/rec set-point suits LiFePO4 + a cold-behaviour check | `[PENDING]` |
| Placement | Under seat / in subframe (`bodywork.md`); mass kept central + low for the geometry target | `[PENDING]` |

## 4 · Lighting

*All LED (the ~40–60 W lighting load in the `electronics.md` §9 budget). Every fitting must meet the market's road-legal / ECE approval — logged in `emissions_certification.md`'s equipment checklist.*

| Item | Spec | Status |
|---|---|---|
| Headlight | LED — round café unit (low/high + DRL); aim + intensity to approval | `[PENDING]` |
| Tail / brake | LED tail+stop in the cowl/tail tidy (`bodywork.md`) | `[PENDING]` |
| Indicators | LED — front + rear; load-independent flasher (the ECU/relay must not false-flash on LED low draw) | `[PENDING]` |
| Plate light + reflectors | Road-legal plate illumination + required reflectors | `[PENDING]` |

## 5 · Switchgear & control interface

| Item | Spec | Status |
|---|---|---|
| Bar switches | Start / kill, hi-lo beam, indicators, horn, cockpit mode — wired to the ECU (discrete or CAN switch pack) | `[PENDING]` |
| Safety interlocks | Kill switch + sidestand + clutch/neutral logic into the ECU start-permit | `[PENDING]` |
| Horn | Road-legal horn | `[PENDING]` |
| Telltales | ABS/oil/high-beam/turn/warnings render on the cockpit (`electronics.md` §8 M7 cluster) — bar tell-tales optional | — |

## 6 · Open items `[PENDING]`

- Choose **fuse box vs PDM** early — a PDM unlocks the firmware load-shedding in `electronics.md` §9 and simplifies the loom.
- Finalize the **battery size + placement** against the §9 load budget and the geometry/mass target.
- Assemble the **road-legal lighting/horn/mirror** set for EU IVA / UK MSVA (feeds `emissions_certification.md`).

---

# Caveats (electrical)

1. **The loom is bespoke and safety-adjacent** — it carries ride-by-wire, ignition and the charging feed; connector integrity (`electronics.md` §3/§4) is where noise, CAN errors and no-starts trace to. Build it to motorsport standard.
2. **Lighting is an approval item, not just styling** — LED head/tail/indicators must meet the target markets' road-legal standards; log them in the homologation checklist.
