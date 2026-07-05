# Sigma — Emissions & Homologation

*Companion to `README.md`. **Sigma is a low-volume manufactured product for sale — not a one-off** — so the *type* must be **certified / type-approved for market**, not individually approved per unit. Markets: **EU, UK, Canada, Mexico** now; **US is a later phase** (EPA + CARB + NHTSA — the largest single effort, deferred). Path: a new Yamaha CP3 on the custom STM32/Rust ECU, certified to a current standard.*

## Premise

The engine is a **new Yamaha CP3** — a current **Euro 5** powerplant: combustion, cat and closed-loop already designed to meet Euro 5. **But** the custom ECU + custom (catalysed) exhaust take it out of Yamaha's homologated configuration, so — **as a product for sale** — the *type* must be certified on its own merits, and you carry **manufacturer obligations** a one-off never had (type-approval, Conformity of Production, a technical file, a functional-safety case for the ride-by-wire). The Euro 5 base makes the emissions target far easier, but Yamaha's certificate does **not** transfer.

## Technical target — one spec for all markets

Hold the build to **Euro 5+** (strictest of the target markets; in force in the EU since 1 Jan 2024). The CP3 already runs the right architecture; preserve and tune it:

- **Closed-loop stoichiometric fuelling** — the custom ECU runs closed-loop lambda (add LSU 4.9 + CJ125); tune to the limits.
- **Catalytic converter** in the collector — cat volume matched to the CP3's Euro 5 calibration.
- **Evaporative (charcoal) canister** + purge valve.
- **On-board diagnostics (OBD)** to the Euro 5 stage — see OBD note (now **in scope** because it's for sale).

Certify the type to Euro 5+ and the same hardware clears UK, Canadian and Mexican requirements; the US (later) needs its own emissions cert.

## Paperwork path by market — for sale, low-volume manufacturer

| Market | Path (for sale) | Outcome |
|---|---|---|
| **EU** | **Small-series type-approval** under Reg. (EU) 168/2013 (annual per-type volume caps; Euro 5+; **ABS mandatory**; OBD per stage; **Conformity of Production**). | **Type approval** → sell across the EU, build to type |
| **UK** | **GB small-series type approval** (post-Brexit GB scheme, aligned to 168/2013). | GB type approval |
| **Canada** | **Transport Canada** — manufacturer meets **CMVSS**, applies a **National Safety Mark**; emissions to **ECCC / US-EPA-harmonized** limits. *(No longer a personal provincial registration — you're selling.)* | Federal compliance to sell |
| **Mexico** | **NOM homologation** — NOM-042-SEMARNAT (emissions) + safety NOMs; manufacturer/importer registration. | NOM compliance |
| **US** *(later phase)* | **EPA** engine-family Certificate of Conformity + **CARB** (California Executive Order) + **NHTSA / FMVSS** (122 brakes/ABS, 108 lighting, 123 controls…). | **Deferred** — largest cost; Phase 2 |

**Net:** you type-approve the **type** once per regime, then build every unit to it under Conformity of Production. **ABS and OBD are now mandatory** (the one-off exemptions no longer apply). US is deferred to a later phase.

## Market rollout — phased (crawl, walk, run)

*Engineer once to the highest standard (Euro 5+ + ABS + OBD + RbW safety case — the "one spec for all markets"); then **certify market-by-market**, each earlier market funding and de-risking the next. The bike doesn't change per phase — only the paperwork/testing does.*

| Phase | Market | Why here | Gate to advance |
|---|---|---|---|
| **0** | *(engineering readiness)* | ECU proven on the mule → first running, safe, Euro-5+-capable bike | A safe running bike exists — **nothing sells before this** |
| **1** | 🇲🇽 **Mexico** (home / pilot) | Lowest bar for motorcycles — emissions NOM + noise NOM; **light safety regime (no ABS mandate, no type-approval program)**; home market → no import friction. Prove production, CoP, crate-engine supply, DfM and unit economics with paying customers | Profitable small-batch sales + stable production → funds the EU program |
| **2** | 🇪🇺 **EU** + 🇬🇧 **UK** | The premium market that justifies the pricing, but the **biggest NRE**: small-series type-approval (Euro 5+ OBD, ABS, EMC, safety case, Technical Service, CoP). UK GB small-series rides alongside (aligned to EU) | EU small-series type-approval granted → sell across EU + UK |
| **3** | 🇨🇦 **Canada** | CMVSS + National Safety Mark + EPA-harmonised emissions; **CMVSS harmonises with US FMVSS → pre-builds the US safety case** | CMVSS compliance + National Safety Mark |
| **4** | 🇺🇸 **US** *(deferred, largest)* | EPA engine-family cert + CARB + NHTSA/FMVSS; leverages Canada's FMVSS-harmonised work, but EPA/CARB emissions is a separate, expensive cert | Deferred until volume justifies the cost |

**Ordering logic:** MX (learn cheap at home) → EU/UK (the real business, funded by MX) → CA (FMVSS bridge) → US (largest, last). *Alternative — EU-first ("certify to the hardest standard first") — is defensible but front-loads the biggest NRE before demand/production are proven; the MX-first crawl-walk-run is lower-risk.* Note: Mexico accepts only its **own NOM certificate** (no foreign-cert reciprocity), so MX is certified on its own — but the bar sits far below EU/US.

## Manufacturer obligations (selling, not building)

Obligations a one-off never carried — these are now core scope:

- **Type-approval** of the vehicle type (small-series) per regime, then **Conformity of Production (CoP)** — every unit built to the approved type, with audit.
- **Manufacturer identity** — WMI / VIN assignment, statutory data plate, and a **technical file / information folder** per market.
- **Functional-safety case** for the safety-critical **ride-by-wire** (and the ABS integration): documented hazard analysis + the independent monitor / fail-safe (`efi.md` §7). Motorcycles sit largely outside formal ISO 26262 *certification*, but **product-liability exposure makes a real safety case mandatory in practice** — and raises the value of the i.MX 95 / ASIL-B upgrade path for the cockpit (`electronics.md` §8).
- **EMC type-approval**, drive-by **noise**, and each market's construction requirements (road-legal checklist below).
- **Engage a Technical Service / homologation consultant early** (e.g. TÜV, VCA, or a national approval authority) — small-series type-approval is a program, not a form.

## Hardware checklist

- [ ] Closed-loop stoichiometric tune (ECU + **pre-cat** LSU 4.9 wideband)
- [ ] Catalytic converter (in the collector, near-engine for light-off)
- [ ] **Post-cat O₂ (catalyst monitoring, OBD)** — factory Yamaha narrowband repurposed post-cat; ECU compares pre/post-cat switching (`efi.md` §8)
- [ ] Evap / charcoal canister + purge valve
- [ ] Catalysed exhaust system — ties to the open exhaust decision in `engine.md`
- [ ] **OBD to the Euro 5 stage** — MIL + catalyst/misfire/sensor monitoring in the ECU (`efi.md` §8)
- [ ] **Tire speed rating vs certified top speed** — fitted tires must be rated ≥ the certified top speed. Front is Pirelli Scorpion Rally STR 120/70-17 **58H (210 km/h)**; **addressed by the ≤210 km/h top-speed target (`build.md` §1)** — gear/declare to stay within the 58H front; confirm at approval. Rear 170/60-17 72V (240 km/h) is clear. (Tire note in `chassis.md`.)

## Road-legal equipment (approval beyond emissions)

*A product for sale must pass the construction, braking and lighting requirements of type-approval in each market — confirm the exact list per regime + vehicle class. Items cross-referenced to where they're specified.*

- [ ] **ABS — mandatory (for-sale type-approval) + decided: OEM Yamaha ABS.** For a product sold in EU/UK/CA (>125 cc), ABS is **required** — the earlier "individual-approval likely exempt" **no longer applies now that it's for sale**. The donor's OEM Yamaha ABS (modulator + front/rear wheel-speed sensors + tone rings, new with the donor) satisfies it; must be integrated into a **certified braking system** (EU braking reg now; FMVSS 122 in the US phase). Kineo **tone-ring provision** (`chassis.md` §1); ABS↔ECU CAN (`efi.md` §10–11); telltale (`electronics.md` §8).
- [ ] **Dual independent brakes** — front (Brembo M4 + twin 320 mm) and rear (Brembo single) — `chassis.md`.
- [ ] **Lighting (all LED, ECE-approved)** — headlight low/high + position/DRL; tail + brake light; front + rear indicators; plate light; rear + side reflectors — `electrical.md` §4.
- [ ] **Mirrors** — road-legal field of view (`bodywork.md` §2).
- [ ] **Audible warning (horn)** — `electrical.md` §5.
- [ ] **Speedometer** — required units + tolerance; wheel-speed source (final drive, `chassis.md`) rendered on the cockpit (`electronics.md` §8).
- [ ] **Number plate mount + illumination** — `bodywork.md`.
- [ ] **Anti-theft / steering lock** — required against unauthorized use; ties to the immobiliser decision (`electronics.md` §7).
- [ ] **Chain guard** — final-drive contact protection (`chassis.md`).
- [ ] **Sidestand** + start interlock (`electrical.md` §5).
- [ ] **Noise / drive-by sound level** — approval limit; **the binding floor on the short side-exit exhaust** (`engine.md` §2 — too short = too loud to pass). Size the muffler to the limit.
- [ ] **EMC (electromagnetic compatibility)** — a type-approval item, and a real design constraint: the custom ECU + Linux cockpit + switching DC-DC must meet emission/immunity limits (`electronics.md`).

## OBD note

**OBD is now in scope** — a for-sale Euro 5+ bike needs on-board diagnostics per the applicable stage (MIL + catalyst / misfire / sensor-rationality monitoring). EU **small-series** type-approval relaxes *volume/administrative* burden and may soften some monitoring, but plan the ECU for **real OBD**, not none — this is a firmware lift the one-off would have skipped (`efi.md` §8). **Resolved: small-series type-approval for sale → scope the ECU for OBD; confirm the exact small-series relaxations with the approval authority.**

## Caveat

**Not legal advice — and this is now a manufacturing homologation program, not a one-off.** Type-approval, OBD, ABS, EMC, functional safety and Conformity of Production across EU/UK/CA/MX (and later US EPA/CARB/NHTSA) are jurisdiction-specific, change often, and are far heavier than individual approval. **Engage a homologation consultant / Technical Service per market before committing** — treat the regulatory program as a first-class workstream in `build.md`, not paperwork at the end.
