# Sigma — Café Racer Build Spec Sheet

*Flat-black custom café racer · Yamaha CP3 triple · featherbed special*
*(Working title "Sigma": the Σ is mechanical + electronic summed into one machine.)*

Two domains summed into one machine: a bespoke chromoly featherbed chassis (**`chassis.md`**) wrapped around a new Yamaha CP3 inline-triple (**`engine.md`**), plus a ground-up STM32 / Rust engine controller and i.MX 8M Plus digital cockpit (**`electronics.md`**), homologated to Euro 5+ (**`emissions_certification.md`**). This file is the **hub**: strategy, markets, sourcing, finish and the status-tag legend.

- **Strategy:** speed-density (MAP-based), sequential injection, closed-loop lambda, ride-by-wire throttle
- **Product:** a **low-volume manufactured motorcycle for sale** — not a one-off. This means **type-approval / certification for market** (not individual approval), Conformity of Production, and a functional-safety case for the ride-by-wire (see `emissions_certification.md`).
- **Markets:** EU, UK, Canada, Mexico **for sale now**; **US a later phase** (EPA + CARB + NHTSA — largest effort, deferred).
- **Sourcing rule:** components from Canada, Mexico, or EU. *Documented exceptions: the engine is a new Yamaha — Japanese marque, bought new via an EU/CA/MX dealer; the cockpit bar display is China-sourced (no CA/MX/EU bar-panel source exists — see `electronics.md` §8).*
- **Yamaha-first for powertrain ancillaries:** the donor is bought whole and harvested, so its Yamaha parts (ignition coils, injectors, assist/slipper clutch, in-tank fuel pump/regulator, stator + reg/rec, radiator + fan, all factory sensors) are already paid for and CP3-matched — use them, not catalog substitutes. Premium chassis items (Öhlins, Brembo, Kineo) stay as specced; they're the build's purpose, not ancillaries to economize.
- **Finish:** flat black throughout — per-material finish (anodize, powder, cerakote) to suit each component
- **Status tags:** `[LOCKED]` · `[BESPOKE]` one-off fabrication (in-house or commissioned) · `[BUY]` catalog / new · `[PENDING]`

> **Emissions & homologation.** Target = a current-standard (Euro 5+) product **certified for sale**. One technical spec (closed-loop + cat + evap + **OBD**) covers all markets; the certification differs — EU/UK **small-series type-approval**, Canada **CMVSS + National Safety Mark**, Mexico **NOM**, US (later) **EPA/CARB/NHTSA**. **ABS and OBD are now mandatory** (for-sale, not one-off). **Full per-market paths, manufacturer obligations and checklists in `emissions_certification.md`.**

---

# Documents

Sigma is specified across ten files. This one is the **hub** (strategy, markets, sourcing, finish, status-tag legend); the nine companions carry the domain detail.

| File | Contents |
|---|---|
| **`engine.md`** | Engine & powertrain — CP3 spec, donor/harvest plan, premium upgrades, cooling install, fuel system, open engine items |
| **`chassis.md`** | Chassis & running gear — frame + suspension, front end, wheels, brakes (front + rear), tires, final drive/gearing, mechanical sourcing, open chassis decisions |
| **`bodywork.md`** | Bodywork & ergonomics — tank, seat/cowl, subframe, fenders, rider triangle, hand/foot controls |
| **`electronics.md`** | ECU hardware, sensor stack, connectors, i.MX 8M Plus digital cockpit (§8), charging & power budget (§9) |
| **`efi.md`** | ECU **firmware** — real-time architecture, trigger decode, fuel/ignition, ride-by-wire safety, faults, comms, bench→dyno→road bring-up |
| **`development/mule-runbook.md`** | ECU-on-mule how-to — concrete step-by-step to develop/prove the ECU on a used CP3, with per-phase gates |
| **`electrical.md`** | Electrical, lighting & harness — bespoke loom, power distribution, battery, lighting, switchgear |
| **`emissions_certification.md`** | Euro 5+ technical target, per-market homologation paths, emissions hardware + road-legal equipment checklists |
| **`build.md`** | Build plan — mass target, cost budget, procurement/lead times, phases, test & sign-off |

---

# Caveats

1. **Homologation is a manufacturing program, not paperwork** — Sigma is certified **for sale** (Euro 5+ via cat + closed-loop + evap + OBD; ABS mandatory), so it's **type-approval** per market (EU/UK small-series, Canada CMVSS, Mexico NOM; US EPA/CARB/NHTSA later) plus Conformity of Production and a ride-by-wire functional-safety case. Far heavier than a one-off — treat it as a first-class workstream. Full detail in `emissions_certification.md`.
2. **Chassis caveats** — steering-stem safety-criticality and bespoke cost/lead time live in `chassis.md`.
3. **Electronics caveats** — custom ECU, standalone brain, CP3 characterization and the cockpit live in `electronics.md`.
