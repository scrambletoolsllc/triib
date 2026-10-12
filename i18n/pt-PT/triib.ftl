# triib's interface text in European Portuguese. The source is
# i18n/en/triib.ftl; term choices are in docs/glossary/pt-PT.md.

## Language

language-name = Português (Portugal)

## Common

common-close = Fechar
common-more = Mais
common-keep-toolbar-shown = Manter a barra de ferramentas visível
common-auto-hide-toolbar = Ocultar automaticamente a barra de ferramentas

## Settings

settings-title = Definições
settings-general = Geral
settings-appearance = Aspeto
settings-language = Idioma
settings-language-system = Predefinição do sistema: { $language }
settings-language-note = Os campos de texto utilizam o idioma de introdução do sistema.
settings-appearance-system = Sistema
settings-appearance-light = Claro
settings-appearance-dark = Escuro
settings-colors = Cores
settings-system-accent = Utilizar a cor de destaque do sistema
settings-accent-picked = As cores do triib partem da cor abaixo.
settings-accent-omarchy = Do tema do Omarchy, { $theme }.
settings-accent-desktop = Da cor de destaque do ambiente de trabalho.
settings-accent-none = O ambiente de trabalho não tem cor de destaque, pelo que é utilizada a cor abaixo.
settings-motion = Movimento
settings-animations = Animações
settings-animations-note = Efeitos de mola e deslizamento quando algo muda.
settings-animations-reduced = O ambiente de trabalho pede movimento reduzido, pelo que o triib fica imóvel.
settings-outside-control = Controlo externo
settings-allow-outside-control = Permitir controlo externo
settings-allow-outside-control-note = Agentes como o Claude Code podem ver e alterar a sua rede AVB, através do triib --mcp. O triib pergunta antes de cada novo agente.
settings-allowed-agents = Permitidos: { $agents }
settings-forget-agents = Esquecer
agent-prompt-title = Permitir que { $agent } controle o triib?
agent-prompt-body = { $agent } pede para usar o controlo externo do triib: ver as entidades da sua rede e alterá-las, ligando fluxos e definindo formatos, relógios e nomes. Pode desativar o controlo externo nas Definições.
agent-prompt-allow = Permitir
agent-prompt-deny = Não permitir

common-cancel = Cancelar
common-save = Guardar
common-not-set = Não definido
common-unnamed = Sem nome
common-none = Nenhum
common-mac-address = Endereço MAC
common-list-separator = {", "}

## Network interfaces

interface-up = ativa
interface-link-down = link em baixo
interface-wireless = sem fios
interface-hardware-clock = relógio de hardware
interface-hardware-clock-named = relógio de hardware { $clock }
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Escolher uma interface
toolbar-interface = Interface de rede
toolbar-show-virtual = Mostrar interfaces virtuais
toolbar-hide-virtual = Ocultar interfaces virtuais
toolbar-connections = Ligações
toolbar-network = Rede
toolbar-entities = Entidades
toolbar-rediscover = Pedir a todas as entidades que se anunciem
toolbar-rescan = Limpar e voltar a analisar todas as entidades
toolbar-search = Procurar entidades e fluxos
toolbar-presets = Presets
toolbar-log = Registo
toolbar-inspector = Inspetor
toolbar-settings = Definições

## The network's state, in place of a view

state-no-interface = Sem interface
state-no-interface-note = Escolha a interface da rede AVB para descobrir entidades.
state-starting = A iniciar
state-starting-note = A abrir { $interface }.
state-listening = À escuta
state-listening-note = As entidades em { $interface } aparecem aqui à medida que se anunciam.
state-permission-needed = Permissão necessária
state-npcap-needed = Npcap necessário
state-get-npcap = Obter o Npcap
state-copy-command = Copiar o comando
state-cannot-use = Não é possível utilizar { $interface }
state-try-again = Tentar novamente

## Entity list

entities-none-yet = Ainda sem entidades
entities-none-yet-note = Todas as entidades da rede, com as suas funções, classes SR e relógio.

## Inspector

inspector-title = Inspetor
inspector-entity = Entidade
inspector-streams = Fluxos
inspector-controls = Controlos
inspector-diagnostics = Diagnóstico
inspector-descriptors = Descritores
inspector-select = Selecione uma entidade para ver os seus detalhes.
inspector-offline = { $entity } está offline.
inspector-rename = Mudar o nome
inspector-name = Nome
inspector-identify = Identificar
inspector-model-not-read = O modelo de entidade não foi lido.
inspector-no-streams = Sem fluxos.
inspector-no-controls = Nenhum controlo para mostrar.
inspector-no-diagnostics = Nenhuma interface ou contador indicado.
inspector-reading = A ler descritores, { $count } até agora.
inspector-read-failed = Não foi possível ler o modelo de entidade: { $reason }.

