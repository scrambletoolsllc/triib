# triib's interface text in Finnish. Term choices: docs/glossary/fi.md.

## Language

language-name = Suomi

## Common

common-close = Sulje
common-more = Lisää
common-keep-toolbar-shown = Pidä työkalurivi näkyvissä
common-auto-hide-toolbar = Piilota työkalurivi automaattisesti

## Settings

settings-title = Asetukset
settings-general = Yleiset
settings-appearance = Ulkoasu
settings-language = Kieli
settings-language-system = Järjestelmän oletus: { $language }
settings-language-note = Tekstikenttiin kirjoitetaan järjestelmän syöttökielellä.
settings-appearance-system = Järjestelmä
settings-appearance-light = Vaalea
settings-appearance-dark = Tumma
settings-colors = Värit
settings-system-accent = Käytä järjestelmän korostusväriä
settings-accent-picked = Alla oleva väri määrää värit, joita triib käyttää.
settings-accent-omarchy = Omarchy-teemasta: { $theme }.
settings-accent-desktop = Työpöydän korostusväristä.
settings-accent-none = Työpöydällä ei ole korostusväriä, joten käytetään alla olevaa väriä.
settings-motion = Liike
settings-animations = Animaatiot
settings-animations-note = Jousto- ja liukuefektit asioiden muuttuessa.
settings-animations-reduced = Työpöytä pyytää vähentämään liikettä, joten triib pysyy paikallaan.
settings-outside-control = Ulkoinen ohjaus
settings-allow-outside-control = Salli ulkoinen ohjaus
settings-allow-outside-control-note = Agentit, kuten Claude Code, voivat nähdä ja muuttaa AVB-verkkoasi komennon triib --mcp kautta. triib kysyy ennen jokaista uutta agenttia.
settings-allowed-agents = Sallitut: { $agents }
settings-forget-agents = Unohda
settings-storage = Tallennus
settings-storage-presets = Esiasetusten kansio
settings-storage-entities = Entiteettitiedosto
settings-storage-apply = Käytä
settings-storage-choose = Valitse…
settings-storage-note = Vanhaan paikkaan jo tallennetut esiasetukset jäävät sinne; siirrä ne uuteen paikkaan, jos haluat käyttää niitä edelleen. triib-sovelluksen asetukset tallennetaan tiedostoon { $file }.
settings-storage-entities-note = Vielä puuttuva entiteettitiedosto alkaa käytössä olevan kopiona, ja triib-endpointd käynnistyy uudelleen sen kanssa.
dialog-choose-presets = Valitse esiasetusten kansio
dialog-choose-entities = Valitse entiteettitiedoston kansio
settings-full-path = Käytä koko polkua, esimerkiksi ~/Documents/triib.
settings-path-is-folder = { $path } on kansio, ei tiedosto.
settings-folder-missing = Kansiota { $path } ei ole. Luo se ensin tai valitse jokin muu.
settings-path-is-file = { $path } on tiedosto, ei kansio.
settings-cannot-write = triib ei voi kirjoittaa kansioon { $path }: { $error }.
app-file-dialog-failed = Tiedostoikkunan näyttäminen epäonnistui: { $error }
agent-prompt-title = Saako { $agent } ohjata triib-sovellusta?
agent-prompt-body = { $agent } pyytää käyttää triib-sovelluksen ulkoista ohjausta: nähdä verkkosi entiteetit ja muuttaa niitä, yhdistää striimejä ja asettaa muotoja, kelloja ja nimiä. Voit poistaa ulkoisen ohjauksen käytöstä Asetuksissa.
agent-prompt-allow = Salli
agent-prompt-deny = Älä salli

common-cancel = Peruuta
common-save = Tallenna
common-not-set = Ei asetettu
common-unnamed = Nimetön
common-none = Ei mitään
common-mac-address = MAC-osoite
common-list-separator = {", "}

## Network interfaces

interface-up = aktiivinen
interface-link-down = ei linkkiä
interface-wireless = langaton
interface-hardware-clock = laitteistokello
interface-hardware-clock-named = laitteistokello { $clock }
interface-virtual = virtuaalinen

## Toolbar

toolbar-choose-interface = Valitse liitäntä
toolbar-interface = Verkkoliitäntä
toolbar-filter-interfaces = Suodata liitännät
toolbar-hide-virtual = Piilota virtuaaliset liitännät
toolbar-hide-wireless = Piilota langattomat liitännät
toolbar-hide-unclocked = Piilota liitännät, joissa ei ole laitteistokelloa
toolbar-connections = Yhteydet
toolbar-network = Verkko
toolbar-entities = Entiteetit
toolbar-rediscover = Pyydä kaikkia entiteettejä mainostamaan itseään
toolbar-rescan = Tyhjennä ja skannaa kaikki entiteetit uudelleen
toolbar-search = Hae entiteettejä ja striimejä
toolbar-presets = Esiasetukset
toolbar-log = Loki
toolbar-inspector = Tarkastelu
toolbar-settings = Asetukset

## The network's state, in place of a view

state-no-interface = Ei liitäntää
state-no-interface-note = Valitse AVB-verkon liitäntä, jotta entiteettejä voidaan etsiä.
state-starting = Käynnistetään
state-starting-note = Avataan liitäntää { $interface }.
state-listening = Kuunnellaan
state-listening-note = Liitännän { $interface } entiteetit näkyvät tässä sitä mukaa kuin ne mainostavat itseään.
state-permission-needed = Tarvitaan käyttöoikeus
state-npcap-needed = Tarvitaan Npcap
state-get-npcap = Hanki Npcap
state-copy-command = Kopioi komento
state-cannot-use = Liitäntää { $interface } ei voi käyttää
state-try-again = Yritä uudelleen

