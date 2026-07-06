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

## 2 · Cost & manufacturer economics

*Rough order-of-magnitude estimates — USD, ranges not quotes. As a **low-volume manufacturer** the model is **NRE (one-time) + per-unit COGS × volume**. Both are built out below. Numbers are planning figures per the project rule — confirm on quote.*

### 2a · Per-unit parts/materials (production parts only)

*Excludes the NRE items — the used mule, dev boards (EVK) and tooling live in §2b, not per bike.*

| Bucket | Includes | Est. (USD) |
|---|---|---|
| Powertrain | CP3 engine (crate/supply — see finding 1; or ~$9–11k whole donor) + ti exhaust + cooling + covers (`engine.md`) | **$12–16k** |
| Chassis / suspension / brakes | Öhlins FG 621 + STX 46 + Kineo pair + Brembo f+r + bearings/chain/tires (`chassis.md`) | **$11–12k** |
| Electronics / cockpit | ECU BOM + Toradex Verdin SoM + bespoke carrier + display + cameras/modem + wideband (`electronics.md`) | **$2–3k** |
| Electrical / harness | Loom + fuse/relay box + LiFePO4 + LED lighting + switchgear (`electrical.md`) | **$2–2.5k** |
| Bodywork | Bespoke tank + seat/cowl + clip-ons/rearsets + controls (`bodywork.md`) | **$4–5k** |
| Fabrication | Frame, swingarm, yokes, stem, axle, brackets — **materials + machining** | **$3–6k** |
| **Per-unit parts subtotal** | | **~$34–45k** |
| **+ Assembly/fab/tune labour** | ~150–300 h bespoke @ shop rate | **~$10–18k** |
| **≈ Per-unit COGS** | | **~$45–65k** |

### 2b · NRE (one-time, volume-independent)

| Item | Est. (USD) | Note |
|---|---|---|
| **ECU firmware program** | *person-years* / founder time | Rust/Embassy + OBD + safety case (`efi.md`) — the single biggest sunk effort |
| **Cockpit Linux program** | *person-years* / founder time | BSP + maps/cameras/HMI + M7 safety fw (`electronics.md` §8) |
| Homologation / type-approval | ~$30–100k+ | EU/UK/CA/MX small-series program (`emissions_certification.md`) |
| Prototyping | ~$15–30k | Used mule + benches + dev boards (see the ECU-on-mule dev runbook) |
| Tooling / jigs / fixtures | ~$10–40k | Frame jig + repeatable fab fixtures for production |
| **NRE (cash, ex-software-labour)** | **~$55–170k+** | **Dominated by the two software programs if that labour is paid rather than founder time** |

### 2c · Unit economics & volume

- **Small-series caps:** EU small-series type-approval limits annual volume **per type** — realistically **~10–50/yr** for a bespoke builder. Confirm the current cap; exceeding it forces full type-approval.
- **Pricing:** to work at this COGS, retail must be **premium — ~$80–130k+** (boutique/bespoke tier, not mass-market).
- **Breakeven:** NRE ÷ per-unit contribution margin. e.g. ~$300k effective NRE ÷ (~$95k price − ~$60k COGS ≈ $35k margin) ≈ **~9 units to clear NRE**, then profit — but *highly* sensitive to price, COGS, and whether software is paid labour or founder time.

### 2d · Business-model findings (the hard truths)

1. **Harvesting a new donor per bike doesn't scale.** Buying a whole ~$10k MT-09 just to harvest one engine, per unit, is brutal — the rest is surplus. A manufacturer needs a **Yamaha crate-engine / OEM supply arrangement**, or the engine line alone wrecks the margin. *(Reframes `engine.md`'s "buy new donor" for production.)* **⚠ Validated at-risk (2026-07):** no public Yamaha CP3 crate program exists (Yamaha keeps the CP3 in-house). Nearest leads — TKRP complete R9/CP3 race-spare engines (NL), or a direct Yamaha OEM-supply deal (uncertain). **Crate supply is unconfirmed — carry a whole-donor pricing fallback until a deal is signed; if none lands, reconsider the engine or the premise before tooling spend.**
2. **Software NRE dominates and fights low volume.** Two full embedded programs are a fixed sunk cost that *demands* volume to amortize — in direct tension with "low-volume / small-series."
3. **Design-for-manufacture becomes mandatory.** Bespoke-everything + ~200 h/bike won't scale; convert one-off fabrication (frame, swingarm, yokes) to **jig-repeatable** production or per-unit labour stays high.
4. **Position premium boutique.** At ~$45–65k COGS the only viable market is the **$80–130k+ bespoke tier**, competing on craft + tech, not price.

**Bottom line:** viable **only** as a premium boutique manufacturer with (a) a **crate-engine supply deal**, (b) **design-for-manufacture** to cut per-unit labour, and (c) enough volume **within the small-series cap** to amortize a large software NRE. Confirm all three before committing tooling / type-approval spend.

## 3 · Procurement & lead times

*Order long-lead / made-to-order items early — they gate the build. Dates assume **T0 = project start ≈ Sep 2026**; shift with your actual T0. Order-by is driven by the gating phase, not convenience.*

| Item | Order by | Lead | Arrives ~ | Gate / dependency |
|---|---|---|---|---|
| **Mule rig** (used CP3 + run stand + fuel/cooling/exhaust bench) | **T0 (Sep '26)** | — | T0 | Buy **first** — gates the whole ECU program; full BOM in the ECU-on-mule dev runbook |
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
5. **Wiring + first start** — bespoke loom, fuse/relay box, charging; static run on the new engine.
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
| **Electrical durability (bulletproof)** | **Vibration + thermal-cycle + IP/ingress + salt-spray + EMC** on the harness, connectors, instruments and ECU/cockpit — the `electrical.md` §0 standard, per unit-type before production | `[PENDING]` |

---

# Caveats (build)

1. **Sequence gates spend** — freeze geometry before the built-to-order shock; buy the used mule before the new engine; confirm the fork package before the wheels. Cheap checks gate expensive, irreversible steps.
2. **Two software programs run in parallel** — the Rust ECU and the Linux cockpit are each their own project (`electronics.md`); staff/schedule them as such, not as afterthoughts to the fabrication.
3. **Nothing is measured until it's measured** — weights, costs and the charging curve here are targets; confirm on the real build.
