# Sigma — EFI / ECU Firmware

*Companion to `README.md`. The deep spec for the custom engine-management **firmware** — the single largest work item in the build, and the gate on the engine ever running. The ECU **hardware** (MCU choice, sensor stack, connectors, charging) lives in `electronics.md` §1–7; this doc is the software: architecture, real-time model, control strategies, the safety-critical ride-by-wire, and the bench→dyno→road bring-up plan.*

*Project rule throughout: **characterize, don't invent.** Every engine-specific number here (tooth pattern, TDC offset, VE, dead-times, sensor transfer functions) is `[MEASURE]` on the actual CP3 + Yamaha service data — the values in tables are placeholders/shape, not calibration.*

---

## 1 · Scope & principles

**What the firmware must do to make the CP3 rideable:**

1. Decode crank/cam → know engine **position + speed** every cycle (hardware truth).
2. Deliver **sequential fuel** (3-cyl, speed-density) and **sequential ignition** (coil-on-plug, dwell-controlled).
3. Drive the **ride-by-wire throttle** closed-loop from dual rider-demand sensors — *safety-critical*, with an independent monitor and a fail-safe.
4. Run **closed-loop lambda** for the Euro 5+ tune (`emissions_certification.md`).
5. Satisfy or delete the **immobiliser / CAN handshake** so the engine starts.
6. Talk to the **cockpit** (CAN-FD) and a **tuning host** (USB), and never brick a running engine.

**Principles (from `electronics.md` §4, expanded):**
- **Hardware is truth.** Crank/cam are timer input-capture only; injection/ignition are hardware compare outputs. No engine event depends on the async executor's latency.
- **Two time domains.** A hard-real-time layer (ISRs + timers) owns anything angle- or safety-critical; an async layer (Embassy) owns everything that can tolerate milliseconds.
- **Safety is independent, not layered.** The throttle safety monitor and watchdogs do not share code paths with the controller they check.
- **Fail toward the ditch, not the wall.** Every fault has a defined safe state; the default for throttle faults is spring-to-closed + fuel-cut.
- **Rust, `no_std`, no heap in the control path.** Static allocation; `panic = abort` → watchdog reset with limp restart.

## 2 · Platform & toolchain

| Item | Choice | Status |
|---|---|---|
| MCU | **STM32G474** (motor-control grade — 5× fast ADC, HRTIM, adv. timers, FDCAN, CORDIC/FMAC, USB FS). Baseline fallback F405; headroom H743 for on-board knock DSP (`electronics.md` §6) | `[LOCKED]` direction |
| Language / async | **Rust**, `embassy-stm32` HAL + Embassy executor for the async domain; interrupts + PAC for the RT domain | `[LOCKED]` |
| Logging | **defmt** over RTT (bench) + framed logs over USB | `[LOCKED]` |
| Debug / flash | **probe-rs** (CMSIS-DAP / ST-Link) | `[LOCKED]` |
| Host tests | `cargo test` on host for pure logic (decoder, tables, plausibility, fault matrix) — no target needed | `[LOCKED]` |
| HIL | Bench signal-gen for crank/cam; scope + logic analyzer on inj/ign/RbW | `[PENDING]` rig |
| Cal storage | Wear-levelled config in on-chip flash (+ optional external SPI flash for logs) | `[PENDING]` |

## 3 · Real-time architecture (task & interrupt model)

**Interrupt priority (highest → lowest):**

| Prio | Handler | Domain | Job |
|---|---|---|---|
| 0 (top) | Crank input-capture | RT | Timestamp each tooth; decode; update angle/speed; **schedule** the next inj/ign compare events |
| 1 | Cam input-capture | RT | Phase/sync (720° cycle identity) |
| 2 | Inj/ign compare-match | RT | Fire injector open/close + coil dwell/fire at pre-armed timestamps |
| 3 | RbW control tick (HRTIM/timer, ~1 kHz) | RT | Throttle position/torque loop → H-bridge duty |
| 4 | Safety monitor tick (independent timer + WWDG) | Safety | Plausibility on APP/TPS + throttle tracking; trip fail-safe |
| — | ADC DMA complete | async | Hand samples to the async domain |
| low | Embassy executor | async | Lambda loop, state model, CAN, USB, logging, cal |

