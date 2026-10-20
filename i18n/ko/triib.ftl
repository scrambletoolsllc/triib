# triib 인터페이스 텍스트(한국어).
# 용어는 docs/GLOSSARY.md와 docs/glossary/ko.md를 따릅니다.

## Language

language-name = 한국어

## Common

common-close = 닫기
common-more = 더 보기
common-keep-toolbar-shown = 도구 모음 항상 표시
common-auto-hide-toolbar = 도구 모음 자동 숨기기

## Settings

settings-title = 설정
settings-general = 일반
settings-appearance = 모양
settings-language = 언어
settings-language-system = 시스템 기본값: { $language }
settings-language-note = 텍스트 필드에는 시스템 입력 언어로 입력됩니다.
settings-appearance-system = 시스템
settings-appearance-light = 라이트
settings-appearance-dark = 다크
settings-colors = 색상
settings-system-accent = 시스템 강조 색상 사용
settings-accent-picked = 아래 색상을 바탕으로 triib 색상을 정합니다.
settings-accent-omarchy = Omarchy 테마({ $theme })의 색상입니다.
settings-accent-desktop = 데스크톱 강조 색상입니다.
settings-accent-none = 데스크톱에 강조 색상이 없어 아래 색상을 사용합니다.
settings-motion = 동작
settings-animations = 애니메이션
settings-animations-note = 화면이 바뀔 때 튕기고 미끄러지는 효과를 줍니다.
settings-animations-reduced = 데스크톱에서 동작 줄이기를 요청하여 triib 애니메이션을 표시하지 않습니다.
settings-outside-control = 외부 제어
settings-allow-outside-control = 외부 제어 허용
settings-allow-outside-control-note = Claude Code 같은 에이전트가 triib --mcp를 통해 AVB 네트워크를 보고 변경할 수 있습니다. 새 에이전트마다 triib가 먼저 묻습니다.
settings-allowed-agents = 허용됨: { $agents }
settings-forget-agents = 지우기
settings-storage = 저장 위치
settings-storage-presets = 프리셋 폴더
settings-storage-entities = 엔티티 파일
settings-storage-apply = 적용
settings-storage-choose = 선택…
settings-storage-note = 이전 위치에 이미 보관된 프리셋은 그대로 남습니다. 계속 사용하려면 새 위치로 옮기세요. triib 설정은 { $file }에 저장됩니다.
settings-storage-entities-note = 아직 없는 엔티티 파일은 사용 중인 파일의 복사본으로 시작하며, triib-endpointd가 그 파일로 다시 시작합니다.
dialog-choose-presets = 프리셋 폴더 선택
dialog-choose-entities = 엔티티 파일의 폴더 선택
settings-full-path = ~/Documents/triib와 같은 전체 경로를 사용하세요.
settings-path-is-folder = { $path }은(는) 파일이 아니라 폴더입니다.
settings-folder-missing = { $path } 폴더가 없습니다. 먼저 폴더를 생성하거나 다른 폴더를 선택하세요.
settings-path-is-file = { $path }은(는) 폴더가 아니라 파일입니다.
settings-cannot-write = triib에서 { $path }에 쓸 수 없습니다: { $error }.
app-file-dialog-failed = 파일 대화상자를 표시할 수 없습니다: { $error }
agent-prompt-title = { $agent }이(가) triib를 제어하도록 허용하시겠습니까?
agent-prompt-body = { $agent }이(가) triib의 외부 제어를 사용하려고 합니다. 네트워크의 엔티티를 보고 변경하며, 스트림을 연결하고 포맷, 클럭, 이름을 설정합니다. 외부 제어는 설정에서 끌 수 있습니다.
agent-prompt-allow = 허용
agent-prompt-deny = 허용 안 함

common-cancel = 취소
common-save = 저장
common-not-set = 설정 안 됨
common-unnamed = 이름 없음
common-none = 없음
common-mac-address = MAC 주소
common-list-separator = {", "}

## Network interfaces

interface-up = 업
interface-link-down = 링크 다운
interface-wireless = 무선
interface-hardware-clock = 하드웨어 클럭
interface-hardware-clock-named = 하드웨어 클럭 { $clock }
interface-virtual = 가상

## Toolbar

toolbar-choose-interface = 인터페이스 선택
toolbar-interface = 네트워크 인터페이스
toolbar-filter-interfaces = 인터페이스 필터
toolbar-hide-virtual = 가상 인터페이스 숨기기
toolbar-hide-wireless = 무선 인터페이스 숨기기
toolbar-hide-unclocked = 하드웨어 클럭이 없는 인터페이스 숨기기
toolbar-connections = 연결
toolbar-network = 네트워크
toolbar-entities = 엔티티
toolbar-rediscover = 모든 엔티티에 광고 요청
toolbar-rescan = 모든 엔티티를 지우고 다시 스캔
toolbar-search = 엔티티 및 스트림 검색
toolbar-presets = 프리셋
toolbar-log = 로그
toolbar-inspector = 인스펙터
toolbar-settings = 설정
toolbar-close-window = 윈도우 닫기

## The macOS menu bar, named as in macOS's own apps

