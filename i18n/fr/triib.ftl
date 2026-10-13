# triib's interface text in French. The source is i18n/en/triib.ftl; term
# choices are in docs/glossary/fr.md.

## Language

language-name = Français

## Common

common-close = Fermer
common-more = Plus
common-keep-toolbar-shown = Toujours afficher la barre d’outils
common-auto-hide-toolbar = Masquer automatiquement la barre d’outils

## Settings

settings-title = Paramètres
settings-general = Général
settings-appearance = Apparence
settings-language = Langue
settings-language-system = Langue du système : { $language }
settings-language-note = Les champs de texte utilisent la langue de saisie du système.
settings-appearance-system = Système
settings-appearance-light = Clair
settings-appearance-dark = Sombre
settings-colors = Couleurs
settings-system-accent = Utiliser la couleur d’accentuation du système
settings-accent-picked = La couleur ci-dessous sert de base aux couleurs de triib.
settings-accent-omarchy = Tirée du thème Omarchy, { $theme }.
settings-accent-desktop = Tirée de la couleur d’accentuation du bureau.
settings-accent-none = Le bureau n’a pas de couleur d’accentuation, donc la couleur ci-dessous est utilisée.
settings-motion = Mouvement
settings-animations = Animations
settings-animations-note = Effets de ressort et de glissement à chaque changement.
settings-animations-reduced = Le bureau demande de réduire les animations, donc triib reste immobile.
settings-outside-control = Contrôle externe
settings-allow-outside-control = Autoriser le contrôle externe
settings-allow-outside-control-note = Les agents comme Claude Code peuvent voir et modifier votre réseau AVB, via triib --mcp. triib demande avant chaque nouvel agent.
settings-allowed-agents = Autorisés : { $agents }
settings-forget-agents = Oublier
settings-storage = Stockage
settings-storage-presets = Dossier des presets
settings-storage-entities = Fichier des entités
settings-storage-apply = Appliquer
settings-storage-choose = Choisir…
settings-storage-note = Les presets déjà stockés à l’ancien emplacement y restent ; déplacez-les pour continuer à les utiliser. Les réglages de triib sont enregistrés dans { $file }.
settings-storage-entities-note = Un fichier des entités qui n’existe pas encore commence comme une copie de celui en usage, et triib-endpointd redémarre avec lui.
dialog-choose-presets = Choisir le dossier des presets
dialog-choose-entities = Choisir le dossier du fichier des entités
settings-full-path = Utilisez un chemin complet, par exemple ~/Documents/triib.
settings-path-is-folder = { $path } est un dossier, pas un fichier.
settings-folder-missing = Le dossier { $path } n’existe pas. Créez-le d’abord, ou choisissez-en un.
settings-path-is-file = { $path } est un fichier, pas un dossier.
settings-cannot-write = triib ne peut pas écrire dans { $path } : { $error }.
app-file-dialog-failed = Impossible d’afficher la zone de dialogue de fichier : { $error }
agent-prompt-title = Autoriser { $agent } à contrôler triib ?
agent-prompt-body = { $agent } demande à utiliser le contrôle externe de triib : voir les entités de votre réseau et les modifier, en connectant des flux et en réglant formats, horloges et noms. Vous pouvez désactiver le contrôle externe dans les Paramètres.
agent-prompt-allow = Autoriser
agent-prompt-deny = Ne pas autoriser

common-cancel = Annuler
common-save = Enregistrer
common-not-set = Non défini
common-unnamed = Sans nom
common-none = Aucun
common-mac-address = Adresse MAC
common-list-separator = {", "}

## Network interfaces

interface-up = active
interface-link-down = liaison inactive
interface-wireless = sans fil
interface-hardware-clock = horloge matérielle
interface-hardware-clock-named = horloge matérielle { $clock }
interface-virtual = virtuelle

## Toolbar

toolbar-choose-interface = Choisir une interface
toolbar-interface = Interface réseau
toolbar-filter-interfaces = Filtrer les interfaces
toolbar-hide-virtual = Masquer les interfaces virtuelles
toolbar-hide-wireless = Masquer les interfaces sans fil
toolbar-hide-unclocked = Masquer les interfaces sans horloge matérielle
toolbar-connections = Connexions
toolbar-network = Réseau
toolbar-entities = Entités
toolbar-rediscover = Demander à chaque entité de s’annoncer
toolbar-rescan = Effacer et réanalyser toutes les entités
toolbar-search = Rechercher des entités et des flux
toolbar-presets = Presets
toolbar-log = Journal
toolbar-inspector = Inspecteur
toolbar-settings = Paramètres

## The network's state, in place of a view

state-no-interface = Aucune interface
state-no-interface-note = Choisissez l’interface du réseau AVB pour découvrir les entités.
state-starting = Démarrage
state-starting-note = Ouverture de { $interface }.
state-listening = À l’écoute
state-listening-note = Les entités présentes sur { $interface } apparaissent ici à mesure qu’elles s’annoncent.
state-permission-needed = Autorisation requise
state-npcap-needed = Npcap requis
state-get-npcap = Obtenir Npcap
state-copy-command = Copier la commande
state-cannot-use = Impossible d’utiliser { $interface }
state-try-again = Réessayer

