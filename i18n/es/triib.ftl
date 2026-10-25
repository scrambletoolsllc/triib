# triib's interface text in Spanish. The source is i18n/en/triib.ftl; term
# choices are in docs/glossary/es.md.

## Language

language-name = Español

## Common

common-close = Cerrar
common-more = Más
common-keep-toolbar-shown = Mantener visible la barra de herramientas
common-auto-hide-toolbar = Ocultar la barra de herramientas automáticamente

## Settings

settings-title = Ajustes
settings-general = General
settings-appearance = Apariencia
settings-language = Idioma
settings-language-system = Predeterminado del sistema: { $language }
settings-language-note = En los campos de texto se escribe con el idioma de entrada del sistema.
settings-appearance-system = Sistema
settings-appearance-light = Claro
settings-appearance-dark = Oscuro
settings-colors = Colores
settings-system-accent = Usar el color de énfasis del sistema
settings-accent-picked = Los colores de triib parten del color de abajo.
settings-accent-omarchy = Del tema de Omarchy, { $theme }.
settings-accent-desktop = Del color de énfasis del escritorio.
settings-accent-none = El escritorio no tiene color de énfasis, así que se usa el color de abajo.
settings-motion = Movimiento
settings-animations = Animaciones
settings-animations-note = Rebotes y deslizamientos cuando algo cambia.
settings-animations-reduced = El escritorio pide movimiento reducido, así que triib se queda quieto.
settings-outside-control = Control externo
settings-allow-outside-control = Permitir el control externo
settings-allow-outside-control-note = Los agentes como Claude Code pueden ver y cambiar tu red AVB mediante triib --mcp. triib pregunta antes de cada agente nuevo.
settings-allowed-agents = Permitidos: { $agents }
settings-forget-agents = Olvidar
settings-storage = Almacenamiento
settings-storage-presets = Carpeta de presets
settings-storage-entities = Archivo de entidades
settings-storage-apply = Aplicar
settings-storage-choose = Elegir…
settings-storage-note = Los presets ya guardados en una ubicación anterior se quedan allí; muévelos para seguir usándolos. Los ajustes de triib se guardan en { $file }.
settings-storage-entities-note = Un archivo de entidades que aún no exista empieza como copia del que se usa, y triib-endpointd se reinicia con él.
dialog-choose-presets = Elige la carpeta de presets
dialog-choose-entities = Elige la carpeta del archivo de entidades
settings-full-path = Usa una ruta completa, como ~/Documents/triib.
settings-path-is-folder = { $path } es una carpeta, no un archivo.
settings-folder-missing = No existe la carpeta { $path }. Créala primero o elige otra.
settings-path-is-file = { $path } es un archivo, no una carpeta.
settings-cannot-write = triib no puede escribir en { $path }: { $error }.
app-file-dialog-failed = No se ha podido mostrar el diálogo de archivos: { $error }
agent-prompt-title = ¿Permitir que { $agent } controle triib?
agent-prompt-body = { $agent } pide usar el control externo de triib: ver las entidades de tu red y cambiarlas, conectando flujos y ajustando formatos, relojes y nombres. Puedes desactivar el control externo en Ajustes.
agent-prompt-allow = Permitir
agent-prompt-deny = No permitir

common-cancel = Cancelar
common-save = Guardar
common-not-set = Sin definir
common-unnamed = Sin nombre
common-none = Ninguno
common-mac-address = Dirección MAC
common-list-separator = {", "}

## Network interfaces

interface-up = activa
interface-link-down = enlace caído
interface-wireless = inalámbrica
interface-hardware-clock = reloj de hardware
interface-hardware-clock-named = reloj de hardware { $clock }
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Elegir una interfaz
toolbar-interface = Interfaz de red
toolbar-filter-interfaces = Filtrar interfaces
toolbar-hide-virtual = Ocultar interfaces virtuales
toolbar-hide-wireless = Ocultar interfaces inalámbricas
toolbar-hide-unclocked = Ocultar interfaces sin reloj de hardware
toolbar-connections = Conexiones
toolbar-network = Red
toolbar-entities = Entidades
toolbar-rediscover = Pedir a todas las entidades que se anuncien
toolbar-rescan = Borrar y volver a escanear todas las entidades
toolbar-search = Buscar entidades y flujos
toolbar-presets = Presets
toolbar-log = Registro
toolbar-inspector = Inspector
toolbar-settings = Ajustes
toolbar-close-window = Cerrar ventana

## The macOS menu bar, named as in macOS's own apps

menu-about = Acerca de triib
menu-settings = Ajustes…
menu-services = Servicios
menu-hide = Ocultar triib
menu-hide-others = Ocultar otros
menu-show-all = Mostrar todo
menu-quit = Salir de triib
menu-file = Archivo
menu-presets = Presets…
menu-close-window = Cerrar ventana
menu-edit = Edición
menu-cut = Cortar
menu-copy = Copiar
menu-paste = Pegar
menu-select-all = Seleccionar todo
menu-view = Visualización
menu-full-screen = Entrar en pantalla completa
menu-window = Ventana
menu-minimize = Minimizar
menu-zoom = Zoom
menu-bring-all-to-front = Traer todo al frente
menu-help = Ayuda
menu-website = Sitio web de triib
menu-release-notes = Notas de la versión

## The network's state, in place of a view

