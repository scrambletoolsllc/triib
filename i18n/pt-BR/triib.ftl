# triib's interface text in Brazilian Portuguese. The source is
# i18n/en/triib.ftl; term choices are in docs/glossary/pt-BR.md.

## Language

language-name = Português (Brasil)

## Common

common-close = Fechar
common-more = Mais
common-keep-toolbar-shown = Manter a barra de ferramentas visível
common-auto-hide-toolbar = Ocultar a barra de ferramentas automaticamente

## Settings

settings-title = Configurações
settings-general = Geral
settings-appearance = Aparência
settings-language = Idioma
settings-language-system = Padrão do sistema: { $language }
settings-language-note = Os campos de texto usam o idioma de entrada do sistema.
settings-appearance-system = Sistema
settings-appearance-light = Claro
settings-appearance-dark = Escuro
settings-colors = Cores
settings-system-accent = Usar a cor de destaque do sistema
settings-accent-picked = As cores do triib partem da cor abaixo.
settings-accent-omarchy = Do tema do Omarchy, { $theme }.
settings-accent-desktop = Da cor de destaque da área de trabalho.
settings-accent-none = A área de trabalho não tem cor de destaque, então é usada a cor abaixo.
settings-motion = Movimento
settings-animations = Animações
settings-animations-note = Efeitos de mola e deslizamento quando algo muda.
settings-animations-reduced = A área de trabalho pede movimento reduzido, então o triib fica parado.
settings-outside-control = Controle externo
settings-allow-outside-control = Permitir controle externo
settings-allow-outside-control-note = Agentes como o Claude Code podem ver e alterar sua rede AVB, por meio do triib --mcp. O triib pergunta antes de cada novo agente.
settings-allowed-agents = Permitidos: { $agents }
settings-forget-agents = Esquecer
agent-prompt-title = Permitir que { $agent } controle o triib?
agent-prompt-body = { $agent } pede para usar o controle externo do triib: ver as entidades da sua rede e alterá-las, conectando fluxos e definindo formatos, relógios e nomes. Você pode desativar o controle externo em Configurações.
agent-prompt-allow = Permitir
agent-prompt-deny = Não permitir

common-cancel = Cancelar
common-save = Salvar
common-not-set = Não definido
common-unnamed = Sem nome
common-none = Nenhum
common-mac-address = Endereço MAC
common-list-separator = {", "}

## Network interfaces

interface-up = ativa
interface-link-down = link inativo
interface-wireless = sem fio
interface-hardware-clock = relógio de hardware
interface-hardware-clock-named = relógio de hardware { $clock }
interface-virtual = virtual

## Toolbar

toolbar-choose-interface = Escolher uma interface
toolbar-interface = Interface de rede
toolbar-show-virtual = Mostrar interfaces virtuais
toolbar-hide-virtual = Ocultar interfaces virtuais
toolbar-connections = Conexões
toolbar-network = Rede
toolbar-entities = Entidades
toolbar-rediscover = Pedir a todas as entidades que se anunciem
toolbar-rescan = Limpar e escanear novamente todas as entidades
toolbar-search = Pesquisar entidades e fluxos
toolbar-presets = Presets
toolbar-log = Log
toolbar-inspector = Inspetor
toolbar-settings = Configurações

## The network's state, in place of a view

state-no-interface = Sem interface
state-no-interface-note = Escolha a interface da rede AVB para descobrir entidades.
state-starting = Iniciando
state-starting-note = Abrindo { $interface }.
state-listening = Escutando
state-listening-note = As entidades em { $interface } aparecem aqui à medida que se anunciam.
state-permission-needed = Permissão necessária
state-npcap-needed = Npcap necessário
state-get-npcap = Obter o Npcap
state-copy-command = Copiar o comando
state-cannot-use = Não é possível usar { $interface }
state-try-again = Tentar novamente

## Entity list

entities-none-yet = Nenhuma entidade ainda
entities-none-yet-note = Todas as entidades da rede, com suas funções, classes SR e relógio.

## Inspector

