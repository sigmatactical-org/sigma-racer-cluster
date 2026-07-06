# Sigma — Bodywork & Ergonomics

*Companion to `README.md`. The bodywork half of the café racer's character — **minimalist**: tank, seat/cowl, subframe, fenders, the rider triangle (clip-ons + rearsets) and the hand/foot controls. Chassis & running gear live in `chassis.md`; engine in `engine.md`; electrical/switchgear in `electrical.md`; the cockpit display in `electronics.md` §8. Flat-black finish and the sourcing rule come from the `README.md` hub.*

---

## 1 · Bodywork

**Design language — minimalist `[LOCKED]`.** Strip to the essential: the bodywork's job is to *get out of the way* and let the mechanicals be the aesthetic — the featherbed frame, the CP3 triple, and the Öhlins / Brembo / Kineo jewellery are the styling (they're the build's purpose, `README.md`). Principles:

- **Few parts, clean lines** — slim tank, solo seat, short tail, small round LED headlight.
- **Expose the mechanical, hide the ugly** — route the loom internally, tuck the cockpit compute + DC-DC out of sight, minimise visible fasteners and brackets.
- **Flat-black restraint** — no graphics, no chrome; interest comes from material + texture contrast, not decoration.
- **Integrate the tech invisibly** — the 6.86" bar cockpit (`electronics.md` §8) is the single modern touch, flush-mounted; everything else reads classic-minimal.
- Minimalism also serves the **≤190 kg target** (`build.md` §1) — less bodywork, less mass.

| Item | Spec | Status |
|---|---|---|
| Design language | **Minimalist** — expose the mechanicals, few clean parts, flat-black restraint, tech hidden (principles above) | `[LOCKED]` |
| Fuel tank | Bespoke **slim café teardrop**, hand-formed alloy; minimal — no graphics, flush/hidden filler; knee-cutaways to the narrow triple; capacity + vent + evap take-off to suit (`engine.md`) | `[BESPOKE]` |
| Seat / cowl | **Solo** café seat + minimal short tail cowl; **rugged tactical texture** — waxed-canvas / mil-webbing upholstery, knee grips; flat black; no pillion | `[BESPOKE]` |
| Subframe | Minimal **exposed tube** subframe to the featherbed rear (part of the look); carries seat + tucked LED tail + plate; sized to the shock/linkage top mount | `[BESPOKE]` |
| Front fender | *(Speced in `chassis.md` §2 — minimal blade off the FG 621 lowers)* | — |
| Rear / tail | Minimal **tail-tidy**: tucked LED tail/stop + plate hanger (road-legal placement — `emissions_certification.md`); no bulky rear fender | `[PENDING]` |
| Headlight | Small **round LED** headlight, minimal or no nacelle; clean mount to the yokes | `[PENDING]` |
| Material / finish | Alloy + carbon + minimal steel; **flat black throughout**, per-material texture; optional **muted tactical accents (olive/FDE) + stencilled markings** per the `README.md` finish/ethos | `[PENDING]` |

## 2 · Rider triangle (ergonomics)

| Item | Spec | Status |
|---|---|---|
| Clip-ons | **Decided: low, classic café** — clamped **at/below the top yoke** for the aggressive committed café position (the pure-café call, over the scrambler-comfort option). Premium **adjustable**, to the FG 621 43 mm tubes; final height set by the bespoke yoke (`chassis.md` §2) | `[LOCKED]` position / `[PENDING]` part |
| Rearsets | **Premium adjustable billet, foldable, blacked-out** — Gilles Tooling (AT/DE) or Bonamici / Rizoma (IT), EU-sourced; **rear Brembo master integrated** into the pedal; shift linkage to the CP3 gearbox, **GP-shift optional**. Adjustable to fit a range of riders (manufacturer) | `[LOCKED]` spec / `[PENDING]` part |
| Seat height / reach | Set by subframe + seat build; validate rider fit against the frozen chassis geometry | `[PENDING]` |
| Mirrors | **Decided: round mirrors, mounted off the handlebars** (stalk / bracket off the top yoke or cowl — keeps the bar area clean, suits the minimalist look). Must meet road-legal field-of-view for approval | `[LOCKED]` type / `[PENDING]` part |

## 3 · Controls & actuation

| Item | Spec | Status |
|---|---|---|
| Throttle | Ride-by-wire twistgrip — **dual APP sensors** read by the ECU (`electronics.md` §2/§6); no cable. Grip assembly must house/interface the APP unit | `[LOCKED]` (RbW) / `[PENDING]` grip |
| Clutch actuation | **Decided: hydraulic — Brembo RCS clutch master to mirror the 19 RCS brake master** (same radial-master line/look on both bars). The CP3 clutch is **cable from the factory**, so this needs a **hydraulic slave conversion** (Magura Hymec-class kit, MT-09-fit, or bespoke slave — `engine.md`). Retains the assist/slipper clutch | `[LOCKED]` hydraulic / `[PENDING]` slave |
| Front brake | Brembo 19 RCS radial master + lever *(speced in `chassis.md` §2)* | — |
| Levers / grips | **Matched Brembo RCS folding levers** both bars (brake + clutch — same line/look); minimal grips to suit | `[LOCKED]` pair / `[PENDING]` grips |
| Switchgear | Bars switches (start/kill, lights, indicators, horn, mode) → the ECU / CAN — **speced in `electrical.md`** | — |

## 4 · Open items `[PENDING]`

- Whole domain is early: lock the **frozen chassis geometry** (`chassis.md`) and the **subframe/shock top-mount** before committing tank/seat/subframe shapes.
- Clutch actuation **resolved: hydraulic Brembo RCS** (§3) — validate the hydraulic slave conversion on the CP3 (`engine.md`).
- Confirm **road-legal** mirror field-of-view, plate/light placement and rider-visibility items for EU/UK **type-approval** (for sale — `emissions_certification.md`).

---

# Caveats (bodywork)

1. **Bodywork keys off frozen geometry** — tank, seat and subframe shapes can't finalize until the chassis geometry and shock/linkage top mount are locked; build the rider triangle to the validated chassis, not the other way round.
2. **Road-legal equipment is part of bodywork** — mirrors, plate/lighting placement and rider ergonomics feed the homologation checklist; treat them as approval items, not just styling.