menu-about = triib에 관하여
menu-settings = 설정…
menu-services = 서비스
menu-hide = triib 가리기
menu-hide-others = 기타 가리기
menu-show-all = 모두 보기
menu-quit = triib 종료
menu-file = 파일
menu-presets = 프리셋…
menu-close-window = 윈도우 닫기
menu-edit = 편집
menu-cut = 오려두기
menu-copy = 복사하기
menu-paste = 붙여넣기
menu-select-all = 모두 선택
menu-view = 보기
menu-full-screen = 전체 화면 시작
menu-window = 윈도우
menu-minimize = 최소화
menu-zoom = 확대/축소
menu-bring-all-to-front = 모두 앞으로 가져오기
menu-help = 도움말
menu-website = triib 웹사이트
menu-release-notes = 릴리스 노트

## The network's state, in place of a view

state-no-interface = 선택된 인터페이스 없음
state-no-interface-note = 엔티티를 탐색하려면 AVB 네트워크에 연결된 인터페이스를 선택하세요.
state-starting = 시작 중
state-starting-note = { $interface } 인터페이스를 여는 중입니다.
state-listening = 수신 대기 중
state-listening-note = { $interface }의 엔티티가 광고하면 여기에 표시됩니다.
state-permission-needed = 권한 필요
state-npcap-needed = Npcap 필요
state-get-npcap = Npcap 받기
state-copy-command = 명령 복사
state-cannot-use = { $interface } 인터페이스를 사용할 수 없음
state-try-again = 다시 시도

## Entity list

entities-none-yet = 아직 엔티티 없음
entities-none-yet-note = 네트워크의 모든 엔티티를 역할, SR 클래스, 클럭과 함께 표시합니다.

## Inspector

inspector-title = 인스펙터
inspector-entity = 엔티티
inspector-streams = 스트림
inspector-controls = 컨트롤
inspector-diagnostics = 진단
inspector-descriptors = 디스크립터
inspector-select = 엔티티를 선택하면 세부 정보가 표시됩니다.
inspector-offline = { $entity } 엔티티가 오프라인입니다.
inspector-heard = 자신을 알리지 않습니다. triib가 보낸 명령에서 이를 확인했으며, 명령 없이 5분이 지날 때까지 목록에 표시합니다.
inspector-rename = 이름 바꾸기
inspector-name = 이름
inspector-identify = 식별
inspector-model-not-read = 엔티티 모델을 아직 읽지 않았습니다.
inspector-no-streams = 스트림이 없습니다.
inspector-no-controls = 표시할 컨트롤이 없습니다.
inspector-no-diagnostics = 보고된 인터페이스나 카운터가 없습니다.
inspector-reading = 디스크립터를 읽는 중입니다({ $count }개 완료).
inspector-read-failed = 엔티티 모델을 읽을 수 없습니다: { $reason }.

entity-section = 엔티티
entity-name = 이름
entity-triib-controller = triib 컨트롤러
entity-triib-on = { $host }의 triib
entity-triib-here = 이 컴퓨터의 triib
entity-controller-on = { $host }의 컨트롤러
entity-controller-here = 이 컴퓨터의 컨트롤러
entity-controller-at = { $mac }의 컨트롤러
entity-group = 그룹
entity-product = 제품
entity-firmware = 펌웨어
entity-serial-number = 일련번호
entity-configuration = 구성
entity-configuration-of = { $name } ({ $number }/{ $count })
entity-configuration-holds = 입력 { $inputs }, 출력 { $outputs }, { $channels }채널 스트림
entity-configuration-holds-mixed = 입력 { $inputs }, 출력 { $outputs }
entity-configuration-numbered = 구성 { $number }
entity-milan = Milan
entity-media-clock = 미디어 클럭
entity-clock-domain = 클럭 도메인
entity-sampling-rate = 샘플링 레이트
clock-source-numbered = 소스 { $index }
rate-pull = 풀 { $pull }

stream-format = 포맷
stream-id = 스트림 ID
stream-destination = 대상
stream-max-transit = 최대 전송 시간
stream-listeners = 리스너
stream-listener-gone = { $entity } 입력 { $input }, 오프라인
stream-disconnect-listener = 연결 끊기

avb-interfaces = AVB 인터페이스
avb-interface = 인터페이스
avb-interface-clock-identity = 클럭 ID
avb-interface-grandmaster = 그랜드마스터
avb-interface-grandmaster-domain = { $grandmaster }, 도메인 { $domain }
avb-interface-peer-delay = 피어 지연
avb-interface-running = 실행 중
avb-interface-none-reported = 보고 없음
avb-interface-path = 경로
avb-interface-own-grandmaster = 자체가 그랜드마스터
avb-interface-hops = 그랜드마스터에서 { $count }홉
avb-interface-link-up = 링크 업
avb-interface-link-down = 링크 다운
avb-interface-grandmaster-changes = 그랜드마스터 변경
avb-interface-frames-sent = 송신 프레임
avb-interface-frames-received = 수신 프레임
avb-interface-crc-errors = CRC 오류

tree-firmware = 펌웨어 { $version }
tree-descriptor-types = 디스크립터 유형 { $count }개
tree-clock = 클럭
tree-clock-source-from = { $kind }, { $location } { $index }에서
tree-clock-domain-using = { $source } 사용
tree-clusters = 클러스터 { $count }개
tree-maps = 맵 { $count }개

