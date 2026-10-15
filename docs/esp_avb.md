# esp_avb findings

Issues in esp_avb noticed while building triib, for whoever works on
esp_avb next. Each finding has the time it was found (Pacific time), what
was seen, where it is in the code and what the standard says.

Code references are to esp_avb 2.19.0 (commit `5e75bd3`). The devices
captured may run older or newer builds.

## Open

### 1. ACMP PDUs are sent in the long form Milan forbids (2026-10-03 22:52 PDT)

- **Seen:** a GET_RX_STATE_COMMAND in the short form (control_data_length
  44, 56 octets) from a controller got a GET_RX_STATE_RESPONSE in the 2021
  long form (84, 96 octets) from the wired endpoint, and the same from the
  Wi-Fi endpoint. Responses with an error status came back short. The
  macOS entity on the same network answers short.
- **Code:** `avb_send_acmp_command` (`atdecc.c:3078`) always sets
  control_data_length to 84, so commands the listener sends (PROBE_TX) are
  long too. `avb_send_acmp_response` (`atdecc.c:3120`) keeps the command's
  length on errors but switches to 84 on success.
- **Standard:** Milan 1.3, 5.5.2.2: "A Milan device shall send and accept
  this truncated PDU, and may accept the longer PDU."
- **Fix:** send 44 in commands and responses. Keep accepting 84.
- **Evidence:** `crates/atdecc/tests/captures/bench-adp-acmp.pcap`, frames
  9 and 10, and the other GET_*_STATE pairs.

### 2. ENTITY_DISCOVER is ignored (2026-10-03 22:52 PDT)

- **Seen:** after an ENTITY_DISCOVER for all entities at 22:52:52.257, the
  wired endpoint's next ENTITY_AVAILABLE came 4.32 s later, outside Milan's
  0 to 4 s window and exactly on its regular 5.01 s cadence. The macOS
  entity answered in 20 ms.
- **Code:** `avb_process_adp` (`atdecc.c:1224`) handles ENTITY_DEPARTING
  and ENTITY_AVAILABLE only; nothing handles ENTITY_DISCOVER.
- **Standard:** IEEE 1722.1-2021, 6.2.7 (rcvdDiscover) and Milan 1.3,
  5.6.3.5.4: on an ENTITY_DISCOVER for entity_id zero or ours, stop the
  advertise timer, wait a random 0 to 4 s, and advertise.
- **Impact:** a controller starting up waits for the next periodic
  advertisement (up to 5 s) instead of seeing the entity at once.
- **Evidence:** same capture, frames 2, 3, 34.

### 3. Advertisements have no random delay (2026-10-03 22:52 PDT)

- **Seen:** ENTITY_AVAILABLE every 4.99 to 5.08 s from both endpoints, in
  two captures.
- **Code:** `avb.c:1000` sends whenever more than
  `ADP_ENTITY_AVAIL_INTERVAL_MSEC` (5000, `avb.h:97`) have passed since
  the last one.
- **Standard:** Milan 1.3, 5.6.3: TMR_ADVERTISE (5 s) is followed by
  TMR_DELAY, a random 0 to 4 s, before each advertisement; after startup
  with link up the delay is 0 to 2 s. IEEE 1722.1-2021, 6.2.4.2.2
  (randomDeviceDelay) has the same intent.
- **Impact:** endpoints powered up together keep advertising in step,
  bunching ADP traffic.

### 4. No re-advertisement when the grandmaster changes (2026-10-03 23:11 PDT)

- **Seen:** from reading the code only, not on the wire.
- **Code:** the periodic send in `avb.c:1000` is the only trigger for
  ENTITY_AVAILABLE; nothing sends one when the grandmaster changes.
- **Standard:** Milan 1.3, 5.6.3.5.7: the GM_CHANGE event re-advertises
  (after the random delay), so controllers see the new
  gptp_grandmaster_id promptly.
- **Impact:** small with a 5 s cadence; the new grandmaster shows up within
  5 s.

### 5. Wi-Fi endpoints do not set CLASS_A_SUPPORTED (2026-10-03 23:11 PDT)