entity-section = Entidade
entity-name = Nome
entity-group = Grupo
entity-product = Produto
entity-firmware = Firmware
entity-serial-number = Número de série
entity-configuration = Configuração
entity-configuration-of = { $name } ({ $number } de { $count })
entity-milan = Milan
entity-media-clock = Relógio de média
entity-clock-domain = Domínio de relógio
entity-sampling-rate = Frequência de amostragem
clock-source-numbered = Fonte { $index }
rate-pull = pull { $pull }

stream-inputs = Entradas de fluxo
stream-outputs = Saídas de fluxo
stream-max-transit-time = Tempo de trânsito máx. { $time }

avb-interfaces = Interfaces AVB
avb-interface = Interface
avb-interface-clock-identity = Identidade do relógio
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domínio { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Em execução
avb-interface-none-reported = Nenhum indicado
avb-interface-path = Caminho
avb-interface-own-grandmaster = É o seu próprio grandmaster
avb-interface-hops = { $count ->
    [one] A { $count } salto do grandmaster
    [many] A { $count } saltos do grandmaster
   *[other] A { $count } saltos do grandmaster
}
avb-interface-link-up = Link ativo
avb-interface-link-down = Link em baixo
avb-interface-grandmaster-changes = Mudanças de grandmaster
avb-interface-frames-sent = Tramas enviadas
avb-interface-frames-received = Tramas recebidas
avb-interface-crc-errors = Erros de CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [one] { $count } tipo de descritor
    [many] { $count } tipos de descritor
   *[other] { $count } tipos de descritor
}
tree-clock = Relógio
tree-clock-source-from = { $kind }, de { $location } { $index }
tree-clock-domain-using = Utiliza { $source }
tree-clusters = { $count ->
    [one] { $count } cluster
    [many] { $count } clusters
   *[other] { $count } clusters
}
tree-maps = { $count ->
    [one] { $count } mapa
    [many] { $count } mapas
   *[other] { $count } mapas
}

advert-not-advertised = Não anunciado
advert-identity = Identidade
advert-entity-id = ID da entidade
advert-entity-model = Modelo de entidade
advert-roles = Funções
advert-talker = Talker
advert-listener = Listener
advert-clock = Relógio
advert-btc = BTC
advert-gptp-domain = Domínio gPTP
advert-sr-classes = Classes SR
advert-indexes = Índices do modelo de entidade
advert-identify-control = Controlo de identificação
advert-avb-interface = Interface AVB
advert-advertising = Anúncio
advert-valid-time = Tempo de validade
advert-available-index = Índice de disponibilidade
advert-association = Associação
advert-capabilities = Capacidades

## Status bar

status-entities = { $count ->
    [one] { $count } entidade
    [many] { $count } entidades
   *[other] { $count } entidades
}
status-not-discovering = Descoberta inativa
status-discovering = A descobrir
status-discovering-as = A descobrir como { $controller }
status-stopped = Parado devido a um erro
status-alarm = Alarme
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } e mais { $count }

## Descriptions of entities, shared by the views

role-talker = talker { $count }
role-listener = listener { $count }
role-controller = controlador
role-none = sem funções
classes-a-and-b = A e B
clock-no-gptp = Sem gPTP

read-not-read = Não lido
read-reading = A ler, { $count } até agora
read-ready-unreadable = { $count ->
    [one] Pronto, { $count } ilegível
    [many] Pronto, { $count } ilegíveis
   *[other] Pronto, { $count } ilegíveis
}
read-ready-cached = Pronto, da cache
read-ready = Pronto
read-failed = Falhou: { $reason }

milan-no = Não
milan-before-1-3 = anterior à 1.3
milan-certified = { $version }, certificado { $certification }
milan-not-certified = { $version }, não certificado