state-no-interface = Sin interfaz
state-no-interface-note = Elige la interfaz de la red AVB para descubrir entidades.
state-starting = Iniciando
state-starting-note = Abriendo { $interface }.
state-listening = Escuchando
state-listening-note = Las entidades de { $interface } aparecen aquí a medida que se anuncian.
state-permission-needed = Se necesita permiso
state-npcap-needed = Se necesita Npcap
state-get-npcap = Obtener Npcap
state-copy-command = Copiar el comando
state-permission-watch = triib continúa por sí solo en cuanto se ejecute el comando.
state-cannot-use = No se puede usar { $interface }
state-try-again = Reintentar

## Entity list

entities-none-yet = Aún no hay entidades
entities-none-yet-note = Todas las entidades de la red, con sus roles, clases SR y reloj.
entities-filter = Filtrar entidades
entities-hide-others = Ocultar entidades de otros dispositivos
entities-hide-controllers = Ocultar controladores
entities-hide-other-domains = Ocultar entidades fuera del dominio de reloj de este equipo
entities-hidden = { $count ->
    [one] { $count } entidad oculta
    [many] { $count } entidades ocultas
   *[other] { $count } entidades ocultas
}

## Inspector

inspector-title = Inspector
inspector-entity = Entidad
inspector-streams = Flujos
inspector-controls = Controles
inspector-diagnostics = Diagnóstico
inspector-descriptors = Descriptores
inspector-select = Selecciona una entidad para ver sus detalles.
inspector-offline = { $entity } está fuera de línea.
inspector-heard = No se anuncia: triib lo ha oído en los comandos que ha enviado, y lo mantiene en la lista hasta que pasen 5 minutos sin ninguno.
inspector-rename = Cambiar nombre
inspector-name = Nombre
inspector-identify = Identificar
inspector-model-not-read = Su modelo de entidad no se ha leído.
inspector-no-streams = Sin flujos.
inspector-no-controls = No hay controles que mostrar.
inspector-no-diagnostics = No informa de interfaces ni de contadores.
inspector-reading = Leyendo descriptores, { $count } hasta ahora.
inspector-read-failed = No se pudo leer el modelo de entidad: { $reason }.

entity-section = Entidad
entity-name = Nombre
entity-triib-controller = Controlador triib
entity-triib-on = triib en { $host }
entity-triib-here = triib en este equipo
entity-controller-on = Controlador en { $host }
entity-controller-here = Controlador en este equipo
entity-controller-at = Controlador en { $mac }
entity-group = Grupo
entity-product = Producto
entity-firmware = Firmware
entity-serial-number = Número de serie
entity-configuration = Configuración
entity-configuration-of = { $name } ({ $number } de { $count })
entity-configuration-holds = { $inputs } entradas, { $outputs } salidas, flujos de { $channels } canales
entity-configuration-holds-mixed = { $inputs } entradas, { $outputs } salidas
entity-configuration-numbered = Configuración { $number }
entity-milan = Milan
entity-media-clock = Reloj de medios
entity-clock-domain = Dominio de reloj
entity-sampling-rate = Frecuencia de muestreo
clock-source-numbered = Fuente { $index }
rate-pull = pull { $pull }

stream-format = Formato
stream-id = ID del flujo
stream-destination = Destino
stream-max-transit = Tiempo de tránsito máx.
stream-listeners = Oyentes
stream-listener-gone = { $entity }, entrada { $input }, desconectado
stream-disconnect-listener = Desconectar

avb-interfaces = Interfaces AVB
avb-interface = Interfaz
avb-interface-clock-identity = Identidad del reloj
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, dominio { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = En ejecución
avb-interface-none-reported = No informa ninguno
avb-interface-path = Ruta
avb-interface-own-grandmaster = Es su propio grandmaster
avb-interface-hops = { $count ->
    [one] A { $count } salto del grandmaster
    [many] A { $count } saltos del grandmaster
   *[other] A { $count } saltos del grandmaster
}
avb-interface-link-up = Enlace activo
avb-interface-link-down = Enlace caído
avb-interface-grandmaster-changes = Cambios de grandmaster
avb-interface-frames-sent = Tramas enviadas
avb-interface-frames-received = Tramas recibidas
avb-interface-crc-errors = Errores CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } tipo de descriptor
    [many] { $count } tipos de descriptor
   *[other] { $count } tipos de descriptor
}
tree-clock = Reloj
tree-clock-source-from = { $kind }, desde { $location } { $index }
tree-clock-domain-using = Usa { $source }
tree-clusters = { $count ->
    [one] { $count } clúster
    [many] { $count } clústeres
   *[other] { $count } clústeres
}
tree-maps = { $count ->
    [one] { $count } mapa
    [many] { $count } mapas
   *[other] { $count } mapas
}

advert-not-advertised = Sin anunciar
advert-identity = Identidad
advert-entity-id = ID de entidad
advert-entity-model = Modelo de entidad
advert-roles = Roles
advert-talker = Talker
advert-listener = Listener
advert-clock = Reloj
advert-gptp-domain = Dominio gPTP
advert-sr-classes = Clases SR
advert-indexes = Índices del modelo de entidad
advert-identify-control = Control de identificación
advert-avb-interface = Interfaz AVB
advert-advertising = Anuncio
advert-valid-time = Tiempo de validez
advert-available-index = Índice de disponibilidad
advert-association = Asociación
advert-capabilities = Capacidades

## Status bar