## Entity list

entities-none-yet = Aucune entité pour l’instant
entities-none-yet-note = Chaque entité du réseau, avec ses rôles, ses classes SR et son horloge.

## Inspector

inspector-title = Inspecteur
inspector-entity = Entité
inspector-streams = Flux
inspector-controls = Contrôles
inspector-diagnostics = Diagnostic
inspector-descriptors = Descripteurs
inspector-select = Sélectionnez une entité pour en voir les détails.
inspector-offline = { $entity } est hors ligne.
inspector-heard = Il ne s’annonce pas : triib l’a entendu dans les commandes qu’il a envoyées, et le garde dans la liste jusqu’à 5 minutes sans commande.
inspector-rename = Renommer
inspector-name = Nom
inspector-identify = Identifier
inspector-model-not-read = Son modèle d’entité n’a pas été lu.
inspector-no-streams = Aucun flux.
inspector-no-controls = Aucun contrôle à afficher.
inspector-no-diagnostics = Aucune interface ni aucun compteur signalé.
inspector-reading = Lecture des descripteurs, { $count } lus jusqu’ici.
inspector-read-failed = Impossible de lire le modèle d’entité : { $reason }.

entity-section = Entité
entity-name = Nom
entity-triib-controller = Contrôleur triib
entity-triib-on = triib sur { $host }
entity-triib-here = triib sur cet ordinateur
entity-controller-on = Contrôleur sur { $host }
entity-controller-here = Contrôleur sur cet ordinateur
entity-controller-at = Contrôleur à { $mac }
entity-group = Groupe
entity-product = Produit
entity-firmware = Firmware
entity-serial-number = Numéro de série
entity-configuration = Configuration
entity-configuration-of = { $name } ({ $number } sur { $count })
entity-milan = Milan
entity-media-clock = Horloge média
entity-clock-domain = Domaine d’horloge
entity-sampling-rate = Fréquence d’échantillonnage
clock-source-numbered = Source { $index }
rate-pull = pull { $pull }

stream-format = Format
stream-id = ID du flux
stream-destination = Destination
stream-max-transit = Temps de transit max.

avb-interfaces = Interfaces AVB
avb-interface = Interface
avb-interface-clock-identity = Identité d’horloge
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domaine { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = En fonctionnement
avb-interface-none-reported = Aucun signalé
avb-interface-path = Chemin
avb-interface-own-grandmaster = Son propre grandmaster
avb-interface-hops = { $count ->
    [one] À { $count } saut du grandmaster
    [many] À { $count } sauts du grandmaster
   *[other] À { $count } sauts du grandmaster
}
avb-interface-link-up = Liaison active
avb-interface-link-down = Liaison inactive
avb-interface-grandmaster-changes = Changements de grandmaster
avb-interface-frames-sent = Trames envoyées
avb-interface-frames-received = Trames reçues
avb-interface-crc-errors = Erreurs CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } type de descripteur
    [many] { $count } types de descripteurs
   *[other] { $count } types de descripteurs
}
tree-clock = Horloge
tree-clock-source-from = { $kind }, depuis { $location } { $index }
tree-clock-domain-using = Utilise { $source }
tree-clusters = { $count ->
    [one] { $count } cluster
    [many] { $count } clusters
   *[other] { $count } clusters
}
tree-maps = { $count ->
    [one] { $count } mappage
    [many] { $count } mappages
   *[other] { $count } mappages
}

advert-not-advertised = Non annoncé
advert-identity = Identité
advert-entity-id = ID d’entité
advert-entity-model = Modèle d’entité
advert-roles = Rôles
advert-talker = Talker
advert-listener = Listener
advert-clock = Horloge
advert-btc = BTC
advert-gptp-domain = Domaine gPTP
advert-sr-classes = Classes SR
advert-indexes = Index du modèle d’entité
advert-identify-control = Contrôle d’identification
advert-avb-interface = Interface AVB
advert-advertising = Annonce
advert-valid-time = Durée de validité
advert-available-index = Index de disponibilité
advert-association = Association
advert-capabilities = Capacités

## Status bar

status-entities = { $count ->
    [one] { $count } entité
    [many] { $count } entités
   *[other] { $count } entités
}
status-not-discovering = Découverte inactive
status-discovering = Découverte en cours
status-discovering-as = Découverte en tant que { $controller }
status-stopped = Arrêt sur erreur
status-alarm = Alarme
status-alarm-of = { $entity } : { $alarm }
status-alarm-more = { $count ->
    [one] { $alarm } et { $count } autre
    [many] { $alarm } et { $count } autres
   *[other] { $alarm } et { $count } autres
}

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = contrôleur
role-none = aucun rôle
classes-a-and-b = A et B
clock-no-gptp = Pas de gPTP