**Angle-domain scheduling (the core trick).** The crank ISR maintains `(engine_angle, angular_velocity, last_tooth_timestamp)`. For each cylinder it computes the *time* of the next injection start/end and ignition dwell-start/fire from target **angles**, using velocity to predict across the interval, and arms hardware timer compare channels. Result: fuel and spark land at commanded crank angles regardless of executor state, with acceleration handled by re-prediction each tooth.

**Shared state.** RT→async is single-producer via lock-free cells / `AtomicU32` snapshots (RPM, load, phase, fault flags). async→RT (new cal/targets) is published double-buffered so the RT side never reads a half-written table.

## 4 · Trigger decode & sync `[MEASURE]`

- **Read the wheel off the engine.** CP3 crank reluctor pattern (tooth count, missing-tooth) + cam pattern are **decoded as found** from the actual engine + Yamaha service data — *never assumed* (`electronics.md` §5).
- **Sensor front-end.** Characterize VR vs Hall on the factory sensor; Hall feeds logic-level direct, VR needs a conditioner (MAX9924-class). Confirm air-gap/supply on the engine.
- **Decoder state machine:** `LOST → SYNCING → SYNC_CRANK → SYNC_FULL`. Missing-tooth detection by inter-tooth ratio threshold; cam edge resolves the 720° cycle (which of two TDCs). Noise/'too fast'/'too slow' tooth rejection with a resync fallback.
- **Outputs:** absolute engine angle, filtered RPM, cycle phase, and a **sync-confidence** flag that gates spark (no spark until `SYNC_FULL`).
- **TDC offset** `[MEASURE]` on the real engine; **240° even-fire** cylinder spacing sets the per-cylinder base angles.
- **Test:** replay recorded/​generated crank+cam waveforms on the bench rig and in host unit tests (golden captures → expected angle stream), including a dropped/extra-tooth and a startup-from-cold case.

## 5 · Fuel strategy

**Model:** speed-density — `air = f(MAP, IAT, RPM, VE)`, `fuel = air / AFR_target`, corrected and scheduled sequentially.

| Element | Approach | Status |
|---|---|---|
| VE table | RPM × MAP, interpolated (CORDIC/FMAC-assisted) | `[MEASURE]` (dyno) |
| Charge temp | IAT + CLT blend for density + wall temp | `[MEASURE]` |
| Injector model | Dead-time vs **Vbat**, slope/offset, small-pulse non-linearity | `[MEASURE]` per injector |
| Sequential timing | Injection end-angle target per cylinder; phased on cam sync | `[PENDING]` |
| Transient | Accel/decel enrichment (wall-wetting τ/x model or simpler async pump) | `[PENDING]` |
| Start/warmup | Cranking pulse + warmup enrichment vs CLT; afterstart decay | `[PENDING]` |
| Closed-loop | Lambda trim (short-term) + learned long-term table (§8) | `[LOCKED]` strategy |
| Cut logic | Overrun fuel-cut + resume; rev-limit fuel-cut (with ign-cut) | `[PENDING]` |

Drivers: 3× low-side injector outputs, hardware-timed (one per cylinder), from compare channels (§3).

## 6 · Ignition strategy

- **Advance table** RPM × load `[MEASURE]`; per-cylinder trim allowed.
- **Dwell control** vs Vbat to a target coil charge; over-dwell protection; drive the **CP3 factory coil-on-plug** (one output per cylinder).
- **Knock** — factory knock sensor (or piezo) → windowed detection; **per-cylinder retard** + recovery ramp; conservative until characterized.
- **Rev limiter** — soft (progressive spark retard/cut) → hard (spark + fuel cut). Never leave fuel injecting with spark cut on the cat (raw-fuel protection ties to emissions).
- **No-sync = no spark** (gated by §4 confidence). Spark scheduling identical to fuel: pre-armed compare events.

## 7 · Ride-by-wire (safety-critical) — the crux

