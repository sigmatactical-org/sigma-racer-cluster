# Sigma — Engine & Powertrain

*Companion to `README.md`. The powertrain half of Sigma: the harvested Yamaha CP3 triple, its donor/harvest plan and the premium engine upgrades. Chassis & running gear live in `chassis.md`; the custom ECU that runs this engine is in `electronics.md`; emissions/homologation in `emissions_certification.md`. Strategy, sourcing rule, finish and the status-tag legend live in the `README.md` hub.*

---

## 1 · Powertrain (mechanical)

| Item | Spec | Status |
|---|---|---|
| Engine | Yamaha CP3 — 890 cc liquid-cooled inline-3, DOHC 12v, crossplane crank (even firing, 240° spacing) | `[LOCKED]` |
| Output | ~117 hp (claimed) @ ~10,000 rpm, ~93 Nm; ~10,500 rpm redline | — |
| Induction | EFI, ride-by-wire (YCC-T) | — |
| Cooling / drive | Liquid-cooled · chain final | — |
| Emissions base | Euro 5 from the factory — a head start on re-homologation (see `emissions_certification.md`) | — |
| Sourcing | **New** — harvested from a current Yamaha donor: engine + gearbox + ride-by-wire throttle bodies + factory sensors | `[BUY]` new donor |

**CP3 donor models (identical engine — buy the cheapest to harvest):**

- Yamaha MT-09 — cheapest CP3 donor; first choice for a harvest
- Yamaha XSR900 — café/retro variant (same engine + frame as MT-09)
- Yamaha Tracer 9 / YZF-R9 — same 890 cc CP3, touring / sportbike tunes

**Buy notes:**

- The custom ECU (see **`electronics.md`**) replaces the donor's ECU and loom — but you **keep** the engine, gearbox, ride-by-wire throttle bodies, factory sensors, **ignition coils, injectors, the assist/slipper clutch, the in-tank fuel pump + regulator, the stator + reg/rec, the radiator + fan, and the OEM ABS modulator + wheel-speed sensors + tone rings** (ABS decision — `chassis.md` §1); the ECU is built to interface with all of it. Default to the donor's Yamaha parts for powertrain ancillaries — already bought with the engine and matched to the CP3.
- The CP3 is **ride-by-wire**: the throttle body's motor + dual throttle-position and dual grip sensors must be driven and read by the custom ECU — a safety-critical subsystem (see `electronics.md` §6).
- The donor may need an **immobiliser / CAN handshake** to run; the custom ECU must replicate or cleanly delete it.
- Cheapest path to a new engine is a new **MT-09** — harvest engine + throttle bodies + sensors + wiring reference; the rest of the bike is surplus.
- **Prototyping:** develop and bench the ECU on a cheap **used CP3** (used MT-09) before committing the new engine — same trigger, sensors and ride-by-wire as the build engine, so the work carries straight across.

## 2 · Engine upgrades (premium)

*A premium café racer earns premium engine kit — but the **Euro 5+ homologation ceiling governs**: no de-cat, no high-comp pistons, no hot cams. The gains come from breathing + tune + firmware + thermal + finish, not cubic power. This deliberately reverses the "Yamaha-first, economize the ancillaries" rule for **select** items (exhaust, cooling, covers); the factory injectors, coils, pump and assist/slipper clutch stay — they're genuinely good.*

| Upgrade | Spec | Status |
|---|---|---|
| Exhaust | **Premium full titanium system with a metal catalyst in the collector** — hand-built ti header + carbon/ti muffler (Akrapovič / SC-Project / Arrow CP3 systems as reference). **Must retain a high-flow cat** for Euro 5+ — *not* a de-cat race pipe. The build's statement engine piece | `[LOCKED]` direction / `[PENDING]` system |
| Performance tune | Custom-ECU dyno tune to the actual cat + intake — recovers the response/midrange the restricted OEM map leaves on the table, while holding closed-loop stoich for the certificate. Not a separate buy: part of the ECU work (see `electronics.md`) | `[LOCKED]` |
| Quickshifter | **Bidirectional quickshifter / autoblipper** — firmware-native: the ECU already owns ignition cut + ride-by-wire throttle, so clutchless up/down shifts cost almost nothing to add (see `electronics.md` §6) | `[LOCKED]` feature |
| Intake | Velocity stacks + premium filter (DNA / Sprint) into a bespoke airbox; retune to suit | `[PENDING]` |
| Cooling | **Uprated thermal** — higher-capacity alloy radiator + **oil cooler** + premium silicone hoses. Genuinely worth it: tight featherbed packaging, front-mount radiator, black bike in hot-market sun | `[PENDING]` recommended |
| Covers / finish | Billet engine covers (CNC Racing / Gilles), flat-black per the finish rule; titanium engine fasteners; iridium plugs. Jewellery — the factory clutch is retained, so a billet clutch cover is cosmetic | `[PENDING]` |