outcome-status = estado { $status }
outcome-no-response = sem resposta
outcome-not-possible = não é possível
outcome-connect = Não foi possível ligar { $talker } a { $listener }: { $reason }.
outcome-disconnect = Não foi possível desligar { $listener }: { $reason }.
outcome-identify = Não foi possível identificar { $entity }: { $reason }.
outcome-rename = Não foi possível mudar o nome de { $what } para «{ $name }»: { $reason }.
outcome-rename-group = Não foi possível mudar o nome do grupo de { $entity } para «{ $name }»: { $reason }.
outcome-format-streaming = Não foi possível alterar o formato de { $stream }: está a transmitir. Desligue-o primeiro.
outcome-format = Não foi possível alterar o formato de { $stream }: { $reason }.
outcome-sampling-rate = Não foi possível alterar a frequência de amostragem de { $entity }: { $reason }.
outcome-clock-source = Não foi possível alterar a fonte de relógio de { $entity }: { $reason }.
outcome-map = Não foi possível mapear o canal em { $entity }: { $reason }.
outcome-unmap = Não foi possível remover o mapeamento do canal em { $entity }: { $reason }.
outcome-control = Não foi possível definir «{ $control }» em { $entity }: { $reason }.
outcome-control-numbered = Não foi possível definir o controlo { $index } em { $entity }: { $reason }.

stream-not-connected = Sem ligação
stream-from = De { $stream }
stream-from-receiving = De { $stream }, a receber
stream-from-waiting = De { $stream }, a aguardar o talker
stream-from-failed = De { $stream }, a reserva do talker falhou: { $reason }
stream-sending-to = A enviar para { $destination }

failure-no-response = não respondeu
failure-refused = recusou com { $status }
failure-malformed = não foi possível descodificar a resposta
failure-on-this-computer = é executada neste computador; leia-a a partir de outro

msrp-failure-1 = largura de banda insuficiente
msrp-failure-2 = recursos insuficientes no switch
msrp-failure-3 = largura de banda insuficiente para a classe de tráfego
msrp-failure-4 = ID de fluxo em uso por outro talker
msrp-failure-5 = endereço de destino já em uso
msrp-failure-6 = substituído por um fluxo de rank superior
msrp-failure-7 = a latência indicada mudou
msrp-failure-8 = a porta de saída não é compatível com AVB
msrp-failure-9 = utilize outro endereço de destino
msrp-failure-10 = sem recursos MSRP
msrp-failure-11 = sem recursos MMRP
msrp-failure-12 = não é possível guardar o endereço de destino
msrp-failure-13 = a prioridade não é de uma classe SR
msrp-failure-14 = tramas demasiado grandes para o meio
msrp-failure-15 = limite de fan-in da porta atingido
msrp-failure-16 = o primeiro valor mudou para um fluxo registado
msrp-failure-17 = VLAN bloqueada na porta de saída
msrp-failure-18 = etiquetagem VLAN desativada na porta de saída
msrp-failure-19 = a prioridade da classe SR não coincide
msrp-failure-unknown = motivo desconhecido
msrp-failure-at = { $reason }, no switch { $bridge }

## Entity list columns

column-vendor = Fabricante
column-model = Modelo
column-state = Estado
column-entity-model-id = ID do modelo de entidade
column-talker-streams = Fluxos de talker
column-listener-streams = Fluxos de listener
column-avb-lite = AVB Lite
column-egress = Tráfego de saída
column-wireless = Sem fios

## Settings file

settings-no-place = Não há onde guardar as definições: a pasta pessoal é desconhecida.
settings-unusable = Não foi possível utilizar { $path }: { $error }.
settings-unsaved = Não foi possível guardar { $path }: { $error }.

column-remove = Remover coluna
column-move-left = Mover para a esquerda
column-move-right = Mover para a direita
column-add = Adicionar uma coluna
common-percent = { $value }%

## Network view