## Entity list

entities-none-yet = Ei vielä entiteettejä
entities-none-yet-note = Kaikki verkon entiteetit rooleineen, SR-luokkineen ja kelloineen.

## Inspector

inspector-title = Tarkastelu
inspector-entity = Entiteetti
inspector-streams = Striimit
inspector-controls = Säätimet
inspector-diagnostics = Diagnostiikka
inspector-descriptors = Kuvaajat
inspector-select = Valitse entiteetti nähdäksesi sen tiedot.
inspector-offline = Entiteetti { $entity } ei ole verkossa.
inspector-heard = Se ei ilmoita itsestään: triib kuuli sen lähettämistä komennoista ja pitää sen luettelossa, kunnes 5 minuuttia kuluu ilman komentoa.
inspector-rename = Nimeä uudelleen
inspector-name = Nimi
inspector-identify = Tunnista
inspector-model-not-read = Sen entiteettimallia ei ole luettu.
inspector-no-streams = Ei striimejä.
inspector-no-controls = Ei näytettäviä säätimiä.
inspector-no-diagnostics = Liitäntöjä tai laskureita ei raportoitu.
inspector-reading = Luetaan kuvaajia, { $count } tähän mennessä.
inspector-read-failed = Entiteettimallia ei voitu lukea: { $reason }.

entity-section = Entiteetti
entity-name = Nimi
entity-triib-controller = triib-ohjain
entity-triib-on = triib laitteessa { $host }
entity-triib-here = triib tässä tietokoneessa
entity-controller-on = Ohjain laitteessa { $host }
entity-controller-here = Ohjain tässä tietokoneessa
entity-controller-at = Ohjain osoitteessa { $mac }
entity-group = Ryhmä
entity-product = Tuote
entity-firmware = Laiteohjelmisto
entity-serial-number = Sarjanumero
entity-configuration = Kokoonpano
entity-configuration-of = { $name } ({ $number }/{ $count })
entity-configuration-holds = { $inputs } sisään, { $outputs } ulos, { $channels } kanavan striimit
entity-configuration-holds-mixed = { $inputs } sisään, { $outputs } ulos
entity-configuration-numbered = Kokoonpano { $number }
entity-milan = Milan
entity-media-clock = Mediakello
entity-clock-domain = Kelloalue
entity-sampling-rate = Näytteenottotaajuus
clock-source-numbered = Lähde { $index }
rate-pull = pull { $pull }

stream-format = Muoto
stream-id = Striimin ID
stream-destination = Kohde
stream-max-transit = Suurin läpikulkuaika
stream-listeners = Kuuntelijat
stream-listener-gone = { $entity } tulo { $input }, ei verkossa
stream-disconnect-listener = Katkaise

avb-interfaces = AVB-liitännät
avb-interface = Liitäntä
avb-interface-clock-identity = Kellon tunniste
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, toimialue { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Käynnissä
avb-interface-none-reported = Ei raportoitu
avb-interface-path = Polku
avb-interface-own-grandmaster = Itse grandmaster
avb-interface-hops = { $count ->
    [one] { $count } hyppy grandmasterista
   *[other] { $count } hyppyä grandmasterista
}
avb-interface-link-up = Linkki muodostui
avb-interface-link-down = Linkki katkesi
avb-interface-grandmaster-changes = Grandmasterin vaihdokset
avb-interface-frames-sent = Lähetetyt kehykset
avb-interface-frames-received = Vastaanotetut kehykset
avb-interface-crc-errors = CRC-virheet

tree-firmware = Laiteohjelmisto { $version }
tree-descriptor-types = { $count ->
    [one] { $count } kuvaajatyyppi
   *[other] { $count } kuvaajatyyppiä
}
tree-clock = Kello
tree-clock-source-from = { $kind }, lähde: { $location } { $index }
tree-clock-domain-using = Käyttää: { $source }
tree-clusters = { $count ->
    [one] { $count } klusteri
   *[other] { $count } klusteria
}
tree-maps = { $count ->
    [one] { $count } kartoitus
   *[other] { $count } kartoitusta
}

advert-not-advertised = Ei mainostettu
advert-identity = Tunnistetiedot
advert-entity-id = Entiteetin ID
advert-entity-model = Entiteettimalli
advert-roles = Roolit
advert-talker = Talker
advert-listener = Listener
advert-clock = Kello
advert-btc = BTC
advert-gptp-domain = gPTP-toimialue
advert-sr-classes = SR-luokat
advert-indexes = Entiteettimallin indeksit
advert-identify-control = Tunnistussäädin
advert-avb-interface = AVB-liitäntä
advert-advertising = Mainostus
advert-valid-time = Voimassaoloaika
advert-available-index = Saatavuusindeksi
advert-association = Assosiaatio
advert-capabilities = Ominaisuudet

## Status bar

status-entities = { $count ->
    [one] { $count } entiteetti
   *[other] { $count } entiteettiä
}
status-not-discovering = Ei etsitä
status-discovering = Etsitään
status-discovering-as = Etsitään tunnuksella { $controller }
status-stopped = Pysähtyi virheeseen
status-alarm = Hälytys
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $count ->
    [one] { $alarm } ja { $count } muu
   *[other] { $alarm } ja { $count } muuta
}

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = ohjain
role-none = ei rooleja
classes-a-and-b = A ja B
clock-no-gptp = Ei gPTP:tä