read-not-read = Non lu
read-reading = Lecture en cours, { $count } lus
read-ready-unreadable = { $count ->
    [one] Prêt, { $count } illisible
    [many] Prêt, { $count } illisibles
   *[other] Prêt, { $count } illisibles
}
read-ready-cached = Prêt, depuis le cache
read-ready = Prêt
read-failed = Échec : { $reason }

milan-no = Non
milan-before-1-3 = avant 1.3
milan-certified = { $version }, certifié { $certification }
milan-not-certified = { $version }, non certifié

outcome-status = statut { $status }
outcome-no-response = pas de réponse
outcome-not-possible = impossible
outcome-connect = Impossible de connecter { $talker } à { $listener } : { $reason }.
outcome-disconnect = Impossible de déconnecter { $listener } : { $reason }.
outcome-identify = Impossible d’identifier { $entity } : { $reason }.
outcome-rename = Impossible de renommer { $what } en « { $name } » : { $reason }.
outcome-rename-group = Impossible de renommer le groupe de { $entity } en « { $name } » : { $reason }.
outcome-format-streaming = Impossible de changer le format de { $stream } : il est en cours de transmission. Déconnectez-le d’abord.
outcome-format = Impossible de changer le format de { $stream } : { $reason }.
outcome-max-transit-time = Impossible de changer le temps de transit max. de { $stream } : { $reason }.
outcome-max-transit-time-streaming = Impossible de changer le temps de transit max. de { $stream } : il est en cours de transmission. Déconnectez-le d’abord.
outcome-sampling-rate = Impossible de changer la fréquence d’échantillonnage de { $entity } : { $reason }.
outcome-clock-source = Impossible de changer la source d’horloge de { $entity } : { $reason }.
outcome-map = Impossible de mapper le canal sur { $entity } : { $reason }.
outcome-unmap = Impossible de supprimer le mappage du canal sur { $entity } : { $reason }.
outcome-control = Impossible de régler « { $control } » sur { $entity } : { $reason }.
outcome-control-numbered = Impossible de régler le contrôle { $index } sur { $entity } : { $reason }.

stream-not-connected = Non connecté
stream-from = Depuis { $stream }
stream-from-receiving = Depuis { $stream }, réception en cours
stream-from-waiting = Depuis { $stream }, en attente du talker
stream-from-failed = Depuis { $stream }, la réservation du talker a échoué : { $reason }
stream-sending-to = Envoi vers { $destination }

failure-no-response = l’entité n’a pas répondu
failure-refused = l’entité a refusé avec { $status }
failure-malformed = sa réponse n’a pas pu être décodée
failure-on-this-computer = l’entité s’exécute sur cet ordinateur ; lisez-la depuis un autre

msrp-failure-1 = bande passante insuffisante
msrp-failure-2 = ressources du commutateur insuffisantes
msrp-failure-3 = bande passante insuffisante pour la classe de trafic
msrp-failure-4 = ID de flux utilisé par un autre talker
msrp-failure-5 = adresse de destination déjà utilisée
msrp-failure-6 = préempté par un flux de rang supérieur
msrp-failure-7 = la latence signalée a changé
msrp-failure-8 = le port sortant n’est pas compatible AVB
msrp-failure-9 = utiliser une autre adresse de destination
msrp-failure-10 = ressources MSRP épuisées
msrp-failure-11 = ressources MMRP épuisées
msrp-failure-12 = impossible de stocker l’adresse de destination
msrp-failure-13 = la priorité n’est pas une priorité de classe SR
msrp-failure-14 = trames trop grandes pour le support
msrp-failure-15 = limite de fan-in du port atteinte
msrp-failure-16 = première valeur modifiée pour un flux enregistré
msrp-failure-17 = VLAN bloqué sur le port sortant
msrp-failure-18 = étiquetage VLAN désactivé sur le port sortant
msrp-failure-19 = priorité de classe SR discordante
msrp-failure-unknown = raison inconnue
msrp-failure-at = { $reason }, au commutateur { $bridge }

## Entity list columns

column-vendor = Fabricant
column-model = Modèle
column-state = État
column-entity-model-id = ID du modèle d’entité
column-talker-streams = Flux talker
column-listener-streams = Flux listener
column-avb-lite = AVB Lite
column-egress = Trafic sortant
column-wireless = Sans fil

## Settings file

settings-no-place = Impossible de conserver les paramètres : le dossier personnel est inconnu.
settings-unusable = Impossible d’utiliser { $path } : { $error }.
settings-unsaved = Impossible d’enregistrer { $path } : { $error }.

column-remove = Supprimer la colonne
column-move-left = Déplacer à gauche
column-move-right = Déplacer à droite
column-add = Ajouter une colonne
common-percent = { $value }{"\u202F"}%

## Network view

