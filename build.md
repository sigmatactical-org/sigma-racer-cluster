# Sigma — Build Plan, Budget & Validation

*Companion to `README.md`. The whole-build rollup the domain docs don't carry: mass target, cost budget, long-lead procurement, build phases and the test/sign-off plan. Domain detail lives in `engine.md`, `chassis.md`, `electronics.md`, `electrical.md`, `bodywork.md`; homologation in `emissions_certification.md`.*

*Numbers here are **targets/estimates to fill on the actual build** — per the project rule, don't treat them as measured until they are.*

---

## 1 · Mass target

| Item | Note | Status |
|---|---|---|
| Target kerb weight | Set a number and hold the build to it — a steel featherbed + a Linux cockpit fight lightness. Log a target (e.g. sub-190 kg wet) and track components against it | `[PENDING]` |
| Heavy items to watch | Steel frame, battery (LiFePO4 helps), cockpit compute + display, radiator/oil-cooler, twin discs | — |
| Distribution | Front/rear bias + CoG height feed the geometry target (`chassis.md`) | `[PENDING]` |

## 2 · Cost budget

*Fill per line; premium chassis/engine items are the build's purpose (not to economize — `README.md`), so budget them, don't cut them.*

| Bucket | Includes | Status |
|---|---|---|
| Powertrain | New CP3 donor + used proto mule; exhaust; cooling; covers (`engine.md`) | `[PENDING]` |
| Chassis / suspension / brakes | Öhlins FG 621 + STX 46, Brembo, Kineo wheels, bearings (`chassis.md`) | `[PENDING]` |
| Electronics / cockpit | ECU BOM + i.MX 8M Plus EVK→SoM + display + cameras/modem (`electronics.md`) | `[PENDING]` |
| Electrical / harness | Loom, PDM/fusing, battery, lighting, switchgear (`electrical.md`) | `[PENDING]` |
| Bodywork | Tank, seat/cowl, subframe, controls (`bodywork.md`) | `[PENDING]` |
| Fabrication / bespoke | Frame, swingarm, yokes, stem, axle, brackets, cradle | `[PENDING]` |
| Homologation | Approval/inspection fees per market (`emissions_certification.md`) | `[PENDING]` |

## 3 · Procurement & lead times

*Order the long-lead / made-to-order items early — they gate the build.*

| Item | Lead | Note |
|---|---|---|
| Kineo wheels (f+r) | up to ~18 weeks | Made-to-order; order early — keys off the chosen 320 mm disc (`chassis.md`) |
| Öhlins STX 46 (built-to-order) | order-dependent | Eye-to-eye + rate from the frozen linkage; can't order until linkage is frozen |
| Öhlins FG 621 fork | catalog, confirm stock | Verify Zodiac package contents (axle/caliper mounts) at order |
| CP3 donor (new) + used mule | market-dependent | Buy the used mule first for ECU proto |
| Display sample + driver board | vendor sample | RFQ sent (6.86" 1280×480 MIPI 1000-nit) — `electronics.md` §8 |
| Exhaust system | TBD on selection | Direction locked (ti + cat); part `[PENDING]` (`engine.md`) |

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