status-entities = { $count ->
    [one] { $count } entidad
    [many] { $count } entidades
   *[other] { $count } entidades
}
status-not-discovering = Descubrimiento inactivo
status-discovering = Descubriendo
status-discovering-as = Descubriendo como { $controller }
status-stopped = Detenido por un error
status-alarm = Alarma
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } y { $count } más

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controlador
role-none = sin roles
classes-a-and-b = A y B
clock-no-gptp = Sin gPTP

read-not-read = Sin leer
read-reading = Leyendo, { $count } hasta ahora
read-ready-unreadable = { $count ->
    [one] Listo, { $count } ilegible
    [many] Listo, { $count } ilegibles
   *[other] Listo, { $count } ilegibles
}
read-ready-cached = Listo, desde la caché
read-ready = Listo
read-failed = Error: { $reason }

milan-no = No
milan-before-1-3 = anterior a 1.3
milan-certified = { $version }, certificado { $certification }
milan-not-certified = { $version }, sin certificar

outcome-status = estado { $status }
outcome-no-response = sin respuesta
outcome-not-possible = no es posible
outcome-connect = No se pudo conectar { $talker } a { $listener }: { $reason }.
outcome-disconnect = No se pudo desconectar { $listener }: { $reason }.
outcome-disconnect-listener = No se pudo desconectar { $listener } de { $talker }: { $reason }.
outcome-identify = No se pudo identificar { $entity }: { $reason }.
outcome-rename = No se pudo cambiar el nombre de { $what } a «{ $name }»: { $reason }.
outcome-rename-group = No se pudo cambiar el nombre del grupo de { $entity } a «{ $name }»: { $reason }.
outcome-format-streaming = No se pudo cambiar el formato de { $stream }: está transmitiendo. Desconéctalo primero.
outcome-format = No se pudo cambiar el formato de { $stream }: { $reason }.
outcome-max-transit-time = No se pudo cambiar el tiempo de tránsito máx. de { $stream }: { $reason }.
outcome-max-transit-time-streaming = No se pudo cambiar el tiempo de tránsito máx. de { $stream }: está transmitiendo. Desconéctalo primero.
outcome-sampling-rate = No se pudo cambiar la frecuencia de muestreo de { $entity }: { $reason }.
outcome-clock-source = No se pudo cambiar la fuente de reloj de { $entity }: { $reason }.
outcome-configuration = No se pudo cambiar la configuración de { $entity }: { $reason }.
outcome-configuration-streaming = { $entity } mantiene su configuración mientras sus flujos estén activos: desconéctalos primero.
outcome-map = No se pudo mapear el canal en { $entity }: { $reason }.
outcome-unmap = No se pudo quitar el mapeo del canal en { $entity }: { $reason }.
outcome-control = No se pudo ajustar «{ $control }» en { $entity }: { $reason }.
outcome-control-numbered = No se pudo ajustar el control { $index } en { $entity }: { $reason }.

stream-not-connected = Sin conectar
stream-from = De { $stream }
stream-from-receiving = De { $stream }, recibiendo
stream-from-waiting = De { $stream }, esperando al talker
stream-from-failed = De { $stream }, falló la reserva del talker: { $reason }
stream-sending-to = Enviando a { $destination }

failure-no-response = no respondió
failure-refused = rechazó la petición con { $status }
failure-malformed = no se pudo decodificar su respuesta
failure-on-this-computer = se ejecuta en este equipo; léela desde otro

msrp-failure-1 = ancho de banda insuficiente
msrp-failure-2 = recursos insuficientes en el switch
msrp-failure-3 = ancho de banda insuficiente para la clase de tráfico
msrp-failure-4 = ID de flujo en uso por otro talker
msrp-failure-5 = dirección de destino ya en uso
msrp-failure-6 = desplazado por un flujo de mayor rango
msrp-failure-7 = la latencia informada ha cambiado
msrp-failure-8 = el puerto de salida no es compatible con AVB
msrp-failure-9 = usar otra dirección de destino
msrp-failure-10 = sin recursos MSRP
msrp-failure-11 = sin recursos MMRP
msrp-failure-12 = no se puede almacenar la dirección de destino
msrp-failure-13 = la prioridad no es la de una clase SR
msrp-failure-14 = tramas demasiado grandes para el medio
msrp-failure-15 = alcanzado el límite de fan-in del puerto
msrp-failure-16 = cambió el primer valor de un flujo registrado
msrp-failure-17 = VLAN bloqueada en el puerto de salida
msrp-failure-18 = etiquetado VLAN desactivado en el puerto de salida
msrp-failure-19 = la prioridad de la clase SR no coincide
msrp-failure-unknown = motivo desconocido
msrp-failure-at = { $reason }, en el switch { $bridge }

## Entity list columns

column-vendor = Fabricante
column-entity-image = Imagen de la entidad
column-vendor-image = Imagen del fabricante
column-model = Modelo
column-state = Estado
column-entity-model-id = ID del modelo de entidad
column-talker-streams = Flujos de talker
column-listener-streams = Flujos de listener
column-avb-lite = AVB Lite
column-egress = Tráfico de salida
column-wireless = Inalámbrico

## Settings file

settings-no-place = No hay dónde guardar los ajustes: no se conoce la carpeta personal.
settings-unusable = No se pudo usar { $path }: { $error }.
settings-unsaved = No se pudo guardar { $path }: { $error }.

