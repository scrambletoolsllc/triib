# Security and failover: plan

A plan for securing and adding failover to triib and the `atdecc` crate:
the controller, the host entities that run in avb-endpointd, and the
crates both use. Most of the protocol work lands in the avb crates
(atdecc, avb-stream, avb-lite, avb-mrp) and avb-endpointd, so the plan
says what each crate needs, in what order, and how releases move
through the projects that use them. Testing failover needs the
bench bridge, scrambox, in a test configuration, described at the end.

Status: proposal, 2026-11-11 (UTC), revised 2026-11-12 after review
by the avb, avb-endpointd, gptp and bridge projects, with the decisions
at the end settled. Nothing here is built.

References: Milan 1.3 5.4.2.1 and 5.4.2.2 (ACQUIRE_ENTITY,
LOCK_ENTITY), chapter 8 (seamless network redundancy), Annex B.1
(media clock holdover through a grandmaster change) and Annex C (the
STREAM_INPUT/OUTPUT redundancy extension); IEEE 1722.1-2021 7.4.2
(LOCK_ENTITY) and 7.6 (security services).

## Where things stand

Security:

- triib never sends LOCK_ENTITY or ACQUIRE_ENTITY and shows no lock
  holder. atdecc's controller has no lock requests and no lock state.
  ENTITY_LOCKED, ENTITY_ACQUIRED and NOT_AUTHENTICATED all end as a
  generic `Refusal::Aem(status)`.
- atdecc's entity role implements LOCK_ENTITY with Milan's 60 s
  timeout, gating the SET_* and audio-mapping commands. A holder that
  departs keeps the lock until it times out, ACMP is never gated, and
  ACQUIRE_ENTITY answers NOT_IMPLEMENTED (Milan asks for
  NOT_SUPPORTED).
- The controller trusts every source:
  - An ADP advertisement from any MAC replaces a known entity's MAC, so
    later AECP commands go to whoever sent it.
  - ENTITY_DEPARTING from any MAC removes the entity.
  - AECP responses are matched on IDs and sequence numbers, never on
    source MAC. Overheard success responses and any ACMP success
    response update the model and the bindings.
- Most controller tables are unbounded: entities, models, sessions,
  the cache and the per-entity command queue. The entity role's
  registrations stop at 16, but its `known` map is unbounded.
- `tests/robustness.rs` runs 200,000 random frames through the codecs.
  Nothing reaches the controller or entity state machines, and there is
  no fuzzing.
- 1722.1's security services (keys, AUTHENTICATE, transport security,
  stream encryption) exist in atdecc only as command and status codes.
  Milan 1.3 doesn't mention them.
- atdecc has no `#[non_exhaustive]`, so every new event, refusal, field
  or variant is a breaking change for triib, plaa, avb-endpointd,
  avb-stream and avb-lite.
- avb-endpointd's entities can be changed by any controller, with no
  allowlist. The daemon has no command socket: triib edits its config
  under an flock. Its sockets:
  - The interface claims, the abstract names `@avb-endpointd/<if>` and
    `@triib-endpointd/<if>`, have no file permissions, so any local
    user can reach or squat them.
  - The MRP sharing socket lives in the user's 0700 runtime folder
    (`$XDG_RUNTIME_DIR/avb-mrp-<if>.sock`, never anywhere else), so
    other users can't reach it. It has no peer check.

  Its user unit carries no hardening, by design. NoNewPrivileges would
  strip avb-netraw's file capability, and in a user unit the
  mount-namespace options imply PrivateUsers, which keeps the
  capability from acting on the host's network.
- `triib --mcp` is on by default. It asks the user the first time an
  agent name connects, but the name is whatever the client reports, and
  once allowed, no change asks again.

Failover:

- triib reads one interface and keys entities by entity ID alone. When
  an entity advertises on two interfaces, its MAC flips and
  EntityChanged fires on each alternation.
- `StreamDescriptor` parses no redundancy fields, so neither triib nor
  atdecc knows about redundant pairs. The entity role models one
  AVB_INTERFACE.
- An entity that goes offline is dropped. On its return triib reads it
  again, helped by its model cache. Connections are not restored unless
  the device kept them, and presets are only recalled by hand.
- triib shows a grandmaster change in the Grandmaster column and the
  inspector's counter, but sends no notice.