- **Seen:** the Wi-Fi endpoint's entity_capabilities are `0x0200c68a`:
  CLASS_B_SUPPORTED set, CLASS_A_SUPPORTED clear. The wired endpoint's are
  `0x0200c58a`, the other way round.
- **Code:** `avb.c:616`, on purpose: the wireless hop admits Class B only,
  per `profiles/avb_wireless.md` 4.
- **Standard:** Milan 1.3, 5.6.2 requires CLASS_A_SUPPORTED to be 1.
- **Note:** a deliberate profile choice, not a bug, but a strict Milan
  controller may flag these endpoints. triib should show them as AVB
  Wireless rather than as non-Milan. Worth stating in the wireless profile
  that it overrides Milan 5.6.2.

### 6. AVB Lite mode is invisible to controllers (2026-10-03 23:11 PDT)

- **Seen:** from reading the code. esp_avb tracks whether it has fallen
  back to AVB Lite (`state->avb_lite`, set in
  `avb_update_avb_lite_from_ptp`, `avb.c:157`) but exposes it only through
  the local status API (`esp_avb.h:157`), not over ATDECC.
- **Standard:** `profiles/avb_lite.md` 2.2 says a device "must clearly
  advertise its active operating mode to the controller" but defines no
  mechanism.
- **Fix:** see triib's plan (AVB Lite section): an AECP vendor unique
  query under the AVB Lite MA-S OUI, sub-protocol `0x003`, returning
  capable, active, fallback reason, PTP profile and domain, media VLAN and
  unicast fan-out, with an unsolicited notification on change. Implement
  in esp_avb, triib and the profile together.

### 7. Stale comment on the CVU protocol ID (2026-10-03 23:11 PDT)

- **Code:** `atdecc.h:1434` comments `protocol_id` as
  `00-11-22-33-00-00`; the value used is `CVU_PROTOCOL_ID`
  (`avb.h:140`), `8C 1F 64 36 C0 02`, as in `profiles/avb_lite.md` 6.
- **Fix:** update the comment.

### 8. The Wi-Fi endpoint's grandmaster keeps changing (2026-10-03 23:22 PDT)

- **Seen:** the Wi-Fi endpoint's gptp_grandmaster_id alternates between
  its own clock identity (`0xfc012cfffefdfe80`) and the network's grandmaster
  (`0x0001f2fffeff3b14`, the AVB switch), which every wired entity
  reports throughout. Own clock at 22:51:39 to 22:51:49, the network's
  from 22:51:54 and at 22:52:54, its own again at 23:22:43 to 23:22:53.
- **Impact:** while it advertises its own clock it is not in the AVB
  domain, so streams to or from it cannot work, and a controller shows it
  in a different clock domain.
- **Cause:** not known. Could be esp_ptp on the endpoint, the bridge's time
  relay or the Wi-Fi time sync; needs the endpoint's and the bridge's logs
  from a flip.
- **Evidence:** ADP from the endpoint in captures at those times; triib
  reports each flip as an entity change.

### 9. Vendor and model names point at the wrong string (2026-10-04 01:02 PDT)

- **Seen:** the ENTITY descriptor's vendor_name_string and
  model_name_string are both 0, so controllers show "Configuration" for
  vendor and model (STRINGS 0, string 0). The intended strings are there,
  "ACME" and "AVB Device Model 1" at STRINGS 1, strings 4 and 5. Seen on
  the wired and the Wi-Fi endpoint; tshark decodes the same values.
- **Code:** the READ_DESCRIPTOR handler for ENTITY (`atdecc.c:249` to
  `254`) sets `vendor_name_ref` to 12 and `model_name_ref` to 13 on a
  local copy of the descriptor, then copies `state->own_entity`, not the
  copy, into the response, so the change is lost.
- **Fix:** copy `descriptor` into `msg->descriptor_data`, or set the
  references in `state->own_entity` once at startup.

### 10. Wi-Fi endpoint streams claim Class A (2026-10-04 01:02 PDT)

- **Seen:** every STREAM_INPUT and STREAM_OUTPUT of the Wi-Fi endpoint has
  CLASS_A and CLASS_B in stream_flags, while its ADP entity_capabilities
  say Class B only (finding 5) and the wireless profile admits Class B
  only.
