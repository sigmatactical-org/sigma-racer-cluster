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
- **Production supply (for-sale scale):** harvesting a whole new donor per bike **does not scale** (`build.md` §2d — the surplus donor wrecks per-unit margin). For volume, pursue a **Yamaha crate-engine / OEM supply arrangement**; the donor-harvest model is fine for prototyping and the first few units only.

## 2 · Engine upgrades (premium)

*A premium café racer earns premium engine kit — but the **Euro 5+ homologation ceiling governs**: no de-cat, no high-comp pistons, no hot cams. The gains come from breathing + tune + firmware + thermal + finish, not cubic power. This deliberately reverses the "Yamaha-first, economize the ancillaries" rule for **select** items (exhaust, cooling, covers); the factory injectors, coils, pump and assist/slipper clutch stay — they're genuinely good.*

| Upgrade | Spec | Status |
|---|---|---|
| Exhaust | **Bespoke titanium — shortest practical: 3-into-1 collector → high-flow metal cat (near-engine, light-off) → very short SIDE-exit muffler, barely protruding** (dual stubby tips one side; single is the more minimal option — pick at fab). Compact/tactical, on-identity. **Pre-cat wideband + post-cat O₂** bracket the cat (closed-loop + OBD catalyst monitoring — `electronics.md` §2, `efi.md` §8). ⚠ **Shortness is floored by the drive-by noise limit** — a for-sale bike must pass type-approval noise (`emissions_certification.md`); too short = too loud. Not a de-cat pipe | `[LOCKED]` bespoke |
| Performance tune | Custom-ECU dyno tune to the actual cat + intake — recovers the response/midrange the restricted OEM map leaves on the table, while holding closed-loop stoich for the certificate. Not a separate buy: part of the ECU work (see `electronics.md`) | `[LOCKED]` |
| Quickshifter | **Bidirectional quickshifter / autoblipper** — firmware-native: the ECU already owns ignition cut + ride-by-wire throttle, so clutchless up/down shifts cost almost nothing to add (see `electronics.md` §6) | `[LOCKED]` feature |
| Intake | Velocity stacks + premium filter (DNA / Sprint) into a bespoke airbox; retune to suit | `[PENDING]` |
| Cooling | **Extreme cooling `[LOCKED]`** (hot-climate — Mexico pilot market, black bike, tight featherbed): oversized alloy radiator + **dedicated oil cooler** + high-CFM (poss. dual) ECU fan + forced-airflow ducting/shroud + high-perf coolant + hot-climate thermostat. Full detail in §3 | `[LOCKED]` |
| Covers / finish | Billet engine covers (CNC Racing / Gilles), flat-black per the finish rule; titanium engine fasteners; iridium plugs. Jewellery — the factory clutch is retained, so a billet clutch cover is cosmetic | `[PENDING]` |

**Avoid** (breaks Euro 5+ and/or reliability on a one-off): high-compression pistons, hot cams, port-and-polish, de-cat. The CP3 is at its best breathing freely and tuned well, not bored and cammed.

## 3 · Cooling system (install) — extreme

*Liquid-cooled CP3 in a cooling-hostile package (enclosed featherbed, front-mount rad, black bike, hot pilot market). Spec'd for **extreme hot-weather capability** — oversized rad + oil cooler + high-CFM forced airflow — not bare factory cooling. Fan control + overtemp derate live in the ECU (`electronics.md` §6, `efi.md` §9).*

| Item | Spec | Status |
|---|---|---|
| Radiator | **Oversized alloy radiator** — max frontal area / thicker higher-fin core that packages in the featherbed; factory rad kept only as fallback | `[LOCKED]` / `[BESPOKE]`+`[BUY]` |
| Oil cooler | **Dedicated engine oil cooler** (thermostatic sandwich-plate take-off), placed for clean airflow — the big win for sustained hot-weather load | `[LOCKED]` / `[BUY]` |
| Fan | **High-CFM electric fan (possibly dual) on a shroud**, ECU-controlled/thermostatic (`electronics.md` §6); sized for **hot-weather idle-in-traffic** (worst case — no ram air). ⚠ Higher fan draw stresses the idle charging balance (`electronics.md` §9) | `[BESPOKE]` |
| Airflow / ducting | Bespoke ducting + fan shroud to **force air through the core** — in a tight featherbed airflow, not just core size, is the limit | `[BESPOKE]` |
| Coolant + thermostat | High-performance coolant; **hot-climate thermostat** opening temp; premium silicone hoses | `[BUY]` |
| Water pump | Engine-internal (CP3 mechanical) | — |
| Temp sensing | Factory CLT + **added oil-temp sensor** → ECU (overtemp fan force-on + power derate, `efi.md` §9) | `[BUY]` |
| Expansion / overflow | Bespoke expansion + catch tank, adequate volume, packaged in the frame | `[BESPOKE]` |

**Packaging + interactions.** Airflow is the real constraint in the enclosed featherbed — force it with ducting + a fan shroud, resolved on the frame jig (`chassis.md`). Three couplings to hold: (1) the **radiator guard** (chassis protection) must be **airflow-open mesh**, not a choke; (2) the high-CFM fan runs hardest at **hot idle exactly when the alternator is weakest** — size the charging + LiFePO4 buffer for it (`electronics.md` §9); (3) **exhaust heat is now short side-exit** (§2 — no longer under the tail), so the under-seat battery/electronics are **clear of it**; instead shield the short side outlet from the **rider's leg/boot + rearset**, and confirm **lean-angle/ground clearance**. Resolve on the jig.

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

- **Exhaust system** — **resolved: bespoke ti, shortest-practical 3-into-1 + near-engine cat → short side-exit outlet(s), barely protruding, pre/post-cat O₂ (§2).** In-house/commissioned fab; validate **drive-by noise vs the type-approval limit** (the floor on how short), rider-leg/rearset heat, and lean-angle/ground clearance.
- *(Clutch resolved: retain the CP3's factory assist/slipper clutch — it comes with the engine/gearbox. Primary drive is internal to the CP3.)* **Actuation converted to hydraulic** — a Magura Hymec-class slave (MT-09-fit) or bespoke slave replaces the factory cable, so the **Brembo RCS clutch master** (matched to the brake — `bodywork.md` §3) can drive the retained clutch.

---

# Caveats (engine)

1. **Homologation ceiling governs the upgrades** — Euro 5+ means catalysed exhaust + closed-loop tune; no de-cat, high-comp pistons or hot cams. Premium here is breathing + tune + firmware + thermal + finish, not cubic power.
2. **The engine is unrideable until the ECU is proven** — trigger decode, ride-by-wire, immobiliser handshake and fail-safes all live in `electronics.md`; prototype on a used CP3 first.