column-remove = Quitar columna
column-move-left = Mover a la izquierda
column-move-right = Mover a la derecha
column-add = Añadir una columna
common-percent = { $value } %

## Network view

netmap-empty = Aún no hay red que mostrar
netmap-empty-note = Las entidades aparecen aquí cuando se han leído y han indicado en qué dominio de reloj están.
netmap-focus-clock-path = la ruta del reloj de { $name }
netmap-focus-streams = los flujos de { $name }
netmap-showing = Mostrando { $what }
netmap-devices = { $count ->
    [one] { $count } dispositivo
    [many] { $count } dispositivos
   *[other] { $count } dispositivos
}
netmap-bridges = { $count ->
    [one] { $count } switch
    [many] { $count } switches
   *[other] { $count } switches
}
netmap-show-map = Mostrar el mapa
netmap-show-details = Mostrar los detalles
stream-numbered = Flujo { $index }
netmap-bridge = Switch
netmap-access-point = Punto de acceso
netmap-plain-switch = Switch sin AVB
netmap-device = Dispositivo
netmap-this-computer = Este equipo
netmap-connected = Conectado
netmap-advertised = Anunciado, ningún listener listo
netmap-advertised-unknown-domain = Anunciado, ningún listener listo (no se conoce el dominio de reloj de { $listener })
netmap-failed-at = La reserva falló en { $bridge }: { $reason }
netmap-failed = La reserva falló: { $reason }
netmap-no-bridge-on = Ningún switch detectado en { $interface }
netmap-cannot-listen-on = No se puede escuchar gPTP en { $interface }
netmap-behind-plain-switch = Detrás de un switch sin AVB
netmap-on-this-computer = En este equipo
netmap-path-not-reported = Ruta no informada
netmap-gptp-not-reported = gPTP no informado
netmap-domain = Dominio gPTP { $domain }
netmap-ptp-domain = Dominio PTP { $domain }
netmap-domain-unknown = Dominio desconocido
netmap-synced = Sincronizado
netmap-sync = Sincronización
netmap-not-synced = No sincronizado
netmap-triib-on = triib en { $interface }
netmap-through-count = { $count } en tránsito
netmap-out = { $count } de salida
netmap-in = { $count } de entrada
netmap-failed-count = { $count } con fallo
netmap-advertised-only = Solo anunciado
netmap-failed-state = Fallido
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = Enviando, sin ningún listener conocido vinculado
netmap-no-listener = Sin listener conocido
netmap-apart-no-path = No ha informado de su ruta; sigue al grandmaster { $grandmaster }
netmap-apart-unreported = No ha informado de su estado gPTP
netmap-apart-no-neighbor = Ningún switch detectado en la interfaz de este equipo
netmap-apart-cannot-listen = Este equipo no puede escuchar gPTP en su interfaz
netmap-apart-plain-switch = Este ordenador está detrás de un switch que no participa en AVB y ningún dispositivo que lo comparte ejecuta AVB Lite allí, así que no está en ningún dominio
netmap-apart-on-this-computer = Se ejecuta en este equipo; léela desde otro equipo para ver su estado de gPTP
netmap-switch-state = No interviene en AVB: representa los switches que hay delante de los dispositivos de abajo, que no encontraron ningún switch AVB y recurrieron a AVB Lite, siguiendo a un mismo grandmaster PTP
netmap-clock-tree = Árbol de reloj
netmap-no-grandmaster = Ningún grandmaster detectado
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Requiere atención
netmap-nodes-below = Nodos por debajo
netmap-bridges-below = Switches por debajo
netmap-clock-path = Ruta del reloj
netmap-hops = Saltos hasta el grandmaster
netmap-link-delay = Retardo del enlace
netmap-bridge-port = Puerto del switch
netmap-link-drops = Caídas del enlace
netmap-synced-to-grandmaster = Sincronizado con el grandmaster
netmap-host-no-gptp = No sincronizado: este equipo no ejecuta gPTP
netmap-host-no-ptp = No sincronizado: este equipo no sigue al grandmaster de este dominio
netmap-link-no-gptp = No sincronizado: gPTP no funciona en su enlace
netmap-ptp-offset-high = No sincronizado: { $offset } respecto al grandmaster, por encima de los 50 µs que admite AVB Lite
netmap-ptp-no-offset = No sincronizado: no ha medido ningún offset respecto al grandmaster
netmap-audio = Audio
netmap-media-clock-streams = Flujos de reloj de medios
netmap-audio-streams = Flujos de audio
netmap-bound = { $count ->
    [one] { $count } vinculado
    [many] { $count } vinculados
   *[other] { $count } vinculados
}
netmap-flowing = Fluyendo
netmap-advertised-state = Anunciado
netmap-media-clock-stream = Flujo de reloj de medios
netmap-audio-stream = Flujo de audio
netmap-reaches = Llega hasta
netmap-passing-count = { $count ->
    [one] { $count } flujo en tránsito
    [many] { $count } flujos en tránsito
   *[other] { $count } flujos en tránsito
}
netmap-through = En tránsito
netmap-passing-through = Flujos en tránsito
netmap-sending = Enviando
netmap-receiving = Recibiendo
netmap-problems = Problemas
netmap-help-back = Haz clic en el fondo para volver a la vista general.
netmap-help-stream = Haz clic en un flujo para inspeccionarlo, o en el fondo para volver a la vista general.
netmap-help-clock = El reloj fluye desde el grandmaster, a través de cada switch, hasta todos los nodos del árbol. Una línea gris discontinua es un enlace en el que no funciona gPTP. Haz clic en un dispositivo o en su cable para inspeccionar su ruta del reloj; haz clic en el fondo para quitar la selección.
netmap-help-domains = Cada zona es un dominio de reloj. En un dominio gPTP, el reloj baja desde el grandmaster a través de los switches AVB. En un dominio PTP, los dispositivos ejecutan AVB Lite y siguen a su grandmaster de extremo a extremo, sincronizados dentro de 50 µs; una tarjeta discontinua representa los switches sin AVB tras los que algunos recurrieron a AVB Lite. Un dispositivo que no comparte su dominio con ningún otro tiene una zona propia.
netmap-help-media-clock = Solo flujos de reloj de medios (CRF), dibujados igual que los de audio: un cable para las tramas de cada flujo en cada enlace, con el color de su talker. Haz clic en un cable para inspeccionar su flujo, o en un dispositivo para ver sus flujos; haz clic en el fondo para quitar la selección.
netmap-help-audio = Cada cable representa las tramas de un flujo en un enlace, y entra y sale de cada switch que atraviesa. Un flujo multicast tiene un solo cable mientras los caminos hacia sus listeners van juntos, y se ramifica donde se separan; un flujo unicast, como los que puede enviar AVB Lite, tiene un cable propio desde el talker hasta cada listener. El color depende del talker: cada talker tiene un tono y cada uno de sus flujos es un matiz de ese tono en todas las ramas. Los puntos en movimiento indican que el audio fluye, incluso desde un talker que envía sin ningún listener conocido; una línea roja fija es una reserva fallida y una línea gris fija es un flujo anunciado sin ningún listener listo; ambas terminan donde termina la reserva. Los dispositivos de la columna central se conectan directamente al switch del grandmaster. Haz clic en un cable para inspeccionar su flujo, en un cable compartido para ver todos sus listeners, o en un dispositivo para ver sus flujos; haz clic en el fondo para quitar la selección.