- avb-endpointd:
  - holds the last grandmaster for 5 s after gptpd goes silent, and
    starts gptpd after 8 s;
  - only counts grandmaster changes;
  - re-advertises after a link bounce;
  - restarts on failure, and re-applies saved listener bindings when it
    starts;
  - exits with 68, which systemd doesn't restart, if its interface is
    missing at start.

  One entity cannot span two interfaces.
- The bench is two gPTP networks, and nothing joins them for AVB:
  - the MOTU switch's, which this desktop is on, with the MOTU as
    grandmaster at priority1 246;
  - scrambox's, with the amp, the PreSonus and the Mac mini, and the
    PreSonus as grandmaster.

  scrambox's sixth port is its uplink to the house network, which drops
  VLAN 2 and PTP. scrambox runs a single bridge, with no VLAN
  enforcement and no fault injection. Its gptpd relay is built not
  grandmaster-capable (priority1 255) and has no option to change that.
  On this desktop, only the I210 has a hardware clock.

## Threat model

What this guards against, most common first:

1. **Another controller changing an entity mid-operation.** This is
   usually an accident, such as Hive and triib on the same network.
   Entity locks answer it.
2. **A host on the AVB network sending wrong or forged frames.** The
   aim is that no frame crashes triib or the daemon, grows memory
   without bound, or silently redirects commands. A forgery should
   surface as a warning rather than a change. Examples:
   - advertisements for an entity it doesn't own;
   - departures;
   - floods of fresh entity IDs;
   - malformed PDUs.
3. **Other local users and processes on the computer:**
   - squatting avb-endpointd's sockets;
   - an agent misnaming itself to `triib --mcp`;
   - files another user can write.

Out of scope: an attacker on the network who copies an entity's MAC
and entity ID exactly. Only cryptography stops that (1722.1
authorization, MACsec), which Milan devices don't support. See "Not
planned".

## Security work

### S1. Entity locks in the controller (atdecc, triib)

- **atdecc controller:**
  - Lock and unlock requests, sent as LOCK_ENTITY with and without the
    UNLOCK flag. An UNLOCK sent with no lock held queries the lock
    state, as Milan suggests.
  - Per-entity lock state (holder, expiry), taken from responses,
    overheard responses and unsolicited notifications.
  - A held lock is renewed before its 60 s run out, and released when
    the controller stops.
  - A `Refusal::Locked { holder }` read from the ENTITY_LOCKED payload,
    and a `Refusal::Acquired`.
  - ACQUIRE_ENTITY only for an entity whose ADP shows persistent
    acquire, and not in the first release. Milan entities refuse it.
- **triib:**
  - Wraps multi-command changes in a lock: preset recall, a format
    change with its rate and clock source, channel mapping edits, and
    (with F3) both sides of a redundant pair.
  - Shows a lock badge in the entity table and inspector, with the
    holder named from the controllers it has heard.
  - Gets a "Keep locked" switch in the inspector, which renews every
    30 s and releases when triib quits.
  - When an entity is locked by another controller, refuses with the
    holder's name rather than a status code.
  - Adds `lock` and `unlock` tools to `triib --mcp`.