- **Code:** `atdecc.c:460` sets both flags on every stream regardless of
  the port's medium.
- **Fix:** on a Wi-Fi medium port set CLASS_B only, matching `avb.c:616`.

### 11. Registered controllers are never checked (2026-10-04 01:08 PDT)

- **Seen:** triib registered for unsolicited notifications with both
  endpoints (both answered SUCCESS) and then listened for 74 s; neither
  sent a CONTROLLER_AVAILABLE command in that time.
- **Code:** `avb_send_aecp_cmd_controller_available` (`atdecc.c:69`) is
  defined but never called, and the response handler
  (`avb_process_aecp_rsp_controller_available`, `atdecc.c:2780`) does
  nothing.
- **Standard:** Milan 1.3, 5.4.5.4: after each command from a registered
  controller the entity starts a 30 to 60 s monitor timer; when it
  expires it sends CONTROLLER_AVAILABLE (with a retry) and, if the
  controller does not answer, deregisters it and sends it an unsolicited
  DEREGISTER_UNSOLICITED_NOTIFICATION.
- **Impact:** controllers that go away without deregistering (a crash, a
  pulled cable, a laptop closed) stay registered, so the endpoint keeps
  sending them notifications and its table of registered controllers can
  fill up.

### 12. GET_AS_PATH answers with a made-up path (2026-10-04 14:16 PDT)

- **Seen:** found reading the code while checking what a controller can
  learn about the network's layout; not yet seen on the wire.
- **Code:** `avb_send_aecp_rsp_get_as_path` (`atdecc.c:2624`) builds the
  path from the grandmaster's identity, the selected source's identity when it
  differs, and the endpoint's own identity, instead of copying the
  pathSequence of the PathTrace TLV of the last Announce received. The
  command table in `atdecc.h:87` still marks the command unsupported.
- **Standard:** IEEE 1722.1-2021, 7.4.41.2: path_sequence is the
  pathSequence of the latest 802.1AS Announce PathTrace TLV, which holds
  every PTP Instance from the grandmaster to the sender (802.1AS-2020,
  10.3.9.23). Milan 1.3, 5.4.2.24 requires the command, and an
  unsolicited response when the path changes.
- **Impact:** with more than one bridge between the grandmaster and the
  endpoint, the bridges in between are missing, so a controller cannot
  place the endpoint in the network.
- **Seen since (2026-10-04 14:50 PDT):** the wired endpoint answered
  GET_AS_PATH on the bench with two identities, the switch (the grandmaster) and
  itself, which with one bridge happens to match.

### 13. GET_COUNTERS answers with stale bytes and a wrong length (2026-10-04 14:50 PDT)

- **Seen:** the wired endpoint's GET_COUNTERS response for its
  AVB_INTERFACE has control_data_length 156 in a 160 octet AECPDU, so it
  claims 8 octets more than it carries; a strict decoder drops it. Its
  counters_valid is 0x23 (LINK_UP, LINK_DOWN, GPTP_GM_CHANGED), but
  LINK_UP is 0 with the link up, and the quadlets after the valid ones
  hold what looks like MSRP state: stream IDs and the Wi-Fi endpoint's
  MAC address.
- **Code:** `avb_process_aecp_cmd_get_counters` (`atdecc.c:2688`) zeroes
  the response, then copies `sizeof(aecp_get_counters_rsp_s)` octets of
  the received message over it, so the counters block is whatever the
  receive buffer held after the 28 octet command. control_data_length is
  `sizeof(aecp_get_counters_rsp_s) - sizeof(atdecc_header_s)`, which
  counts target_entity_id. The counters themselves are never kept: the
  valid bits are set and the values left as copied.
- **Standard:** IEEE 1722.1-2021, 7.4.42.2: the counters_block holds the
  counters, and control_data_length counts from controller_entity_id on
  (148 here). Milan 1.3, 5.3.6.3: LINK_UP equals LINK_DOWN while the link
  is down and LINK_DOWN + 1 while it is up; GPTP_GM_CHANGED counts
  grandmaster changes since boot.