## Connections

matrix-nothing-shown = No hay flujos que mostrar
matrix-nothing-shown-note = Cambia la búsqueda o los filtros para ver más flujos.
matrix-empty = No hay flujos que conectar
matrix-empty-note = Los flujos de talker y de listener se encuentran aquí cuando se han leído las entidades que los tienen.
matrix-all-streams = Todos los flujos
matrix-connectable-only = Ocultar lo que no puede conectarse
matrix-none-hidden = Todos los flujos mostrados pueden conectarse
matrix-hidden = { $count ->
    [one] { $count } flujo oculto
    [many] { $count } flujos ocultos
   *[other] { $count } flujos ocultos
}
matrix-own = Las salidas de una entidad no se conectan a sus propias entradas.
matrix-working = En curso.
matrix-waiting-change = Esperando el último cambio de esta entrada.
matrix-connected = Conectada y recibiendo. Haz clic para desconectar.
matrix-bound-waiting = Vinculada, esperando el flujo del talker. Haz clic para desconectar.
matrix-bound-failed = Vinculada, pero falló la reserva del talker: { $reason }. Haz clic para desconectar.
matrix-bound-formats-differ = Vinculada, pero los formatos difieren: el talker envía { $sent } y la entrada está configurada en { $set }. Haz clic para desconectar.
matrix-formats-match = Los formatos coinciden ({ $format }). Haz clic para conectar.
matrix-format-must-change = La entrada admite { $sent }, pero está configurada en { $set }, así que puede no sonar hasta que cambie su formato. Haz clic para elegir un formato.
matrix-format-input = Cambiar la entrada a
matrix-format-both = Cambiar ambos a
matrix-format-talker-busy = El talker transmite a otros listeners, así que su formato no cambia.
matrix-format-anyway = Conectar sin cambiar los formatos
matrix-incompatible = La entrada no admite { $sent }. Está configurada en { $set }.
matrix-other-domain = Siguen dominios de reloj distintos, así que ningún flujo puede pasar entre ellos.
matrix-group-none = Sin conectar. Expande para conectar los flujos uno a uno.
matrix-group-connected = { $count ->
    [one] { $count } conectado. Expande para verlo.
    [many] { $count } conectados. Expande para ver cada uno.
   *[other] { $count } conectados. Expande para ver cada uno.
}
matrix-outputs-expand = { $count ->
    [one] { $count } salida de flujo. Haz clic en la flecha para expandir o en el nombre para inspeccionarla.
    [many] { $count } salidas de flujo. Haz clic en la flecha para expandir o en el nombre para inspeccionarla.
   *[other] { $count } salidas de flujo. Haz clic en la flecha para expandir o en el nombre para inspeccionarla.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } salida de flujo. Haz clic en la flecha para contraer o en el nombre para inspeccionarla.
    [many] { $count } salidas de flujo. Haz clic en la flecha para contraer o en el nombre para inspeccionarla.
   *[other] { $count } salidas de flujo. Haz clic en la flecha para contraer o en el nombre para inspeccionarla.
}
matrix-inputs-expand = { $count ->
    [one] { $count } entrada de flujo. Haz clic en la flecha para expandir o en el nombre para inspeccionarla.
    [many] { $count } entradas de flujo. Haz clic en la flecha para expandir o en el nombre para inspeccionarla.
   *[other] { $count } entradas de flujo. Haz clic en la flecha para expandir o en el nombre para inspeccionarla.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } entrada de flujo. Haz clic en la flecha para contraer o en el nombre para inspeccionarla.
    [many] { $count } entradas de flujo. Haz clic en la flecha para contraer o en el nombre para inspeccionarla.
   *[other] { $count } entradas de flujo. Haz clic en la flecha para contraer o en el nombre para inspeccionarla.
}
matrix-stream-inspect = { $detail } Haz clic para inspeccionar { $entity }.
matrix-point = Señala una celda
matrix-point-note = para ver su talker y su listener y si sus formatos son compatibles.
matrix-legend-waiting = Vinculada, esperando el flujo
matrix-legend-trouble = Vinculada, algo falla
matrix-legend-open = Conexión posible
matrix-legend-change = Antes hay que cambiar el formato de la entrada
matrix-legend-incompatible = Formatos incompatibles
matrix-legend-other-domain = Otro dominio de reloj
matrix-talker-outputs = Salidas de talker
matrix-listener-inputs = Entradas de listener