read-not-read = Ei luettu
read-reading = Luetaan, { $count } tähän mennessä
read-ready-unreadable = { $count ->
    [one] Valmis, { $count } lukukelvoton
   *[other] Valmis, { $count } lukukelvotonta
}
read-ready-cached = Valmis, välimuistista
read-ready = Valmis
read-failed = Epäonnistui: { $reason }

milan-no = Ei
milan-before-1-3 = ennen versiota 1.3
milan-certified = { $version }, sertifioitu { $certification }
milan-not-certified = { $version }, ei sertifioitu

outcome-status = tila { $status }
outcome-no-response = ei vastausta
outcome-not-possible = ei mahdollista
outcome-connect = Striimiä { $talker } ei voitu yhdistää striimiin { $listener }: { $reason }.
outcome-disconnect = Striimin { $listener } yhteyttä ei voitu katkaista: { $reason }.
outcome-disconnect-listener = Kohteen { $listener } yhteyttä kohteeseen { $talker } ei voitu katkaista: { $reason }.
outcome-identify = Entiteettiä { $entity } ei voitu tunnistaa: { $reason }.
outcome-rename = Kohteen { $what } uudeksi nimeksi ei voitu asettaa ”{ $name }”: { $reason }.
outcome-rename-group = Entiteetin { $entity } ryhmän uudeksi nimeksi ei voitu asettaa ”{ $name }”: { $reason }.
outcome-format-streaming = Striimin { $stream } muotoa ei voitu muuttaa: se on käynnissä. Katkaise sen yhteys ensin.
outcome-format = Striimin { $stream } muotoa ei voitu muuttaa: { $reason }.
outcome-max-transit-time = Striimin { $stream } suurinta läpikulkuaikaa ei voitu muuttaa: { $reason }.
outcome-max-transit-time-streaming = Striimin { $stream } suurinta läpikulkuaikaa ei voitu muuttaa: se on käynnissä. Katkaise sen yhteys ensin.
outcome-sampling-rate = Entiteetin { $entity } näytteenottotaajuutta ei voitu muuttaa: { $reason }.
outcome-clock-source = Entiteetin { $entity } kellolähdettä ei voitu muuttaa: { $reason }.
outcome-configuration = Entiteetin { $entity } kokoonpanoa ei voitu muuttaa: { $reason }.
outcome-configuration-streaming = { $entity } pitää kokoonpanonsa, kun sen striimit ovat käynnissä: katkaise niiden yhteys ensin.
outcome-map = Kanavaa ei voitu kartoittaa entiteetissä { $entity }: { $reason }.
outcome-unmap = Kanavan kartoitusta ei voitu poistaa entiteetissä { $entity }: { $reason }.
outcome-control = Säädintä ”{ $control }” ei voitu asettaa entiteetissä { $entity }: { $reason }.
outcome-control-numbered = Säädintä { $index } ei voitu asettaa entiteetissä { $entity }: { $reason }.

stream-not-connected = Ei yhdistetty
stream-from = Lähde: { $stream }
stream-from-receiving = Lähde: { $stream }, vastaanotetaan
stream-from-waiting = Lähde: { $stream }, odotetaan talkeria
stream-from-failed = Lähde: { $stream }, talkerin varaus epäonnistui: { $reason }
stream-sending-to = Lähetetään osoitteeseen { $destination }

failure-no-response = se ei vastannut
failure-refused = se kieltäytyi tilalla { $status }
failure-malformed = sen vastausta ei voitu purkaa
failure-on-this-computer = se toimii tällä tietokoneella; lue se toiselta

msrp-failure-1 = kaistanleveys ei riitä
msrp-failure-2 = sillan resurssit eivät riitä
msrp-failure-3 = liikenneluokan kaistanleveys ei riitä
msrp-failure-4 = toinen talker käyttää striimin ID:tä
msrp-failure-5 = kohdeosoite on jo käytössä
msrp-failure-6 = korkeamman tason striimi syrjäytti
msrp-failure-7 = raportoitu viive muuttui
msrp-failure-8 = lähtöportti ei tue AVB:tä
msrp-failure-9 = käytä toista kohdeosoitetta
msrp-failure-10 = MSRP-resurssit loppuivat
msrp-failure-11 = MMRP-resurssit loppuivat
msrp-failure-12 = kohdeosoitetta ei voi tallentaa
msrp-failure-13 = prioriteetti ei ole SR-luokan prioriteetti
msrp-failure-14 = kehykset ovat liian suuria siirtotielle
msrp-failure-15 = portin fan-in-raja saavutettu
msrp-failure-16 = rekisteröidyn striimin ensimmäinen arvo muuttui
msrp-failure-17 = VLAN on estetty lähtöportissa
msrp-failure-18 = VLAN-merkintä on pois käytöstä lähtöportissa
msrp-failure-19 = SR-luokan prioriteetti ei täsmää
msrp-failure-unknown = tuntematon syy
msrp-failure-at = { $reason } (silta { $bridge })

## Entity list columns

column-vendor = Valmistaja
column-model = Malli
column-state = Tila
column-entity-model-id = Entiteettimallin ID
column-talker-streams = Talker-striimit
column-listener-streams = Listener-striimit
column-avb-lite = AVB Lite
column-egress = Lähtevä liikenne
column-wireless = Langaton