advert-not-advertised = 광고 안 됨
advert-identity = 식별 정보
advert-entity-id = 엔티티 ID
advert-entity-model = 엔티티 모델
advert-roles = 역할
advert-talker = 토커
advert-listener = 리스너
advert-clock = 클럭
advert-gptp-domain = gPTP 도메인
advert-sr-classes = SR 클래스
advert-indexes = 엔티티 모델 인덱스
advert-identify-control = 식별 컨트롤
advert-avb-interface = AVB 인터페이스
advert-advertising = 광고
advert-valid-time = 유효 시간
advert-available-index = 가용 인덱스
advert-association = 어소시에이션
advert-capabilities = 기능

## Status bar

status-entities = 엔티티 { $count }개
status-not-discovering = 탐색 안 함
status-discovering = 탐색 중
status-discovering-as = { $controller } 컨트롤러로 탐색 중
status-stopped = 오류로 중지됨
status-alarm = 알람
status-alarm-of = { $entity }: { $alarm }
status-alarm-more = { $alarm } 외 { $count }건

## Descriptions of entities, shared by the views

role-talker = 토커 { $count }
role-listener = 리스너 { $count }
role-controller = 컨트롤러
role-none = 역할 없음
classes-a-and-b = A 및 B
clock-no-gptp = gPTP 없음

read-not-read = 읽지 않음
read-reading = 읽는 중({ $count }개 완료)
read-ready-unreadable = 준비됨({ $count }개 읽을 수 없음)
read-ready-cached = 준비됨(캐시에서)
read-ready = 준비됨
read-failed = 실패: { $reason }

milan-no = 미지원
milan-before-1-3 = 1.3 이전
milan-certified = { $version }, { $certification } 인증
milan-not-certified = { $version }, 미인증

outcome-status = 상태 { $status }
outcome-no-response = 응답 없음
outcome-not-possible = 불가능
outcome-connect = { $talker }을(를) { $listener }에 연결할 수 없습니다: { $reason }.
outcome-disconnect = { $listener }의 연결을 끊을 수 없습니다: { $reason }.
outcome-disconnect-listener = { $talker }에서 { $listener }의 연결을 끊을 수 없습니다: { $reason }.
outcome-identify = { $entity }을(를) 식별할 수 없습니다: { $reason }.
outcome-rename = { $what }의 이름을 “{ $name }”(으)로 바꿀 수 없습니다: { $reason }.
outcome-rename-group = { $entity }의 그룹 이름을 “{ $name }”(으)로 바꿀 수 없습니다: { $reason }.
outcome-format-streaming = { $stream }의 포맷을 변경할 수 없습니다: 스트리밍 중입니다. 먼저 연결을 끊으세요.
outcome-format = { $stream }의 포맷을 변경할 수 없습니다: { $reason }.
outcome-max-transit-time = { $stream }의 최대 전송 시간을 변경할 수 없습니다: { $reason }.
outcome-max-transit-time-streaming = { $stream }의 최대 전송 시간을 변경할 수 없습니다: 스트리밍 중입니다. 먼저 연결을 끊으세요.
outcome-sampling-rate = { $entity }의 샘플링 레이트를 변경할 수 없습니다: { $reason }.
outcome-clock-source = { $entity }의 클럭 소스를 변경할 수 없습니다: { $reason }.
outcome-configuration = { $entity }의 구성을 변경할 수 없습니다: { $reason }.
outcome-configuration-streaming = { $entity }는 스트림이 실행 중인 동안 구성을 유지합니다. 먼저 연결을 끊으세요.
outcome-map = { $entity }의 채널을 매핑할 수 없습니다: { $reason }.
outcome-unmap = { $entity }의 채널 매핑을 해제할 수 없습니다: { $reason }.
outcome-control = { $entity }의 “{ $control }”을(를) 설정할 수 없습니다: { $reason }.
outcome-control-numbered = { $entity }의 컨트롤 { $index }을(를) 설정할 수 없습니다: { $reason }.

stream-not-connected = 연결 안 됨
stream-from = { $stream }에서
stream-from-receiving = { $stream }에서 수신 중
stream-from-waiting = { $stream }에서, 토커 대기 중
stream-from-failed = { $stream }에서, 토커 예약 실패: { $reason }
stream-sending-to = { $destination }에 송신 중

failure-no-response = 응답하지 않음
failure-refused = { $status } 상태로 거부됨
failure-malformed = 응답을 디코딩할 수 없음
failure-on-this-computer = 이 컴퓨터에서 실행 중이므로 다른 컴퓨터에서 읽어야 함

msrp-failure-1 = 대역폭 부족
msrp-failure-2 = 브리지 자원 부족
msrp-failure-3 = 트래픽 클래스 대역폭 부족
msrp-failure-4 = 다른 토커가 사용 중인 스트림 ID
msrp-failure-5 = 목적지 주소가 이미 사용 중
msrp-failure-6 = 더 높은 순위의 스트림에 선점됨
msrp-failure-7 = 보고된 레이턴시가 변경됨
msrp-failure-8 = 이그레스 포트가 AVB 미지원
msrp-failure-9 = 다른 목적지 주소 사용 필요
msrp-failure-10 = MSRP 자원 부족
msrp-failure-11 = MMRP 자원 부족
msrp-failure-12 = 목적지 주소를 저장할 수 없음
msrp-failure-13 = 우선순위가 SR 클래스 우선순위가 아님
msrp-failure-14 = 전송 매체에 비해 프레임이 너무 큼
msrp-failure-15 = 팬인 포트 한도 도달
msrp-failure-16 = 등록된 스트림의 FirstValue 변경됨
msrp-failure-17 = 이그레스 포트에서 VLAN 차단됨
msrp-failure-18 = 이그레스 포트에서 VLAN 태깅 비활성화됨
msrp-failure-19 = SR 클래스 우선순위 불일치
msrp-failure-unknown = 알 수 없는 이유
msrp-failure-at = { $reason }, 브리지 { $bridge }에서