- **atdecc entity,** Milan conformance fixes that break nothing, so
  they can ship as atdecc 0.2.x ahead of 0.3.0:
  - Release the lock when its holder departs or its ADP times out.
  - Answer ACQUIRE_ENTITY with NOT_SUPPORTED (5.4.2.1).
  - Answer LOCK_ENTITY on any descriptor but ENTITY with NOT_SUPPORTED
    (5.4.2.2). Today the descriptor type is ignored.
  - When a lock times out, send registered controllers an unsolicited
    notification (5.4.2.2's note). Today it clears silently.
  - Gate ACMP with the lock: while locked, BIND_RX and UNBIND_RX from
    another controller get CONTROLLER_NOT_AUTHORIZED (5.5.3.6.3,
    Tables 5.32 and 5.38).

### S2. Protecting host entities (atdecc entity, avb-endpointd, triib)

- **Allowlist:** in avb-endpointd's config, per entity, the default
  entity included: `controllers = [entity IDs]`. When the list is set,
  only those controllers may change the entity, by AEM sets or ACMP
  binds. Others get what a Milan entity locked by another controller
  answers, which controllers already handle:
  - ENTITY_LOCKED for AEM;
  - CONTROLLER_NOT_AUTHORIZED for the listener's BIND_RX and UNBIND_RX.
    A Milan talker takes no controller ACMP.
  - ENTITY_LOCKED for LOCK_ENTITY too, the UNLOCK query included, with
    locked_id set to the entity's own ID. triib then shows "protected
    by its allowlist" when the holder is the entity itself. Listed
    controllers lock normally.

  NOT_AUTHENTICATED isn't used, as it points controllers at 1722.1
  authentication, which is left out (decision 1). Reads stay open. The
  daemon's own paths bypass the hook: its pool listeners are bound
  through the entity directly, and saved bindings are restored at
  start. The list is empty by default, as Milan entities have no list
  (decision 2). A `read_only = true` refuses every change. atdecc's
  entity role gets the hook, and the daemon passes the list in.
- **Limits:** a controller's entity ID is a header field anyone can
  write. So the allowlist guards against mistakes, not attackers, and
  the docs should say so. Pairing the ID with the MAC the controller
  advertises from narrows forgery to someone who also copies the MAC.
- **triib:** an inspector setting for host entities, "Only these
  controllers may change it", with triib itself and the controllers
  heard on the network to pick from.

### S3. Hardening atdecc against forged and malformed frames

- **Controller:**
  - Bind each entity, per `interface_index`, to the MAC it was
    enumerated from. Another MAC advertising the same entity ID and
    interface raises a new `Event::Conflict` rather than moving the
    entity. The entity moves only once the old MAC has stayed silent
    past its valid time, as when a card is replaced. triib shows the
    conflict as a warning on the entity.
  - Accept ENTITY_DEPARTING only from the entity's MAC.
  - Accept AECP responses only from the MAC the command went to, and
    overheard responses only from the target entity's MAC.
  - Update bindings from ACMP responses only when the source is the
    responding entity: the listener for RX messages, the talker for TX.
- **Bounds,** all set in `Config`:
  - a maximum number of entities (1,024 by default), evicting offline
    entities first, then the least recently heard;
  - a cap on each entity's command queue;
  - at most 8 enumerations at once, so a flood of new IDs can't take
    the controller's bandwidth or CPU;
  - a size limit on the model cache.
- **Entity:** bound the `known` map, and key unsolicited registrations
  by controller and MAC.
- **Fuzzing:**
  - cargo-fuzz targets for `decode` on each PDU, and for sequences
    through `Controller` and `Entity`;
  - proptest round trips for every encoder;
  - a corpus seeded from `tests/captures`;
  - a scheduled CI job that fuzzes main for a fixed time, so pushes
    aren't slowed.

### S4. The local surface (avb-endpointd, triib)

- **avb-endpointd:**
  - Set the MRP sharing socket to 0600, and check its peer with
    SO_PEERCRED, as defense in depth. The same goes for the app ring's
    socket, `$XDG_RUNTIME_DIR/avb-endpointd/<if>.app`, when it ships.
  - When an interface claim is held by another user, report who holds
    it: connect to the abstract name, read SO_PEERCRED (the uid and pid
    set at listen()), and close. Squatting an abstract name can't be
    stopped short of a privileged helper, so it stays a residual risk.
  - Unit options: only UMask=0077 and LimitCORE=0 are safe today. In a
    user unit, ProtectSystem, PrivateTmp, ProtectHome and the other
    mount-namespace options imply PrivateUsers=yes, and inside a user
    namespace avb-netraw's capability can't act on the host's network,
    just as NoNewPrivileges strips it.
  - Real sandboxing needs the privileged half split out: avb-netraw as
    a socket-activated system service that hands sockets over
    SCM_RIGHTS after an SO_PEERCRED check, say against an `avb` group.
    The user daemon could then take NoNewPrivileges and the namespace
    options. raw-ether's broker already passes descriptors, so it's a
    transport change in raw-ether and the packaging.
  - Card launch time weighs on this. SO_TXTIME on CLOCK_TAI needs
    CAP_NET_ADMIN, which would put cap_net_raw and cap_net_admin on a
    binary any local user can run. Decision 9 takes the split instead:
    only the avb-netraw service holds the two capabilities, and
    avb-endpointd turns card launch time on once it runs that way.
- **triib MCP** (defaults in decision 3):
  - tie an agent's approval to its process, through SO_PEERCRED on the
    control socket and the program's path, not only its reported name;
  - add an optional "Ask before each change" for disconnect, format,
    rate and clock source;
  - log every agent action with the agent's name in the log view;
  - lock entities during an agent's multi-step change.

### S5. Visibility

- Name the controller behind each change triib overhears, taking its
  name from the overheard response's controller ID. For example: "Hive
  set the format of Stage Box input 2".
- Warn on lock contention, MAC conflicts, duplicate entity IDs and
  departures that didn't come from the entity.

### Not planned

These would also mean replacing the bench's switches:

- **1722.1 authorization** (AUTH_ADD_TOKEN, AUTHENTICATE): its token
  travels in clear text, and no Milan device implements it.
- **Transport security, stream encryption (AEF) and MACsec:** a Milan
  network can't carry them, as its devices and bridges don't speak
  them. They would also add encryption to the stream path's real-time
  deadlines.

All of them can be revisited if a closed installation needs them.
Decision 1 asks about authorization.

## Failover work

### F1. Grandmaster changes and gPTP loss (avb-stream, avb-endpointd, gptpd, triib)

Milan Annex B.1:
- A media clock holds over at least 5 s through a grandmaster change.
- Talkers set the tu bit for 0.25 s.
- A stream stable for 60 s doesn't unlock if the network agrees on the
  new grandmaster within 5 s.
- Bridges pass the change on within 50 ms.

Where the host entities stand (no figures yet):
- avb-endpointd's 5 s hold isn't media holdover. It only delays what
  the entity reports (GET_AVB_INFO's grandmaster and AS path) while
  gptpd names none or goes silent, so controllers don't see X, then
  none, then Y. A longer hold would only hide a real loss.
- Holdover is avb-stream's MediaClock. With no grandmaster's time, the
  last line carries on, unlocked. A new line that doesn't continue the
  old one waits for 3 records in a row (about 375 ms at gptpd's
  125 ms), then bumps the generation. So a grandmaster change that
  steps gPTP time restarts the talker's timeline with mr toggled.
- AAF and AM824 never set tu; only CRF has the field.
- So B.1's regaining without loss likely fails today.

What to do:
- In avb-stream: on a generation change caused by a grandmaster
  change, keep the sample count running, re-express presentation times
  on the new line, set tu for 250 ms, and keep mr for real restarts.
- Measure the host talkers and listeners against each of B.1's figures,
  with forced grandmaster changes (T2). T2 logs gptpd's record state
  every 125 ms across the change, since the span with no grandmaster
  (the announce receipt timeout, 3 s by default) decides holdover.
- gptpd passes a grandmaster change on as soon as the Announce arrives.
  T2 measures a bridge's 50 ms rather than assuming it. A bridge option,
  `priority1 N`, would make the relay grandmaster-capable. It should
  be small, as the relay already originates time when an end station
  on the bridge is grandmaster.
- triib:
  - a log notice and a short-lived badge when a domain's grandmaster
    changes;
  - a per-domain count of changes in the network view;
  - entities that disagree on the grandmaster for more than 5 s marked
    in the network view.

### F2. Controller recovery (triib, atdecc)

- **Keep connections:** an optional setting, off by default. triib
  remembers the connections it made, and puts back any binding a
  returning listener has lost. Milan listeners keep their own, so this
  matters for plain 1722.1 devices. triib does it under a lock, and
  logs each reconnection.
- **Link loss on triib's own interface:** keep the model cache and
  matrix, show "Link down" in place of emptying the views, and pick up
  again when the link returns. Check this against today's behaviour
  before changing it.
- A lock held by a triib that crashes expires after 60 s by protocol.
  With S1's lock state, the next triib sees and takes over its own
  stale lock.

### F3. Milan seamless network redundancy, controller side (atdecc, triib)

Under Milan chapter 8, a redundant entity advertises the same entity ID
and model on its Primary (AVB_INTERFACE 0) and Secondary
(AVB_INTERFACE 1) interfaces. Each interface has its own MAC, gPTP, MSRP
and MAAP, and the streams come in primary and secondary pairs.

- **atdecc controller:**
  - Record an entity's presence per `interface_index`: MAC,
    available_index, grandmaster and last heard. Report an interface
    coming and going as its own events, and set an entity offline only
    when every interface has gone.
  - Send AECP to the MAC on the network the controller is on.
  - Parse the Annex C fields (`redundant_offset`,
    `number_of_redundant_streams`, `redundant_streams`) and GET_MILAN_INFO's
    REDUNDANCY bit.
  - Expose redundant pairs on the model.
  - Add a helper that binds or unbinds both sides of a pair: primary
    listener to primary talker, secondary to secondary.
  - Register for unsolicited notifications on each interface the
    controller reaches, since Milan registers per interface.
- **triib:**
  - In the matrix, a redundant pair is one row or column, marked R,
    with a dot for each path: connected, streaming or interrupted.
    Connecting a pair connects both paths. A pair connected on one path
    only shows as a warning.
  - The inspector shows both paths' counters, and the entity table both
    MACs.
  - A lost path is logged ("Secondary path to Stage Box lost") and
    marked degraded, not failed.
- **triib on two interfaces at once,** as a second step: one network
  thread per interface, with an entity heard on both merged into one.
  Left for later (decision 4). A controller on one network can already
  bind both sides of a pair. Being on both shows each network's own
  grandmaster and advertisements.

### F4. Redundant host entities (atdecc entity, avb-endpointd, avb-stream, avb-mrp, gptpd, triib)

Milan 8.3's requirements, by crate:

- **atdecc entity:**
  - two AVB_INTERFACE descriptors, each with its own ADP state machine,
    advertising the same entity ID and model with `interface_index`;
  - redundant STREAM_INPUT/OUTPUT descriptors with the Annex C fields;
  - SET_STREAM_FORMAT and SET_MAX_TRANSIT_TIME applied to both sides of
    a pair, or to neither;
  - audio mappings mirrored onto the other side of the pair;
  - paired clock sources;
  - unsolicited registrations per interface;
  - ACMP responses on the interface the command came in on, and Milan's
    CONNECT_TX rule (8.3.2.5);
  - the REDUNDANCY bit in GET_MILAN_INFO.
- **avb-endpointd,** whose design is endpointd's call. It chose one
  daemon owning both interfaces: only one process can send the same
  samples sample-exact on both paths, from one PipeWire device or app
  ring, and Milan's both-sides-or-neither rule for formats and mappings
  is one AEM state.
  - `avb-endpointd@<primary>` with `secondary = "<interface>"` in the
    config. The daemon claims both interfaces, and the entity ID comes
    from the primary's MAC.
  - Per interface: gPTP status and a MediaClock, an MRP participant and
    sharing socket, MAAP, raw sockets and `avb-link@`, with nothing
    bridged between them.
  - Secondary stream IDs distinct from the primary's.
  - Refusing to start if a Linux bridge joins the two interfaces, and
    refusing redundancy, with a warning, if either interface hears the
    entity's own ADP or MAAP from the other, meaning both are on one
    network.
  - Card launch time works per port, as each has its own PHC and ETF
    queue.
- **avb-stream,** with one MediaClock per interface, each mapping its
  gPTP time to CLOCK_MONOTONIC:
  - A redundant talker picks each frame's presentation instant on the
    monotonic clock, and stamps each copy through its own interface's
    line.
  - A redundant listener maps each path's times back through its own
    line and merges on that one timeline. It plays the primary and
    fills any gap from the secondary, sample for sample, recovers its
    media clock from whichever stream is active (8.3.2.1.3), and counts
    each path.
- **gptpd:** two end-station instances on one host. They are already
  separate units per interface, so this is a check, not new work.
- **triib:** a host entity's setup offers "Redundant, primary X,
  secondary Y" when the computer has two interfaces with hardware time
  stamping. Its matrix and inspector use F3's views.
- **Hardware:** this desktop has one such interface, the I210. Testing
  F4 needs a new card with two ports and hardware time stamping
  (decision 5), not here yet.

### F5. The daemon and its interface (avb-endpointd)

- When the interface disappears while the daemon runs (a USB adapter
  unplugged, or a driver reset), keep running and reopen it when
  netlink reports it back. Today it exits, or retries a socket that's
  gone. triib shows "Waiting for interface".
- Wait the same way for an interface missing at start, such as a USB
  adapter that comes up late, in place of exiting with 68. Like the
  exit code, an in-process wait avoids restart loops.
- Measure what listeners hear when the daemon is killed mid-stream, and
  when the I210's driver resets, which bounces the link and steps the
  PHC (T5).

## Crate changes and releases

| Crate | Repository | Changes | Version |
|---|---|---|---|
| atdecc | avb | S1's entity conformance fixes; then `#[non_exhaustive]` on public enums and structs, with constructors or Default first for any struct triib or plaa build with literals (Config included); S1 lock requests and state; S2 hook; S3 checks, bounds and fuzzing; F3 per-interface presence and Annex C; F4 entity with two interfaces | 0.2.x for the entity fixes, with 0.2.1's DISCONNECT_TX fix; 0.3.0 for the breaking pass with S1 to S3; 0.3.x for F3 and F4 once additions no longer break |
| avb-lite | avb | atdecc 0.3 | minor |
| avb-net | avb | card launch time, on raw-ether 0.2, which only adds the launch kind; atdecc and avb-mrp depend on avb-net and re-export its MacAddress, and avb-lite sits on it through them, so it stays 0.1.x until atdecc 0.3 | 0.1.6 |
| avb-stream | avb | the app ring and card launch time; then atdecc 0.3, whose StreamFormat its API carries (formats, Media/Aaf/Am824/Crf::of, TalkerConfig.format); F1 holdover and tu; F4 redundant talker and listener | 0.4.0 now; 0.5.0 with atdecc 0.3 |
| avb-mrp | avb (bridge module: Linux-AVB-Bridge) | two independent instances on one host, F4, if anything is needed; no atdecc dependency | patch |
| avb-endpointd | Linux-AVB-Endpoint | avb-net 0.1.6 and avb-stream 0.4; S1 lock release; S2 allowlist; S4 sockets, claim holder report, unit options and avb-netraw as a system service; F4 redundant daemon; F5 interface return and waiting at start | breaking with atdecc 0.3, as its API carries atdecc::EntityId; again for F4 |
| gptpd, raw-ether | gptp (decision 7) | raw-ether's broker transport for the avb-netraw service (S4); gptpd's F1 check; a `priority1 N` bridge option making the relay grandmaster-capable, for the secondary network's grandmaster or T2 on scrambox's network | patch |
| Linux-AVB-Bridge | Linux-AVB-Bridge | T3 `avb-fault`; T4 two-bridge mode, an estimated 2 to 4 days; lanmind reading both instances | its own |
| triib | triib | S1, S2 and S4 to S5 UI; packaging for the avb-netraw service; F1 to F3 UI; F4 setup | 1.4.0 for S, 1.5.0 for F3 and F4 |
| plaa | plaa | moves to atdecc 0.3 with the others | its own |

How the updates move:

1. **Needs go to each project.** triib's needs go to the avb,
   Linux-AVB-Endpoint, gptp and bridge projects, each with an API
   sketch and the test that shows it, never as edits to their
   repositories. Each project designs and builds its own.
2. **What ships first** (M0 below): atdecc 0.2.1 with S1's entity
   fixes; then avb-net 0.1.6 and avb-stream 0.4.0, carrying the app
   ring and card launch time; then avb-endpointd on those. avb-stream
   doesn't wait for atdecc 0.3, which breaks it anyway, so waiting would
   save nobody a move. avb-net stays 0.1.x: atdecc and avb-mrp
   re-export its MacAddress, so an avb-net 0.2.0 beside atdecc 0.2
   would leave triib, plaa and avb-endpointd with two MacAddress types.
   triib and plaa each checked that avb-stream 0.4.0 needs only a
   version bump.
3. **One breaking release.** The `#[non_exhaustive]` pass and S1 to S3
   ship together as atdecc 0.3.0, so the API breaks once.
   - avb-stream 0.5, avb-lite and avb-endpointd follow, then triib and
     plaa move on the same day.
   - triib uses avb-endpointd's config and status types, so they must
     agree on one atdecc.
   - plaa hears the release plan before the API pass starts.
4. **Then additions.** F3 and F4 follow as 0.3.x additions, then avb-stream
   and avb-endpointd, then triib.
5. **Releases.** Each project tags and publishes its releases after
   CI passes. triib moves to a release in a commit of its own.
6. **Bookings.**
   - Builds on the bench desktop are scheduled, with every step
     listed, namespace runs included, and kept out of measured runs.
     They run uncapped at -j 2, idle class, pinned to two CPUs, one at a
     time.
   - Bench tests are scheduled the same way, with everyone using the
     bench.

## Tests

| Test | Covers | Where | Bench impact |
|---|---|---|---|
| T1 | S1 to S3: locks, allowlist, forged frames, floods, malformed PDUs | A controller and an entity in two network namespaces joined by a veth pair; lock contention against Hive on the bench | None in the namespaces; locks on the bench only on entities the test owns |
| T2 | F1: grandmaster change and loss | Each network in turn. The MOTU's: this desktop's gptpd set below priority1 246 live, through gptp::status's set_priority1, then back. scrambox's: the PreSonus's port taken down with `avb-fault`, for loss without a power cut; an end station or the relay option for a change | The network under test, scheduled with everyone on the bench |
| T3 | F2, F5: a link lost and restored, or gPTP lost on a live link | A scrambox port taken down for N seconds and brought back, or only its gPTP dropped; the I210 driver reset; avb-endpointd killed | The devices behind that port |
| T4 | F3, F4: seamless redundancy | Two independent networks (below) | The bench is split for the test |
| T5 | F5: daemon and interface | The desktop: the daemon killed, the interface removed and returned | The desktop's entities |

The PreSonus can't be powered off alone: the Shelly plug that feeds it
also feeds scrambox and the MOTU.

Forged frames never go onto the shared bench, because they would reach
its other controllers and entities. They stay
in network namespaces, or, when real hardware is needed, on scrambox's
looped port pair with both ports out of br0. That makes it a private
cable between two NICs. `avb-loop` takes only one port out, and frames
from it would enter br0 through the other and reach the whole network.
Both ports were down on 2026-11-12, so the cable needs checking first.
Using the pair is a scheduled change on scrambox.

Results are taken from:
- a tap on the bench: talker gaps and listener starves;
- a real-time probe on the host;
- the listeners' counters;
- `tools/avb_audio_analyzer.py` on a known signal. Seamless means not
  one sample is lost when a path fails.

### scrambox's test configuration

Today scrambox runs one bridge, br0. It has:
- five i226-V AVB ports: one to the amp, one to the PreSonus (the
  grandmaster), one to the Mac mini, and two cabled to each other as a
  loop;
- a sixth port, the uplink to the house network, which stays with br0
  in any split;
- a Wi-Fi access point.

It can't yet act as two networks:
- avb-bridged flushes its whole nft reservation set when it starts
  (`flush_set` in `crates/avb-bridged/src/fqtss.rs`), so a second
  instance would drop the first's streams;
- the units are written for br0 only;
- gptpd's relay is never grandmaster-capable and has no priority
  option.

For T3, a small scrambox addition, `avb-fault`:
- **Port down:** takes a named port down for N seconds, N capped at
  300.
- **Recovery:** a transient systemd timer (`systemd-run
  --on-active=N`), started before the port goes down, brings it back up
  even if the ssh session or the script dies. On igc a port takes about
  3 s to link, then to become asCapable.
- **Refusals:** the uplink, br0's management path and the Wi-Fi port.
  It allows one fault at a time.
- **gPTP-only mode:** drops only gPTP on a named port for N seconds,
  with an nft rule and the same timer. That loses gPTP without losing
  the link, which F1 needs and a port down can't separate.
- **Restarts on demand:** named with their cost: avb-bridged (its
  reservations re-made, under 1 s) or gptpd (about 3 s without Sync on
  every port).
- **Log:** the times it acted, and the times gptpd saw asCapable go and
  return, all in UTC.

For T4, scrambox runs two bridges (decision 6):
- **Ports:** br0, the primary, and br1, the secondary, with two or more
  ports each. A port-to-bridge mapping in bridge.conf (for example
  `port <interface> avb br1`) says which port goes where.
- **Reservations:** one nft set keyed by port name serves both
  instances, if each avb-bridged replaces only its own ports' elements
  when it starts. That is a smaller change than a set per instance,
  keeps one rule, and suits lanmind's single forward chain.
- **br1's identity:** its own MAC, which gives its gptpd its own clock
  identity, and its own LocalClock (`--clock`, one of its ports).
- **Daemons:** instance units for avb-bridged and gptpd, and networkd
  rendering for br1. The status paths (`--status`, `--streams` and
  `--gptp-status` on avb-bridged, `--status` on gptpd) already exist,
  so per-instance paths are configuration, not code. The Wi-Fi access
  point, its time publisher and bundler stay bound to br0's gptpd.
- **Grandmasters:** each network needs one. The primary has the
  PreSonus. The secondary needs a grandmaster-capable end station, or
  the relay's `priority1 N` option (F1).
- **Switching:** two configurations for scrambox, normal and split,
  selected by one command. Each switch restarts both gptpd instances
  and avb-bridged, about 3 s without Sync on every port with
  reservations re-made, so every switch is a bench-wide booking. Moving
  a port between bridges doesn't drop its link.
- **Size:** an estimated 2 to 4 days, mostly
  avb-bridge-setup's rendering and its tests, then a booked bench check
  of both modes. lanmind would also need to read both instances.

The alternative, not chosen, was the MOTU switch as the secondary
network, recabled by hand for each run.

Either way T4 needs two redundant endpoints, the talker and listener
sides of a pair:
- this desktop with a two-port card (F4), not here yet;
- a Milan device with redundancy, if the bench has one, or a second
  Linux host running avb-endpointd.

ESP32-P4 boards have one Ethernet port and the Mac can't be redundant,
so neither qualifies. Decision 5 covers the hardware.

The split configuration is scheduled each time, because the amp, the
Mac mini and the PreSonus sit behind scrambox. A command
returns scrambox to the normal bench, and its first run is checked.

## Order of work

0. **M0, what's in flight** (decision 8).
   - atdecc 0.2.1 (the DISCONNECT_TX fix) with S1's entity conformance
     fixes, as 0.2.x.
   - avb-net 0.1.6 and avb-stream 0.4.0: the app ring and card launch
     time.
   - avb-endpointd on those. triib and plaa move with a version bump.
1. **M1, locks and checks.**
   - S1, S3 and the `#[non_exhaustive]` pass, as atdecc 0.3.0.
   - triib's lock UI and conflict warnings.
   - The coordinated move of avb-lite, avb-stream, avb-endpointd,
     triib and plaa.
   - Tests: T1. Fuzzing runs from here on.
2. **M2, host entities and the local surface.** S2's allowlist, S4 and
   S5. Tests: T1 and T5.
3. **M3, grandmaster and link failover.** F1, F2 and F5, with scrambox's
   `avb-fault`. Tests: T2, T3 and T5.
4. **M4, redundancy in the controller.** F3, first proved against
   atdecc's entity role with two interfaces in namespaces, built early
   for that purpose. Then against a redundant Milan device, if there is
   one. Tests: T1-style namespaces, then T4.
5. **M5, redundant host entities.** F4, once the hardware and scrambox's
   split configuration are in place. Tests: T4.

## Decisions

Settled on 2026-11-12:

1. 1722.1 authorization (tokens and AUTHENTICATE): left out.
2. Host entities' allowlist default: open, as Milan entities are.
3. `triib --mcp` stays on by default, as it asks the first time each
   agent connects. "Ask before each change" is added, off by default.
4. triib on two interfaces at once (F3's second step): left for later.
   Binding both sides of a pair from one network covers the usual
   case.
5. Hardware: this desktop gets a new card with two ports and hardware
   time stamping, not here yet. M5 and T4 wait for it. T2, T3 and T5
   need only the one port.
6. T4's networks: two bridges on scrambox (option a).
7. gptpd: the gptp repository (gptp, gptpd and raw-ether) is
   maintained alongside the bridge. It builds the F1 check and the
   relay's `priority1 N` option.
8. atdecc 0.3.0's breaking pass starts after M0. avb-stream doesn't
   wait for it, as atdecc 0.3 breaks avb-stream anyway. The pass ships
   as one coordinated move with avb-stream 0.5, avb-lite,
   avb-endpointd, triib and plaa.
9. Card launch time's CAP_NET_ADMIN: avb-netraw becomes a
   socket-activated system service that hands sockets to permitted
   users, rather than a binary any local user can run carrying
   cap_net_raw and cap_net_admin. It also lets the user daemon be
   sandboxed.

Still open: the other redundant endpoint for T4, a Milan device with
redundancy or a second Linux host.