netmap-empty = Ainda não há rede para mostrar
netmap-empty-note = As entidades aparecem aqui depois de lidas e de indicarem onde se encontram na árvore gPTP.
netmap-focus-clock-path = o caminho do relógio de { $name }
netmap-focus-streams = os fluxos de { $name }
netmap-showing = A mostrar { $what }
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
netmap-show-map = Mostrar o mapa
netmap-show-details = Mostrar os detalhes
stream-numbered = Fluxo { $index }
netmap-bridge = Switch
netmap-access-point = Ponto de acesso
netmap-device = Dispositivo
netmap-this-computer = Este computador
netmap-connected = Ligado
netmap-advertised = Anunciado, nenhum listener pronto
netmap-advertised-off-tree = Anunciado, nenhum listener pronto ({ $listener } não está na árvore gPTP)
netmap-failed-at = A reserva falhou em { $bridge }: { $reason }
netmap-failed = A reserva falhou: { $reason }
netmap-no-bridge-on = Nenhum switch detetado em { $interface }
netmap-cannot-listen-on = Não é possível escutar o gPTP em { $interface }
netmap-on-this-computer = Neste computador
netmap-path-not-reported = Caminho não indicado
netmap-gptp-not-reported = gPTP não indicado
netmap-off-tree = Fora da árvore gPTP
netmap-off-ptp = Fora da árvore PTP
netmap-not-lite = Fora do AVB Lite
netmap-lite-not-reported = AVB Lite não indicado
netmap-synced = Sincronizado
netmap-not-synced = Não sincronizado
netmap-triib-on = triib em { $interface }
netmap-through-count = { $count } em trânsito
netmap-out = { $count } de saída
netmap-in = { $count } de entrada
netmap-failed-count = { $count } com falha
netmap-advertised-only = Apenas anunciado
netmap-failed-state = Com falha
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-apart-own-grandmaster = Fora da árvore gPTP: é o seu próprio grandmaster
netmap-apart-no-path = O caminho não foi indicado; segue o grandmaster { $grandmaster }
netmap-apart-unreported = Não indicou o seu estado gPTP
netmap-apart-no-neighbor = Nenhum switch detetado na interface deste computador
netmap-apart-cannot-listen = Este computador não consegue escutar o gPTP na sua interface
netmap-apart-on-this-computer = É executada neste computador; leia-a a partir de outro computador para ver o estado do gPTP
netmap-apart-not-lite = Não executa o AVB Lite, pelo que não segue o grandmaster
netmap-apart-lite-unreported = Não indica nada sobre o AVB Lite, pelo que não se sabe o que segue
netmap-clock-tree = Árvore de relógio
netmap-no-grandmaster = Nenhum grandmaster detetado
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Requer atenção
netmap-nodes-below = Nós abaixo
netmap-bridges-below = Switches abaixo
netmap-clock-path = Caminho do relógio
netmap-hops = Saltos até ao grandmaster
netmap-link-delay = Atraso do link
netmap-bridge-port = Porta do switch
netmap-link-drops = Quedas do link
netmap-synced-to-grandmaster = Sincronizado com o grandmaster
netmap-host-no-gptp = Não sincronizado: este computador não executa gPTP
netmap-link-no-gptp = Não sincronizado: o gPTP não está ativo no seu link
netmap-ptp-offset-high = Não sincronizado: { $offset } em relação ao grandmaster, acima dos 50 µs que o AVB Lite permite
netmap-ptp-no-offset = Não sincronizado: não mediu nenhum offset em relação ao grandmaster
netmap-audio = Áudio
netmap-media-clock-streams = Fluxos de relógio de média
netmap-audio-streams = Fluxos de áudio
netmap-bound = { $count ->
    [one] { $count } vinculado
    [many] { $count } vinculados
   *[other] { $count } vinculados
}
netmap-flowing = A fluir
netmap-advertised-state = Anunciado
netmap-media-clock-stream = Fluxo de relógio de média
netmap-audio-stream = Fluxo de áudio
netmap-reaches = Chega até
netmap-passing-count = { $count ->
    [one] { $count } fluxo em trânsito
    [many] { $count } fluxos em trânsito
   *[other] { $count } fluxos em trânsito
}
netmap-through = Em trânsito
netmap-passing-through = Fluxos em trânsito
netmap-sending = A enviar
netmap-receiving = A receber
netmap-problems = Problemas
netmap-help-back = Clique no fundo para voltar à vista geral.
netmap-help-stream = Clique num fluxo para o inspecionar ou no fundo para voltar à vista geral.
netmap-help-ptp = No AVB Lite, o relógio flui de ponta a ponta, do grandmaster até cada dispositivo, passando por switches que não participam, pelo que nenhum é mostrado. Um dispositivo está sincronizado enquanto segue o grandmaster com uma margem de 50 µs. Clique num dispositivo ou no seu fio para inspecionar o relógio; clique no fundo para limpar a seleção.
netmap-help-clock = O relógio flui do grandmaster, passando por cada switch, até todos os nós da árvore. Uma linha cinzenta tracejada é um link onde o gPTP não está ativo. Clique num dispositivo ou no seu fio para inspecionar o caminho do relógio; clique no fundo para limpar a seleção.
netmap-help-media-clock = Apenas fluxos de relógio de média (CRF), desenhados como os de áudio: um fio por fluxo, com a cor do talker. Clique num fio para inspecionar o seu fluxo ou num dispositivo para ver os seus fluxos; clique no fundo para limpar a seleção.
netmap-help-audio = Cada fluxo tem o seu próprio fio, que entra e sai de cada switch que atravessa. A cor depende do talker: cada talker tem um matiz, e os seus fluxos são tons desse matiz. Pontos em movimento indicam que o áudio está a fluir; uma linha vermelha parada é uma reserva falhada e uma linha cinzenta parada é um fluxo anunciado sem nenhum listener pronto; ambas terminam onde a reserva termina. Os dispositivos na coluna do meio ligam-se diretamente ao switch do grandmaster. Clique num fio para inspecionar o seu fluxo ou num dispositivo para ver os seus fluxos; clique no fundo para limpar a seleção.