## Entity list columns

column-vendor = 제조사
column-model = 모델
column-state = 상태
column-entity-model-id = 엔티티 모델 ID
column-talker-streams = 토커 스트림
column-listener-streams = 리스너 스트림
column-avb-lite = AVB Lite
column-egress = 이그레스
column-wireless = 무선

## Settings file

settings-no-place = 설정을 저장할 곳이 없습니다: 홈 폴더를 알 수 없습니다.
settings-unusable = { $path }을(를) 사용할 수 없습니다: { $error }.
settings-unsaved = { $path }을(를) 저장할 수 없습니다: { $error }.

column-remove = 열 제거
column-move-left = 왼쪽으로 이동
column-move-right = 오른쪽으로 이동
column-add = 열 추가
common-percent = { $value }%

## Network view

netmap-empty = 아직 표시할 네트워크 없음
netmap-empty-note = 엔티티를 읽고 gPTP 트리에서의 위치가 보고되면 여기에 표시됩니다.
netmap-focus-clock-path = { $name }의 클럭 경로
netmap-focus-streams = { $name }의 스트림
netmap-showing = { $what } 표시 중
netmap-devices = 장치 { $count }개
netmap-bridges = 브리지 { $count }개
netmap-show-map = 맵 보기
netmap-show-details = 세부 정보 보기
stream-numbered = 스트림 { $index }
netmap-bridge = 브리지
netmap-access-point = 액세스 포인트
netmap-device = 장치
netmap-this-computer = 이 컴퓨터
netmap-connected = 연결됨
netmap-advertised = 광고됨, 준비된 리스너 없음
netmap-advertised-off-tree = 광고됨, 준비된 리스너 없음({ $listener }: gPTP 트리에 없음)
netmap-failed-at = { $bridge }에서 예약 실패: { $reason }
netmap-failed = 예약 실패: { $reason }
netmap-no-bridge-on = { $interface }에서 감지된 브리지 없음
netmap-cannot-listen-on = { $interface }에서 gPTP를 수신할 수 없음
netmap-on-this-computer = 이 컴퓨터에서 실행
netmap-path-not-reported = 경로 보고 없음
netmap-gptp-not-reported = gPTP 보고 없음
netmap-off-tree = gPTP 트리에 없음
netmap-off-ptp = PTP 트리에 없음
netmap-not-lite = AVB Lite 미실행
netmap-lite-not-reported = AVB Lite 보고 없음
netmap-synced = 동기화됨
netmap-not-synced = 동기화 안 됨
netmap-triib-on = { $interface }의 triib
netmap-through-count = 통과 { $count }
netmap-out = 송신 { $count }
netmap-in = 수신 { $count }
netmap-failed-count = 실패 { $count }
netmap-advertised-only = 광고만 됨
netmap-failed-state = 실패
netmap-stream-item = { $talker } → { $listener } · { $state }
netmap-sending-unheard = 송신 중, 바인딩된 알려진 리스너 없음
netmap-no-listener = 알려진 리스너 없음
netmap-apart-own-grandmaster = gPTP 트리에 없음: 자체가 그랜드마스터입니다
netmap-apart-no-path = 경로가 보고되지 않았으며 그랜드마스터 { $grandmaster }을(를) 따릅니다
netmap-apart-unreported = gPTP 상태를 보고하지 않았습니다
netmap-apart-no-neighbor = 이 컴퓨터의 인터페이스에서 감지된 브리지 없음
netmap-apart-cannot-listen = 이 컴퓨터는 인터페이스에서 gPTP를 수신할 수 없음
netmap-apart-on-this-computer = 이 컴퓨터에서 실행 중이므로 gPTP 상태를 보려면 다른 컴퓨터에서 읽어야 함
netmap-apart-not-lite = AVB Lite 미실행이므로 그랜드마스터를 따르지 않습니다
netmap-apart-lite-unreported = AVB Lite 관련 보고가 없어 무엇을 따르는지 알 수 없습니다
netmap-clock-tree = 클럭 트리
netmap-no-grandmaster = 감지된 그랜드마스터 없음
netmap-grandmaster-is = 그랜드마스터: { $grandmaster }
netmap-needs-attention = 확인 필요
netmap-nodes-below = 하위 노드
netmap-bridges-below = 하위 브리지
netmap-clock-path = 클럭 경로
netmap-hops = 그랜드마스터로부터 홉 수
netmap-link-delay = 링크 지연
netmap-bridge-port = 브리지 포트
netmap-link-drops = 링크 다운 횟수
netmap-synced-to-grandmaster = 그랜드마스터에 동기화됨
netmap-host-no-gptp = 동기화 안 됨: 이 컴퓨터에서 gPTP 미실행
netmap-link-no-gptp = 동기화 안 됨: 링크에서 gPTP 미실행
netmap-ptp-offset-high = 동기화 안 됨: 그랜드마스터 기준 { $offset }, AVB Lite 허용치 50 µs 초과
netmap-ptp-no-offset = 동기화 안 됨: 그랜드마스터 기준 오프셋 측정값 없음
netmap-audio = 오디오
netmap-media-clock-streams = 미디어 클럭 스트림
netmap-audio-streams = 오디오 스트림
netmap-bound = { $count }개 바인딩됨
netmap-flowing = 전송 중
netmap-advertised-state = 광고됨
netmap-media-clock-stream = 미디어 클럭 스트림
netmap-audio-stream = 오디오 스트림
netmap-reaches = 도달 지점
netmap-passing-count = 통과 스트림 { $count }개
netmap-through = 통과
netmap-passing-through = 통과 중
netmap-sending = 송신 중
netmap-receiving = 수신 중
netmap-problems = 문제
netmap-help-back = 배경을 클릭하면 개요로 돌아갑니다.
netmap-help-stream = 스트림을 클릭하면 세부 정보를 보고, 배경을 클릭하면 개요로 돌아갑니다.
netmap-help-ptp = AVB Lite 환경에서는 클럭이 그랜드마스터에서 모든 장치로 엔드투엔드로 전달되며, 중간의 스위치는 관여하지 않으므로 표시되지 않습니다. 장치는 그랜드마스터를 50 µs 이내로 따르는 동안 동기화된 상태입니다. 장치나 그 연결선을 클릭하면 클럭을 보고, 배경을 클릭하면 선택이 해제됩니다.
netmap-help-clock = 클럭은 그랜드마스터에서 각 브리지를 거쳐 트리의 모든 노드로 전달됩니다. 회색 점선은 gPTP 미실행 링크입니다. 장치나 그 연결선을 클릭하면 클럭 경로를 보고, 배경을 클릭하면 선택이 해제됩니다.
netmap-help-media-clock = 미디어 클럭(CRF) 스트림만 오디오와 같은 방식으로 표시합니다. 링크마다 각 스트림의 프레임을 연결선 하나로 그리고, 토커별로 색을 입힙니다. 연결선을 클릭하면 해당 스트림을, 장치를 클릭하면 그 장치의 스트림을 보고, 배경을 클릭하면 선택이 해제됩니다.
netmap-help-audio = 각 연결선은 한 링크 위를 지나는 한 스트림의 프레임을 나타내며, 지나가는 모든 브리지에 들어갔다 나옵니다. 멀티캐스트 스트림은 리스너로 가는 경로가 겹치는 동안에는 연결선 하나로 이어지다가 경로가 갈라지는 곳에서 분기합니다. AVB Lite에서 보낼 수 있는 유니캐스트 스트림은 토커에서 각 리스너까지 저마다 연결선을 따로 가집니다. 색상은 토커별로 구분되어 토커마다 색조가 하나씩 정해지고, 그 토커의 각 스트림은 모든 분기에서 같은 색조의 한 명암으로 표시됩니다. 움직이는 점은 오디오가 흐르고 있다는 뜻이며, 알려진 리스너 없이 송신 중인 토커에서도 마찬가지입니다. 멈춘 빨간 선은 예약 실패, 멈춘 회색 선은 준비된 리스너 없이 광고된 상태이며, 둘 다 예약이 멈춘 곳에서 끝납니다. 가운데 열의 장치는 그랜드마스터의 브리지에 직접 연결됩니다. 연결선을 클릭하면 해당 스트림을, 공유 연결선을 클릭하면 그 연결선의 모든 리스너를, 장치를 클릭하면 그 장치의 스트림을 보고, 배경을 클릭하면 선택이 해제됩니다.

