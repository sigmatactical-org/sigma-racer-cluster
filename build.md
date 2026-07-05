# Sigma — Build Plan, Budget & Validation

*Companion to `README.md`. The whole-build rollup the domain docs don't carry: mass target, cost budget, long-lead procurement, build phases and the test/sign-off plan. Domain detail lives in `engine.md`, `chassis.md`, `electronics.md`, `electrical.md`, `bodywork.md`; homologation in `emissions_certification.md`.*

*Numbers here are **targets/estimates to fill on the actual build** — per the project rule, don't treat them as measured until they are.*

---

## 1 · Targets

*Vehicle-level targets set now and held to — they cascade into gearing, tire rating, geometry and homologation.*

| Target | Value | Cascade |
|---|---|---|
| **Kerb weight (wet)** | **≤ 190 kg** (stretch ~185) | Steel featherbed + Linux cockpit fight lightness; forged Kineo wheels, ti exhaust and a LiFePO4 battery claw it back. Track per group below |
| **Top speed** | **~200 km/h geared/declared; hard ceiling 210 km/h** | Keeps the fitted **58H front tire (210 km/h)** legal and sets the **gearing** envelope (`chassis.md`) — resolves the tire-rating homologation flag (`emissions_certification.md`) |
| **Weight distribution** | ~50/50 static, low CoG | Feeds the geometry targets (`chassis.md` §1) |

**Mass budget (planning allowances, kg).** Rough per-group allowances toward the ≤190 kg wet target — track actuals against these as parts arrive.

| Group | Allowance (kg) |
|---|---|
| Engine + gearbox (CP3) | ~55 |
| Frame + subframe (chromoly) | ~15 |
| Swingarm + linkage | ~6 |
| Wheels + tires + brakes | ~24 |
| Forks + yokes + steering | ~14 |
| Rear shock | ~2.5 |
| Fuel system + tank + ~14 L fuel | ~15 |
| Exhaust (titanium) | ~5 |
| Cooling (rad + oil cooler + coolant) | ~6 |
| Electrical (loom + LiFePO4 + lighting) | ~6 |
| Cockpit + ECU + mounts | ~3 |
| Bodywork (seat, panels, fender) | ~8 |
| Controls, fluids, fasteners, misc | ~9 |
| **Running total** | **~170** → **hold ≤ 190 wet** (~20 kg margin) |

## 2 · Cost budget

*Rough order-of-magnitude estimates — **USD, parts/materials only, labour excluded** (in-house fabrication + the ECU/cockpit software effort are the big unpriced inputs). Ranges, not quotes; confirm on order. Premium chassis/engine items are the build's purpose (`README.md`) — budgeted, not cut.*

| Bucket | Includes | Est. (USD) |
|---|---|---|
| Powertrain | New CP3 donor (~$9–11k) + used mule (~$5–6k) + ti exhaust + cooling + covers (`engine.md`) | **$17–22k** |
| Chassis / suspension / brakes | Öhlins FG 621 (~$2.5k) + STX 46 + Kineo pair (~$3–3.5k) + Brembo f+r + bearings/chain/tires (`chassis.md`) | **$11–12k** |
| Electronics / cockpit | ECU BOM + i.MX 8M Plus EVK→SoM + carrier + display + cameras/modem + wideband (`electronics.md`) | **$3–4k** |
| Electrical / harness | Loom + PDM/fusing + LiFePO4 + LED lighting + switchgear (`electrical.md`) | **$2–2.5k** |
| Bodywork | Bespoke tank + seat/cowl + clip-ons/rearsets + controls (`bodywork.md`) | **$4–5k** |
| Fabrication / bespoke | Frame, swingarm, yokes, stem, axle, brackets, cradle — **materials + machining** (in-house labour excluded) | **$3–6k** |
| Homologation **(NRE, not per-unit)** | **Small-series type-approval program** across EU/UK/CA/MX — Technical Service fees + EMC/emissions/noise/brake lab testing + Conformity-of-Production setup (US EPA/CARB/NHTSA is a later, larger phase). Far heavier than one-off approval (`emissions_certification.md`) | **~$30–100k+** |
| **Subtotal** | | **~$42–55k** |
| Contingency (~15–20%) | one-off / bespoke overruns | **~$7–11k** |
| **Total (parts/materials)** | | **~$50–65k** |