netmap-empty = Aucun réseau à afficher pour l’instant
netmap-empty-note = Les entités apparaissent ici une fois lues, quand elles ont indiqué leur place dans l’arbre gPTP.
netmap-focus-clock-path = Chemin d’horloge de { $name }
netmap-focus-streams = Flux de { $name }
netmap-showing = Affichage : { $what }
netmap-devices = { $count ->
    [one] { $count } appareil
    [many] { $count } appareils
   *[other] { $count } appareils
}
netmap-bridges = { $count ->
    [one] { $count } commutateur
    [many] { $count } commutateurs
   *[other] { $count } commutateurs
}
netmap-show-map = Afficher la carte
netmap-show-details = Afficher les détails
stream-numbered = Flux { $index }
netmap-bridge = Commutateur
netmap-access-point = Point d’accès
netmap-device = Appareil
netmap-this-computer = Cet ordinateur
netmap-connected = Connecté
netmap-advertised = Annoncé, aucun listener prêt
netmap-advertised-off-tree = Annoncé, aucun listener prêt ({ $listener } n’est pas dans l’arbre gPTP)
netmap-failed-at = Échec de la réservation sur { $bridge } : { $reason }
netmap-failed = Échec de la réservation : { $reason }
netmap-no-bridge-on = Aucun commutateur détecté sur { $interface }
netmap-cannot-listen-on = Impossible d’écouter gPTP sur { $interface }
netmap-on-this-computer = Sur cet ordinateur
netmap-path-not-reported = Chemin non signalé
netmap-gptp-not-reported = gPTP non signalé
netmap-off-tree = Hors de l’arbre gPTP
netmap-off-ptp = Hors de l’arbre PTP
netmap-not-lite = Hors AVB Lite
netmap-lite-not-reported = AVB Lite non signalé
netmap-synced = Synchronisé
netmap-not-synced = Non synchronisé
netmap-triib-on = triib sur { $interface }
netmap-through-count = { $count } en transit
netmap-out = { $count } émis
netmap-in = { $count ->
    [one] { $count } reçu
    [many] { $count } reçus
   *[other] { $count } reçus
}
netmap-failed-count = { $count } en échec
netmap-advertised-only = Annoncé uniquement
netmap-failed-state = Échec
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Hors de l’arbre gPTP : c’est son propre grandmaster
netmap-apart-no-path = Son chemin n’a pas été signalé ; il suit le grandmaster { $grandmaster }
netmap-apart-unreported = L’appareil n’a pas signalé son état gPTP
netmap-apart-no-neighbor = Aucun commutateur détecté sur l’interface de cet ordinateur
netmap-apart-cannot-listen = Cet ordinateur ne peut pas écouter gPTP sur son interface
netmap-apart-on-this-computer = Elle s’exécute sur cet ordinateur ; lisez-la depuis un autre ordinateur pour voir son état gPTP
netmap-apart-not-lite = L’appareil n’exécute pas AVB Lite, il ne suit donc pas le grandmaster
netmap-apart-lite-unreported = L’appareil ne signale rien d’AVB Lite, on ne sait donc pas qui il suit
netmap-clock-tree = Arbre d’horloge
netmap-no-grandmaster = Aucun grandmaster détecté
netmap-grandmaster-is = Grandmaster : { $grandmaster }
netmap-needs-attention = Attention requise
netmap-nodes-below = Nœuds en aval
netmap-bridges-below = Commutateurs en aval
netmap-clock-path = Chemin d’horloge
netmap-hops = Sauts depuis le grandmaster
netmap-link-delay = Délai de liaison
netmap-bridge-port = Port du commutateur
netmap-link-drops = Coupures de liaison
netmap-synced-to-grandmaster = Synchronisé sur le grandmaster
netmap-host-no-gptp = Non synchronisé : cet ordinateur n’exécute pas gPTP
netmap-link-no-gptp = Non synchronisé : gPTP ne fonctionne pas sur sa liaison
netmap-ptp-offset-high = Non synchronisé : { $offset } par rapport au grandmaster, au-delà des 50 µs permis par AVB Lite
netmap-ptp-no-offset = Non synchronisé : aucun décalage mesuré par rapport au grandmaster
netmap-audio = Audio
netmap-media-clock-streams = Flux d’horloge média
netmap-audio-streams = Flux audio
netmap-bound = { $count ->
    [one] { $count } lié
    [many] { $count } liés
   *[other] { $count } liés
}
netmap-flowing = En transmission
netmap-advertised-state = Annoncé
netmap-media-clock-stream = Flux d’horloge média
netmap-audio-stream = Flux audio
netmap-reaches = Atteint
netmap-passing-count = { $count ->
    [one] { $count } flux en transit
    [many] { $count } flux en transit
   *[other] { $count } flux en transit
}
netmap-through = Via
netmap-passing-through = En transit
netmap-sending = Envoi
netmap-receiving = Réception
netmap-problems = Problèmes
netmap-help-back = Cliquez sur l’arrière-plan pour revenir à la vue d’ensemble.
netmap-help-stream = Cliquez sur un flux pour l’inspecter, ou sur l’arrière-plan pour revenir à la vue d’ensemble.
netmap-help-ptp = Dans AVB Lite, l’horloge va de bout en bout, du grandmaster à chaque appareil, à travers des commutateurs qui n’y prennent aucune part ; aucun n’est donc affiché. Un appareil est synchronisé tant qu’il suit le grandmaster à 50 µs près. Cliquez sur un appareil ou sur son fil pour inspecter son horloge ; cliquez sur l’arrière-plan pour désélectionner.
netmap-help-clock = L’horloge part du grandmaster et passe par chaque commutateur jusqu’à chaque nœud de l’arbre. Une ligne grise discontinue est une liaison sur laquelle gPTP ne fonctionne pas. Cliquez sur un appareil ou sur son fil pour inspecter son chemin d’horloge ; cliquez sur l’arrière-plan pour désélectionner.
netmap-help-media-clock = Flux d’horloge média (CRF) uniquement, dessinés comme l’audio : un fil par flux, coloré selon le talker. Cliquez sur un fil pour inspecter son flux, ou sur un appareil pour voir ses flux ; cliquez sur l’arrière-plan pour désélectionner.
netmap-help-audio = Chaque flux a son propre fil, qui entre dans chaque commutateur traversé et en ressort. La couleur dépend du talker : chaque talker a une teinte, et ses flux en sont des nuances. Des points en mouvement indiquent que l’audio circule ; une ligne rouge fixe est une réservation en échec et une ligne grise fixe un flux annoncé sans listener prêt ; toutes deux s’arrêtent là où s’arrête la réservation. Les appareils de la colonne centrale sont connectés directement au commutateur du grandmaster. Cliquez sur un fil pour inspecter son flux, ou sur un appareil pour voir ses flux ; cliquez sur l’arrière-plan pour désélectionner.