## Connections

matrix-nothing-shown = Nenhum fluxo para mostrar
matrix-nothing-shown-note = Altere a pesquisa ou os filtros para ver mais fluxos.
matrix-empty = Nenhum fluxo para ligar
matrix-empty-note = Os fluxos de talker e de listener encontram-se aqui depois de lidas as entidades que os têm.
matrix-all-streams = Todos os fluxos
matrix-connectable-only = Ocultar o que não pode ser ligado
matrix-none-hidden = Todos os fluxos mostrados podem ser ligados
matrix-hidden = { $count ->
    [one] { $count } fluxo oculto
    [many] { $count } fluxos ocultos
   *[other] { $count } fluxos ocultos
}
matrix-own = As saídas de uma entidade não se ligam às suas próprias entradas.
matrix-working = Em curso.
matrix-waiting-change = A aguardar a última alteração nesta entrada.
matrix-connected = Ligada e a receber. Clique para desligar.
matrix-bound-waiting = Vinculada, a aguardar o fluxo do talker. Clique para desligar.
matrix-bound-failed = Vinculada, mas a reserva do talker falhou: { $reason }. Clique para desligar.
matrix-bound-formats-differ = Vinculada, mas os formatos diferem: o talker envia { $sent } e a entrada está definida para { $set }. Clique para desligar.
matrix-formats-match = Os formatos coincidem ({ $format }). Clique para ligar.
matrix-format-must-change = A entrada aceita { $sent }, mas está definida para { $set }, pelo que pode não reproduzir até o formato mudar. Clique para escolher um formato.
matrix-format-input = Mudar a entrada para
matrix-format-both = Mudar ambos para
matrix-format-talker-busy = O talker transmite para outros listeners, pelo que o seu formato se mantém.
matrix-format-anyway = Ligar sem mudar os formatos
matrix-incompatible = A entrada não aceita { $sent }. Está definida para { $set }.
matrix-group-none = Sem ligação. Expanda para ligar os fluxos um a um.
matrix-group-connected = { $count ->
    [one] { $count } ligado. Expanda para o ver.
    [many] { $count } ligados. Expanda para ver cada um.
   *[other] { $count } ligados. Expanda para ver cada um.
}
matrix-outputs-expand = { $count ->
    [one] { $count } saída de fluxo. Clique na seta para expandir ou no nome para a inspecionar.
    [many] { $count } saídas de fluxo. Clique na seta para expandir ou no nome para a inspecionar.
   *[other] { $count } saídas de fluxo. Clique na seta para expandir ou no nome para a inspecionar.
}
matrix-outputs-collapse = { $count ->
    [one] { $count } saída de fluxo. Clique na seta para recolher ou no nome para a inspecionar.
    [many] { $count } saídas de fluxo. Clique na seta para recolher ou no nome para a inspecionar.
   *[other] { $count } saídas de fluxo. Clique na seta para recolher ou no nome para a inspecionar.
}
matrix-inputs-expand = { $count ->
    [one] { $count } entrada de fluxo. Clique na seta para expandir ou no nome para a inspecionar.
    [many] { $count } entradas de fluxo. Clique na seta para expandir ou no nome para a inspecionar.
   *[other] { $count } entradas de fluxo. Clique na seta para expandir ou no nome para a inspecionar.
}
matrix-inputs-collapse = { $count ->
    [one] { $count } entrada de fluxo. Clique na seta para recolher ou no nome para a inspecionar.
    [many] { $count } entradas de fluxo. Clique na seta para recolher ou no nome para a inspecionar.
   *[other] { $count } entradas de fluxo. Clique na seta para recolher ou no nome para a inspecionar.
}
matrix-stream-inspect = { $detail } Clique para inspecionar { $entity }.
matrix-point = Aponte para uma célula
matrix-point-note = para ver o talker e o listener e se os formatos são compatíveis.
matrix-legend-waiting = Vinculada, a aguardar o fluxo
matrix-legend-trouble = Vinculada, há um problema
matrix-legend-open = Ligação possível
matrix-legend-change = O formato da entrada tem de mudar primeiro
matrix-legend-incompatible = Formatos incompatíveis
matrix-talker-outputs = Saídas de talker
matrix-listener-inputs = Entradas de listener

