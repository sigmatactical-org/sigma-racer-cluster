# Sigma — Electrical, Lighting & Harness

*Companion to `README.md`. The bike's nervous system below the ECU: the bespoke harness, power distribution/protection, the battery, road-legal lighting, and the switchgear that feeds the custom ECU. The ECU, connector standards and charging/power budget live in `electronics.md` (§3 connectors, §9 charging); hand-control ergonomics in `bodywork.md`. Flat-black finish and the sourcing rule come from the `README.md` hub.*

---

## 0 · Reliability standard — mil-spec `[LOCKED]`

*Non-negotiable: **the harness, connectors and instruments are built to a mil-spec baseline** — mil-spec wire and termination practice throughout, aerospace/mil-derived connectors on the rugged and safety-critical runs. On a product for sale this is warranty, liability and brand — a marginal pin or a chafed wire is a recall, not an annoyance. Governs the harness (`electrical.md`), the cockpit + ECU (`electronics.md`) and the ECU firmware I/O (`efi.md`).*

> **Honest scope.** Full **MIL-DTL-38999** circular connectors *everywhere* are overkill and impractical on a motorcycle — weight, bulk and cost (a bike-sized loom would gain kilos and hundreds of dollars per unit). The target is **mil-spec wire + termination + mil-derived motorsport connectors**, with true 38999 reserved for the few runs that genuinely warrant it. This delivers mil-grade reliability without mil-grade bulk.

**Connectors**

- **Mil-derived, bike-appropriate:** **Deutsch Autosport (ASU) / Souriau 8STA** (lightweight, derived from MIL-DTL-38999) on rugged / safety-critical / engine-bay runs; **Deutsch DTM / AMPSEAL** (sealed IP67+) elsewhere (`electronics.md` §3). Positive-lock, **keyed**, gold contacts on signal pins. *(Souriau is French — EU-sourced, per the rule.)*
- **Termination — mil-spec:** **M22520** calibrated crimp tooling; environmental **solder-sleeve splices** (Raychem); mil-spec backshells + strain-relief boots. **No solder-only joints on vibration paths.**
- **Minimise connector count** — every connector is a failure point; sealed inline splices where no service disconnect is needed.
- Redundant safety signals kept physically separate (dual APP / dual TPS — `efi.md` §7).

**Wire**

- **Mil-spec wire throughout** — **MIL-W-22759 (M22759, ETFE/Tefzel)**: abrasion-, heat- and fluid-resistant; gauge for current **and** voltage drop. *(No automotive TXL/GXL — mil wire everywhere.)*
- **Protection** — **Raychem DR-25** adhesive heat-shrink / mil-spec convoluted conduit; routed **off vibration paths, away from exhaust/engine heat**; strain relief at every termination, service loops, secured every ~100–150 mm.
- **Star grounding** — separate signal and load grounds; **shielded twisted pair** for sensitive signals (crank/cam, CAN, lambda) — where noise and CAN errors trace (`efi.md` §4).

**Instruments / compute (cockpit + ECU)**

- **Conformal-coated PCBs** (IPC-CC-830 / mil-grade), vibration-damped mounts, no unsupported heavy components; **automotive-temp** parts throughout.
- **Sealed, IP-rated enclosures** for the ECU and cockpit compute; the bespoke DC-DC carries load-dump / transient / reverse protection (`electronics.md` §8).
- Sunlight-readable, automotive-temp display + M7 instant-on safety telltales (`electronics.md` §8).

**Validation** — every unit-type proven to mil environmental methods: **vibration + thermal-cycle + ingress (MIL-STD-810)**, **EMC (MIL-STD-461)**, plus salt-spray — before production (`build.md` §5); **Conformity of Production** holds the standard on every unit shipped.

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
| Fuse / relay box | **Decided: central sealed fuse box + relays** (automotive micro-fuse + ISO relays) — cheaper, serviceable, no firmware dependency for base power. **Load-shedding is ECU-driven**: the ECU switches relays on the sheddable circuits (cockpit / cameras / modem) under low voltage (`electronics.md` §9), instead of a programmable PDM | `[LOCKED]` / `[BUY]` |
| Main / ignition | Ignition switch or keyless + main relay; interacts with the immobiliser/CAN handshake decision (`electronics.md` §7) | `[PENDING]` |
| Charging feed | Factory stator + **SH847-class series reg/rec** → battery (decision + power budget in `electronics.md` §9) | `[BUY]` (per §9) |
| Reverse / transient protection | Automotive-grade at the DC-DC and fuse-box feed inputs (the cockpit DC-DC front-end is speced in `electronics.md` §8) | `[BESPOKE]` |

## 3 · Battery

| Item | Spec | Status |
|---|---|---|
| Type | **LiFePO4** — the idle/transient buffer chosen in `electronics.md` §9; lighter than lead-acid | `[BUY]` |
| Sizing | To cold-crank the CP3 + hold the worst-case idle deficit (~200 W short-term, per the §9 load budget) | `[PENDING]` |
| Charging profile | Lithium-appropriate regulated voltage; confirm the reg/rec set-point suits LiFePO4 + a cold-behaviour check | `[PENDING]` |
| Placement | Central + low for the geometry target (`bodywork.md`); under-seat is fine — the exhaust is now a short **side-exit** (`engine.md` §2), so the battery + electronics are clear of exhaust heat | `[PENDING]` |

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

- **Fuse box + relays chosen** (§2) — load-shedding runs via **ECU-switched relays** on the sheddable circuits (`electronics.md` §9), not a PDM. Size the sheddable-circuit relays + ECU driver outputs accordingly.
- Finalize the **battery size + placement** against the §9 load budget and the geometry/mass target.
- Assemble the **road-legal lighting/horn/mirror** set for EU IVA / UK MSVA (feeds `emissions_certification.md`).

---

# Caveats (electrical)

1. **The loom is bespoke and safety-adjacent** — it carries ride-by-wire, ignition and the charging feed; connector integrity (`electronics.md` §3/§4) is where noise, CAN errors and no-starts trace to. Build it to motorsport standard.
2. **Lighting is an approval item, not just styling** — LED head/tail/indicators must meet the target markets' road-legal standards; log them in the homologation checklist.