*This is the subsystem that makes the whole project a real engineering effort. Treat it with automotive-grade rigor even though the build is not formally ASIL-certified.*

**Sensor set (redundant):** dual **APP** (rider demand at the grip) + dual **TPS** (throttle plate position), plus the H-bridge motor on the CP3 throttle body (`electronics.md` §2/§6).

**Control:** rider demand (APP) → target throttle **position** (Phase 1) / **torque map** (Phase 2) → PID + feed-forward → H-bridge duty with current limit; ~1 kHz loop (§3). Limp target = idle-only.

**The independent safety monitor** (separate timer + windowed watchdog, minimal shared code):
1. **APP plausibility** — the two APP signals must track within a band and sum/mirror correctly; disagreement → fault.
2. **TPS plausibility** — dual TPS agree; sensor range/short/open checks.
3. **Tracking** — actual throttle (TPS) must follow commanded target within a time/error envelope; runaway or stuck-open → trip.
4. **Torque monitor (Phase 2)** — modeled torque ≤ driver-requested torque + margin (the ISO 26262 "torque security" pattern); excess → trip.

**Fail-safe (any trip):** cut H-bridge → spring returns plate to its default closed/limp stop → **fuel cut** → limp/​stop state, fault latched, cockpit warning. Recovery only via a defined re-arm (key cycle / zero-demand).

**Start permit:** engine will not start unless APP at idle, both sensor pairs plausible, kill/​sidestand/​clutch interlocks satisfied (`electrical.md` §5), and decoder not faulted.

**Development rigor:** RbW logic is host-unit-tested against a fault-injection matrix (every single-sensor failure, stuck plate, motor open/short, supply sag) *before* it ever drives the real motor; then proven on a **bench throttle-body rig** with the plate physically watched, then on the mule.

## 8 · Closed-loop lambda & emissions

- **Sensor:** Bosch **LSU 4.9 + CJ125** over SPI (`electronics.md` §2); post-collector single sensor for the triple.
- **Warmup:** CJ125 heater control; no closed-loop until light-off; protect sensor on overrun.
- **Loop:** PI trim around stoich (λ=1) in the closed-loop region; **long-term fuel trim** learned into a correction table and persisted; open-loop enrichment for power/thermal protection zones.
- **Emissions tie-in:** the tune holds stoich for the cat (`emissions_certification.md`); catalyst-overtemp protection; small-series route means **no full OBD-II** required (keeps diagnostic scope sane — decide before locking cal scope).

## 9 · Safety architecture & fault handling

**Watchdogs:** independent **IWDG** (free-running, un-stoppable) + **WWDG** windowed on the safety task; brown-out reset; clock-security (CSS). Panic → abort → watchdog reset → **limp restart**.

**Fault matrix (each fault → detection → safe action → recovery):**

| Fault | Detect | Action |
|---|---|---|
| Loss of crank sync | decoder timeout/ratio | spark+fuel cut, engine stops safely, retry sync |
| APP disagreement | plausibility band | RbW fail-safe (closed + fuel cut) |
| TPS disagreement / stuck | plausibility + tracking | RbW fail-safe |
| Throttle runaway | tracking envelope | RbW fail-safe, latch |
| Lambda sensor fail | range/heater fault | drop to open-loop map, warn |
| CLT/IAT/MAP fail | range/rationality | default + limp map, warn |
| Overtemp (CLT) | threshold | fan force-on, power limit, warn (`electronics.md` §9) |
| Undervoltage | Vbat | shed loads (PDM, `electrical.md`), protect start |
| Firmware fault/hang | IWDG/WWDG | reset + limp restart |

Every action raises a coded fault to the cockpit; nothing silently degrades.

## 10 · Immobiliser / CAN handshake `[MEASURE]`

- Confirm on the donor what the CP3 needs to **start and run** (the OEM immobiliser/CAN chatter).
- Decision path: **replicate** the minimal handshake on FDCAN, or **cleanly delete** it — whichever the standalone community + bench testing show is robust. Must not depend on the missing OEM cluster.
- Ties to the anti-theft/steering-lock road-legal item (`emissions_certification.md`) and the keyless/main-relay design (`electrical.md` §2).