## Settings file

settings-no-place = Asetuksia ei voi tallentaa minnekään: kotikansio ei ole tiedossa.
settings-unusable = Tiedostoa { $path } ei voitu käyttää: { $error }.
settings-unsaved = Tiedostoa { $path } ei voitu tallentaa: { $error }.

column-remove = Poista sarake
column-move-left = Siirrä vasemmalle
column-move-right = Siirrä oikealle
column-add = Lisää sarake
common-percent = { $value }{" "}%

## Network view

netmap-empty = Ei vielä näytettävää verkkoa
netmap-empty-note = Entiteetit näkyvät tässä, kun ne on luettu ja ne ovat kertoneet paikkansa gPTP-puussa.
netmap-focus-clock-path = kellopolku ({ $name })
netmap-focus-streams = striimit ({ $name })
netmap-showing = Näytetään { $what }
netmap-devices = { $count ->
    [one] { $count } laite
   *[other] { $count } laitetta
}
netmap-bridges = { $count ->
    [one] { $count } silta
   *[other] { $count } siltaa
}
netmap-show-map = Näytä kartta
netmap-show-details = Näytä tiedot
stream-numbered = Striimi { $index }
netmap-bridge = Silta
netmap-access-point = Tukiasema
netmap-device = Laite
netmap-this-computer = Tämä tietokone
netmap-connected = Yhdistetty
netmap-advertised = Mainostettu, ei valmista listeneriä
netmap-advertised-off-tree = Mainostettu, ei valmista listeneriä ({ $listener } ei ole gPTP-puussa)
netmap-failed-at = Varaus epäonnistui sillassa { $bridge }: { $reason }
netmap-failed = Varaus epäonnistui: { $reason }
netmap-no-bridge-on = Liitännässä { $interface } ei havaittu siltaa
netmap-cannot-listen-on = gPTP:tä ei voi kuunnella liitännässä { $interface }
netmap-on-this-computer = Tällä tietokoneella
netmap-path-not-reported = Polkua ei raportoitu
netmap-gptp-not-reported = gPTP:tä ei raportoitu
netmap-off-tree = Ei gPTP-puussa
netmap-off-ptp = Ei PTP-puussa
netmap-not-lite = Ei AVB Lite -profiilissa
netmap-lite-not-reported = AVB Lite -tilaa ei raportoitu
netmap-synced = Synkronoitu
netmap-not-synced = Ei synkronoitu
netmap-triib-on = triib liitännässä { $interface }
netmap-through-count = { $count ->
    [one] { $count } läpi kulkeva
   *[other] { $count } läpi kulkevaa
}
netmap-out = { $count ->
    [one] { $count } lähtevä
   *[other] { $count } lähtevää
}
netmap-in = { $count ->
    [one] { $count } saapuva
   *[other] { $count } saapuvaa
}
netmap-failed-count = { $count ->
    [one] { $count } epäonnistunut
   *[other] { $count } epäonnistunutta
}
netmap-advertised-only = Vain mainostettu
netmap-failed-state = Epäonnistui
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Ei gPTP-puussa: se on itse grandmaster
netmap-apart-no-path = Sen polkua ei raportoitu; se seuraa grandmasteria { $grandmaster }
netmap-apart-unreported = Se ei ole raportoinut gPTP-tilaansa
netmap-apart-no-neighbor = Tämän tietokoneen liitännässä ei havaittu siltaa
netmap-apart-cannot-listen = Tämä tietokone ei voi kuunnella gPTP:tä liitännässään
netmap-apart-on-this-computer = Se toimii tällä tietokoneella; lue se toiselta tietokoneelta nähdäksesi sen gPTP-tilan
netmap-apart-not-lite = Se ei käytä AVB Lite -profiilia, joten se ei seuraa grandmasteria
netmap-apart-lite-unreported = Se ei raportoi mitään AVB Lite -profiilista, joten ei tiedetä, mitä se seuraa
netmap-clock-tree = Kellopuu
netmap-no-grandmaster = Grandmasteria ei havaittu
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Vaatii huomiota
netmap-nodes-below = Alemmat solmut
netmap-bridges-below = Alemmat sillat
netmap-clock-path = Kellopolku
netmap-hops = Hypyt grandmasterista
netmap-link-delay = Linkin viive
netmap-bridge-port = Sillan portti
netmap-link-drops = Linkin katkokset
netmap-synced-to-grandmaster = Synkronoitu grandmasteriin
netmap-host-no-gptp = Ei synkronoitu: tämä tietokone ei käytä gPTP:tä
netmap-link-no-gptp = Ei synkronoitu: sen linkillä ei ole gPTP:tä käytössä
netmap-ptp-offset-high = Ei synkronoitu: poikkeama grandmasterista { $offset }, yli 50 µs:n rajan, jonka AVB Lite sallii
netmap-ptp-no-offset = Ei synkronoitu: se ei ole mitannut poikkeamaa grandmasterista
netmap-audio = Ääni
netmap-media-clock-streams = Mediakellostriimit
netmap-audio-streams = Äänistriimit
netmap-bound = { $count ->
    [one] { $count } sidottu
   *[other] { $count } sidottua
}
netmap-flowing = Kulkee
netmap-advertised-state = Mainostettu
netmap-media-clock-stream = Mediakellostriimi
netmap-audio-stream = Äänistriimi
netmap-reaches = Ulottuu
netmap-passing-count = { $count ->
    [one] { $count } läpi kulkeva striimi
   *[other] { $count } läpi kulkevaa striimiä
}
netmap-through = Läpi kulkevat
netmap-passing-through = Kulkee läpi
netmap-sending = Lähtevät
netmap-receiving = Saapuvat
netmap-problems = Ongelmat
netmap-help-back = Palaa yleiskuvaan napsauttamalla taustaa.
netmap-help-stream = Napsauta striimiä tarkastellaksesi sitä tai taustaa palataksesi yleiskuvaan.
netmap-help-ptp = AVB Lite -profiilissa kello kulkee päästä päähän grandmasterista jokaiseen laitteeseen sellaisten kytkinten kautta, jotka eivät osallistu siihen, joten niitä ei näytetä. Laite on synkronoitu niin kauan kuin se seuraa grandmasteria 50 µs:n tarkkuudella. Napsauta laitetta tai sen johdinta tarkastellaksesi sen kelloa; tyhjennä valinta napsauttamalla taustaa.
netmap-help-clock = Kello kulkee grandmasterista kunkin sillan kautta puun jokaiseen solmuun. Katkoviivainen harmaa viiva on linkki, jolla gPTP ei ole käytössä. Napsauta laitetta tai sen johdinta tarkastellaksesi sen kellopolkua; tyhjennä valinta napsauttamalla taustaa.
netmap-help-media-clock = Vain mediakellostriimit (CRF), piirrettynä samoin kuin ääni: yksi johdin striimiä kohden, väritettynä talkerin mukaan. Napsauta johdinta tarkastellaksesi sen striimiä tai laitetta nähdäksesi sen striimit; tyhjennä valinta napsauttamalla taustaa.
netmap-help-audio = Jokaisella striimillä on oma johtimensa, joka tulee jokaiseen ylittämäänsä siltaan ja lähtee siitä. Väri määräytyy talkerin mukaan: jokaisella talkerilla on oma sävynsä, ja sen striimit ovat tämän sävyn vivahteita. Liikkuvat pisteet tarkoittavat, että ääni kulkee; paikallaan oleva punainen viiva on epäonnistunut varaus ja paikallaan oleva harmaa viiva mainostettu striimi ilman valmista listeneriä; molemmat päättyvät siihen, mihin varaus päättyy. Keskimmäisen sarakkeen laitteet ovat suoraan yhteydessä grandmasterin siltaan. Napsauta johdinta tarkastellaksesi sen striimiä tai laitetta nähdäksesi sen striimit; tyhjennä valinta napsauttamalla taustaa.