## Connections

matrix-nothing-shown = 표시할 스트림 없음
matrix-nothing-shown-note = 검색어나 필터를 바꾸면 더 많은 스트림을 볼 수 있습니다.
matrix-empty = 연결할 스트림 없음
matrix-empty-note = 스트림이 있는 엔티티를 읽으면 토커 스트림과 리스너 스트림이 여기에 표시됩니다.
matrix-all-streams = 모든 스트림
matrix-connectable-only = 연결할 수 없는 항목 숨기기
matrix-none-hidden = 표시된 모든 스트림을 연결할 수 있음
matrix-hidden = 스트림 { $count }개 숨김
matrix-own = 엔티티의 출력은 자신의 입력에 연결되지 않습니다.
matrix-working = 처리 중입니다.
matrix-waiting-change = 이 입력의 마지막 변경을 기다리는 중입니다.
matrix-connected = 연결되어 수신 중입니다. 클릭하면 연결을 끊습니다.
matrix-bound-waiting = 바인딩되어 토커의 스트림을 기다리는 중입니다. 클릭하면 연결을 끊습니다.
matrix-bound-failed = 바인딩되었지만 토커의 예약이 실패했습니다: { $reason }. 클릭하면 연결을 끊습니다.
matrix-bound-formats-differ = 바인딩되었지만 포맷이 다릅니다(토커 송신 { $sent }, 입력 설정 { $set }). 클릭하면 연결을 끊습니다.
matrix-formats-match = 포맷 일치({ $format }). 클릭하면 연결합니다.
matrix-format-must-change = 입력이 { $sent } 포맷을 받을 수 있지만 현재 { $set } 포맷으로 설정되어 있어, 포맷을 바꾸기 전에는 재생되지 않을 수 있습니다. 클릭하여 포맷을 고르세요.
matrix-format-input = 입력을 다음으로 변경
matrix-format-both = 둘 다 다음으로 변경
matrix-format-talker-busy = 토커가 다른 리스너에게도 스트리밍하고 있어 포맷은 그대로 둡니다.
matrix-format-anyway = 포맷을 바꾸지 않고 연결
matrix-incompatible = 입력이 { $sent } 포맷을 받을 수 없습니다. 현재 설정은 { $set } 포맷입니다.
matrix-group-none = 연결 안 됨. 펼쳐서 스트림을 하나씩 연결하세요.
matrix-group-connected = { $count }개 연결됨. 펼쳐서 각각 확인하세요.
matrix-outputs-expand = 스트림 출력 { $count }개. 화살표를 클릭하면 펼치고, 이름을 클릭하면 세부 정보를 봅니다.
matrix-outputs-collapse = 스트림 출력 { $count }개. 화살표를 클릭하면 접고, 이름을 클릭하면 세부 정보를 봅니다.
matrix-inputs-expand = 스트림 입력 { $count }개. 화살표를 클릭하면 펼치고, 이름을 클릭하면 세부 정보를 봅니다.
matrix-inputs-collapse = 스트림 입력 { $count }개. 화살표를 클릭하면 접고, 이름을 클릭하면 세부 정보를 봅니다.
matrix-stream-inspect = { $detail } 클릭하면 { $entity }의 세부 정보를 봅니다.
matrix-point = 셀을 가리키면
matrix-point-note = 토커와 리스너, 두 포맷의 일치 여부가 표시됩니다.
matrix-legend-waiting = 바인딩됨, 스트림 대기 중
matrix-legend-trouble = 바인딩됨, 문제 있음
matrix-legend-open = 연결 가능
matrix-legend-change = 먼저 입력 포맷 변경 필요
matrix-legend-incompatible = 포맷 호환 불가
matrix-talker-outputs = 토커 출력
matrix-listener-inputs = 리스너 입력

