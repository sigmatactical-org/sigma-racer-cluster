# Sigma — Chassis & Mechanical

*Companion to `README.md`. The chassis half of Sigma (the Σ's first domain): a bespoke chromoly featherbed chassis + running gear, built around the harvested Yamaha CP3 triple. The **engine & powertrain** (CP3 + premium upgrades) live in `engine.md`; electronics/ECU in `electronics.md`; emissions/homologation in `emissions_certification.md`. The strategy, sourcing rule, finish and the status-tag legend live in the `README.md` hub.*

---

## 1 · Chassis

| Item | Spec | Status |
|---|---|---|
| Builder | In-house fabrication | — |
| Frame | Bespoke featherbed-style chromoly steel, built around the CP3 + radiator (narrow inline-3 packages tight; front-mount radiator). Modern featherbed special — Triton-lineage triple | `[BESPOKE]` |
| Rear shock | **Öhlins STX 46 Blackline** — 46 mm high-pressure gas **monotube**, black body + spring. Order the **piggyback + compression-adjustable** configuration with the **hydraulic preload** option to get full adjustability (compression + rebound + preload) — *not all STX 46 variants carry the compression adjuster, so specify it explicitly.* Built to order: eye-to-eye length + spring rate derived from the **frozen** rising-rate linkage motion ratio & rider weight. ⚠ Ride height: length-adjustable lower eyelet is a TTX-tier feature and unconfirmed on the STX 46 — either option it, or set ride height via the built-to-order eye-to-eye length + linkage (the 170/60-17 rear's ~6 mm rise is baked in at order time). Matched to the FG 621 front | `[LOCKED]` / `[BUY]` built-to-order |
| Monoshock linkage | Bespoke rising-rate linkage to the Öhlins shock | `[BESPOKE]` |
| Swingarm | Bespoke fabricated, chain drive, matched to the linkage, sized for the **170 rear** (was 160 — see tire note). Check tire-to-swingarm/chain clearance at the 170 section width | `[BESPOKE]` |
| Rear wheel | Kineo true-tubeless spoked — 17 × 4.5", matched to the front; includes 8-pin cush drive + Ergal sprocket carrier. Matte black (Italy). 4.5" rim suits the 170/60-17. **⚠ Order with ABS tone-ring (pulse-ring) provision** — can't be added later; the ABS decision gates the wheel order | `[BUY]` made-to-order |
| Rear tire | **170/60-17** — Pirelli Scorpion Rally STR (72V). *(Was 160/60-17; STR is not made in 160/60-17, so moved to 170/60-17, an available STR size that suits the 4.5" rim — see tire note.)* | `[LOCKED]` |

**Tire note (Pirelli Scorpion Rally STR — front + rear).** Decision: run the road-biased rally tire for a scrambler-flavoured stance that still rides honestly on tarmac. Front **120/70-17 (58H)**, rear **170/60-17 (72V)** — a matched pair, both genuine STR sizes.

- **Why 170, not 160.** The STR is an ADV tire and Pirelli does not catalogue it in 160/60-17 (the original locked rear). The available 17" STR rears are 150/60, 150/70, 170/60, 180/55; 170/60-17 is the best fit for the 17 × 4.5" Kineo rim. So the rear lock moved 160 → 170.
- **Geometry knock-on.** 170/60-17 is ~12 mm larger in overall diameter than 160/60-17 → ~6 mm more rear ride height at the axle, plus a small final-drive/speedo change. Absorbed at **shock-order time** via the built-to-order eye-to-eye length + the bespoke rising-rate linkage (the STX 46's on-the-fly ride-height adjuster is unconfirmed — see rear-shock note). Set ride height/trail **on the STR**, not on a sport tire: this is a tall, round ADV profile and steers slower than sport rubber, so the chassis must be validated on the actual tire.
- **Swingarm.** Was "sized for the 160 rear" → now 170; confirm tire-to-arm and tire-to-chain clearance at the wider section.
- **Front speed rating — homologation check.** The STR front 120/70-17 is rated **58H (210 km/h)**. A standalone CP3 runs near that, and EU/UK approval requires the fitted tire's speed rating to meet or exceed the certified top speed. Confirm the 58H front is acceptable (or restrict/declare top speed accordingly) before treating the front size as final. Rear 72V (240 km/h) is clear. *(Logged in `emissions_certification.md`.)*

### Geometry targets

*Targets to design the frame + yokes to, and **validate on the actual STR tire** (a tall ADV profile steers slower than sport rubber — tire note). Trail = featherbed neck rake + yoke offset, set at the bespoke yoke (§2); ride height absorbs the 170/60-17's ~6 mm rise at shock-order.*

| Parameter | Target | Status |
|---|---|---|
| Rake (head angle) | ~24.5° | `[PENDING]` validate on STR |
| Trail | ~100 mm | `[PENDING]` — set via yoke offset (§2) |
| Wheelbase | ~1,430 mm | `[PENDING]` |
| Seat height | ~810 mm | `[PENDING]` (`bodywork.md`) |
| Weight distribution | ~50/50 static, low CoG | `[PENDING]` (`build.md` §1) |
| Ride height | Set on the STR at shock-order (§1 rear shock) | `[PENDING]` |

### Rear brake

*Brembo, matched to the front (§2). Single rear disc on the Kineo rear wheel; the master + pedal integrate with the rearsets (`bodywork.md`). Keys off the disc, same as the front.*

| Item | Spec | Status |
|---|---|---|
| Rear disc | Single floating disc (~220–245 mm) — Kineo builds the carrier to the chosen disc + the rear wheel/Ergal carrier (§1), same "key off the disc" logic as the front | `[BUY]` |
| Caliper | Brembo — 2-piston floating (e.g. P2 34) or billet, black; matched aesthetic to the front M4 | `[BUY]` |
| Caliper bracket | Bespoke billet bracket to the swingarm (`[BESPOKE]` swingarm, §1) | `[BESPOKE]` |
| Master cylinder | Brembo — integrated into the rearset pedal (`bodywork.md` §2) | `[BUY]` |
| Line | Braided stainless — Spiegler / Goodridge (as front) | `[BUY]` |

**ABS — decided: donor OEM Yamaha ABS.** Fit the new CP3 donor's **OEM Yamaha ABS** — modulator + front/rear wheel-speed sensors + tone rings, **new with the donor** (no calibratable standalone road ABS exists to buy). **ABS is mandatory** now that Sigma is a for-sale type-approved product (>125 cc, EU/UK/CA) — must be integrated into a certified braking system (`emissions_certification.md`). Requires: **Kineo wheels ordered with tone-ring provision** (front + rear rows), front + rear brake lines routed **through the modulator**, and the **ABS↔ECU CAN handshake** (`efi.md` §10–11, same class as the immobiliser). Cockpit ABS telltale already provided (`electronics.md` §8). Logged in `emissions_certification.md`.

### Final drive (chain & gearing)

| Item | Spec | Status |
|---|---|---|
| Chain | Premium X/Z-ring — DID / RK; **520** for lightness on a ~117 hp triple (525 if load margin wanted) | `[BUY]` |
| Front sprocket | Steel, on the CP3 countershaft; tooth count set by the gearing choice | `[BUY]` |
| Rear sprocket | On the Kineo **8-pin cush-drive + Ergal sprocket carrier** (§1); alloy or steel | `[BUY]` |
| Gearing | **Final-drive ratio is a decision, not a default** — sets top speed + acceleration. Gear to the **~200 km/h / ≤210 km/h top-speed target** (`build.md` §1), which keeps the **58H front (210 km/h — tire note)** legal; bias café/road within that ceiling | `[PENDING]` |
| Speed pickup | Wheel-speed signal for the cockpit speedo + any ABS — source (front wheel / countershaft) read by the ECU (`electronics.md`) | `[PENDING]` |

**Gearing ↔ homologation.** Top speed set by the final drive must not exceed the fitted tire's speed rating; if a taller ratio pushes past the 58H front's 210 km/h, either regear, restrict/declare top speed, or revisit the front tire (tire note + `emissions_certification.md`).

## 2 · Front End

| Item | Spec | Status |
|---|---|---|
| Fork | Öhlins FG 621 — 43 mm conventional (RWU, non-inverted), black. Largest non-inverted Öhlins: 800 mm length, 120 mm stroke, NIX 30 cartridge, 9.5 N/mm springs, 32 mm axle, fully adjustable. Ships universal (no yokes, no caliper/fender mounts). Source: Zodiac (NL, ships MX) / EU dealer, ~$2,300–2,800 | `[LOCKED]` / `[BUY]` |
| Yokes | Bespoke (in-house) — cut to FG 621 43 mm tubes + featherbed neck + café-trail offset | `[BESPOKE]` |
| Steering stem | Bespoke billet steel (4140 chromoly), heat-treated — not aluminium. Safety-critical: validate for steering loads | `[BESPOKE]` |
| Headstock bearing | **Yamaha CP3-family tapered set — upper 25×47×15, lower 30×55×17** (+ seals). The asymmetric pair the MT-09/XSR900/XSR900 GP run — *not* a symmetric 30×55×17 both-ends set. Design the bespoke stem journals + frame-neck bore to these two sizes. Tapered roller = rigid, preload-adjustable. ⚠ Confirm the exact set on a **gen-2 (’21+) donor / current Yamaha microfiche** — the 2021 frame was redesigned; upper 25×47×15 / lower 30×55×17 is confirmed on gen-1 (’14–’18) and is the family norm, but verify it carries to the GP before cutting journals. | `[LOCKED]` |
| Stem geometry reference | **Yamaha XSR900 GP** (gen-2 CP3 platform — MT-09/XSR900 ’21+ frame). Reference only; same-engine, same-power-class steering head, so a better-matched reference than a ~200 hp superbike stem. Final stem dims from a **measured donor stem + Yamaha microfiche**, never invented | `[LOCKED]` |
| Bearing kit | All Balls **22-2004** — MT-09/XSR900 set (upper 25×47×15 + lower 30×55×17, top+bottom+seals). The asymmetric set is the Yamaha middleweight norm (and shared with e.g. Suzuki Katana 600/750 ’89+), so sourcing stays trivial. Verify the gen-2/GP part number against a current catalog. | `[BUY]` |

**Offset / trail.** Set at the yoke (bespoke). Final trail = featherbed neck rake + yoke offset — independent of the Yamaha stem (the stem only sets the bearing interface). Stem dimensions come from a measured XSR900 GP / CP3-family donor stem + Yamaha microfiche — never invented.

**Rigidity note.** Switching to the Yamaha set drops the *upper* bearing from 30×55×17 (the old symmetric reference) to 25×47×15 — a smaller upper journal. For ~117 hp this is not the limiting element: neck-tube section and stem diameter dominate steering-head stiffness, both bespoke, and this is the exact set Yamaha runs behind the same CP3 engine. Keep a larger-taper upper on the table only if a neck-stiffness check calls for it; it buys marginal rigidity at the cost of a bigger, heavier neck and the loss of the Yamaha set's drop-in availability.

### Front wheel & brake

| Item | Spec | Status |
|---|---|---|
| Front wheel | Kineo true-tubeless spoked — 17 × 3.5", 28-spoke, forged 7000-alloy hub + CNC billet rim, matte black. TÜV-certified. Made to order around the 32 mm axle + chosen discs (Italy). **⚠ Order with ABS tone-ring (pulse-ring) provision** — can't be added later; the ABS decision gates the wheel order | `[BUY]` made-to-order |
| Front tire | **120/70-17 (58H)** — Pirelli Scorpion Rally STR. ⚠ 58H = 210 km/h; verify vs certified top speed for EU/UK approval (see tire note + `emissions_certification.md`) | `[LOCKED]` |
| Front axle | **32 mm — use the FG 621's supplied axle.** The fork is built around a 32 mm wheel axle and ships with axle sleeve/spacer hardware; the made-to-order Kineo wheel is built with bearings/spacers to suit the 32 mm axle. ⚠ Confirm exact box contents (axle + how many spacer sets) with Zodiac/Öhlins — a bespoke spindle is most likely *not* needed | `[BUY]` / confirm contents |
| Front discs | Twin 320 mm floating — Brembo; pick any stocked 320 mm floating pattern, Kineo builds the carriers to the chosen disc | `[BUY]` |
| Calipers | Brembo M4 (M4.34) cast monobloc, 4 × 34 mm, **100 mm radial mount** (Euro pitch), black. KBA homologation docs available. | `[BUY]` |
| Caliper brackets | Bespoke billet radial brackets — 100 mm caliper pitch, set to the 320 mm disc radius; the bracket replaces Brembo's offset adapters | `[BESPOKE]` |
| Master cylinder | Brembo 19 RCS radial (Brembo's recommended match for the 34 mm-piston M4) | `[BUY]` |
| Brake lines | Braided stainless — Spiegler (DE) / Goodridge | `[BUY]` |
| Front fender | Bespoke stays off the FG 621 lowers; carbon or flat-black alloy | `[BESPOKE]` |

**Build order (everything keys off the disc).** Pick the 320 mm disc first — its PCD/offset is the datum. Kineo then builds the hub, bearings, spacers and disc carriers to that disc + the 32 mm axle; the axle is fabricated in-house to the fork feet; the bespoke radial bracket places the M4 (100 mm pitch) at the 320 mm friction radius.

**Verify with the fork in hand — and check what the Zodiac package includes.** The bare Öhlins FG 621 lower has no caliper provision, *but* Zodiac's FG620/621 package (this spec's fork source) reportedly ships fork legs with axle clamps that already have **caliper mounting points**. Confirm directly with Zodiac which version carries them and what pattern — likely Harley-oriented, so it may need adaptation for the M4's 100 mm radial pitch. If the Zodiac mounts suit or adapt, the caliper problem is largely solved at purchase. **Decided: no machining of the fork.** If mounts aren't included, use a bespoke **axle-datum / foot-clamp** caliper bracket (Öhlins to sanction clamp location — not machined pads), with Rebuffini's FG620/621 brackets (Italy, EU) as reference/base. Also order the Kineo wheel early — lead time runs up to ~18 weeks.

## 3 · Sourcing Summary (mechanical)

| Buy now (catalog / new) | Buy used (prototyping) | Bespoke (one-off fabrication) |
|---|---|---|
| New CP3 donor (MT-09) — engine + box + RbW + sensors | Used CP3 (used MT-09) — ECU bench/proto mule | Featherbed frame (in-house) |
| Öhlins FG 621 fork (black) — Zodiac/EU | | Swingarm + monoshock linkage |
| Öhlins STX 46 Blackline rear shock | | Triple yokes (43 mm) |
| All Balls 22-2004 bearing kit (Yamaha set) | | Billet-steel steering stem |
| Kineo front + rear wheels (made-to-order) | | Engine build/display cradle |
| Brembo M4 calipers + 320 mm discs | | Front axle (32 mm) + caliper brackets + fender stays |
| Brembo 19 RCS master + braided lines | | |

## 4 · Open Decisions (chassis) `[PENDING]`

- **Bodywork & ergonomics** — tank, seat/cowl, subframe, rider triangle, controls: **now in `bodywork.md`.**

*(Exhaust and clutch are powertrain decisions — see `engine.md` §2–3. Electrical/lighting/harness live in `electrical.md`.)*

---

# Caveats (chassis)

1. **Steering stem is safety-critical** — billet steel + heat-treat, dimensions from a measured XSR900 GP / CP3-family donor stem + Yamaha microfiche, validated in-house. No invented numbers.
2. **Bespoke = cost + lead time** — frame, swingarm, yokes, stem and cradle are one-off-grade, not catalog parts.

*(Emissions/homologation is its own project — see `emissions_certification.md`. Electronics caveats live in `electronics.md`.)*
