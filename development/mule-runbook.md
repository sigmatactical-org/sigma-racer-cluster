# Sigma — ECU-on-Mule Runbook

*Companion to `README.md`. The concrete, step-by-step procedure for developing and proving the custom ECU on a **used CP3 "mule"** before the new build engine sees a single injection event. This is the execution checklist; the firmware architecture and control strategy live in `efi.md`, the hardware in `electronics.md`. Phases map to the `efi.md` §12 bring-up gates (noted per phase).*

> **Golden rule:** every failure that can hurt an engine happens here, on a cheap used lump — never on the new build engine. Nothing advances a phase until its **gate** passes.

---

## Rig bill of materials

| Item | Note | Status |
|---|---|---|
| Used CP3 donor (used MT-09) | Running or known-good engine; the disposable dev engine | `[BUY]` used |
| Engine run stand / cradle | Bench-mount the engine (or keep the used donor's frame) | `[BESPOKE]` |
| Development harness | Bench loom, ECU ↔ engine; uses the connector standards in `electronics.md` §3 | `[BESPOKE]` |
| Custom ECU (G474 dev board) | The unit under test (`efi.md` §2) | `[BESPOKE]` |
| Fuel rig | In-tank pump in a jug/surge tank, or bench fuel supply + return | `[PENDING]` |
| Cooling | Factory radiator + fan + coolant | `[BUY]` donor |
| Test exhaust | Short header + **wideband bung** for the LSU 4.9 | `[BESPOKE]` |
| Battery + charging | Battery + factory stator + reg/rec (`electronics.md` §9) | `[BUY]` |
| Instrumentation | Scope, logic analyzer, USB/defmt logging, multimeter | `[BUY]` |
| **Safety kit** | Big red **kill switch**, CO2/dry-powder extinguisher, eye/ear protection, **CO awareness (ventilation)**, coolant-temp + oil-pressure gauges | `[BUY]` |

---

## Phase 0 · Acquire & prep the mule
*(prerequisite to `efi.md` stage 6)*

1. [ ] Buy a **used MT-09 / CP3** — cheapest running example; document the model year (gen-1 vs gen-2 matters for trigger + bearings).
2. [ ] Get **Yamaha service data + wiring diagrams** for that exact year.
3. [ ] Mount the engine on the **run stand** (or keep it in the donor frame stripped to essentials).
4. [ ] Inventory the harvest: throttle body, all sensors, coils, injectors, in-tank pump, stator + reg/rec, radiator + fan — confirm each is present and undamaged.
5. [ ] Photograph and label **every OEM connector** before disturbing the loom.

**Gate 0:** engine on a stand, service data in hand, connectors documented.

## Phase 1 · Characterize the engine (engine not running)
*(this is the "measure, don't invent" work `efi.md` §4–6 depends on)*

6. [ ] **Crank/cam sensor:** determine VR vs Hall; scope the signal while spinning the engine on the starter (plugs out). Capture the **tooth / missing-tooth pattern** and the cam pattern. Save the waveforms.
7. [ ] **TDC reference:** find TDC #1 (via the crank access) and **measure the gap-to-TDC offset** against the trigger pattern.
8. [ ] **Analog sensors:** record transfer functions — CLT + IAT (resistance vs temp), MAP (V vs pressure), dual TPS + dual APP (V vs position across full travel).
9. [ ] **Loads:** measure injector + coil resistance; confirm connector pinouts.
10. [ ] **Immobiliser/CAN:** sniff the OEM bus at key-on/crank; log what the engine expects to start (`efi.md` §10).

**Gate 1:** a characterization sheet — trigger pattern, TDC offset, every sensor curve, injector/coil data, immobiliser findings. **These become the firmware's starting constants.**

## Phase 2 · ECU hardware bring-up (bench, no engine)
*(`efi.md` stage 2)*

11. [ ] Bring up the **G474 board:** clocks, power rails, defmt/RTT, USB.
12. [ ] Verify peripherals: timer input-capture, compare outputs, ADC-DMA, HRTIM, FDCAN loopback, SPI (CJ125).
13. [ ] Build and continuity-test the **development harness** to the documented pinouts.

**Gate 2:** all peripherals verified; harness rings out clean.

## Phase 3 · Bench trigger with a synthesized signal
*(`efi.md` stage 3)*

14. [ ] Feed a **signal-generated crank + cam** (using the Phase-1 pattern) into the ECU.
15. [ ] Confirm the decoder produces correct **angle + RPM**; sweep RPM including hard acceleration/deceleration.
16. [ ] Scope the **injection + ignition outputs** against commanded angles.

**Gate 3:** timing correct across the RPM sweep incl. transients; sync-confidence gates spark correctly.

## Phase 4 · Driver bench (dummy loads)
*(`efi.md` stage 4)*

17. [ ] Drive the **3 injector low-side outputs** into dummy loads/injectors — verify pulse widths, no cross-talk.
18. [ ] Drive the **3 coil-on-plug outputs** into the coils + a spark tester — verify **dwell vs Vbat**, no false fire.

**Gate 4:** correct dwell, correct pulses, no spurious events.

## Phase 5 · Ride-by-wire rig — SAFETY GATE
*(`efi.md` stage 5 / gate 5 = milestone **M3**)*

19. [ ] Mount the **throttle body on the bench**, plate visible, nothing downstream.
20. [ ] Bring up **dual APP + dual TPS** reads and the **H-bridge** motor control; tune the position loop.
21. [ ] Bring up the **independent safety monitor** (plausibility + tracking + torque security — `efi.md` §7).
22. [ ] Run the **full fault-injection matrix live:** disconnect each APP, each TPS, force disagreement, stall the plate, open/short the motor, sag the supply.
23. [ ] Confirm **every fault → fail-safe** (motor off → spring-closed → fuel-cut, latched) with **no runaway**.

**Gate 5 (hard safety gate):** every fault drives the plate safely closed. **No engine start is permitted until this passes.**

## Phase 6 · Engine run prep
*(prerequisite to first start)*

24. [ ] Plumb the **fuel rig** (pump primed, filter, pressure checked, return); leak-check.
25. [ ] Connect **cooling** (coolant filled, fan wired to the ECU) and the **test exhaust** with the wideband + CJ125.
26. [ ] Connect **battery + charging**; verify reg/rec set-point suits the battery (`electronics.md` §9).
27. [ ] Resolve the **immobiliser** — replicate or delete per Phase-1 findings; confirm the ECU grants start-permit.
28. [ ] **Safety brief the cell:** kill switch reachable, extinguisher, ventilation for CO, oil-pressure + coolant-temp gauges live.

**Gate 6:** fuel/cooling/exhaust/charging plumbed and leak-free; immobiliser satisfied; safety kit in place.

## Phase 7 · First start & idle — "the engine runs"
*(`efi.md` stage 6 = milestone **M4**)*

29. [ ] **Crank with fuel disabled** — confirm sync holds under real cranking, oil pressure comes up, no fault storms.
30. [ ] Enable fuel; **attempt first start** with conservative cranking/warmup enrichment.
31. [ ] Achieve **stable idle**; watch coolant temp, verify the fan trigger.
32. [ ] Bring up **closed-loop lambda** after light-off; confirm λ=1 hold.
33. [ ] Re-run key **fail-safes on the running engine** (kill, throttle fault → fuel cut).

**Gate 7:** stable idle, clean sync, closed-loop working, fail-safes proven on a live engine.

## Phase 8 · Dyno tune
*(`efi.md` stage 7)*

34. [ ] Put the mule on a **dyno**; build the **VE + ignition** maps under load (RPM × MAP).
35. [ ] Tune **transient** (accel/decel), **warmup**, **overrun fuel-cut**, **rev-limiter**.
36. [ ] Characterize **knock**; set retard + safety margins.
37. [ ] Commission the **bidirectional quickshifter** (`engine.md` §2).
38. [ ] Verify the tune holds **stoich for the cat** and is emissions-clean (`emissions_certification.md`).

**Gate 8:** complete, safe, emissions-clean map; no knock; quickshifter working.

## Phase 9 · Transfer to the build engine

39. [ ] **Freeze** firmware + calibration from the mule.
40. [ ] Install the **new CP3** in the bike; flash the frozen build.
41. [ ] **Re-validate the deltas only** — new-engine trigger/sensor tolerances, the real exhaust/cat, final airbox — then road shakedown (`build.md` §5).

**Gate 9:** build engine runs on the mule-proven firmware with only final validation outstanding.

---

# Caveats (mule)

1. **Do not skip Phase 5.** The ride-by-wire safety gate is the reason the mule exists — prove the fail-safe on the bench before any engine start.
2. **The mule is disposable; the build engine is not.** Every mistake belongs here. The new CP3 only ever runs mule-proven firmware.
3. **Characterization (Phase 1) is the foundation** — every firmware constant traces to a measurement taken here, not a number carried from another engine.
4. **Budget dyno calendar time** — Phase 8 tuning is often more elapsed time than writing the firmware; it's on the critical path (`build.md`).