## Connections

matrix-nothing-shown = Aucun flux à afficher
matrix-nothing-shown-note = Modifiez la recherche ou les filtres pour voir plus de flux.
matrix-empty = Aucun flux à connecter
matrix-empty-note = Les flux des talkers et des listeners se rencontrent ici une fois lues les entités qui les portent.
matrix-all-streams = Tous les flux
matrix-connectable-only = Masquer ce qui ne peut pas se connecter
matrix-none-hidden = Tous les flux affichés peuvent se connecter
matrix-hidden = { $count ->
    [one] { $count } flux masqué
    [many] { $count } flux masqués
   *[other] { $count } flux masqués
}
matrix-own = Les sorties d’une entité ne se connectent pas à ses propres entrées.
matrix-working = En cours.
matrix-waiting-change = En attente de la dernière modification de cette entrée.
matrix-connected = Connecté, réception en cours. Cliquez pour déconnecter.
matrix-bound-waiting = Liée, en attente du flux du talker. Cliquez pour déconnecter.
matrix-bound-failed = Liée, mais la réservation du talker a échoué : { $reason }. Cliquez pour déconnecter.
matrix-bound-formats-differ = Liée, mais les formats diffèrent : le talker envoie { $sent }, l’entrée est réglée sur { $set }. Cliquez pour déconnecter.
matrix-formats-match = Formats compatibles ({ $format }). Cliquez pour connecter.
matrix-format-must-change = L’entrée accepte { $sent } mais est réglée sur { $set }, elle risque donc de ne pas jouer tant que son format n’aura pas changé. Cliquez pour choisir un format.
matrix-format-input = Passer l’entrée en
matrix-format-both = Passer les deux en
matrix-format-talker-busy = Le talker diffuse vers d’autres listeners, son format reste donc inchangé.
matrix-format-anyway = Connecter sans changer les formats
matrix-incompatible = L’entrée n’accepte pas { $sent }. Elle est réglée sur { $set }.
matrix-group-none = Non connecté. Développez pour connecter les flux un par un.
matrix-group-connected = { $count ->
    [one] { $count } connecté. Développez pour le voir.
    [many] { $count } connectés. Développez pour voir chacun d’eux.
   *[other] { $count } connectés. Développez pour voir chacun d’eux.
}
matrix-outputs-expand = { $count ->
    [one] { $count } sortie de flux. Cliquez sur la flèche pour développer, sur le nom pour inspecter l’entité.
    [many] { $count } sorties de flux. Cliquez sur la flèche pour développer, sur le nom pour inspecter l’entité.
   *[other] { $count } sorties de flux. Cliquez sur la flèche pour développer, sur le nom pour inspecter l’entité.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } sortie de flux. Cliquez sur la flèche pour réduire, sur le nom pour inspecter l’entité.
    [many] { $count } sorties de flux. Cliquez sur la flèche pour réduire, sur le nom pour inspecter l’entité.
   *[other] { $count } sorties de flux. Cliquez sur la flèche pour réduire, sur le nom pour inspecter l’entité.
}
matrix-inputs-expand = { $count ->
    [one] { $count } entrée de flux. Cliquez sur la flèche pour développer, sur le nom pour inspecter l’entité.
    [many] { $count } entrées de flux. Cliquez sur la flèche pour développer, sur le nom pour inspecter l’entité.
   *[other] { $count } entrées de flux. Cliquez sur la flèche pour développer, sur le nom pour inspecter l’entité.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } entrée de flux. Cliquez sur la flèche pour réduire, sur le nom pour inspecter l’entité.
    [many] { $count } entrées de flux. Cliquez sur la flèche pour réduire, sur le nom pour inspecter l’entité.
   *[other] { $count } entrées de flux. Cliquez sur la flèche pour réduire, sur le nom pour inspecter l’entité.
}
matrix-stream-inspect = { $detail } Cliquez pour inspecter { $entity }.
matrix-point = Pointez une cellule
matrix-point-note = pour voir son talker et son listener, et si leurs formats concordent.
matrix-legend-waiting = Liée, en attente du flux
matrix-legend-trouble = Liée, quelque chose ne va pas
matrix-legend-open = Connexion possible
matrix-legend-change = Le format de l’entrée doit d’abord changer
matrix-legend-incompatible = Formats incompatibles
matrix-talker-outputs = Sorties des talkers
matrix-listener-inputs = Entrées des listeners