## Connections

matrix-nothing-shown = Ei näytettäviä striimejä
matrix-nothing-shown-note = Muuta hakua tai suodattimia nähdäksesi lisää striimejä.
matrix-empty = Ei yhdistettäviä striimejä
matrix-empty-note = Talker- ja listener-striimit kohtaavat tässä, kun niitä sisältävät entiteetit on luettu.
matrix-all-streams = Kaikki striimit
matrix-connectable-only = Piilota ne, joita ei voi yhdistää
matrix-none-hidden = Kaikki näytetyt striimit voi yhdistää
matrix-hidden = { $count ->
    [one] { $count } striimi piilotettu
   *[other] { $count } striimiä piilotettu
}
matrix-own = Entiteetin lähtöjä ei yhdistetä sen omiin tuloihin.
matrix-working = Työn alla.
matrix-waiting-change = Odotetaan tämän tulon viimeisintä muutosta.
matrix-connected = Yhdistetty ja vastaanottaa. Katkaise yhteys napsauttamalla.
matrix-bound-waiting = Sidottu, odotetaan talkerin striimiä. Katkaise yhteys napsauttamalla.
matrix-bound-failed = Sidottu, mutta talkerin varaus epäonnistui: { $reason }. Katkaise yhteys napsauttamalla.
matrix-bound-formats-differ = Sidottu, mutta muodot eroavat: talker lähettää muotoa { $sent }, tulon asetus on { $set }. Katkaise yhteys napsauttamalla.
matrix-formats-match = Muodot vastaavat ({ $format }). Yhdistä napsauttamalla.
matrix-format-must-change = Tulo hyväksyy muodon { $sent }, mutta sen asetus on { $set }, joten se ei ehkä toista ennen kuin sen muotoa muutetaan. Valitse muoto napsauttamalla.
matrix-format-input = Vaihda tulo muotoon
matrix-format-both = Vaihda molemmat muotoon
matrix-format-talker-busy = Talker lähettää muille listenereille, joten sen muoto pysyy.
matrix-format-anyway = Yhdistä muuttamatta muotoja
matrix-incompatible = Tulo ei hyväksy muotoa { $sent }. Sen asetus on { $set }.
matrix-group-none = Ei yhdistetty. Laajenna yhdistääksesi striimit yksitellen.
matrix-group-connected = { $count ->
    [one] { $count } yhdistetty. Laajenna nähdäksesi sen.
   *[other] { $count } yhdistettyä. Laajenna nähdäksesi jokaisen.
}
matrix-outputs-expand = { $count ->
    [one] { $count } striimilähtö. Laajenna napsauttamalla nuolta, tarkastele napsauttamalla nimeä.
   *[other] { $count } striimilähtöä. Laajenna napsauttamalla nuolta, tarkastele napsauttamalla nimeä.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } striimilähtö. Kutista napsauttamalla nuolta, tarkastele napsauttamalla nimeä.
   *[other] { $count } striimilähtöä. Kutista napsauttamalla nuolta, tarkastele napsauttamalla nimeä.
}
matrix-inputs-expand = { $count ->
    [one] { $count } striimitulo. Laajenna napsauttamalla nuolta, tarkastele napsauttamalla nimeä.
   *[other] { $count } striimituloa. Laajenna napsauttamalla nuolta, tarkastele napsauttamalla nimeä.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } striimitulo. Kutista napsauttamalla nuolta, tarkastele napsauttamalla nimeä.
   *[other] { $count } striimituloa. Kutista napsauttamalla nuolta, tarkastele napsauttamalla nimeä.
}
matrix-stream-inspect = { $detail } Tarkastele entiteettiä { $entity } napsauttamalla.
matrix-point = Osoita solua
matrix-point-note = nähdäksesi sen talkerin ja listenerin sekä sen, sopivatko niiden muodot yhteen.
matrix-legend-waiting = Sidottu, odotetaan striimiä
matrix-legend-trouble = Sidottu, jokin on vialla
matrix-legend-open = Voi yhdistää
matrix-legend-change = Tulon muotoa on ensin muutettava
matrix-legend-incompatible = Muodot eivät voi sopia yhteen
matrix-talker-outputs = Talker-lähdöt
matrix-listener-inputs = Listener-tulot

