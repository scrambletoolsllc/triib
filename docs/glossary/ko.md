# Korean (ko) terms

How `i18n/ko/triib.ftl` renders the glossary's roles and terms of art.

| English | Rendering | Note |
|---|---|---|
| talker | 토커 | The role IEEE 802.1Q and 1722.1 define; matrix header 토커 출력. |
| listener | 리스너 | Matrix header 리스너 입력. |
| grandmaster | 그랜드마스터 | Never a compound built on 마스터 alone. |
| entity | 엔티티 | Not 장치: an entity can be software. The spelling practitioners use; the standard loanword spelling is 엔터티. |
| entity model | 엔티티 모델 | |
| controller | 컨트롤러 | |
| stream | 스트림 | |
| stream input, stream output | 스트림 입력, 스트림 출력 | |
| connection, connect, bind | 연결, 연결하다, 바인딩 | "Bound" is 바인딩됨; disconnect is 연결 끊기. |
| media clock | 미디어 클럭 | 클럭 as audio and IT practitioners write it; the standard spelling is 클록. |
| clock domain | 클럭 도메인 | |
| clock source | 클럭 소스 | |
| sampling rate | 샘플링 레이트 | |
| bridge | 브리지 | The IEEE 802.1 term. 스위치 only where the English says switch (an AVB Lite fallback reason). |
| reservation | 예약 | As in 자원 예약; "reservation failed" is 예약 실패. |
| egress | 이그레스 | Egress port is 이그레스 포트. Chosen over 송신, which renders "sending". |
| link | 링크 | Link up / down: 링크 업 / 링크 다운. |
| peer delay | 피어 지연 | Link delay is 링크 지연. |
| offset | 오프셋 | |
| hop | 홉 | "Hops from grandmaster" is 그랜드마스터로부터 홉 수. |
| unicast, multicast | 유니캐스트, 멀티캐스트 | |
| fan-out | 팬아웃 | Fan-in (MSRP code 15) is 팬인. The AVB Lite fan-out line is a phrase. |
| descriptor | 디스크립터 | |
| cluster | 클러스터 | Not 그룹, which is the entity group. |
| stream port | 스트림 포트 | Shown as 포트 { $number } under channel mappings. |
| channel mapping | 채널 매핑 | Map / unmap: 매핑 / 매핑 해제. |
| control | 컨트롤 | |
| preset | 프리셋 | Recall is 리콜, as on mixing consoles. |
| identify | 식별 | As the Windows display settings say it; the IDENTIFY control is 식별 컨트롤. |
| counter | 카운터 | |
| locked, lost lock | 락, 락 손실 | The PLL usage; media lock is 미디어 락. Not 잠금, the lay word for a screen or door lock. |
| holding over | 홀드오버 | As in telecom and GNSS clocks. The state is 홀드오버 중, beside 락됨 and 락 안 됨. |
| station | 스테이션 | The IEEE 802.11 term (STA); "3 stations" is 스테이션 3개. |
| access point | 액세스 포인트 | Written with a space, as Korean networking manuals do. |
| beacon | 비콘 | Mode B is Mode B, 비콘 기반. |
| signal | 신호 강도 | The label for the RSSI in dBm. |
| interrupted | 중단 | |
| timestamp | 타임스탬프 | "Timestamp uncertain" is 타임스탬프 불확실. |
| advertise, advertised | 광고, 광고됨 | As network engineers say it of routes (경로 광고). Also for an entity "announcing itself" (ADP). |
| interface | 인터페이스 | |
| hardware clock | 하드웨어 클럭 | |
| virtual (interface) | 가상 | 가상 인터페이스. |

## Choices a native speaker should check

- 엔티티 over the standard spelling 엔터티.
- 클럭 over the standard spelling 클록, throughout (미디어 클럭, 클럭 소스, 하드웨어 클럭).
- 광고 for advertise, where Microsoft's Korean writes 알림 for router advertisements.
- 이그레스 for egress, rather than 송신 or 출력.
- 락 / 락 손실 for locked and lost lock.
- 리콜 for recalling a preset, rather than 불러오기.
- 업 for an interface that is up, beside 링크 다운.
- Particles after values use the 을(를), 과(와), (으)로 forms Korean software writes, or a noun such as 인터페이스 or 포맷 takes the particle.
- No particle is written against a Latin term, because the tests need each standards word on its own: "triib 앱이", "gPTP 미실행", "스트림 ID".
- 스테이션 for a Wi-Fi station, where consumer settings say 클라이언트. 락됨 / 락 안 됨 for the station's time, parallel to 동기화됨 / 동기화 안 됨.