**Avoid** (breaks Euro 5+ and/or reliability on a one-off): high-compression pistons, hot cams, port-and-polish, de-cat. The CP3 is at its best breathing freely and tuned well, not bored and cammed.

## 3 · Cooling system (install)

*Liquid-cooled CP3 in a tight featherbed — the frame is built **around** the engine + a front-mount radiator (`chassis.md` §1). Cooling is Yamaha-first (rad + fan come with the donor); the packaging and routing are bespoke, and the uprated-thermal option is in §2.*

| Item | Spec | Status |
|---|---|---|
| Radiator | CP3 factory radiator retained (Yamaha-first), front-mounted; **higher-capacity alloy rad optional** (see §2 — worth it for the tight packaging + hot markets) | `[BUY]` donor / `[PENDING]` upgrade |
| Fan | Factory fan, **ECU-controlled / thermostatic** (aux PWM/GPIO — `electronics.md` §6); kept off the alternator at idle unless coolant demands it (`electronics.md` §9) | `[BUY]` / `[BESPOKE]` control |
| Water pump | Engine-internal (CP3 mechanical) | — |
| Thermostat | Factory | `[BUY]` donor |
| Coolant temp | Factory CLT sensor read by the ECU (`electronics.md` §2) | — |
| Hoses / routing | **Bespoke** routing around the narrow inline-3; premium silicone (see §2) | `[BESPOKE]` |
| Expansion / overflow | Bespoke expansion + catch tank, packaged in the frame | `[BESPOKE]` |
| Oil cooler | Optional uprated-thermal item (see §2) — placement for clean airflow | `[PENDING]` |

**Packaging note.** The front-mount radiator + fan clearance + airflow path must be resolved on the frame jig (`chassis.md`); a black bike in hot markets and the enclosed featherbed argue for the alloy-rad + oil-cooler option rather than bare factory cooling.

## 4 · Fuel system

*Speed-density EFI on the retained CP3 fuel hardware (Yamaha-first). The tank **shape/capacity/filler** is bodywork (`bodywork.md`); the pump, lines, regulator and evap live here. Injector control and pump drive are the ECU's (`electronics.md`).*

| Item | Spec | Status |
|---|---|---|
| Fuel pump | CP3 factory **in-tank pump + regulator** retained; ECU-driven (aux — `electronics.md` §6) | `[BUY]` donor |
| Injectors | Factory CP3 injectors; sequential, ECU-timed (`electronics.md` §2/§6) | `[BUY]` donor |
| Fuel rail | Factory rail, adapted to the intake | `[BUY]` donor |
| Tank | Bespoke — capacity/filler/vent in `bodywork.md`; pump mount + fuel take-off packaged here | `[BESPOKE]` (bodywork) |
| Lines / filter | **Bespoke** EFI-rated line + filter; sealed quick-connects to the in-tank pump | `[BESPOKE]` / `[BUY]` |
| Evap / charcoal canister | **Charcoal canister + purge valve** — Euro 5+ requirement (`emissions_certification.md`); purge is **ECU-controlled**; route canister + rollover vent | `[PENDING]` (emissions) |
| Filler / vent | Road-legal filler + rollover vent to the canister | `[PENDING]` |

**Evap note.** The charcoal canister + purge valve is a homologation item (in the emissions hardware checklist) but a *fuel-system* install — packaging the canister, the tank rollover vent and the purge line into the café bodywork needs planning early, not after the tank is built.

## 5 · Open engine items `[PENDING]`

- **Exhaust system** — direction resolved (premium full titanium + metal cat in the collector — see §2); the **specific system is still `[PENDING]`** (Akrapovič / SC-Project / Arrow / bespoke). When chosen, add its country of origin per the sourcing rule.
- *(Clutch resolved: retain the CP3's factory assist/slipper clutch — it comes with the engine/gearbox. Primary drive is internal to the CP3.)*

---

# Caveats (engine)

1. **Homologation ceiling governs the upgrades** — Euro 5+ means catalysed exhaust + closed-loop tune; no de-cat, high-comp pistons or hot cams. Premium here is breathing + tune + firmware + thermal + finish, not cubic power.
2. **The engine is unrideable until the ECU is proven** — trigger decode, ride-by-wire, immobiliser handshake and fail-safes all live in `electronics.md`; prototype on a used CP3 first.