common-thousands-separator = {" "}

## Diagnostics

diag-since-start = Laskettu entiteetin käynnistymisestä lähtien.
diag-stream-input = Striimitulo
diag-stream-output = Striimilähtö
diag-locked = { $count ->
    [0] ei lukittunut
    [one] lukittui kerran
   *[other] lukittui { $number } kertaa
}
diag-lost-lock = { $count ->
    [0] ei menettänyt lukitusta
    [one] menetti lukituksen kerran
   *[other] menetti lukituksen { $number } kertaa
}
diag-frames-in = { $count ->
    [one] { $number } saapuva kehys
   *[other] { $number } saapuvaa kehystä
}
diag-frames-out = { $count ->
    [one] { $number } lähtevä kehys
   *[other] { $number } lähtevää kehystä
}
diag-media-locked = { $count ->
    [0] ei lukittunut mediakelloon
    [one] lukittui mediakelloon kerran
   *[other] lukittui mediakelloon { $number } kertaa
}
diag-lost-media-lock = { $count ->
    [0] ei menettänyt mediakellon lukitusta
    [one] menetti mediakellon lukituksen kerran
   *[other] menetti mediakellon lukituksen { $number } kertaa
}
diag-interrupted = { $count ->
    [0] ei keskeytynyt
    [one] keskeytyi kerran
   *[other] keskeytyi { $number } kertaa
}
diag-out-of-sequence = { $count ->
    [one] { $number } kehys väärässä järjestyksessä
   *[other] { $number } kehystä väärässä järjestyksessä
}
diag-media-resets = { $count ->
    [one] { $number } median nollaus
   *[other] { $number } median nollausta
}
diag-timestamps-uncertain = { $count ->
    [0] ei epävarmoja aikaleimoja
    [one] epävarmat aikaleimat kerran
   *[other] epävarmat aikaleimat { $number } kertaa
}
diag-no-timestamp = { $count ->
    [one] { $number } kehys ilman aikaleimaa
   *[other] { $number } kehystä ilman aikaleimaa
}
diag-unsupported-format = { $count ->
    [one] { $number } kehys tukemattomassa muodossa
   *[other] { $number } kehystä tukemattomassa muodossa
}
diag-late = { $count ->
    [one] { $number } kehys myöhässä
   *[other] { $number } kehystä myöhässä
}
diag-early = { $count ->
    [one] { $number } kehys etuajassa
   *[other] { $number } kehystä etuajassa
}
diag-started = { $count ->
    [0] ei käynnistynyt
    [one] käynnistyi kerran
   *[other] käynnistyi { $number } kertaa
}
diag-stopped = { $count ->
    [0] ei pysähtynyt
    [one] pysähtyi kerran
   *[other] pysähtyi { $number } kertaa
}
diag-reservation-failed = talkerin varaus epäonnistui: { $reason }
diag-latency = { $microseconds } µs kertynyttä viivettä

## AVB Lite

lite-active = Aktiivinen
lite-active-untagged = Aktiivinen, merkitsemätön
lite-active-vlan = Aktiivinen, VLAN { $vlan }
lite-capable = Tuettu
lite-mode = Toimintatila
lite-mode-capable = AVB (AVB Lite -yhteensopiva)
lite-because = Syy
lite-fallback-none = syytä ei annettu
lite-fallback-endpoint = toisen päätepisteen ilmoitus tuli läpi, joten niiden välissä ei ole AVB-siltaa
lite-fallback-unanswered = yhdeksään peer delay -pyyntöön ei vastattu
lite-fallback-responders = kaksi tai useampi vastasi yhteen peer delay -pyyntöön, joten kytkin ei ole AVB-silta
lite-fallback-configured = käyttäjä tai ohjain asetti sen
lite-fallback-other = syy, jota profiili ei nimeä
lite-other-profile = Muu profiili
lite-ptp-domain = { $profile }, toimialue { $domain }
lite-offset = Poikkeama
lite-offset-from = { $offset } (grandmaster: { $grandmaster })
lite-media-vlan = Media-VLAN
lite-untagged = Merkitsemätön
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Enintään { $count } listener striimiä kohden, sitten multicast
   *[other] Enintään { $count } listeneriä striimiä kohden, sitten multicast
}
lite-link = Linkki
lite-bandwidth = Kaistanleveys
lite-egress-of = { $used } / { $link }, { $share }
lite-egress-of-assumed = { $used } / { $link }, { $share }, oletuksena gigabitin linkki
lite-egress-reported = Entiteetin oma laskelma sen hyväksymistä striimeistä.
lite-egress-worked-out = Laskettu sen yhdistettyjen striimilähtöjen muodoista.
lite-alarm-offset = PTP-poikkeama { $offset }, yli 50 µs:n rajan, jonka AVB Lite sallii
lite-alarm-egress = Lähtevä liikenne { $share } linkistä, yli striimeille sallitun rajan { $limit }