inspector-title = Inspetor
inspector-entity = Entidade
inspector-streams = Fluxos
inspector-controls = Controles
inspector-diagnostics = Diagnóstico
inspector-descriptors = Descritores
inspector-select = Selecione uma entidade para ver seus detalhes.
inspector-offline = { $entity } está offline.
inspector-rename = Renomear
inspector-name = Nome
inspector-identify = Identificar
inspector-model-not-read = O modelo de entidade não foi lido.
inspector-no-streams = Sem fluxos.
inspector-no-controls = Nenhum controle para mostrar.
inspector-no-diagnostics = Nenhuma interface ou contador informado.
inspector-reading = Lendo descritores, { $count } até agora.
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
entity-media-clock = Relógio de mídia
entity-clock-domain = Domínio de relógio
entity-sampling-rate = Taxa de amostragem
clock-source-numbered = Fonte { $index }
rate-pull = pull { $pull }

stream-format = Formato
stream-id = ID do fluxo
stream-destination = Destino
stream-max-transit = Tempo de trânsito máx.

avb-interfaces = Interfaces AVB
avb-interface = Interface
avb-interface-clock-identity = Identidade do relógio
avb-interface-grandmaster = Grandmaster
avb-interface-grandmaster-domain = { $grandmaster }, domínio { $domain }
avb-interface-peer-delay = Peer delay
avb-interface-running = Em execução
avb-interface-none-reported = Nenhum informado
avb-interface-path = Caminho
avb-interface-own-grandmaster = É o próprio grandmaster
avb-interface-hops = { $count ->
    [one] A { $count } salto do grandmaster
    [many] A { $count } saltos do grandmaster
   *[other] A { $count } saltos do grandmaster
}
avb-interface-link-up = Link ativo
avb-interface-link-down = Link inativo
avb-interface-grandmaster-changes = Mudanças de grandmaster
avb-interface-frames-sent = Quadros enviados
avb-interface-frames-received = Quadros recebidos
avb-interface-crc-errors = Erros de CRC