## 11 · Comms & tuning

| Link | Use |
|---|---|
| **FDCAN** | Cockpit data (RPM, load, temps, lambda, gear, faults, telltales — `electronics.md` §8 M7 cluster) + possible immobiliser handshake. Defined **data dictionary** (message IDs, scaling) is a shared contract with the cockpit team. |
| **USB (FS)** | Tuning host: read/write cal tables live, real-time gauges, fault log, firmware update path. |
| **SPI** | CJ125 lambda, knock IC, external log flash. |

Quickshifter (engine upgrade, `engine.md` §2) lives here: bidirectional — **ignition cut** on up-shift + **throttle blip** (RbW) on down-shift, gated by load/RPM and a shift sensor.

## 12 · Bring-up & validation plan (gated)

*No stage starts before the previous one's gate passes. This is the critical path of the whole build.*

1. **Host logic** — decoder, tables, plausibility, fault matrix as pure Rust, `cargo test` with golden captures + fault injection. **Gate:** matrix green.
2. **Board bring-up** — G474 board, clocks, defmt, ADC-DMA, timers, FDCAN loopback. **Gate:** all peripherals verified.
3. **Bench signal-gen** — synthesized crank/cam → confirm angle stream + inj/ign timing on a scope against commands. **Gate:** timing within spec across an RPM sweep incl. accel.
4. **Driver bench** — injector/coil low-side + COP drivers into dummy loads; dwell/pulse verified. **Gate:** no false fire, correct dwell vs Vbat.
5. **RbW rig** — throttle body on the bench, plate watched; run the full fault-injection set live. **Gate:** every fault → fail-safe, no runaway. *Safety gate — no engine start before this passes.*
6. **Engine on the used CP3 mule** — real trigger/sensors; first start; idle; closed-loop light-off; immobiliser resolved. **Gate:** stable idle + clean sync, fail-safes proven on a running engine.
7. **Dyno** — VE/ignition/lambda tune to Euro 5+; transient/warmup/overrun; knock characterization; quickshifter. **Gate:** map complete, emissions-clean, no knock.
8. **Road shakedown** — heat, vibration, EMC, real-world transients on the mule, then the build engine. **Gate:** durability + rideability signed off (`build.md` §5).

## 13 · Risk register

| Risk | Mitigation |
|---|---|
| **RbW is genuinely dangerous if wrong** | Independent monitor + fail-safe; bench-prove the fault set before any engine start (gate 5) |
| Trigger pattern mis-decoded | Measure off engine; golden-capture tests; sync-confidence gates spark |
| Immobiliser blocks start | Resolve on the mule early (stage 6 prerequisite), community cross-check |
| Timing jitter under load | Hardware compare scheduling, not software; HRTIM; keep RT ISRs short |
| Cal takes longer than "firmware" | The tune (VE/ign/lambda/transient) is often more calendar time than the code — budget dyno time |
| Two software programs at once | ECU + cockpit are separate builds with a CAN contract between them (`electronics.md`) |

## 14 · Milestones

`M0` host logic green → `M1` board + peripherals → `M2` bench timing (synth trigger) → `M3` drivers + **RbW rig safety gate** → `M4` first start on mule → `M5` closed-loop idle + immobiliser solved → `M6` dyno tune complete → `M7` road shakedown signed off. **M3 is the hard safety gate; M4 is "the engine runs."**

---

# Caveats (EFI)

1. **The engine is unrideable until this is proven** — trigger, RbW, immobiliser, fail-safes and the tune all have to pass their gates; there is no shortcut around the bench and dyno.
2. **Ride-by-wire safety is not optional rigor** — build and bench-prove the independent monitor + fail-safe before the motor ever drives the real plate. Gate 5 exists for a reason.
3. **Characterize, don't invent** — every engine number here is measured on the CP3 + Yamaha data; the tables in this doc are architecture, not calibration.
4. **Prototype on a used CP3** — do all of stages 1–7 on the cheap mule before the new build engine sees a single injection event.