- **Impact:** controllers that check the length see no response; those
  that do not show wrong counters and leak internal state.
- **Fix:** copy only the command's 28 octets, fill the block from kept
  counters, and use `sizeof(aecp_get_counters_rsp_s) -
  sizeof(atdecc_header_s) - sizeof(unique_id_t)`.
- **Also seen (2026-10-05 16:58 PDT):** the CLOCK_DOMAIN and both
  STREAM_OUTPUT responses carry the same stale block as the interface's
  (0x3a710000 at position 4, an address and flags at 9 to 11). For the
  stream outputs counters_valid is 0x9b, marking positions 3, 4 and 7,
  so the stale quadlet at 4 shows as TIMESTAMP_UNCERTAIN; STREAM_START
  reads 0 on an output that is sending. The stream output case's comment
  that the values are "zero-initialized from memset above" is wrong for
  the same reason, and no stream output event is counted. The
  STREAM_INPUT responses are filled by `avb_get_stream_in_counters` and
  read cleanly.

### 14. Wi-Fi endpoint does not answer GET_AS_PATH or GET_COUNTERS (2026-10-04 14:50 PDT)

- **Seen:** on the bench the Wi-Fi endpoint answered GET_AVB_INFO
  (grandmaster its own identity, peer delay 0, asCapable clear) but sent
  nothing back for GET_AS_PATH or GET_COUNTERS on its AVB_INTERFACE 0,
  each sent twice in each of two runs. The wired endpoint, also
  reporting firmware 1.0.0, answers both, so the Wi-Fi one may run an
  older build.
- **Code:** in the build reviewed, both handlers answer interface 0, but
  `avb_process_aecp_cmd_get_as_path` (`atdecc.c:2672`) returns without a
  response for any other index, and `avb_process_aecp_cmd_get_counters`
  returns without one for descriptor types it does not handle.
- **Standard:** IEEE 1722.1-2021, 9.3.5.3.3: every command received is
  answered, one not implemented with a correctly sized response and
  NOT_IMPLEMENTED; other failures carry their own status, such as
  NO_SUCH_DESCRIPTOR.
- **Impact:** the controller waits out its timeout and a retry for each
  command, and cannot tell "not supported" from "lost".

### 15. SET_SAMPLING_RATE is not implemented (2026-10-04 19:00 PDT)

- **Seen:** the wired endpoint answered SET_SAMPLING_RATE for AUDIO_UNIT
  0 with NOT_IMPLEMENTED, asking for 96000 and for 48000, though its
  audio unit lists 44100, 48000, 88200, 96000 and 192000 as supported.
- **Code:** the AEM command switch (`atdecc.c:2891` to `2950`) has no
  case for SET_SAMPLING_RATE or GET_SAMPLING_RATE, so both fall through
  to the default (`atdecc.c:2952`) and answer NOT_IMPLEMENTED.
- **Standard:** Milan 1.3, 5.4.2.13 and 5.4.2.14: SET_SAMPLING_RATE and
  GET_SAMPLING_RATE are required for each Audio Unit of the current
  configuration.
- **Impact:** no controller can change the endpoint's sampling rate,
  though its AEM model says it supports five; Hive and triib offer the
  rates and get refused.

### 16. Unsolicited notifications are never sent (2026-10-04 19:00 PDT)

- **Seen:** with triib's app registered for notifications with the wired
  endpoint, triib's CLI (a different controller ID) renamed its entity,
  group and a stream input, and changed a stream input's format; the
  endpoint answered SUCCESS each time and sent no notification to the
  app. The Mac mini, given the same commands, notified each of its four
  registered controllers every time.
- **Code:** `avb_send_aecp_unsol_get_stream_info` (`atdecc.c:215`) and
  `avb_send_aecp_unsol_get_counters` (`atdecc.c:234`) are stubs marked
  "not implemented"; the SET_NAME (`atdecc.c:1440`), SET_STREAM_FORMAT
  (`atdecc.c:2222`) and SET_CLOCK_SOURCE (`atdecc.c:2396`) handlers send
  only the response. Registration is one flag for every controller
  (`unsol_notif_enabled`, `avb.h:918`, set at `atdecc.c:2004`), not a
  table of registered controllers with their addresses and sequence IDs.