## AVB Wireless

wireless-station = Asema
wireless-access-point = Tukiasema
wireless-role = Rooli
wireless-mode = Toimintatila
wireless-time = Aika
wireless-mode-a-ftm = Mode A, 802.1AS FTM:n kautta
wireless-mode-a-tm = Mode A, 802.1AS TM:n kautta
wireless-mode-b = Mode B, majakkakehyksistä
wireless-no-time = Ei aikaa
wireless-other-mode = Toimintatila, jota profiili ei nimeä
wireless-locked = Lukittu
wireless-holdover = Holdover-tilassa
wireless-not-locked = Ei lukittu
wireless-row-locked = Asema, lukittu
wireless-row-holdover = Asema, holdover-tilassa
wireless-row-not-locked = Asema, ei lukittu
wireless-row-access-point = { $count ->
    [one] Tukiasema, { $count } asema
   *[other] Tukiasema, { $count } asemaa
}
wireless-link = Linkki
wireless-channel = kanava { $channel }
wireless-not-known = Ei tiedossa
wireless-signal = Signaali
wireless-rate = Lähetysnopeus
wireless-ftm-valid = { $share } kelvollisia
wireless-rtt = edestakainen viive { $rtt }
wireless-bursts = { $count ->
    [one] purskeissa { $count } kehys
   *[other] purskeissa { $count } kehystä
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = tukiasema myöntää FTM-purskeita, joissa on muu määrä kehyksiä kuin kolme tai kaksi
wireless-reason-measurement = ei FTM:ää eikä TM:ää tukiaseman kanssa
wireless-reason-signaling = ei gPTP-capable Signaling -viestiä tukiasemalta
wireless-reason-other = syy, jota profiili ei nimeä
wireless-servo = Servovirhe
wireless-stations = Asemat
wireless-station-count = { $count ->
    [one] { $count } asema
   *[other] { $count } asemaa
}
wireless-no-ftm = ilman FTM:ää
wireless-unserved = Palvelemattomat listenerit
wireless-frame-budget = Kehysbudjetti
wireless-frame-use = { $used } / { $budget } kehystä sekunnissa
wireless-stream-frames = Striimikehykset
wireless-frames-of = { $readdressed } asemille, { $unmapped } ilman listeneriä, { $dropped } hylätty, { $restored } asemilta
wireless-class-a-allowed = Sallittu
wireless-class-a-not-allowed = Ei sallittu
wireless-alarm-not-locked = Wi-Fi-aika ei ole lukittunut tukiasemaan
wireless-alarm-holdover = Wi-Fi-aika holdover-tilassa, lukitus tukiasemaan menetetty
wireless-alarm-unserved = { $count ->
    [one] { $count } listener Wi-Fi-portissa palvelematta, yli unicast-rajan
   *[other] { $count } listeneriä Wi-Fi-portissa palvelematta, yli unicast-rajan
}

## Log

log-all = Kaikki
log-warnings = Varoitukset
log-pause = Tauko
log-resume = Jatka
log-clear = Tyhjennä
log-empty = Jokainen ATDECC-kehys, jonka triib lähettää ja vastaanottaa, näkyy tässä uusin ensin.
log-none-match = Mikään säilytetty kehys ei vastaa suodatinta.
log-frames = { $count ->
    [one] { $count } kehys
   *[other] { $count } kehystä
}
log-shown-of = { $shown } / { $all } kehystä
log-sent = Lähetetty
log-heard = Vastaanotettu
log-not-decoded = Ei purettu
log-warning-short = { $missing ->
    [one] Sen control_data_length -kenttä ulottuu { $missing } oktetin kehyksen lopun yli.
   *[other] Sen control_data_length -kenttä ulottuu { $missing } oktettia kehyksen lopun yli.
}
log-warning-undecodable = Sitä ei voi purkaa: { $error }.
log-warning-long-acmp = Se on pitkässä ACMP-muodossa, jota Milan-entiteetti ei saa lähettää (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Kanavakartoitukset
mapping-inputs = Tulot
mapping-outputs = Lähdöt
mapping-port = portti { $number }
mapping-fixed = kiinteä
mapping-not-read = Ei vielä luettu.
mapping-no-clusters = Ei klustereita.
mapping-no-streams = Ei äänistriimejä.
mapping-none = Ei kartoituksia.
mapping-not-mapped = Ei kartoitettu
mapping-cluster-numbered = Klusteri { $index }

## Presets

presets-note = Esiasetus tallentaa kunkin entiteetin kellolähteet, näytteenottotaajuudet, striimien muodot, säätimet ja yhteydet. Kun se palautetaan, muutetaan vain se, mikä eroaa.
presets-none = Ei vielä tallennettuja esiasetuksia.
presets-connections = { $count ->
    [one] { $count } yhteys
   *[other] { $count } yhteyttä
}
presets-recall = Palauta
presets-delete = Poista
presets-no-place = Esiasetuksia ei voi tallentaa minnekään: kotikansio ei ole tiedossa.
presets-undeletable = Tiedostoa { $path } ei voitu poistaa: { $error }.
presets-saved = { $count ->
    [one] Tallennettu ”{ $name }”, { $count } entiteetti.
   *[other] Tallennettu ”{ $name }”, { $count } entiteettiä.
}
presets-nothing-differs = Mikään ei eroa esiasetuksesta ”{ $name }”.
presets-recalling = { $count ->
    [one] Palautetaan ”{ $name }”: { $count } muutos.
   *[other] Palautetaan ”{ $name }”: { $count } muutosta.
}
presets-missing = { $report } Ei paikalla tai ei luettu: { $missing }.
presets-deleted = Poistettu ”{ $name }”.
presets-host-note = Se tallentaa myös tämän tietokoneen omat entiteetit ja käynnistää ne uudelleen palautettaessa.
presets-host-endpoints = { $count } tässä tietokoneessa
presets-starting-host = Käynnistetään tämän tietokoneen entiteetit esiasetukselle ”{ $name }”; muu seuraa, kun ne ovat taas käytössä.

## Controls

control-numbered = Säädin { $index }
control-not-shown = Ei näytetä tässä
control-option = Vaihtoehto { $number }

## Network errors

network-permission = triib tarvitsee oikeuden lähettää ja vastaanottaa raakoja Ethernet-kehyksiä.
network-needs-npcap = triib tarvitsee Npcapin raakojen Ethernet-kehysten lähettämiseen ja vastaanottamiseen.
network-npcap-administrators = Npcap sallii raakojen Ethernet-kehysten lähettämisen ja vastaanottamisen vain järjestelmänvalvojille. Suorita triib järjestelmänvalvojana tai asenna Npcap uudelleen ilman vain järjestelmänvalvojille -valintaa.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

common-decimal-separator = {","}

## This computer's own entities

host-add-entity = Lisää entiteetti
host-show-mine = Näytä vain tämän tietokoneen omat entiteetit
host-show-all = Näytä kaikki entiteetit
host-new-entity = Isännän entiteetti { $number }
host-failed = Lisääminen tähän tietokoneeseen ei onnistunut: { $reason }
host-needs-clock = Tämän tietokoneen omat entiteetit tarvitsevat langallisen liitännän, jossa on PTP-laitteistokello
host-no-ptp4l = ptp4l ei vastaa, joten tämän tietokoneen striimit eivät pysy gPTP-ajassa
host-elsewhere = triib-endpointd on käynnissä liitännässä { $interface } toisen käyttäjän tai root-käyttäjän oikeuksin, joten tämän tietokoneen entiteetit toimivat siellä, eivät täällä
host-foreign-mrp = Toinen ohjelma tekee MSRP- tai MVRP-ilmoituksia liitännässä { $interface } tämän tietokoneen osoitteesta ja voi siten perua sen, mitä tämän tietokoneen striimit tarvitsevat
host-alarm-foreign-mrp = Toinen ohjelma tällä tietokoneella tekee MSRP- tai MVRP-ilmoituksia liitännässä { $interface }
host-state = Tila
host-streaming = Lähettää
host-waiting = Odottaa listeneriä
host-listening = Kuuntelee
host-bound = Sidottu, odottaa talkeria
host-unbound = Ei sidottu
host-audio-from = Ääni lähteestä
host-audio-to = Ääni kohteeseen
host-channels = Kanavat
host-virtual-device = Virtuaalilaite
host-add-stream = Lisää striimi
host-add-audio-input = Äänitulo
host-add-audio-output = Äänilähtö
host-add-clock-input = Mediakellon tulo
host-add-clock-output = Mediakellon lähtö
host-remove-stream = Poista striimi
host-silence = Hiljaisuus
host-tone = Testiääni
host-nowhere = Ei minnekään
host-default-device = Oletuslaite
host-remove = Poista tästä tietokoneesta

mac-audio-device = Äänilaite
mac-no-audio-device = Ei mitään, kun mikään striimi ei ole yhdistetty
mac-entity-note = macOS ajaa tätä entiteettiä. Kun jokin sen striimeistä on yhdistetty, macOS luo äänilaitteen, jonka kautta apit toistavat ja nauhoittavat, ja siinä on kumpaankin suuntaan niin monta kanavaa kuin kokoonpano määrää.
mac-open-audio-midi-setup = Avaa Audio-MIDI-asetukset
mac-avb-off = AVB on pois päältä liitännässä { $interface }, joten tällä Macilla ei ole omaa AVB-entiteettiä. Laita se päälle Audio-MIDI-asetusten verkkolaiteselaimessa.

listen-section = Kuuntelu
listen-start = Kuuntele
listen-stop = Lopeta kuuntelu
listen-channels = Kanavat
listen-channel = Kanava { $number }
listen-channel-pair = Kanavat { $first }–{ $last }
listen-output = Lähtö
listen-default-output = Oletuslähtö
listen-gain = Vahvistus
listen-gain-db = { $db } dB
listen-starting = Käynnistetään…
listen-waiting-input = Odotetaan laitetta { $device }, jonka macOS luo, kun jokin entiteetin striimeistä on yhdistetty.
listen-waiting-output = Odotetaan laitetta { $device }.
listen-playing = Toistetaan, huiput { $db } dB
listen-playing-silent = Toistetaan, hiljaista