common-thousands-separator = {" "}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Contado desde que se inició la entidad.
diag-stream-input = Entrada de flujo
diag-stream-output = Salida de flujo
diag-locked = { $count ->
    [0] no se enganchó
    [one] se enganchó una vez
    [many] se enganchó { $number } veces
   *[other] se enganchó { $number } veces
}
diag-lost-lock = { $count ->
    [0] no perdió el enganche
    [one] perdió el enganche una vez
    [many] perdió el enganche { $number } veces
   *[other] perdió el enganche { $number } veces
}
diag-frames-in = { $count ->
    [one] { $number } trama recibida
    [many] { $number } tramas recibidas
   *[other] { $number } tramas recibidas
}
diag-frames-out = { $count ->
    [one] { $number } trama enviada
    [many] { $number } tramas enviadas
   *[other] { $number } tramas enviadas
}
diag-media-locked = { $count ->
    [0] no se enganchó al reloj de medios
    [one] se enganchó al reloj de medios una vez
    [many] se enganchó al reloj de medios { $number } veces
   *[other] se enganchó al reloj de medios { $number } veces
}
diag-lost-media-lock = { $count ->
    [0] no perdió el enganche al reloj de medios
    [one] perdió el enganche al reloj de medios una vez
    [many] perdió el enganche al reloj de medios { $number } veces
   *[other] perdió el enganche al reloj de medios { $number } veces
}
diag-interrupted = { $count ->
    [0] no se interrumpió
    [one] se interrumpió una vez
    [many] se interrumpió { $number } veces
   *[other] se interrumpió { $number } veces
}
diag-out-of-sequence = { $count ->
    [one] { $number } trama fuera de secuencia
    [many] { $number } tramas fuera de secuencia
   *[other] { $number } tramas fuera de secuencia
}
diag-media-resets = { $count ->
    [one] { $number } reinicio de medios
    [many] { $number } reinicios de medios
   *[other] { $number } reinicios de medios
}
diag-timestamps-uncertain = { $count ->
    [0] sin marcas de tiempo inciertas
    [one] marcas de tiempo inciertas una vez
    [many] marcas de tiempo inciertas { $number } veces
   *[other] marcas de tiempo inciertas { $number } veces
}
diag-no-timestamp = { $count ->
    [one] { $number } trama sin marca de tiempo
    [many] { $number } tramas sin marca de tiempo
   *[other] { $number } tramas sin marca de tiempo
}
diag-unsupported-format = { $count ->
    [one] { $number } trama en un formato no admitido
    [many] { $number } tramas en un formato no admitido
   *[other] { $number } tramas en un formato no admitido
}
diag-late = { $count ->
    [one] { $number } trama con retraso
    [many] { $number } tramas con retraso
   *[other] { $number } tramas con retraso
}
diag-early = { $count ->
    [one] { $number } trama con adelanto
    [many] { $number } tramas con adelanto
   *[other] { $number } tramas con adelanto
}
diag-started = { $count ->
    [0] no se inició
    [one] se inició una vez
    [many] se inició { $number } veces
   *[other] se inició { $number } veces
}
diag-stopped = { $count ->
    [0] no se detuvo
    [one] se detuvo una vez
    [many] se detuvo { $number } veces
   *[other] se detuvo { $number } veces
}
diag-reservation-failed = falló la reserva del talker: { $reason }
diag-latency = { $microseconds } µs de latencia acumulada

## AVB Lite