common-thousands-separator = {"\u202F"}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Compté depuis le démarrage de l’entité.
diag-stream-input = Entrée de flux
diag-stream-output = Sortie de flux
diag-locked = { $count ->
    [0] non verrouillé
    [1] verrouillé une fois
    [2] verrouillé deux fois
   *[other] verrouillé { $number } fois
}
diag-lost-lock = { $count ->
    [0] verrouillage jamais perdu
    [1] verrouillage perdu une fois
    [2] verrouillage perdu deux fois
   *[other] verrouillage perdu { $number } fois
}
diag-frames-in = { $count ->
    [one] { $number } trame reçue
    [many] { $number } trames reçues
   *[other] { $number } trames reçues
}
diag-frames-out = { $count ->
    [one] { $number } trame envoyée
    [many] { $number } trames envoyées
   *[other] { $number } trames envoyées
}
diag-media-locked = { $count ->
    [0] non verrouillé sur l’horloge média
    [1] verrouillé sur l’horloge média une fois
    [2] verrouillé sur l’horloge média deux fois
   *[other] verrouillé sur l’horloge média { $number } fois
}
diag-lost-media-lock = { $count ->
    [0] verrouillage sur l’horloge média jamais perdu
    [1] verrouillage sur l’horloge média perdu une fois
    [2] verrouillage sur l’horloge média perdu deux fois
   *[other] verrouillage sur l’horloge média perdu { $number } fois
}
diag-interrupted = { $count ->
    [0] non interrompu
    [1] interrompu une fois
    [2] interrompu deux fois
   *[other] interrompu { $number } fois
}
diag-out-of-sequence = { $count ->
    [one] { $number } trame hors séquence
    [many] { $number } trames hors séquence
   *[other] { $number } trames hors séquence
}
diag-media-resets = { $count ->
    [one] { $number } réinitialisation média
    [many] { $number } réinitialisations média
   *[other] { $number } réinitialisations média
}
diag-timestamps-uncertain = { $count ->
    [0] horodatages jamais incertains
    [1] horodatages incertains une fois
    [2] horodatages incertains deux fois
   *[other] horodatages incertains { $number } fois
}
diag-no-timestamp = { $count ->
    [one] { $number } trame sans horodatage
    [many] { $number } trames sans horodatage
   *[other] { $number } trames sans horodatage
}
diag-unsupported-format = { $count ->
    [one] { $number } trame dans un format non pris en charge
    [many] { $number } trames dans un format non pris en charge
   *[other] { $number } trames dans un format non pris en charge
}
diag-late = { $count ->
    [one] { $number } trame en retard
    [many] { $number } trames en retard
   *[other] { $number } trames en retard
}
diag-early = { $count ->
    [one] { $number } trame en avance
    [many] { $number } trames en avance
   *[other] { $number } trames en avance
}
diag-started = { $count ->
    [0] non démarré
    [1] démarré une fois
    [2] démarré deux fois
   *[other] démarré { $number } fois
}
diag-stopped = { $count ->
    [0] non arrêté
    [1] arrêté une fois
    [2] arrêté deux fois
   *[other] arrêté { $number } fois
}
diag-reservation-failed = la réservation du talker a échoué : { $reason }
diag-latency = { $microseconds } µs de latence accumulée

## AVB Lite

