# Glossary

How triib's text carries the words of AVB, Milan and IEEE 1722.1 (ATDECC)
into other languages. The English source is `i18n/en/triib.ftl`. Each
language's term choices are in `docs/glossary/<language>.md`, so they can
be checked and changed in one place.

The readers are people who set up and run audio networks: live sound,
installation, broadcast. They know the standards' words in English. A
translation should read as natural to them as a good manual in their own
language. It should never be clever, and never replace a term of art with
a lay word that loses its meaning.

## Three kinds of words

### 1. Kept as written, in every language

These are names, not words. They stay in Latin script exactly as English
writes them, including their case:

- **Standards and profiles:** AVB, AVB Lite, AVB Wireless, Milan, ATDECC,
  IEEE 1722.1, IEEE 802.1Q, 802.1AS, PTP, gPTP, SRP, MSRP, MMRP, CVU SRP,
  and AVB Wireless's Mode A and Mode B.
- **Protocols and their parts:** ADP, AECP, ACMP, AEM, TLV,
  control_data_length, asCapable.
- **Other acronyms:** CRF (media clock streams), VLAN, MAC (address), CRC,
  SR class letters A and B, FTM and TM (Wi-Fi's timing measurements),
  BSSID.
- **Product names:** triib, Omarchy, Wi-Fi.
- **Units:** Hz, kHz, MHz, GHz, ns, µs, ms, kb/s, Mb/s, Gb/s, dBm.
- **Notations:** stream formats such as `48k 8ch` or `48k CRF`, interface
  names such as `enp6s0`, clock names such as `ptp0`.

triib's tests check that every one of these in an English message is still
there, unchanged, in its translation.

Some names the standards give reach the interface from the code, not from
the Fluent files, so they stay English in every language:

- descriptor types (Stream input, Clock source)
- capability flags (Audio source, Media clock sink)
- AEM and ACMP command and status names (Read descriptor, Not supported)

The log's one-line frame summaries also stay English, as packet analyzers
show them. `triib-cli` is English only.

### 2. Roles the standards name: talker and listener

A talker sends streams and a listener receives them. These are the roles
IEEE 802.1Q and 1722.1 define, and practitioners use the English words.

- **Languages written in Latin script:** keep **Talker** and **Listener**
  as loanwords. Follow the language's capitalization and grammar (German
  capitalizes them as nouns). Do not swap in a word for sender or
  receiver.
- **Japanese and Korean:** use the established transliterations トーカー /
  リスナー and 토커 / 리스너.
- **Chinese:** use 发送端 / 接收端 (zh-CN) and 發送端 / 接收端 (zh-TW). This
  is how Chinese TSN and AVB writing renders the roles.
- **Other scripts:** keep **Talker** and **Listener** in Latin script, the
  way these languages carry other networking terms.

**Grandmaster** is the clock all others follow (IEEE 1588, 802.1AS). Keep
it in Latin script, except in Japanese (グランドマスター) and Korean
(그랜드마스터). Never render it with a word built on "master" or "slave"
(主, Haupt-, maître, мастер). Where the language would otherwise reach for
such a word, keep the English term.

### 3. Terms of art, translated consistently

Each of these has one meaning in triib. Use the term that networked audio
and networking practitioners use in the language. If they use the English
word, use the English word. Once a term is chosen, use it everywhere,
including inside longer messages.