lite-active = Activo
lite-active-untagged = Activo, sin etiquetar
lite-active-vlan = Activo, VLAN { $vlan }
lite-capable = Compatible
lite-mode = Modo
lite-mode-capable = AVB (compatible con AVB Lite)
lite-because = Motivo
lite-fallback-none = sin especificar
lite-fallback-endpoint = llegó la declaración de otro endpoint, así que no hay ningún switch AVB entre ambos
lite-fallback-unanswered = nueve solicitudes de peer delay quedaron sin respuesta
lite-fallback-responders = respondieron dos o más a una misma solicitud de peer delay, así que el switch no es AVB
lite-fallback-configured = configurado por el operador o por un controlador
lite-fallback-other = un motivo que el perfil no define
lite-other-profile = Otro perfil
lite-ptp-domain = { $profile }, dominio { $domain }
lite-offset = Offset
lite-offset-from = { $offset } respecto a { $grandmaster }
lite-media-vlan = VLAN de medios
lite-untagged = Sin etiquetar
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Hasta { $count } listener por flujo, luego multicast
    [many] Hasta { $count } listeners por flujo, luego multicast
   *[other] Hasta { $count } listeners por flujo, luego multicast
}
lite-link = Enlace
lite-bandwidth = Ancho de banda
lite-egress-of = { $used } de { $link }, { $share }
lite-egress-of-assumed = { $used } de { $link }, { $share }, suponiendo un enlace de gigabit
lite-egress-reported = Según la cuenta de flujos admitidos que lleva la entidad.
lite-egress-worked-out = A partir de los formatos de sus salidas de flujo conectadas.
lite-alarm-offset = Offset PTP de { $offset }, por encima de los 50 µs que admite AVB Lite
lite-alarm-egress = Tráfico de salida al { $share } del enlace, por encima del { $limit } que pueden ocupar los flujos

## AVB Wireless

