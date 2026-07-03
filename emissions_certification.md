# Sigma — Emissions & Homologation

*Companion to `README.md`. Path chosen: a **new** Yamaha CP3 engine, run on the custom STM32/Rust ECU, homologated to a current standard.*

## Premise

The engine is a **new Yamaha CP3** — a current **Euro 5** powerplant. That's a head start: the combustion, cat and closed-loop are already designed to meet Euro 5. **But** swapping in the custom ECU and a custom (catalysed) exhaust takes it out of Yamaha's homologated configuration, so the one-off must be **re-homologated** on its own merits. Net: the Euro 5 base makes hitting the standard far easier, but it doesn't transfer Yamaha's certificate to your bike.

## Technical target — one spec for all markets

Hold the build to **Euro 5+** (the strictest of the target markets; in force in the EU since 1 Jan 2024). The CP3 already runs the right architecture; preserve and tune it:

- **Closed-loop stoichiometric fuelling** — the custom ECU runs closed-loop lambda (add LSU 4.9 + CJ125); tune to the limits.
- **Catalytic converter** in the collector — match/keep cat volume appropriate to the CP3's Euro 5 calibration.
- **Evaporative (charcoal) canister** + purge valve.

Pass EU individual approval to Euro 5+ and it comfortably clears UK approval and Canadian/Mexican in-use inspection.

## Paperwork path by market

| Market | Path | Outcome |
|---|---|---|
| **EU** | National individual approval / small-series type-approval under Reg. (EU) 168/2013. Small-series is exempt from full OBD II. | Genuine current-standard **certificate** |
| **UK** | Individual Vehicle Approval (IVA) / Motorcycle Single Vehicle Approval (MSVA); aligned with the 168/2013 framework post-Brexit. | **Certificate** (per-unit approval) |
| **Canada** | Federal emission rules apply to vehicles built/imported *for sale*; a personal one-off isn't federally certified. Motorcycle limits are harmonized with US EPA. Registered **provincially** (safety + any in-use test). | **Provincial registration** + inspection — no per-unit cert |
| **Mexico** | SEMARNAT NOMs (may follow US or EU limits). Road-legality via **state registration + periodic *verificación*** where the state runs it (e.g. CDMX, Guadalajara). | **State registration** + inspection — no per-unit cert |
| **US** | EPA requires an engine-family Certificate of Conformity; there is no per-unit approval. Builder allowances are narrow: one exempt kit bike per lifetime, or ≤24/yr "custom" bikes that are display-only. | **Out** — the modified one-off is uncertified (the CP3's factory cert doesn't carry once you change ECU/exhaust); display-only, dropped from the road target |

**Net:** EU/UK give an actual certificate; Canada/Mexico give registration + an inspection the same hardware passes; the US is not a road-legal target under this path.

## Hardware checklist

- [ ] Closed-loop stoichiometric tune (ECU + LSU 4.9 wideband)
- [ ] Catalytic converter (collector)
- [ ] Evap / charcoal canister + purge valve
- [ ] Catalysed exhaust system — ties to the open exhaust decision in `engine.md`
- [ ] **Tire speed rating vs certified top speed** — fitted tires must be rated ≥ the bike's approved top speed. Front is Pirelli Scorpion Rally STR 120/70-17 **58H (210 km/h)**; a standalone CP3 runs near that, so confirm 58H clears the certified top speed (or declare/restrict top speed) before approval. Rear 170/60-17 72V (240 km/h) is clear. (See tire note in `chassis.md`.)

## Road-legal equipment (approval beyond emissions)

*Emissions is one half of homologation; a new one-off must also pass the construction, braking and lighting requirements of the individual-approval / single-vehicle path (EU IVA / UK MSVA; CA/MX inspection). Confirm the exact list per market + vehicle class — items below are cross-referenced to where they're specified.*

- [ ] **ABS** — EU/UK generally **mandate ABS on new >125 cc motorcycles**. Fit an aftermarket modulator integrated with the custom ECU wheel-speed sensing, or confirm individual-approval / small-series relief. **Unresolved — gates road approval.** (Rear-brake note, `chassis.md` §1; telltale in `electronics.md` §8.)
- [ ] **Dual independent brakes** — front (Brembo M4 + twin 320 mm) and rear (Brembo single) — `chassis.md`.
- [ ] **Lighting (all LED, ECE-approved)** — headlight low/high + position/DRL; tail + brake light; front + rear indicators; plate light; rear + side reflectors — `electrical.md` §4.
- [ ] **Mirrors** — road-legal field of view (`bodywork.md` §2).
- [ ] **Audible warning (horn)** — `electrical.md` §5.
- [ ] **Speedometer** — required units + tolerance; wheel-speed source (final drive, `chassis.md`) rendered on the cockpit (`electronics.md` §8).
- [ ] **Number plate mount + illumination** — `bodywork.md`.
- [ ] **Anti-theft / steering lock** — EU requires a device against unauthorized use; ties to the immobiliser decision (`electronics.md` §7).
- [ ] **Chain guard** — final-drive contact protection (`chassis.md`).
- [ ] **Sidestand** + start interlock (`electrical.md` §5).
- [ ] **Noise / drive-by sound level** — approval limit; ties to the catalysed exhaust (`engine.md` §2).
- [ ] **EMC (electromagnetic compatibility)** — an EU approval item, and a real design constraint here: the custom ECU + Linux cockpit + switching DC-DC must meet emission/immunity limits (`electronics.md`).

## OBD note

EU small-series approval is exempt from full OBD II, which keeps the custom ECU's firmware scope manageable. The broader (non-small-series) route would add OBD II with catalyst monitoring — a real firmware lift. Decide the EU route (small-series vs. full) before locking the ECU's diagnostic scope.

## Caveat

**Not legal advice.** Emissions homologation is jurisdiction-specific and changes (Euro 5+ is barely a year old). Anything committed here should be confirmed with a homologation specialist and the type-approval / registration authority in each target market.