common-thousands-separator = {" "}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Contado desde o arranque da entidade.
diag-stream-input = Entrada de fluxo
diag-stream-output = Saída de fluxo
diag-locked = { $count ->
    [0] não entrou em lock
    [one] entrou em lock uma vez
    [many] entrou em lock { $number } vezes
   *[other] entrou em lock { $number } vezes
}
diag-lost-lock = { $count ->
    [0] não perdeu o lock
    [one] perdeu o lock uma vez
    [many] perdeu o lock { $number } vezes
   *[other] perdeu o lock { $number } vezes
}
diag-frames-in = { $count ->
    [one] { $number } trama recebida
    [many] { $number } tramas recebidas
   *[other] { $number } tramas recebidas
}
diag-frames-out = { $count ->
    [one] { $number } trama enviada
    [many] { $number } tramas enviadas
   *[other] { $number } tramas enviadas
}
diag-media-locked = { $count ->
    [0] não entrou em lock de média
    [one] entrou em lock de média uma vez
    [many] entrou em lock de média { $number } vezes
   *[other] entrou em lock de média { $number } vezes
}
diag-lost-media-lock = { $count ->
    [0] não perdeu o lock de média
    [one] perdeu o lock de média uma vez
    [many] perdeu o lock de média { $number } vezes
   *[other] perdeu o lock de média { $number } vezes
}
diag-interrupted = { $count ->
    [0] não foi interrompido
    [one] foi interrompido uma vez
    [many] foi interrompido { $number } vezes
   *[other] foi interrompido { $number } vezes
}
diag-out-of-sequence = { $count ->
    [one] { $number } trama fora de sequência
    [many] { $number } tramas fora de sequência
   *[other] { $number } tramas fora de sequência
}
diag-media-resets = { $count ->
    [one] { $number } reinício de média
    [many] { $number } reinícios de média
   *[other] { $number } reinícios de média
}
diag-timestamps-uncertain = { $count ->
    [0] sem timestamps incertos
    [one] timestamps incertos uma vez
    [many] timestamps incertos { $number } vezes
   *[other] timestamps incertos { $number } vezes
}
diag-no-timestamp = { $count ->
    [one] { $number } trama sem timestamp
    [many] { $number } tramas sem timestamp
   *[other] { $number } tramas sem timestamp
}
diag-unsupported-format = { $count ->
    [one] { $number } trama num formato não suportado
    [many] { $number } tramas num formato não suportado
   *[other] { $number } tramas num formato não suportado
}
diag-late = { $count ->
    [one] { $number } trama atrasada
    [many] { $number } tramas atrasadas
   *[other] { $number } tramas atrasadas
}
diag-early = { $count ->
    [one] { $number } trama adiantada
    [many] { $number } tramas adiantadas
   *[other] { $number } tramas adiantadas
}
diag-started = { $count ->
    [0] não iniciou
    [one] iniciou uma vez
    [many] iniciou { $number } vezes
   *[other] iniciou { $number } vezes
}
diag-stopped = { $count ->
    [0] não parou
    [one] parou uma vez
    [many] parou { $number } vezes
   *[other] parou { $number } vezes
}
diag-reservation-failed = a reserva do talker falhou: { $reason }
diag-latency = { $microseconds } µs de latência acumulada

## AVB Lite