| English | Meaning in triib | Guidance |
|---|---|---|
| entity | A device or a piece of software on the network that ATDECC can find and control | The word technical standards use for "entity". Not "device": an entity can be software. |
| entity model | What an entity describes of itself, read with AEM | "entity" plus the language's word for a data model |
| controller | Software that controls entities, as triib does | The networking or control-system word, as for "controller" in a DAW or network manager |
| stream | A flow of audio or media clock samples from one talker | The word networked-audio manuals use for a media stream |
| stream input, stream output | A listener's or a talker's stream endpoint | "stream" plus input/output, matching the matrix's labels |
| connection, connect, bind | Setting a stream input to take a talker's stream | Ordinary words for connect; "bound" is the state of an input set to take a stream |
| media clock | The clock audio samples follow | The audio-engineering term (word clock family) |
| clock domain | A group of an entity's audio that shares one media clock | The audio-engineering term |
| clock source | Where a clock domain takes its clock from | The audio-engineering term |
| sampling rate | Samples per second of audio | The standard audio term |
| bridge | An AVB-capable network switch | The networking term for an IEEE 802.1 bridge, or "AVB switch" where practitioners say switch |
| reservation | Bandwidth the network sets aside for a stream (SRP) | The networking term for a resource reservation |
| egress | Traffic leaving an interface or a bridge port | The networking term; keep "egress" where practitioners do |
| link | The cable or connection between two ports | The networking term |
| peer delay | The time a frame takes over a link to the next device (PTP) | The PTP term; keep the English if practitioners do |
| offset | How far a clock is from the grandmaster | The PTP term |
| hop | One link a path crosses | The networking term |
| unicast, multicast | Sent to one receiver, or to many at once | The networking terms |
| fan-out | Copies of a stream sent one by one before switching to multicast | The networking term, or a short phrase |
| descriptor | A record in an entity model (AEM) | The technical word, as in "file descriptor" |
| cluster | A group of an entity's own audio channels (AEM) | Keep a word that reads as a technical grouping. Not "group", which triib uses for entity groups. |
| stream port | Where streams join clusters (AEM) | "stream" plus the language's word for port |
| channel mapping | Which audio channel goes to which stream channel | The audio-routing term |
| control | A setting an entity exposes, such as gain or mute | The audio word for a control |
| preset | A saved state to recall later | The word audio desks and software use |
| identify | Making a device show itself, such as by blinking | The verb device managers use |
| counter | A count an entity keeps of events | The networking word |
| locked, lost lock | A clock or receiver following, or no longer following, its reference | The audio and clock word for lock (PLL lock) |
| holding over | A clock keeping time on its own after losing its reference | The clock and telecom word for holdover |
| station | A Wi-Fi device associated with an access point | The Wi-Fi term (IEEE 802.11 station, client) |
| access point | The Wi-Fi device stations associate with | The Wi-Fi term, as on a router's settings page |
| beacon | The frame an access point sends a few times a second | The Wi-Fi term |
| signal | The strength of a Wi-Fi signal, in dBm | The Wi-Fi word, as in signal strength |
| interrupted | A stream that stopped unexpectedly | Ordinary word |
| timestamp | The time a frame carries for when to play it | The technical word |
| advertise, advertised | An entity announcing itself, or a talker offering a stream | The networking word for advertise or announce |
| interface | A network interface of the computer or an entity | The networking word |
| hardware clock | A network adapter's own clock for precise timing | The networking or PTP term |
| virtual (interface) | A software interface, such as a bridge or a tunnel | The networking term |

## Style

- **Case:** sentence case for labels and headings, as in English, unless
  the language's rules differ.
- **Punctuation:** follow the language: its full stop, quotation marks
  (« », „ ", 「」), and spacing before colons and percent signs where that
  is the norm.
- **Separators:**
  - `common-list-separator` and `common-thousands-separator` hold the
    language's list comma and thousands separator, written as `{", "}` or
    `{" "}`.
  - `common-percent` places the percent sign as the language does.
- **Plurals:**
  - Write every plural category the language has (CLDR): one, few, many,
    other in Russian; zero, one, two, few, many, other in Arabic; and so on.
    The `*` marks the default.
  - English's exact matches such as `[0]`, `[1]` and `[2]` ("twice") may
    be kept, changed or dropped, as the language reads best.
- **Variables:** keep every `{ $name }`, unchanged, and move it where the
  language needs it. Never translate a variable's name.
- **Length:** buttons, tabs, column headings and toolbar tooltips must
  stay short. The inspector's labels have about 130 pixels.
- **Comments:** lines starting with `#` explain the message beneath them.
  They are not translated.

## The website

triib.run is in every language the app is. The English pages in `site/`
are the source; `site/i18n/en.json` holds their text block by block, and
`site/i18n/LANG.json` each language's, keyed by the English's hash. A tag
with attributes stands as a short name such as `<a1>…</a1>`, and a
release number as `{v}`, so translations see only the words.

- After changing the English pages, run `scripts/site-i18n.py extract`
  to update `en.json`, then `scripts/site-i18n.py check` to list what
  each language now lacks: a changed sentence is a new key, and its old
  translation shows as no longer in the English. Remove those once
  translated again.
- Names of things in the app follow the app's own translation in
  `i18n/LANG/triib.ftl`, and terms this glossary's, so the tour says what
  the app shows.
- `en-GB.json` holds only the texts spelled differently, as the app's
  British English does; the rest come from the US English.
- `scripts/build-site.sh` writes English at the root and each language
  under its own folder, with a language menu and `hreflang` links, right
  to left where the language reads so. A reader who arrives at an English
  page from elsewhere, with a browser that prefers one of the languages,
  is taken to it.