common-thousands-separator = {","}
common-decimal-separator = {"."}

## Diagnostics

diag-since-start = 엔티티가 시작된 이후의 집계입니다.
diag-stream-input = 스트림 입력
diag-stream-output = 스트림 출력
diag-locked = { $count ->
    [0] 락 없음
   *[other] 락 { $number }회
}
diag-lost-lock = { $count ->
    [0] 락 손실 없음
   *[other] 락 손실 { $number }회
}
diag-frames-in = 수신 프레임 { $number }
diag-frames-out = 송신 프레임 { $number }
diag-media-locked = { $count ->
    [0] 미디어 락 없음
   *[other] 미디어 락 { $number }회
}
diag-lost-media-lock = { $count ->
    [0] 미디어 락 손실 없음
   *[other] 미디어 락 손실 { $number }회
}
diag-interrupted = { $count ->
    [0] 중단 없음
   *[other] 중단 { $number }회
}
diag-out-of-sequence = 순서가 어긋난 프레임 { $number }
diag-media-resets = 미디어 리셋 { $number }회
diag-timestamps-uncertain = { $count ->
    [0] 타임스탬프 불확실 없음
   *[other] 타임스탬프 불확실 { $number }회
}
diag-no-timestamp = 타임스탬프 없는 프레임 { $number }
diag-unsupported-format = 지원되지 않는 포맷의 프레임 { $number }
diag-late = 늦은 프레임 { $number }
diag-early = 이른 프레임 { $number }
diag-started = { $count ->
    [0] 시작 없음
   *[other] 시작 { $number }회
}
diag-stopped = { $count ->
    [0] 정지 없음
   *[other] 정지 { $number }회
}
diag-reservation-failed = 토커 예약 실패: { $reason }
diag-latency = 누적 레이턴시 { $microseconds } µs

## AVB Lite

lite-active = 활성
lite-active-untagged = 활성, 태그 없음
lite-active-vlan = 활성, VLAN { $vlan }
lite-capable = 지원
lite-mode = 모드
lite-mode-capable = AVB (AVB Lite 지원)
lite-because = 이유
lite-fallback-none = 제공된 이유 없음
lite-fallback-endpoint = 다른 엔드포인트의 선언이 그대로 전달되어 그 사이에 AVB 브리지가 없음
lite-fallback-unanswered = 피어 지연 요청 9회에 응답 없음
lite-fallback-responders = 피어 지연 요청 한 번에 둘 이상이 응답하여 스위치가 AVB 브리지가 아님
lite-fallback-configured = 운영자 또는 컨트롤러가 설정함
lite-fallback-other = 프로파일에 정의되지 않은 이유
lite-other-profile = 다른 프로파일
lite-ptp-domain = { $profile }, 도메인 { $domain }
lite-offset = 오프셋
lite-offset-from = { $grandmaster } 기준 { $offset }
lite-media-vlan = 미디어 VLAN
lite-untagged = 태그 없음
lite-unicast = 유니캐스트
lite-fanout = 스트림당 리스너 최대 { $count }개, 이후 멀티캐스트
lite-link = 링크
lite-bandwidth = 대역폭
lite-egress-of = { $used } / { $link }, { $share }
lite-egress-of-assumed = { $used } / { $link }, { $share }, 기가비트 링크로 가정
lite-egress-reported = 엔티티가 집계한 허용 스트림 기준입니다.
lite-egress-worked-out = 연결된 스트림 출력의 포맷으로 계산했습니다.
lite-alarm-offset = PTP 오프셋 { $offset }, AVB Lite 허용치 50 µs 초과
lite-alarm-egress = 이그레스가 링크의 { $share }로 스트림 허용치 { $limit } 초과