lite-active = Actif
lite-active-untagged = Actif, non étiqueté
lite-active-vlan = Actif, VLAN { $vlan }
lite-capable = Compatible
lite-mode = Mode
lite-mode-capable = AVB (compatible AVB Lite)
lite-because = Raison
lite-fallback-none = aucune raison donnée
lite-fallback-endpoint = la déclaration d’un autre terminal est parvenue, donc aucun commutateur AVB ne se trouve entre eux
lite-fallback-unanswered = neuf requêtes de peer delay sont restées sans réponse
lite-fallback-responders = plusieurs appareils ont répondu à une même requête de peer delay, donc le commutateur n’est pas compatible AVB
lite-fallback-configured = défini par l’opérateur ou par un contrôleur
lite-fallback-other = une raison non prévue par le profil
lite-other-profile = Autre profil
lite-ptp-domain = { $profile }, domaine { $domain }
lite-offset = Décalage
lite-offset-from = { $offset } par rapport à { $grandmaster }
lite-media-vlan = VLAN média
lite-untagged = Non étiqueté
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Jusqu’à { $count } listener par flux, puis multicast
    [many] Jusqu’à { $count } listeners par flux, puis multicast
   *[other] Jusqu’à { $count } listeners par flux, puis multicast
}
lite-link = Liaison
lite-bandwidth = Bande passante
lite-egress-of = { $used } sur { $link }, { $share }
lite-egress-of-assumed = { $used } sur { $link }, { $share }, liaison gigabit supposée
lite-egress-reported = Selon le décompte que fait l’entité de ses flux admis.
lite-egress-worked-out = D’après les formats de ses sorties de flux connectées.
lite-alarm-offset = Décalage PTP de { $offset }, au-delà des 50 µs permis par AVB Lite
lite-alarm-egress = Trafic sortant à { $share } de la liaison, au-delà des { $limit } permis aux flux

## AVB Wireless