*The parts/materials subtotal is **per-bike** and **excludes the homologation NRE line** (that's a one-time program cost, not per unit). As a **low-volume manufacturer** the real model is **NRE (certification + tooling) + per-unit parts/labour × volume** — this table is only the per-unit parts side. Commissioned fabrication and the US phase would raise both. **build.md needs a proper manufacturer cost model (NRE vs per-unit vs unit economics) — flagged, not yet built.***

## 3 · Procurement & lead times

*Order long-lead / made-to-order items early — they gate the build. Dates assume **T0 = project start ≈ Sep 2026**; shift with your actual T0. Order-by is driven by the gating phase, not convenience.*

| Item | Order by | Lead | Arrives ~ | Gate / dependency |
|---|---|---|---|---|
| **Mule rig** (used CP3 + run stand + fuel/cooling/exhaust bench) | **T0 (Sep '26)** | — | T0 | Buy **first** — gates the whole ECU program; full BOM in `development/mule-runbook.md` |
| Display sample + driver board | T0 (Sep '26) | vendor sample | T0 + 4–6 wk | RFQ ready (6.86" 1280×480 MIPI 1000-nit) — `electronics.md` §8 |
| Öhlins FG 621 fork | T0 + 1 (Oct '26) | catalog | ~2–4 wk | Confirm Zodiac package contents (axle/caliper mounts) at order |
| **Kineo wheels (f+r)** | **T0 + 1 (Oct '26)** | **~18 wk** | **~Feb '27** | Order early — keys off the chosen 320 mm disc (`chassis.md`) |
| Brembo brakes + bearings + tires | T0 + 2 (Nov '26) | catalog | ~2–4 wk | Front-disc pick gates the wheel order above |
| **Öhlins STX 46 shock** | **T0 + 3 (Dec '26)** | ~6–10 wk | ~Feb–Mar '27 | Built-to-order — **can't order until the linkage/geometry is frozen** |
| New CP3 donor | ~T0 + 6–9 ('27) | market | — | Buy **after** the ECU is proven on the mule |
| Bodywork (tank/seat/subframe) | ~T0 + 6–9 ('27) | bespoke | — | After geometry frozen + rolling mock-up |
| Exhaust system | ~T0 + 9 ('27) | TBD | — | Ti + cat; specific system `[PENDING]` (`engine.md`) |

## 4 · Build phases

1. **ECU prototype** — bench + dyno the Rust ECU on a **used CP3 mule**: trigger decode, ride-by-wire, immobiliser handshake, fail-safes (`electronics.md`).
2. **Frame jig + chassis geometry** — freeze rake/trail/wheelbase/ride-height + the rising-rate linkage → *only then* order the built-to-order shock (`chassis.md`).
3. **Rolling mock-up** — frame + forks + wheels + engine cradle; validate clearances (tire-to-arm/chain, radiator packaging).
4. **Subsystems** — cooling, exhaust, electrical loom, cockpit, bodywork mock.
5. **Wiring + first start** — bespoke loom, PDM, charging; static run on the new engine.
6. **Tune + validation** — dyno tune to the cat, closed-loop lambda, quickshifter; road-test shakedown.
7. **Homologation** — emissions + road-legal equipment inspection/approval per market (`emissions_certification.md`).

## 5 · Test & sign-off

| Gate | Validates | Status |
|---|---|---|
| ECU bench/dyno | Fuelling, ignition, ride-by-wire fail-safes, immobiliser (safety-critical before any road use) | `[PENDING]` |
| Chassis validation | Geometry on the **actual STR tire**; steering-stem load check (safety-critical) | `[PENDING]` |
| Charging balance | Measured stator output vs summed load at idle (`electronics.md` §9) | `[PENDING]` |
| Emissions | Closed-loop stoich to Euro 5+ limits (`emissions_certification.md`) | `[PENDING]` |
| Road-legal equipment | Lights/horn/mirrors/speedo/plate for approval | `[PENDING]` |

---

# Caveats (build)

1. **Sequence gates spend** — freeze geometry before the built-to-order shock; buy the used mule before the new engine; confirm the fork package before the wheels. Cheap checks gate expensive, irreversible steps.
2. **Two software programs run in parallel** — the Rust ECU and the Linux cockpit are each their own project (`electronics.md`); staff/schedule them as such, not as afterthoughts to the fabrication.
3. **Nothing is measured until it's measured** — weights, costs and the charging curve here are targets; confirm on the real build.