## AVB Wireless

wireless-station = 스테이션
wireless-access-point = 액세스 포인트
wireless-role = 역할
wireless-mode = 모드
wireless-time = 시간
wireless-mode-a-ftm = Mode A, FTM 기반 802.1AS
wireless-mode-a-tm = Mode A, TM 기반 802.1AS
wireless-mode-b = Mode B, 비콘 기반
wireless-no-time = 시간 동기 없음
wireless-other-mode = 프로파일에 정의되지 않은 모드
wireless-locked = 락됨
wireless-holdover = 홀드오버 중
wireless-not-locked = 락 안 됨
wireless-row-locked = 스테이션, 락됨
wireless-row-holdover = 스테이션, 홀드오버 중
wireless-row-not-locked = 스테이션, 락 안 됨
wireless-row-access-point = 액세스 포인트, 스테이션 { $count }개
wireless-link = 링크
wireless-channel = 채널 { $channel }
wireless-not-known = 알 수 없음
wireless-signal = 신호 강도
wireless-rate = 송신 속도
wireless-ftm-valid = 유효 { $share }
wireless-rtt = 왕복 { $rtt }
wireless-bursts = 버스트당 { $count }프레임
wireless-not-as-capable = FALSE, { $reason }
wireless-reason-bursts = 액세스 포인트가 허용한 FTM 버스트가 3프레임 또는 2프레임이 아님
wireless-reason-measurement = 액세스 포인트와 FTM 및 TM 모두 사용할 수 없음
wireless-reason-signaling = 액세스 포인트로부터 gPTP 지원 Signaling 메시지 없음
wireless-reason-other = 프로파일에 정의되지 않은 이유
wireless-servo = 서보 오차
wireless-stations = 스테이션
wireless-station-count = 스테이션 { $count }개
wireless-no-ftm = FTM 미지원
wireless-unserved = 서비스되지 않는 리스너
wireless-frame-budget = 프레임 예산
wireless-frame-use = 초당 프레임 { $budget }개 중 { $used }개
wireless-stream-frames = 스트림 프레임
wireless-frames-of = 스테이션으로 { $readdressed }, 리스너 없음 { $unmapped }, 폐기 { $dropped }, 스테이션에서 { $restored }
wireless-class-a-allowed = 허용
wireless-class-a-not-allowed = 허용 안 됨
wireless-alarm-not-locked = Wi-Fi 시간이 액세스 포인트에 락되지 않음
wireless-alarm-holdover = Wi-Fi 시간 홀드오버 중, 액세스 포인트와 락 손실
wireless-alarm-unserved = Wi-Fi 포트의 리스너 { $count }개가 서비스되지 않음, 유니캐스트 한도 초과

## Log

log-all = 전체
log-warnings = 경고
log-pause = 일시 중지
log-resume = 재개
log-clear = 지우기
log-empty = triib 앱이 보내고 받는 모든 ATDECC 프레임이 최신순으로 여기에 표시됩니다.
log-none-match = 필터와 일치하는 프레임이 없습니다.
log-frames = 프레임 { $count }개
log-shown-of = 프레임 { $all }개 중 { $shown }개
log-sent = 송신
log-heard = 수신
log-not-decoded = 디코딩 안 됨
log-warning-short = control_data_length 값이 프레임 끝을 { $missing }옥텟 넘어섭니다.
log-warning-undecodable = 디코딩할 수 없습니다: { $error }.
log-warning-long-acmp = 긴 형식의 ACMP 프레임입니다. Milan 엔티티는 이 형식을 보낼 수 없습니다(Milan 1.3, 5.5.2.2).

## Channel mappings

mapping-section = 채널 매핑
mapping-inputs = 입력
mapping-outputs = 출력
mapping-port = 포트 { $number }
mapping-fixed = 고정
mapping-not-read = 아직 읽지 않았습니다.
mapping-no-clusters = 클러스터가 없습니다.
mapping-no-streams = 오디오 스트림이 없습니다.
mapping-none = 매핑이 없습니다.
mapping-not-mapped = 매핑 안 됨
mapping-cluster-numbered = 클러스터 { $index }

## Presets