tree-firmware = Firmware { $version }
tree-descriptor-types = { $count ->
    [0] { $count } tipos de descritor
    [one] { $count } tipo de descritor
    [many] { $count } tipos de descritor
   *[other] { $count } tipos de descritor
}
tree-clock = Relógio
tree-clock-source-from = { $kind }, de { $location } { $index }
tree-clock-domain-using = Usa { $source }
tree-clusters = { $count ->
    [0] { $count } clusters
    [one] { $count } cluster
    [many] { $count } clusters
   *[other] { $count } clusters
}
tree-maps = { $count ->
    [0] { $count } mapas
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
advert-identify-control = Controle de identificação
advert-avb-interface = Interface AVB
advert-advertising = Anúncio
advert-valid-time = Tempo de validade
advert-available-index = Índice de disponibilidade
advert-association = Associação
advert-capabilities = Capacidades

## Status bar

status-entities = { $count ->
    [0] { $count } entidades
    [one] { $count } entidade
    [many] { $count } entidades
   *[other] { $count } entidades
}
status-not-discovering = Descoberta inativa
status-discovering = Descobrindo
status-discovering-as = Descobrindo como { $controller }
status-stopped = Parado por um erro
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
read-reading = Lendo, { $count } até agora
read-ready-unreadable = { $count ->
    [0] Pronto, { $count } ilegíveis
    [one] Pronto, { $count } ilegível
    [many] Pronto, { $count } ilegíveis
   *[other] Pronto, { $count } ilegíveis
}
read-ready-cached = Pronto, do cache
read-ready = Pronto
read-failed = Falhou: { $reason }

milan-no = Não
milan-before-1-3 = anterior à 1.3
milan-certified = { $version }, certificado { $certification }
milan-not-certified = { $version }, não certificado

outcome-status = status { $status }
outcome-no-response = sem resposta
outcome-not-possible = não é possível
outcome-connect = Não foi possível conectar { $talker } a { $listener }: { $reason }.
outcome-disconnect = Não foi possível desconectar { $listener }: { $reason }.
outcome-identify = Não foi possível identificar { $entity }: { $reason }.
outcome-rename = Não foi possível renomear { $what } para “{ $name }”: { $reason }.
outcome-rename-group = Não foi possível renomear o grupo de { $entity } para “{ $name }”: { $reason }.
outcome-format-streaming = Não foi possível alterar o formato de { $stream }: ele está transmitindo. Desconecte-o primeiro.
outcome-format = Não foi possível alterar o formato de { $stream }: { $reason }.
outcome-max-transit-time = Não foi possível alterar o tempo de trânsito máx. de { $stream }: { $reason }.
outcome-max-transit-time-streaming = Não foi possível alterar o tempo de trânsito máx. de { $stream }: ele está transmitindo. Desconecte-o primeiro.
outcome-sampling-rate = Não foi possível alterar a taxa de amostragem de { $entity }: { $reason }.
outcome-clock-source = Não foi possível alterar a fonte de relógio de { $entity }: { $reason }.
outcome-map = Não foi possível mapear o canal em { $entity }: { $reason }.
outcome-unmap = Não foi possível remover o mapeamento do canal em { $entity }: { $reason }.
outcome-control = Não foi possível ajustar “{ $control }” em { $entity }: { $reason }.
outcome-control-numbered = Não foi possível ajustar o controle { $index } em { $entity }: { $reason }.

stream-not-connected = Sem conexão
stream-from = De { $stream }
stream-from-receiving = De { $stream }, recebendo
stream-from-waiting = De { $stream }, aguardando o talker
stream-from-failed = De { $stream }, a reserva do talker falhou: { $reason }
stream-sending-to = Enviando para { $destination }

failure-no-response = não respondeu
failure-refused = recusou com { $status }
failure-malformed = não foi possível decodificar a resposta
failure-on-this-computer = roda neste computador; leia-a de outro

msrp-failure-1 = largura de banda insuficiente
msrp-failure-2 = recursos insuficientes no switch
msrp-failure-3 = largura de banda insuficiente para a classe de tráfego
msrp-failure-4 = ID de fluxo em uso por outro talker
msrp-failure-5 = endereço de destino já em uso
msrp-failure-6 = substituído por um fluxo de rank superior
msrp-failure-7 = a latência informada mudou
msrp-failure-8 = a porta de saída não é compatível com AVB
msrp-failure-9 = use outro endereço de destino
msrp-failure-10 = sem recursos MSRP
msrp-failure-11 = sem recursos MMRP
msrp-failure-12 = não é possível armazenar o endereço de destino
msrp-failure-13 = a prioridade não é de uma classe SR
msrp-failure-14 = quadros grandes demais para o meio
msrp-failure-15 = limite de fan-in da porta atingido
msrp-failure-16 = o primeiro valor mudou para um fluxo registrado
msrp-failure-17 = VLAN bloqueada na porta de saída
msrp-failure-18 = tags VLAN desativadas na porta de saída
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
column-wireless = Sem fio

## Settings file

settings-no-place = Não há onde guardar as configurações: a pasta pessoal é desconhecida.
settings-unusable = Não foi possível usar { $path }: { $error }.
settings-unsaved = Não foi possível salvar { $path }: { $error }.

column-remove = Remover coluna
column-move-left = Mover para a esquerda
column-move-right = Mover para a direita
column-add = Adicionar uma coluna
common-percent = { $value }%

## Network view

netmap-empty = Ainda não há rede para mostrar
netmap-empty-note = As entidades aparecem aqui depois de lidas e de informarem onde estão na árvore gPTP.
netmap-focus-clock-path = o caminho do relógio de { $name }
netmap-focus-streams = os fluxos de { $name }
netmap-showing = Mostrando { $what }
netmap-devices = { $count ->
    [0] { $count } dispositivos
    [one] { $count } dispositivo
    [many] { $count } dispositivos
   *[other] { $count } dispositivos
}
netmap-bridges = { $count ->
    [0] { $count } switches
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
netmap-connected = Conectado
netmap-advertised = Anunciado, nenhum listener pronto
netmap-advertised-off-tree = Anunciado, nenhum listener pronto ({ $listener } não está na árvore gPTP)
netmap-failed-at = A reserva falhou em { $bridge }: { $reason }
netmap-failed = A reserva falhou: { $reason }
netmap-no-bridge-on = Nenhum switch detectado em { $interface }
netmap-cannot-listen-on = Não é possível escutar o gPTP em { $interface }
netmap-on-this-computer = Neste computador
netmap-path-not-reported = Caminho não informado
netmap-gptp-not-reported = gPTP não informado
netmap-off-tree = Fora da árvore gPTP
netmap-off-ptp = Fora da árvore PTP
netmap-not-lite = Fora do AVB Lite
netmap-lite-not-reported = AVB Lite não informado
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
netmap-apart-own-grandmaster = Fora da árvore gPTP: é o próprio grandmaster
netmap-apart-no-path = O caminho não foi informado; segue o grandmaster { $grandmaster }
netmap-apart-unreported = Não informou o estado do gPTP
netmap-apart-no-neighbor = Nenhum switch detectado na interface deste computador
netmap-apart-cannot-listen = Este computador não consegue escutar o gPTP na sua interface
netmap-apart-on-this-computer = Roda neste computador; leia-a de outro computador para ver o estado do gPTP
netmap-apart-not-lite = Não executa o AVB Lite, então não segue o grandmaster
netmap-apart-lite-unreported = Não informa nada sobre o AVB Lite, então não se sabe quem ele segue
netmap-clock-tree = Árvore de relógio
netmap-no-grandmaster = Nenhum grandmaster detectado
netmap-grandmaster-is = Grandmaster: { $grandmaster }
netmap-needs-attention = Requer atenção
netmap-nodes-below = Nós abaixo
netmap-bridges-below = Switches abaixo
netmap-clock-path = Caminho do relógio
netmap-hops = Saltos até o grandmaster
netmap-link-delay = Atraso do link
netmap-bridge-port = Porta do switch
netmap-link-drops = Quedas do link
netmap-synced-to-grandmaster = Sincronizado com o grandmaster
netmap-host-no-gptp = Não sincronizado: este computador não executa gPTP
netmap-link-no-gptp = Não sincronizado: o gPTP não está ativo no link
netmap-ptp-offset-high = Não sincronizado: { $offset } em relação ao grandmaster, acima dos 50 µs que o AVB Lite permite
netmap-ptp-no-offset = Não sincronizado: não mediu nenhum offset em relação ao grandmaster
netmap-audio = Áudio
netmap-media-clock-streams = Fluxos de relógio de mídia
netmap-audio-streams = Fluxos de áudio
netmap-bound = { $count ->
    [0] { $count } vinculados
    [one] { $count } vinculado
    [many] { $count } vinculados
   *[other] { $count } vinculados
}
netmap-flowing = Fluindo
netmap-advertised-state = Anunciado
netmap-media-clock-stream = Fluxo de relógio de mídia
netmap-audio-stream = Fluxo de áudio
netmap-reaches = Chega até
netmap-passing-count = { $count ->
    [0] { $count } fluxos em trânsito
    [one] { $count } fluxo em trânsito
    [many] { $count } fluxos em trânsito
   *[other] { $count } fluxos em trânsito
}
netmap-through = Em trânsito
netmap-passing-through = Fluxos em trânsito
netmap-sending = Enviando
netmap-receiving = Recebendo
netmap-problems = Problemas
netmap-help-back = Clique no fundo para voltar à visão geral.
netmap-help-stream = Clique em um fluxo para inspecioná-lo ou no fundo para voltar à visão geral.
netmap-help-ptp = No AVB Lite, o relógio flui de ponta a ponta, do grandmaster até cada dispositivo, passando por switches que não participam, por isso nenhum é mostrado. Um dispositivo está sincronizado enquanto segue o grandmaster com margem de 50 µs. Clique em um dispositivo ou no seu fio para inspecionar o relógio; clique no fundo para limpar a seleção.
netmap-help-clock = O relógio flui do grandmaster, passando por cada switch, até todos os nós da árvore. Uma linha cinza tracejada é um link onde o gPTP não está ativo. Clique em um dispositivo ou no seu fio para inspecionar o caminho do relógio; clique no fundo para limpar a seleção.
netmap-help-media-clock = Apenas fluxos de relógio de mídia (CRF), desenhados como os de áudio: um fio por fluxo, com a cor do talker. Clique em um fio para inspecionar seu fluxo ou em um dispositivo para ver seus fluxos; clique no fundo para limpar a seleção.
netmap-help-audio = Cada fluxo tem seu próprio fio, que entra e sai de cada switch que atravessa. A cor depende do talker: cada talker tem um matiz, e seus fluxos são tons dele. Pontos em movimento indicam que o áudio está fluindo; uma linha vermelha parada é uma reserva com falha e uma linha cinza parada é um fluxo anunciado sem nenhum listener pronto; ambas terminam onde a reserva termina. Os dispositivos na coluna do meio se conectam diretamente ao switch do grandmaster. Clique em um fio para inspecionar seu fluxo ou em um dispositivo para ver seus fluxos; clique no fundo para limpar a seleção.

## Connections

matrix-nothing-shown = Nenhum fluxo para mostrar
matrix-nothing-shown-note = Altere a pesquisa ou os filtros para ver mais fluxos.
matrix-empty = Nenhum fluxo para conectar
matrix-empty-note = Os fluxos de talker e de listener se encontram aqui depois que as entidades que os têm forem lidas.
matrix-all-streams = Todos os fluxos
matrix-connectable-only = Ocultar o que não pode ser conectado
matrix-none-hidden = Todos os fluxos mostrados podem ser conectados
matrix-hidden = { $count ->
    [0] { $count } fluxos ocultos
    [one] { $count } fluxo oculto
    [many] { $count } fluxos ocultos
   *[other] { $count } fluxos ocultos
}
matrix-own = As saídas de uma entidade não se conectam às suas próprias entradas.
matrix-working = Em andamento.
matrix-waiting-change = Aguardando a última alteração nesta entrada.
matrix-connected = Conectada e recebendo. Clique para desconectar.
matrix-bound-waiting = Vinculada, aguardando o fluxo do talker. Clique para desconectar.
matrix-bound-failed = Vinculada, mas a reserva do talker falhou: { $reason }. Clique para desconectar.
matrix-bound-formats-differ = Vinculada, mas os formatos diferem: o talker envia { $sent } e a entrada está configurada para { $set }. Clique para desconectar.
matrix-formats-match = Os formatos coincidem ({ $format }). Clique para conectar.
matrix-format-must-change = A entrada aceita { $sent }, mas está configurada para { $set }, então pode não tocar até que o formato mude. Clique para escolher um formato.
matrix-format-input = Mudar a entrada para
matrix-format-both = Mudar ambos para
matrix-format-talker-busy = O talker transmite para outros listeners, então o formato dele se mantém.
matrix-format-anyway = Conectar sem mudar os formatos
matrix-incompatible = A entrada não aceita { $sent }. Está configurada para { $set }.
matrix-group-none = Sem conexão. Expanda para conectar os fluxos um a um.
matrix-group-connected = { $count ->
    [0] { $count } conectados. Expanda para ver cada um.
    [one] { $count } conectado. Expanda para vê-lo.
    [many] { $count } conectados. Expanda para ver cada um.
   *[other] { $count } conectados. Expanda para ver cada um.
}
matrix-outputs-expand = { $count ->
    [0] { $count } saídas de fluxo. Clique na seta para expandir ou no nome para inspecioná-la.
    [one] { $count } saída de fluxo. Clique na seta para expandir ou no nome para inspecioná-la.
    [many] { $count } saídas de fluxo. Clique na seta para expandir ou no nome para inspecioná-la.
   *[other] { $count } saídas de fluxo. Clique na seta para expandir ou no nome para inspecioná-la.
}
matrix-outputs-collapse = { $count ->
    [0] { $count } saídas de fluxo. Clique na seta para recolher ou no nome para inspecioná-la.
    [one] { $count } saída de fluxo. Clique na seta para recolher ou no nome para inspecioná-la.
    [many] { $count } saídas de fluxo. Clique na seta para recolher ou no nome para inspecioná-la.
   *[other] { $count } saídas de fluxo. Clique na seta para recolher ou no nome para inspecioná-la.
}
matrix-inputs-expand = { $count ->
    [0] { $count } entradas de fluxo. Clique na seta para expandir ou no nome para inspecioná-la.
    [one] { $count } entrada de fluxo. Clique na seta para expandir ou no nome para inspecioná-la.
    [many] { $count } entradas de fluxo. Clique na seta para expandir ou no nome para inspecioná-la.
   *[other] { $count } entradas de fluxo. Clique na seta para expandir ou no nome para inspecioná-la.
}
matrix-inputs-collapse = { $count ->
    [0] { $count } entradas de fluxo. Clique na seta para recolher ou no nome para inspecioná-la.
    [one] { $count } entrada de fluxo. Clique na seta para recolher ou no nome para inspecioná-la.
    [many] { $count } entradas de fluxo. Clique na seta para recolher ou no nome para inspecioná-la.
   *[other] { $count } entradas de fluxo. Clique na seta para recolher ou no nome para inspecioná-la.
}
matrix-stream-inspect = { $detail } Clique para inspecionar { $entity }.
matrix-point = Aponte para uma célula
matrix-point-note = para ver o talker e o listener e se os formatos são compatíveis.
matrix-legend-waiting = Vinculada, aguardando o fluxo
matrix-legend-trouble = Vinculada, algo está errado
matrix-legend-open = Conexão possível
matrix-legend-change = O formato da entrada precisa mudar antes
matrix-legend-incompatible = Formatos incompatíveis
matrix-talker-outputs = Saídas de talker
matrix-listener-inputs = Entradas de listener

common-thousands-separator = {"."}
common-decimal-separator = {","}

## Diagnostics

diag-since-start = Contado desde que a entidade foi iniciada.
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
    [0] { $number } quadros recebidos
    [one] { $number } quadro recebido
    [many] { $number } quadros recebidos
   *[other] { $number } quadros recebidos
}
diag-frames-out = { $count ->
    [0] { $number } quadros enviados
    [one] { $number } quadro enviado
    [many] { $number } quadros enviados
   *[other] { $number } quadros enviados
}
diag-media-locked = { $count ->
    [0] não entrou em lock de mídia
    [one] entrou em lock de mídia uma vez
    [many] entrou em lock de mídia { $number } vezes
   *[other] entrou em lock de mídia { $number } vezes
}
diag-lost-media-lock = { $count ->
    [0] não perdeu o lock de mídia
    [one] perdeu o lock de mídia uma vez
    [many] perdeu o lock de mídia { $number } vezes
   *[other] perdeu o lock de mídia { $number } vezes
}
diag-interrupted = { $count ->
    [0] não foi interrompido
    [one] foi interrompido uma vez
    [many] foi interrompido { $number } vezes
   *[other] foi interrompido { $number } vezes
}
diag-out-of-sequence = { $count ->
    [0] { $number } quadros fora de sequência
    [one] { $number } quadro fora de sequência
    [many] { $number } quadros fora de sequência
   *[other] { $number } quadros fora de sequência
}
diag-media-resets = { $count ->
    [0] { $number } reinícios de mídia
    [one] { $number } reinício de mídia
    [many] { $number } reinícios de mídia
   *[other] { $number } reinícios de mídia
}
diag-timestamps-uncertain = { $count ->
    [0] sem timestamps incertos
    [one] timestamps incertos uma vez
    [many] timestamps incertos { $number } vezes
   *[other] timestamps incertos { $number } vezes
}
diag-no-timestamp = { $count ->
    [0] { $number } quadros sem timestamp
    [one] { $number } quadro sem timestamp
    [many] { $number } quadros sem timestamp
   *[other] { $number } quadros sem timestamp
}
diag-unsupported-format = { $count ->
    [0] { $number } quadros em formato não suportado
    [one] { $number } quadro em formato não suportado
    [many] { $number } quadros em formato não suportado
   *[other] { $number } quadros em formato não suportado
}
diag-late = { $count ->
    [0] { $number } quadros atrasados
    [one] { $number } quadro atrasado
    [many] { $number } quadros atrasados
   *[other] { $number } quadros atrasados
}
diag-early = { $count ->
    [0] { $number } quadros adiantados
    [one] { $number } quadro adiantado
    [many] { $number } quadros adiantados
   *[other] { $number } quadros adiantados
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
lite-active-untagged = Ativo, sem tag
lite-active-vlan = Ativo, VLAN { $vlan }
lite-capable = Compatível
lite-mode = Modo
lite-mode-capable = AVB (compatível com AVB Lite)
lite-because = Motivo
lite-fallback-none = não especificado
lite-fallback-endpoint = a declaração de outro endpoint chegou, então não há switch AVB entre eles
lite-fallback-unanswered = nove pedidos de peer delay ficaram sem resposta
lite-fallback-responders = dois ou mais responderam a um mesmo pedido de peer delay, então o switch não é AVB
lite-fallback-configured = configurado pelo operador ou por um controlador
lite-fallback-other = um motivo que o perfil não define
lite-other-profile = Outro perfil
lite-ptp-domain = { $profile }, domínio { $domain }
lite-offset = Offset
lite-offset-from = { $offset } em relação a { $grandmaster }
lite-media-vlan = VLAN de mídia
lite-untagged = Sem tag
lite-unicast = Unicast
lite-fanout = { $count ->
    [0] Até { $count } listeners por fluxo, depois multicast
    [one] Até { $count } listener por fluxo, depois multicast
    [many] Até { $count } listeners por fluxo, depois multicast
   *[other] Até { $count } listeners por fluxo, depois multicast
}
lite-link = Link
lite-bandwidth = Largura de banda
lite-egress-of = { $used } de { $link }, { $share }
lite-egress-of-assumed = { $used } de { $link }, { $share }, supondo um link de gigabit
lite-egress-reported = Conforme a entidade contabiliza seus fluxos admitidos.
lite-egress-worked-out = A partir dos formatos de suas saídas de fluxo conectadas.
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
    [0] Ponto de acesso, { $count } estações
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
    [0] rajadas de { $count } quadros
    [one] rajadas de { $count } quadro
    [many] rajadas de { $count } quadros
   *[other] rajadas de { $count } quadros
}
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = o ponto de acesso concede rajadas FTM com tamanho diferente de três ou dois quadros
wireless-reason-measurement = nem FTM nem TM com o ponto de acesso
wireless-reason-signaling = nenhuma mensagem Signaling compatível com gPTP do ponto de acesso
wireless-reason-other = um motivo que o perfil não define
wireless-servo = Erro do servo
wireless-stations = Estações
wireless-station-count = { $count ->
    [0] { $count } estações
    [one] { $count } estação
    [many] { $count } estações
   *[other] { $count } estações
}
wireless-no-ftm = sem FTM
wireless-unserved = Listeners não atendidos
wireless-frame-budget = Orçamento de quadros
wireless-frame-use = { $used } de { $budget } quadros por segundo
wireless-stream-frames = Quadros de fluxo
wireless-frames-of = { $readdressed } para estações, { $unmapped } sem listener, { $dropped } descartados, { $restored } de estações
wireless-class-a-allowed = Permitida
wireless-class-a-not-allowed = Não permitida
wireless-alarm-not-locked = Tempo Wi-Fi sem lock com o ponto de acesso
wireless-alarm-holdover = Tempo Wi-Fi em holdover, perdeu o lock com o ponto de acesso
wireless-alarm-unserved = { $count ->
    [0] { $count } listeners na porta Wi-Fi não atendidos, acima do limite de unicast
    [one] { $count } listener na porta Wi-Fi não atendido, acima do limite de unicast
    [many] { $count } listeners na porta Wi-Fi não atendidos, acima do limite de unicast
   *[other] { $count } listeners na porta Wi-Fi não atendidos, acima do limite de unicast
}

## Log

log-all = Tudo
log-warnings = Avisos
log-pause = Pausar
log-resume = Retomar
log-clear = Limpar
log-empty = Todo quadro ATDECC que o triib envia e recebe aparece aqui, do mais recente para o mais antigo.
log-none-match = Nenhum quadro guardado corresponde ao filtro.
log-frames = { $count ->
    [0] { $count } quadros
    [one] { $count } quadro
    [many] { $count } quadros
   *[other] { $count } quadros
}
log-shown-of = { $shown } de { $all } quadros
log-sent = Enviado
log-heard = Recebido
log-not-decoded = Não decodificado
log-warning-short = O control_data_length indica { $missing } octetos além do fim do quadro.
log-warning-undecodable = Não pôde ser decodificado: { $error }.
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

presets-note = Um preset guarda as fontes de relógio, as taxas de amostragem, os formatos de fluxo, os controles e as conexões de cada entidade. Recuperá-lo altera o que estiver diferente.
presets-none = Nenhum preset salvo ainda.
presets-connections = { $count ->
    [0] { $count } conexões
    [one] { $count } conexão
    [many] { $count } conexões
   *[other] { $count } conexões
}
presets-recall = Recuperar
presets-delete = Excluir
presets-no-place = Não há onde guardar os presets: a pasta pessoal é desconhecida.
presets-undeletable = Não foi possível excluir { $path }: { $error }.
presets-saved = { $count ->
    [0] “{ $name }” salvo com { $count } entidades.
    [one] “{ $name }” salvo com { $count } entidade.
    [many] “{ $name }” salvo com { $count } entidades.
   *[other] “{ $name }” salvo com { $count } entidades.
}
presets-nothing-differs = Nada difere de “{ $name }”.
presets-recalling = { $count ->
    [0] Recuperando “{ $name }”: { $count } alterações.
    [one] Recuperando “{ $name }”: { $count } alteração.
    [many] Recuperando “{ $name }”: { $count } alterações.
   *[other] Recuperando “{ $name }”: { $count } alterações.
}
presets-missing = { $report } Ausentes ou não lidas: { $missing }.
presets-deleted = “{ $name }” excluído.
presets-host-note = Também guarda os talkers e listeners deste computador e os reinicia ao recuperar.
presets-host-endpoints = { $count } neste computador
presets-starting-host = Iniciando os talkers e listeners deste computador para “{ $name }”; o restante vem quando eles voltarem.

## Controls

control-numbered = Controle { $index }
control-not-shown = Não mostrado aqui
control-option = Opção { $number }

## Network errors

network-permission = O triib precisa de permissão para enviar e receber quadros Ethernet brutos.
network-needs-npcap = O triib precisa do Npcap para enviar e receber quadros Ethernet brutos.
network-npcap-administrators = O Npcap só permite que administradores enviem e recebam quadros Ethernet brutos. Execute o triib como administrador ou reinstale o Npcap sem a opção somente para administradores.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own talkers and listeners

host-add-talker = Adicionar talker
host-add-listener = Adicionar listener
host-add-device = Adicionar dispositivo
host-show-mine = Mostrar só os talkers e listeners deste computador
host-show-all = Mostrar todas as entidades
host-new-talker = Talker do host { $number }
host-new-listener = Listener do host { $number }
host-new-device = Dispositivo do host { $number }
host-failed = Não foi possível adicionar a este computador: { $reason }
host-needs-clock = Os talkers e listeners deste computador precisam de uma interface cabeada com relógio de hardware PTP
host-no-ptp4l = O ptp4l não responde, então os fluxos deste computador não conseguem manter o horário gPTP
host-elsewhere = O triib-endpointd roda em { $interface } para outro usuário ou como root, então os talkers e listeners deste computador rodam lá, não aqui
host-foreign-mrp = Outro programa faz declarações MSRP ou MVRP em { $interface } a partir do endereço deste computador e pode retirar aquilo de que os fluxos deste computador precisam
host-alarm-foreign-mrp = Outro programa neste computador faz declarações MSRP ou MVRP em { $interface }
host-state = Estado
host-streaming = Transmitindo
host-waiting = Aguardando um listener
host-listening = Escutando
host-bound = Vinculado, aguardando o talker
host-unbound = Não vinculado
host-audio-from = Áudio de
host-audio-to = Áudio para
host-channels = Canais
host-virtual-device = Dispositivo virtual
host-add-stream = Adicionar fluxo
host-add-audio-input = Entrada de áudio
host-add-audio-output = Saída de áudio
host-add-clock-input = Entrada de relógio de mídia
host-add-clock-output = Saída de relógio de mídia
host-remove-stream = Remover fluxo
host-silence = Silêncio
host-tone = Tom de teste
host-nowhere = Lugar nenhum
host-default-device = Dispositivo padrão
host-remove = Remover deste computador