lite-active = Ativo
lite-active-untagged = Ativo, sem etiqueta
lite-active-vlan = Ativo, VLAN { $vlan }
lite-capable = Compatível
lite-mode = Modo
lite-mode-capable = AVB, compatível com AVB Lite
lite-because = Motivo
lite-fallback-none = não especificado
lite-fallback-endpoint = a declaração de outro endpoint chegou, pelo que não há nenhum switch AVB entre eles
lite-fallback-unanswered = nove pedidos de peer delay ficaram sem resposta
lite-fallback-responders = dois ou mais responderam a um mesmo pedido de peer delay, pelo que o switch não é AVB
lite-fallback-configured = configurado pelo operador ou por um controlador
lite-fallback-other = um motivo que o perfil não define
lite-other-profile = Outro perfil
lite-ptp-domain = { $profile }, domínio { $domain }
lite-offset = Offset
lite-offset-from = { $offset } em relação a { $grandmaster }
lite-media-vlan = VLAN de média
lite-untagged = Sem etiqueta
lite-unicast = Unicast
lite-fanout = { $count ->
    [one] Até { $count } listener por fluxo, depois multicast
    [many] Até { $count } listeners por fluxo, depois multicast
   *[other] Até { $count } listeners por fluxo, depois multicast
}
lite-link = Link
lite-bandwidth = Largura de banda
lite-egress-of = { $used } de { $link }, { $share }
lite-egress-of-assumed = { $used } de { $link }, { $share }, assumindo um link de gigabit
lite-egress-reported = Conforme a entidade contabiliza os seus fluxos admitidos.
lite-egress-worked-out = A partir dos formatos das suas saídas de fluxo ligadas.
lite-alarm-offset = Offset PTP de { $offset }, acima dos 50 µs que o AVB Lite permite
lite-alarm-egress = Tráfego de saída em { $share } do link, acima dos { $limit } que os fluxos podem ocupar

## AVB Wireless