presets-note = 프리셋에는 각 엔티티의 클럭 소스, 샘플링 레이트, 스트림 포맷, 컨트롤, 연결이 저장됩니다. 리콜하면 달라진 부분만 변경합니다.
presets-none = 저장된 프리셋이 아직 없습니다.
presets-connections = 연결 { $count }개
presets-recall = 리콜
presets-delete = 삭제
presets-no-place = 프리셋을 저장할 곳이 없습니다: 홈 폴더를 알 수 없습니다.
presets-undeletable = { $path }을(를) 삭제할 수 없습니다: { $error }.
presets-saved = “{ $name }”을(를) 저장했습니다(엔티티 { $count }개).
presets-nothing-differs = “{ $name }”과(와) 다른 점이 없습니다.
presets-recalling = “{ $name }” 리콜 중: 변경 { $count }건.
presets-missing = { $report } 없거나 읽지 않은 엔티티: { $missing }.
presets-deleted = “{ $name }”을(를) 삭제했습니다.
presets-host-note = 이 컴퓨터 자체의 엔티티도 저장하며, 리콜하면 다시 시작합니다.
presets-host-endpoints = 이 컴퓨터에 { $count }개
presets-starting-host = “{ $name }”을(를) 위해 이 컴퓨터의 엔티티를 시작하는 중입니다. 다시 나타나면 나머지를 적용합니다.

## Controls

control-numbered = 컨트롤 { $index }
control-not-shown = 여기에 표시되지 않음
control-option = 옵션 { $number }

## Network errors

network-permission = triib 앱에 원시 이더넷 프레임을 보내고 받을 권한이 필요합니다.
network-needs-npcap = triib 앱이 원시 이더넷 프레임을 보내고 받으려면 Npcap이 필요합니다.
network-npcap-administrators = Npcap이 관리자에게만 원시 이더넷 프레임 송수신을 허용합니다. triib 앱을 관리자 권한으로 실행하거나, 관리자 전용 옵션 없이 Npcap을 다시 설치하세요.

matrix-stream-format = { $format }.
matrix-stream-format-state = { $format }. { $state }.

## This computer's own entities

host-add-entity = 엔티티 추가
host-show-mine = 이 컴퓨터 자체의 엔티티만 표시
host-show-all = 모든 엔티티 표시
host-new-entity = 호스트 엔티티 { $number }
host-failed = 이 컴퓨터에 추가할 수 없음: { $reason }
host-needs-clock = 이 컴퓨터 자체의 엔티티에는 PTP 하드웨어 클럭이 있는 유선 인터페이스가 필요함
host-no-ptp4l = ptp4l이 응답하지 않아 이 컴퓨터의 스트림이 gPTP 시간을 유지할 수 없음
host-elsewhere = triib-endpointd 데몬이 { $interface }에서 다른 사용자 또는 root 계정으로 실행 중이므로 이 컴퓨터의 엔티티는 여기가 아니라 그쪽에서 실행됨
host-foreign-mrp = 다른 프로그램이 이 컴퓨터의 주소로 { $interface }에서 MSRP 또는 MVRP 선언을 보내고 있어 이 컴퓨터의 스트림에 필요한 것이 철회될 수 있음
host-alarm-foreign-mrp = 이 컴퓨터의 다른 프로그램이 { $interface }에서 MSRP 또는 MVRP 선언 중
host-state = 상태
host-streaming = 스트리밍 중
host-waiting = 리스너 대기 중
host-listening = 수신 대기 중
host-bound = 바인딩됨, 토커 대기 중
host-unbound = 바인딩 안 됨
host-audio-from = 오디오 입력
host-audio-to = 오디오 출력
host-channels = 채널 수
host-virtual-device = 가상 장치
host-add-stream = 스트림 추가
host-add-audio-input = 오디오 입력
host-add-audio-output = 오디오 출력
host-add-clock-input = 미디어 클럭 입력
host-add-clock-output = 미디어 클럭 출력
host-remove-stream = 스트림 제거
host-silence = 무음
host-tone = 테스트 톤
host-nowhere = 출력 안 함
host-default-device = 기본 장치
host-remove = 이 컴퓨터에서 제거

mac-audio-device = 오디오 기기
mac-no-audio-device = 연결된 스트림이 없는 동안은 없음
mac-entity-note = 이 엔티티는 macOS가 실행합니다. 스트림 중 하나가 연결되어 있는 동안 macOS는 앱이 재생하고 녹음하는 오디오 기기를 만들며, 방향별 채널 수는 구성에 따릅니다.
mac-open-audio-midi-setup = Audio MIDI 설정 열기
mac-avb-off = { $interface }에서 AVB가 꺼져 있어 이 Mac에는 자체 AVB 엔티티가 없습니다. Audio MIDI 설정의 네트워크 기기 브라우저에서 켜세요.

listen-section = 모니터
listen-start = 모니터
listen-stop = 모니터 중지
listen-source = 모니터 대상
listen-received = 받는 것
listen-sent = 스트림 { $number }(으)로 보내는 것({ $channels }채널)
listen-sent-needs-macos = 이 Mac이 보내는 것을 들으려면 macOS 14.2 이상이 필요합니다.
listen-channels = 채널
listen-channel = 채널 { $number }
listen-channel-pair = 채널 { $first }–{ $last }
listen-output = 출력
listen-default-output = 기본 출력
listen-gain = 게인
listen-gain-db = { $db } dB
listen-starting = 시작 중…
listen-waiting-input = { $device }을(를) 기다리는 중입니다. macOS는 엔티티의 스트림 중 하나가 연결된 동안 이 기기를 만듭니다.
listen-waiting-output = { $device }을(를) 기다리는 중입니다.
listen-playing = 재생 중, 최고 { $db } dB
listen-playing-silent = 재생 중, 무음