wireless-station = Station
wireless-access-point = Point d’accès
wireless-role = Rôle
wireless-mode = Mode
wireless-time = Temps
wireless-mode-a-ftm = Mode A, 802.1AS sur FTM
wireless-mode-a-tm = Mode A, 802.1AS sur TM
wireless-mode-b = Mode B, par les balises
wireless-no-time = Aucune synchronisation
wireless-other-mode = Un mode non prévu par le profil
wireless-locked = Verrouillé
wireless-holdover = En holdover
wireless-not-locked = Non verrouillé
wireless-row-locked = Station, verrouillée
wireless-row-holdover = Station, en holdover
wireless-row-not-locked = Station, non verrouillée
wireless-row-access-point = { $count ->
    [one] Point d’accès, { $count } station
    [many] Point d’accès, { $count } stations
   *[other] Point d’accès, { $count } stations
}
wireless-link = Liaison
wireless-channel = canal { $channel }
wireless-not-known = Inconnue
wireless-signal = Signal
wireless-rate = Débit
wireless-ftm-valid = { $share } valides
wireless-rtt = aller-retour { $rtt }
wireless-bursts = { $count ->
    [one] rafales de { $count } trame
    [many] rafales de { $count } trames
   *[other] rafales de { $count } trames
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = le point d’accès accorde des rafales FTM d’une autre longueur que trois ou deux trames
wireless-reason-measurement = ni FTM ni TM avec le point d’accès
wireless-reason-signaling = aucun message Signaling compatible gPTP du point d’accès
wireless-reason-other = une raison non prévue par le profil
wireless-servo = Erreur du servo
wireless-stations = Stations
wireless-station-count = { $count ->
    [one] { $count } station
    [many] { $count } stations
   *[other] { $count } stations
}
wireless-no-ftm = sans FTM
wireless-unserved = Listeners non servis
wireless-frame-budget = Budget de trames
wireless-frame-use = { $used } sur { $budget } trames par seconde
wireless-stream-frames = Trames de flux
wireless-frames-of = { $readdressed } vers les stations, { $unmapped } sans listener, { $dropped } rejetées, { $restored } depuis les stations
wireless-class-a-allowed = Autorisée
wireless-class-a-not-allowed = Non autorisée
wireless-alarm-not-locked = Temps Wi-Fi non verrouillé sur le point d’accès
wireless-alarm-holdover = Temps Wi-Fi en holdover, verrouillage sur le point d’accès perdu
wireless-alarm-unserved = { $count ->
    [one] { $count } listener non servi sur le port Wi-Fi, au-delà de la limite unicast
    [many] { $count } listeners non servis sur le port Wi-Fi, au-delà de la limite unicast
   *[other] { $count } listeners non servis sur le port Wi-Fi, au-delà de la limite unicast
}

## Log

log-all = Tout
log-warnings = Avertissements
log-pause = Pause
log-resume = Reprendre
log-clear = Effacer
log-empty = Chaque trame ATDECC que triib envoie et reçoit apparaît ici, la plus récente en premier.
log-none-match = Aucune trame conservée ne correspond au filtre.
log-frames = { $count ->
    [one] { $count } trame
    [many] { $count } trames
   *[other] { $count } trames
}
log-shown-of = { $shown } sur { $all } trames
log-sent = Envoyée
log-heard = Reçue
log-not-decoded = Non décodée
log-warning-short = { $missing ->
    [one] Son control_data_length indique { $missing } octet au-delà de la fin de la trame.
    [many] Son control_data_length indique { $missing } octets au-delà de la fin de la trame.
   *[other] Son control_data_length indique { $missing } octets au-delà de la fin de la trame.
}
log-warning-undecodable = Décodage impossible : { $error }.
log-warning-long-acmp = Elle est au format ACMP long, qu’une entité Milan ne doit pas envoyer (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Mappage des canaux
mapping-inputs = Entrées
mapping-outputs = Sorties
mapping-port = port { $number }
mapping-fixed = fixe
mapping-not-read = Pas encore lu.
mapping-no-clusters = Aucun cluster.
mapping-no-streams = Aucun flux audio.
mapping-none = Aucun mappage.
mapping-not-mapped = Non mappé
mapping-cluster-numbered = Cluster { $index }

## Presets

presets-note = Un preset conserve les sources d’horloge, les fréquences d’échantillonnage, les formats de flux, les contrôles et les connexions de chaque entité. Le rappeler modifie ce qui diffère.
presets-none = Aucun preset enregistré pour l’instant.
presets-connections = { $count ->
    [one] { $count } connexion
    [many] { $count } connexions
   *[other] { $count } connexions
}
presets-recall = Rappeler
presets-delete = Supprimer
presets-no-place = Impossible de conserver des presets : le dossier personnel est inconnu.
presets-undeletable = Impossible de supprimer { $path } : { $error }.
presets-saved = { $count ->
    [one] « { $name } » enregistré avec { $count } entité.
    [many] « { $name } » enregistré avec { $count } entités.
   *[other] « { $name } » enregistré avec { $count } entités.
}
presets-nothing-differs = Rien ne diffère de « { $name } ».
presets-recalling = { $count ->
    [one] Rappel de « { $name } » : { $count } modification.
    [many] Rappel de « { $name } » : { $count } modifications.
   *[other] Rappel de « { $name } » : { $count } modifications.
}
presets-missing = { $report } Entités absentes ou non lues : { $missing }.
presets-deleted = « { $name } » supprimé.
presets-host-note = Il conserve aussi les entités propres à cet ordinateur, et les relance au rappel.
presets-host-endpoints = { $count } sur cet ordinateur
presets-starting-host = Démarrage des entités de cet ordinateur pour « { $name } » ; le reste suivra dès leur retour.

## Controls

control-numbered = Contrôle { $index }
control-not-shown = Non affiché ici
control-option = Option { $number }

## Network errors

network-permission = triib a besoin d’une autorisation pour envoyer et recevoir des trames Ethernet brutes.
network-needs-npcap = triib a besoin de Npcap pour envoyer et recevoir des trames Ethernet brutes.
network-npcap-administrators = Npcap ne permet qu’aux administrateurs d’envoyer et de recevoir des trames Ethernet brutes. Exécutez triib en tant qu’administrateur, ou réinstallez Npcap sans son option réservée aux administrateurs.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own entities

host-add-entity = Ajouter une entité
host-show-mine = Afficher uniquement les entités propres à cet ordinateur
host-show-all = Afficher toutes les entités
host-new-entity = Entité de l’hôte { $number }
host-failed = Impossible de l’ajouter à cet ordinateur : { $reason }
host-needs-clock = Les entités propres à cet ordinateur nécessitent une interface filaire avec une horloge matérielle PTP
host-no-ptp4l = ptp4l ne répond pas, les flux de cet ordinateur ne peuvent donc pas suivre l’heure gPTP
host-elsewhere = triib-endpointd s’exécute sur { $interface } pour un autre utilisateur ou en tant que root, les entités de cet ordinateur s’exécutent donc là-bas, pas ici
host-foreign-mrp = Un autre programme émet des déclarations MSRP ou MVRP sur { $interface } depuis l’adresse de cet ordinateur et peut retirer ce dont ont besoin les flux de cet ordinateur
host-alarm-foreign-mrp = Un autre programme de cet ordinateur émet des déclarations MSRP ou MVRP sur { $interface }
host-state = État
host-streaming = Diffusion
host-waiting = En attente d’un listener
host-listening = À l’écoute
host-bound = Lié, en attente du talker
host-unbound = Non lié
host-audio-from = Audio depuis
host-audio-to = Audio vers
host-channels = Canaux
host-virtual-device = Appareil virtuel
host-add-stream = Ajouter un flux
host-add-audio-input = Entrée audio
host-add-audio-output = Sortie audio
host-add-clock-input = Entrée d’horloge média
host-add-clock-output = Sortie d’horloge média
host-remove-stream = Retirer le flux
host-silence = Silence
host-tone = Tonalité de test
host-nowhere = Nulle part
host-default-device = Périphérique par défaut
host-remove = Retirer de cet ordinateur