wireless-station = Estação
wireless-access-point = Ponto de acesso
wireless-role = Função
wireless-mode = Modo
wireless-time = Tempo
wireless-mode-a-ftm = Mode A, 802.1AS sobre FTM
wireless-mode-a-tm = Mode A, 802.1AS sobre TM
wireless-mode-b = Mode B, a partir dos beacons
wireless-no-time = Sem sincronização
wireless-other-mode = Um modo que o perfil não define
wireless-locked = Em lock
wireless-holdover = Em holdover
wireless-not-locked = Sem lock
wireless-row-locked = Estação, em lock
wireless-row-holdover = Estação, em holdover
wireless-row-not-locked = Estação, sem lock
wireless-row-access-point = { $count ->
    [one] Ponto de acesso, { $count } estação
    [many] Ponto de acesso, { $count } estações
   *[other] Ponto de acesso, { $count } estações
}
wireless-link = Link
wireless-channel = canal { $channel }
wireless-not-known = Desconhecido
wireless-signal = Sinal
wireless-rate = Taxa
wireless-ftm-valid = { $share } válidas
wireless-rtt = ida e volta { $rtt }
wireless-bursts = { $count ->
    [one] rajadas de { $count } trama
    [many] rajadas de { $count } tramas
   *[other] rajadas de { $count } tramas
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = o ponto de acesso concede rajadas FTM com tamanho diferente de três ou duas tramas
wireless-reason-measurement = nem FTM nem TM com o ponto de acesso
wireless-reason-signaling = nenhuma mensagem Signaling compatível com gPTP do ponto de acesso
wireless-reason-other = um motivo que o perfil não define
wireless-servo = Erro do servo
wireless-stations = Estações
wireless-station-count = { $count ->
    [one] { $count } estação
    [many] { $count } estações
   *[other] { $count } estações
}
wireless-no-ftm = sem FTM
wireless-unserved = Listeners não servidos
wireless-frame-budget = Orçamento de tramas
wireless-frame-use = { $used } de { $budget } tramas por segundo
wireless-stream-frames = Tramas de fluxo
wireless-frames-of = { $readdressed } para estações, { $unmapped } sem listener, { $dropped } descartadas, { $restored } de estações
wireless-class-a-allowed = Permitida
wireless-class-a-not-allowed = Não permitida
wireless-alarm-not-locked = Tempo Wi-Fi sem lock com o ponto de acesso
wireless-alarm-holdover = Tempo Wi-Fi em holdover, perdeu o lock com o ponto de acesso
wireless-alarm-unserved = { $count ->
    [one] { $count } listener na porta Wi-Fi não servido, acima do limite de unicast
    [many] { $count } listeners na porta Wi-Fi não servidos, acima do limite de unicast
   *[other] { $count } listeners na porta Wi-Fi não servidos, acima do limite de unicast
}

## Log

log-all = Tudo
log-warnings = Avisos
log-pause = Pausar
log-resume = Retomar
log-clear = Limpar
log-empty = Todas as tramas ATDECC que o triib envia e recebe aparecem aqui, das mais recentes para as mais antigas.
log-none-match = Nenhuma trama guardada corresponde ao filtro.
log-frames = { $count ->
    [one] { $count } trama
    [many] { $count } tramas
   *[other] { $count } tramas
}
log-shown-of = { $shown } de { $all } tramas
log-sent = Enviada
log-heard = Recebida
log-not-decoded = Não descodificada
log-warning-short = O seu control_data_length indica { $missing } octetos além do fim da trama.
log-warning-undecodable = Não pôde ser descodificada: { $error }.
log-warning-long-acmp = Está na forma longa do ACMP, que uma entidade Milan não pode enviar (Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = Mapeamento de canais
mapping-inputs = Entradas
mapping-outputs = Saídas
mapping-port = porta { $number }
mapping-fixed = fixo
mapping-not-read = Ainda não lido.
mapping-no-clusters = Sem clusters.
mapping-no-streams = Sem fluxos de áudio.
mapping-none = Sem mapeamentos.
mapping-not-mapped = Não mapeado
mapping-cluster-numbered = Cluster { $index }

## Presets

presets-note = Um preset guarda as fontes de relógio, as frequências de amostragem, os formatos de fluxo, os controlos e as ligações de cada entidade. Recuperá-lo altera o que for diferente.
presets-none = Ainda não há presets guardados.
presets-connections = { $count ->
    [one] { $count } ligação
    [many] { $count } ligações
   *[other] { $count } ligações
}
presets-recall = Recuperar
presets-delete = Eliminar
presets-no-place = Não há onde guardar os presets: a pasta pessoal é desconhecida.
presets-undeletable = Não foi possível eliminar { $path }: { $error }.
presets-saved = { $count ->
    [one] «{ $name }» guardado com { $count } entidade.
    [many] «{ $name }» guardado com { $count } entidades.
   *[other] «{ $name }» guardado com { $count } entidades.
}
presets-nothing-differs = Nada difere de «{ $name }».
presets-recalling = { $count ->
    [one] A recuperar «{ $name }»: { $count } alteração.
    [many] A recuperar «{ $name }»: { $count } alterações.
   *[other] A recuperar «{ $name }»: { $count } alterações.
}
presets-missing = { $report } Ausentes ou não lidas: { $missing }.
presets-deleted = «{ $name }» eliminado.
presets-host-note = Também guarda os talkers e listeners deste computador e reinicia-os ao recuperar.
presets-host-endpoints = { $count } neste computador
presets-starting-host = A iniciar os talkers e listeners deste computador para «{ $name }»; o resto segue quando voltarem.

## Controls

control-numbered = Controlo { $index }
control-not-shown = Não mostrado aqui
control-option = Opção { $number }

## Network errors

network-permission = O triib precisa de permissão para enviar e receber tramas Ethernet em bruto.
network-needs-npcap = O triib precisa do Npcap para enviar e receber tramas Ethernet em bruto.
network-npcap-administrators = O Npcap só permite que os administradores enviem e recebam tramas Ethernet em bruto. Execute o triib como administrador ou reinstale o Npcap sem a opção apenas para administradores.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own talkers and listeners

host-add-talker = Adicionar talker
host-add-listener = Adicionar listener
host-add-device = Adicionar dispositivo
host-show-mine = Mostrar apenas os talkers e listeners deste computador
host-show-all = Mostrar todas as entidades
host-new-talker = Talker do anfitrião { $number }
host-new-listener = Listener do anfitrião { $number }
host-new-device = Dispositivo do anfitrião { $number }
host-failed = Não foi possível adicionar a este computador: { $reason }
host-needs-clock = Os talkers e listeners deste computador precisam de uma interface com fios com relógio de hardware PTP
host-no-ptp4l = O ptp4l não responde, pelo que os fluxos deste computador não conseguem manter a hora gPTP
host-elsewhere = O triib-endpointd é executado em { $interface } para outro utilizador ou como root, pelo que os talkers e listeners deste computador são executados lá, não aqui
host-foreign-mrp = Outro programa faz declarações MSRP ou MVRP em { $interface } a partir do endereço deste computador e pode retirar aquilo de que os fluxos deste computador precisam
host-alarm-foreign-mrp = Outro programa neste computador faz declarações MSRP ou MVRP em { $interface }
host-state = Estado
host-streaming = A transmitir
host-waiting = À espera de um listener
host-listening = À escuta
host-bound = Associado, à espera do talker
host-unbound = Não associado
host-audio-from = Áudio de
host-audio-to = Áudio para
host-channels = Canais
host-virtual-device = Dispositivo virtual
host-add-output-stream = Adicionar fluxo de saída
host-add-input-stream = Adicionar fluxo de entrada
host-remove-stream = Remover fluxo
host-silence = Silêncio
host-tone = Tom de teste
host-nowhere = Nenhum sítio
host-default-device = Dispositivo predefinido
host-remove = Remover deste computador