wireless-station = Estación
wireless-access-point = Punto de acceso
wireless-role = Rol
wireless-mode = Modo
wireless-time = Tiempo
wireless-mode-a-ftm = Mode A, 802.1AS sobre FTM
wireless-mode-a-tm = Mode A, 802.1AS sobre TM
wireless-mode-b = Mode B, a partir de las balizas
wireless-no-time = Sin sincronización
wireless-other-mode = Un modo que el perfil no define
wireless-locked = Enganchado
wireless-holdover = En holdover
wireless-not-locked = No enganchado
wireless-row-locked = Estación, enganchada
wireless-row-holdover = Estación, en holdover
wireless-row-not-locked = Estación, no enganchada
wireless-row-access-point = { $count ->
    [one] Punto de acceso, { $count } estación
    [many] Punto de acceso, { $count } estaciones
   *[other] Punto de acceso, { $count } estaciones
}
wireless-link = Enlace
wireless-channel = canal { $channel }
wireless-not-known = Desconocido
wireless-signal = Señal
wireless-rate = Velocidad
wireless-ftm-valid = { $share } válidas
wireless-rtt = ida y vuelta { $rtt }
wireless-bursts = { $count ->
    [one] ráfagas de { $count } trama
    [many] ráfagas de { $count } tramas
   *[other] ráfagas de { $count } tramas
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = el punto de acceso concede ráfagas FTM de un tamaño distinto de tres o dos tramas
wireless-reason-measurement = ni FTM ni TM con el punto de acceso
wireless-reason-signaling = ningún mensaje Signaling compatible con gPTP del punto de acceso
wireless-reason-other = un motivo que el perfil no define
wireless-servo = Error del servo
wireless-stations = Estaciones
wireless-station-count = { $count ->
    [one] { $count } estación
    [many] { $count } estaciones
   *[other] { $count } estaciones
}
wireless-no-ftm = sin FTM
wireless-unserved = Listeners no atendidos
wireless-frame-budget = Presupuesto de tramas
wireless-frame-use = { $used } de { $budget } tramas por segundo
wireless-stream-frames = Tramas de flujo
wireless-frames-of = { $readdressed } a estaciones, { $unmapped } sin listener, { $dropped } descartadas, { $restored } desde estaciones
wireless-class-a-allowed = Permitida
wireless-class-a-not-allowed = No permitida
wireless-alarm-not-locked = Tiempo Wi-Fi no enganchado al punto de acceso
wireless-alarm-holdover = Tiempo Wi-Fi en holdover, perdió el enganche con el punto de acceso
wireless-alarm-unserved = { $count ->
    [one] { $count } listener en el puerto Wi-Fi sin atender, por encima del límite de unicast
    [many] { $count } listeners en el puerto Wi-Fi sin atender, por encima del límite de unicast
   *[other] { $count } listeners en el puerto Wi-Fi sin atender, por encima del límite de unicast
}

## Log

log-all = Todo
log-warnings = Advertencias
log-pause = Pausar
log-resume = Reanudar
log-clear = Borrar
log-empty = Aquí aparecen todas las tramas ATDECC que triib envía y recibe, de la más reciente a la más antigua.
log-none-match = Ninguna trama guardada coincide con el filtro.
log-frames = { $count ->
    [one] { $count } trama
    [many] { $count } tramas
   *[other] { $count } tramas
}
log-shown-of = { $shown } de { $all } tramas
log-sent = Enviada
log-heard = Recibida
log-not-decoded = Sin decodificar
log-warning-short = Su control_data_length indica { $missing } octetos más allá del final de la trama.
log-warning-undecodable = No se puede decodificar: { $error }.
log-warning-long-acmp = Está en la forma larga de ACMP, que una entidad Milan no puede enviar (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Mapeo de canales
mapping-inputs = Entradas
mapping-outputs = Salidas
mapping-port = puerto { $number }
mapping-fixed = fijo
mapping-not-read = Aún sin leer.
mapping-no-clusters = Sin clústeres.
mapping-no-streams = Sin flujos de audio.
mapping-none = Sin mapeos.
mapping-not-mapped = Sin mapear
mapping-cluster-numbered = Clúster { $index }

## Presets

presets-note = Un preset guarda las fuentes de reloj, las frecuencias de muestreo, los formatos de flujo, los controles y las conexiones de cada entidad. Al recuperarlo se cambia lo que difiera.
presets-none = Aún no hay presets guardados.
presets-connections = { $count ->
    [one] { $count } conexión
    [many] { $count } conexiones
   *[other] { $count } conexiones
}
presets-recall = Recuperar
presets-delete = Eliminar
presets-no-place = No hay dónde guardar los presets: no se conoce la carpeta personal.
presets-undeletable = No se pudo eliminar { $path }: { $error }.
presets-saved = { $count ->
    [one] Se guardó «{ $name }» con { $count } entidad.
    [many] Se guardó «{ $name }» con { $count } entidades.
   *[other] Se guardó «{ $name }» con { $count } entidades.
}
presets-nothing-differs = Nada difiere de «{ $name }».
presets-recalling = { $count ->
    [one] Recuperando «{ $name }»: { $count } cambio.
    [many] Recuperando «{ $name }»: { $count } cambios.
   *[other] Recuperando «{ $name }»: { $count } cambios.
}
presets-missing = { $report } Ausentes o sin leer: { $missing }.
presets-deleted = Se eliminó «{ $name }».
presets-host-note = También guarda las entidades propias de este equipo, y las vuelve a iniciar al recuperarlo.
presets-host-endpoints = { $count } en este equipo
presets-starting-host = Iniciando las entidades de este equipo para «{ $name }»; el resto seguirá cuando vuelvan.

## Controls

control-numbered = Control { $index }
control-not-shown = No se muestra aquí
control-option = Opción { $number }

## Network errors

network-permission = triib necesita permiso para enviar y recibir tramas Ethernet sin procesar.
network-needs-npcap = triib necesita Npcap para enviar y recibir tramas Ethernet sin procesar.
network-npcap-administrators = Npcap solo permite a los administradores enviar y recibir tramas Ethernet sin procesar. Ejecuta triib como administrador o vuelve a instalar Npcap sin su opción de solo administradores.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own entities

host-add-entity = Añadir entidad
host-new-entity = Entidad del equipo { $number }
host-failed = No se pudo añadir a este equipo: { $reason }
host-needs-clock = Las entidades propias de este equipo necesitan una interfaz cableada con reloj de hardware PTP
host-no-ptp4l = ptp4l no responde, así que los flujos de este equipo no pueden mantener la hora gPTP
host-starting = Iniciando
host-starting-entity = Iniciando { $name }
host-setting-up = Preparando { $interface } para AVB
host-no-link = Sin enlace en { $interface }
host-starting-ptp4l = Iniciando ptp4l
host-no-linuxptp = linuxptp no está instalado, así que los flujos de este equipo no pueden mantener la hora gPTP
host-waiting-network = Esperando la red
host-elsewhere = triib-endpointd se ejecuta en { $interface } para otro usuario o como root, así que las entidades de este equipo se ejecutan allí, no aquí
host-foreign-mrp = Otro programa emite declaraciones MSRP o MVRP en { $interface } desde la dirección de este equipo y puede retirar lo que necesitan los flujos de este equipo
host-alarm-foreign-mrp = Otro programa de este equipo emite declaraciones MSRP o MVRP en { $interface }
host-state = Estado
host-streaming = Transmitiendo
host-waiting = Esperando un listener
host-listening = Escuchando
host-bound = Vinculado, esperando al talker
host-unbound = Sin vincular
host-audio-from = Audio desde
host-audio-to = Audio hacia
host-channels = Canales
host-virtual-device = Dispositivo virtual
host-add-stream = Añadir flujo
host-add-audio-input = Entrada de audio
host-add-audio-output = Salida de audio
host-add-clock-input = Entrada de reloj de medios
host-add-clock-output = Salida de reloj de medios
host-remove-stream = Quitar flujo
host-silence = Silencio
host-tone = Tono de prueba
host-nowhere = Ninguna parte
host-default-device = Dispositivo predeterminado
host-remove = Quitar de este equipo

mac-audio-device = Dispositivo de audio
mac-no-audio-device = Ninguno mientras no haya un flujo conectado
mac-entity-note = macOS ejecuta esta entidad. Mientras alguno de sus flujos esté conectado, macOS crea el dispositivo de audio por el que las apps reproducen y graban, con tantos canales en cada sentido como fije la configuración.
mac-open-audio-midi-setup = Abrir Configuración de Audio MIDI
mac-avb-off = AVB está desactivado en { $interface }, así que este Mac no tiene una entidad AVB propia. Actívalo en el explorador de dispositivos de red de Configuración de Audio MIDI.

listen-section = Escucha
listen-start = Escuchar
listen-stop = Dejar de escuchar
listen-source = Escucha
listen-received = Lo que recibe
listen-sent = Lo que envía por el flujo { $number } ({ $channels } canales)
listen-sent-needs-macos = Para escuchar lo que envía este Mac se necesita macOS 14.2 o posterior.
listen-channels = Canales
listen-channel = Canal { $number }
listen-channel-pair = Canales { $first }–{ $last }
listen-output = Salida
listen-default-output = Salida predeterminada
listen-gain = Ganancia
listen-gain-db = { $db } dB
listen-starting = Iniciando…
listen-waiting-input = Esperando a { $device }, que macOS crea mientras alguno de los flujos de la entidad esté conectado.
listen-waiting-output = Esperando a { $device }.
listen-playing = Reproduciendo, con picos de { $db } dB
listen-playing-silent = Reproduciendo, en silencio