- **Standard:** Milan 1.3, 5.4.5.1 and 5.4.5.3: after each successful
  command that changes the entity's state, an unsolicited notification
  goes to every registered controller except the requesting one, each
  with its own controller_entity_id, destination address and sequence
  ID; GET_STREAM_INFO notifications are also sent whenever a stream's
  ID, destination, latency, MSRP state or failure changes (Table 5.20).
- **Impact:** controllers other than the one making a change keep
  showing the old names, formats and clock source, and do not see
  streams start, stop or fail, until they read the entity again.

### 17. SET_CLOCK_SOURCE is refused while an output streams (2026-10-04 19:00 PDT)

- **Seen:** asking the wired endpoint to switch CLOCK_DOMAIN 0 from
  Internal Clock to its CRF input got STREAM_IS_RUNNING, its stream
  output 0 being bound by the Wi-Fi endpoint.
- **Code:** `avb_process_aecp_cmd_set_clock_source` (`atdecc.c:2428`)
  refuses any change while an output streams, until a glitchless switch
  exists, by its comment; it also refuses the CRF source with
  NOT_SUPPORTED until that input is connected and receiving.
- **Standard:** Milan 1.3, 5.4.2.15 requires SET_CLOCK_SOURCE with no
  such exception, and IEEE 1722.1-2021, 7.4.23 does not list
  STREAM_IS_RUNNING among its errors; Milan reserves that status for
  stream format changes on a bound input or streaming output (5.4.2.7).
- **Impact:** a controller cannot move a streaming endpoint to an
  external media clock without first tearing down its listeners, and
  gets a status that reads as a format problem.

### 18. GET_DYNAMIC_INFO is not implemented (2026-10-04 19:43 PDT)

- **Seen:** reading the wired endpoint with its entity model cached,
  triib asked for its names, stream formats, sampling rate and clock
  source with one GET_DYNAMIC_INFO; the endpoint answered
  NOT_IMPLEMENTED, so triib read those descriptors one by one instead.
- **Code:** the AEM command switch (`atdecc.c:2891` to `2950`) has no
  case for GET_DYNAMIC_INFO, which falls through to the default
  (`atdecc.c:2952`).
- **Standard:** Milan 1.3, 5.4.2.29 requires GET_DYNAMIC_INFO as IEEE
  1722.1-2021, 7.4.76 specifies it: each packed GET answered as if sent
  alone, up to what fits in one response.
- **Impact:** controllers that cache entity models cannot fetch what
  changes in one exchange; each reconnect costs a command per named
  descriptor, which over Wi-Fi, where frames are lost, is slow.

### 19. Stream input mappings are fixed (2026-10-05 16:14 PDT)

- **Seen:** `triib-cli maps` on the wired endpoint found an AUDIO_MAP
  descriptor on STREAM_PORT_INPUT 0 as on the output, fixing stream 0
  channel 0 to cluster 0 channel 0, so there is nothing for GET_AUDIO_MAP,
  ADD_AUDIO_MAPPINGS or REMOVE_AUDIO_MAPPINGS to work on.
- **Code:** `avb_send_aecp_rsp_read_descr_stream_port` (`atdecc.c:763`)
  gives both ports `number_of_maps` 1 (`atdecc.c:772`); the AEM command
  switch has no case for the three mapping commands, which fall through
  to the default (`atdecc.c:2952`).
- **Standard:** Milan 1.3, 5.3.3.9 forbids AUDIO_MAP descriptors on a
  stream port input, as a PAAD-AE implements dynamic mappings on all of
  them, and 5.4.2.26 to 5.4.2.28 require the three commands on every
  stream port input, with mappings kept across power cycles (5.3.10.1).
- **Impact:** a controller cannot route a received stream's channels,
  which matters once an endpoint has more than one channel or stream;
  with one mono stream and cluster the fixed mapping is the only one
  possible, so nothing is lost today.

## Fixed

None yet.
