# Releasing triib

Everything a release ships is built from this repository:

| Path | What |
|---|---|
| `data/io.github.scrambletools.triib.desktop` | Desktop entry |
| `data/io.github.scrambletools.triib.metainfo.xml` | AppStream metadata, with the release history |
| `data/icons/triib-icon.svg` | The app icon, which `scripts/make-icons.sh` renders into `data/icons/hicolor/` (which every package picks up), `data/icons/triib-1024.png` for macOS and `packaging/windows/triib.ico` |
| `packaging/licenses/` | The licenses of the fonts triib carries, from scramble-ui |
| `scripts/cross-setup.sh` | Sets up an x86_64 Ubuntu to cross-compile for RISC-V |
| `scripts/dist-install.sh` | Installs a build of triib, triib-cli and triib-endpointd and the files above into a system layout; the Linux packages use it |
| `packaging/nfpm.yaml`, `packaging/linux/postinstall.sh` | Debian and RPM packages, made with nfpm, which grant the programs `CAP_NET_RAW` as they install and run `packaging/linux/endpoint-check` |
| `packaging/arch/triib/` | AUR package, whose install script does the same |
| `packaging/windows/` | Windows MSI (WiX 5) and zip, made by `build.ps1` |
| `packaging/macos/` | triib.app and its installer package, made by `bundle.sh --pkg` around `distribution.xml`, signed and notarized as [macOS signing](#macos-signing) says, with the capture access helper |
| `.github/workflows/release.yml` | Builds all of them for a tag and drafts the GitHub release once the installer tests pass |
| `packaging/tests/` | The installer tests: each package installed, upgraded, run and removed in a clean system, as [Installer tests](#installer-tests) says |
| `site/`, `scripts/build-site.sh`, `.github/workflows/site.yml` | The website, published to GitHub Pages whenever it changes on main |
| `site/i18n/`, `scripts/site-i18n.py` | The website's text in every language, built into the pages; `docs/GLOSSARY.md` says how |
| `scripts/site-screenshots.sh` | Renders the website's screenshots into `docs/screenshots` from the test bench |

Each package is built for x86_64 and ARM64 on GitHub's runners of that
architecture, and the Linux packages also for RISC-V, cross-compiled by
`scripts/cross-setup.sh` and checked under QEMU; the macOS package is
built for Apple Silicon only.

Packages are built with `TRIIB_PRODUCTION=1`, which makes the installed
copy: settings in `triib.toml` (`~/.config` on Linux, `%APPDATA%\triib`
on Windows, `~/Library/Application Support/triib` on macOS), cache under
`triib`, app id `io.github.scrambletools.triib`. Builds without it are
development builds, which keep their own.

## What each system needs

triib sends and receives raw Ethernet, which each system guards:

- **Linux:** the `CAP_NET_RAW` capability. The .deb, .rpm and AUR
  packages set it on `/usr/bin/triib`, `/usr/bin/triib-cli` and
  `/usr/bin/triib-endpointd` when they install (`setcap`, from libcap).
  The tarball cannot; after unpacking, run `sudo setcap cap_net_raw+ep`
  on the three programs. There is no AppImage or Flatpak, as neither can
  carry the capability. The packages depend on ALSA's and PipeWire's
  libraries for audio, PipeWire's for the virtual audio devices of this
  computer's streams, and recommend linuxptp, whose ptp4l the host
  entities need; triib-endpointd reads the PTP hardware clock, which
  systemd's rules let everyone read. As they install, the packages say
  when no wired interface has a PTP hardware clock, which using this
  computer as an AVB endpoint requires, with the Ethernet cards that have
  one and that no Wi-Fi card can serve yet.
- **macOS:** read and write access to the BPF devices, `/dev/bpf*`,
  which are root's alone at every start. The installer package puts
  `triib.app` in /Applications, links `triib-cli` into /usr/local/bin,
  adds the user installing it to the `access_bpf` group, and installs the
  launch daemon `io.github.scrambletools.triib.bpf`, which opens the
  devices to that group at every start. Wireshark's ChmodBPF uses the
  same group, so either serves both. Group membership takes effect at
  the next login. `sudo "/Library/Application Support/triib/uninstall"`
  removes it all. macOS never hands its own AVB entity the commands
  written to BPF, so triib reaches that entity through Apple's
  AudioVideoBridging framework, which needs nothing more. As it opens,
  the installer warns when the Mac has no AVB Ethernet port, one whose
  controller has a realtime queue, which using this Mac as an AVB
  endpoint requires; the install goes on either way.
- **Windows:** Npcap, from https://npcap.com. Its license does not let
  other installers carry it, so the MSI looks for it and, when it is
  missing, offers Npcap's download page on its last page instead of
  opening triib; triib itself says the same, with a link, until Npcap is
  installed. Npcap installed with "Restrict Npcap driver's access to
  Administrators only" makes triib need to run as administrator, which
  triib explains. The MSI installs for the current user, with no
  administrator prompt, and puts triib's folder on the user's PATH for
  triib-cli. Its last page says that this computer cannot be an AVB
  endpoint on Windows yet.

## Steps

1. Update the version in `Cargo.toml`, the workspace version and the
   versions of the `triib-*` crates under `[workspace.dependencies]`, and
   run `cargo check` so `Cargo.lock` follows; update `pkgver` in
   `packaging/arch/triib/PKGBUILD` and add a `<release>` to the
   metainfo.
2. Write the version's section in `CHANGELOG.md`, as `## [X.Y.Z]`; the
   release notes are taken from it, or made by GitHub when there is none.
   Changes go under `## [Unreleased]` as they land, which the release
   renames to `## [X.Y.Z] - YYYY-MM-DD`.
3. Try the packaging without releasing: run the Release workflow by hand
   (Actions, Release, Run workflow), which builds every package as an
   artifact and runs the installer tests on them, or with "Packages to
   build" only one system's, such as
   `gh workflow run release.yml -f only=windows`.
4. Tag and push: `git tag vX.Y.Z && git push origin vX.Y.Z`. The workflow
   checks that the tag matches `Cargo.toml`, builds the packages and
   drafts a release with them, `SHA256SUMS`, and a `PKGBUILD` with the
   source checksum filled in, beside its `triib.install`. Check the draft
   and publish it. Then point the website's download section
   (`site/index.html`) at the version's packages, put the version on its
   Download button, and update the version on `site/specs.html`; pushing
   `site/` redeploys the website. A release number is `{v}` in the
   translations, so they need nothing new; changed sentences do (see
   `scripts/site-i18n.py check`).
5. AUR: in a clone of `ssh://aur@aur.archlinux.org/triib.git`, replace
   `PKGBUILD` with the one from the release and copy
   `packaging/arch/triib/triib.install`, run
   `makepkg --printsrcinfo >.SRCINFO`, build it once with `makepkg`,
   commit and push.

## Installer tests

GitHub's runners have far more installed than the computers triib goes
to, which can hide what a package leaves out: Windows runners have the
Visual C++ runtime, which a fresh Windows lacks. So the release workflow
tests each package in a clean system, as someone installing it would
meet it, and drafts the release only once every test passes. Each test
installs the last release's package first, when there is an older one,
so the new one upgrades it, and removes triib at the end.

| Package | Where | What |
|---|---|---|
| .deb, .rpm | Fresh containers of Debian 13, Ubuntu 24.04 and 26.04 and Fedora 44, x86_64 and ARM64; Ubuntu 24.04 for RISC-V, under QEMU | `packaging/tests/linux/test-package.sh`: installs with the package's dependencies only, then checks its files, `CAP_NET_RAW`, the endpoint check's warning, that every shared library is found, each program's version as a regular user, raw Ethernet as that user, and the window opening and drawing under a headless sway; then that removing leaves nothing |
| Arch | A fresh Arch container | `build-arch.sh` builds the PKGBUILD from this tree, then the same test |
| MSI | GitHub's Windows Server 2025 and Windows 11 ARM runners | `packaging/tests/windows/test-msi.ps1`: checks every DLL the programs import against those Windows itself provides, then installs and checks the files, the entry in Apps, the PATH, the Start menu shortcut, each program and the window; then that uninstalling leaves nothing |
| MSI, x64 | A Windows Server 2022 container, which has nothing installed | The same, without the window |
| .pkg | Fresh macOS 14, 15 and 26 runners | `packaging/tests/macos/test-pkg.sh`: Gatekeeper's verdict on a downloaded copy, that the programs link only macOS's own libraries and run on the oldest macOS the app claims, then installs and checks the app, triib-cli, the BPF launch daemon and access for the user, and the window; then that the uninstall script leaves nothing |

To run one by hand:

- Linux, with Docker: `packaging/tests/linux/run.sh debian:13
  dist/triib_X.Y.Z-1_amd64.deb [previous.deb]`.
- Windows, on a computer without triib installed, as it installs and
  removes it: `powershell -ExecutionPolicy Bypass -File
  packaging\tests\windows\test-msi.ps1 -Msi triib-X.Y.Z-x64.msi`. For a
  fully clean Windows 11, run it in Windows Sandbox.
- macOS, on a Mac without triib installed, with sudo:
  `packaging/tests/macos/test-pkg.sh triib-X.Y.Z-arm64.pkg`.

When a new DLL fails the Windows check, link it in or install it beside
triib; add it to the script's list only when every Windows triib
supports provides it.

## Windows code signing

Until releases are signed, Windows shows SmartScreen's warning for the
MSI and the programs. The workflow signs them with Microsoft's Azure
Artifact Signing once the repository variables `AZURE_CLIENT_ID`,
`AZURE_TENANT_ID`, `AZURE_SUBSCRIPTION_ID`, `AZURE_SIGNING_ENDPOINT`,
`AZURE_SIGNING_ACCOUNT` and `AZURE_CERTIFICATE_PROFILE` are set, signing
in with GitHub's OpenID Connect token as the environment `release`. The
signing account, identity validation and certificate profile are shared
with prev; the app registration, whose ID is `AZURE_CLIENT_ID`, is
triib's own, and triib.run is its publisher domain, verified by
`site/.well-known/microsoft-identity-association.json`. The app
registration has a federated credential for this repository with the
issuer `https://token.actions.githubusercontent.com`, the audience
`api://AzureADTokenExchange` and the subject
`repo:scrambletoolsllc@339905752/triib@1406492142:environment:release`.
GitHub puts the organization's and repository's IDs in the subject, so
the portal's GitHub Actions scenario, which leaves them out, does not
match; the azure/login step prints the subject a run presents.

## macOS signing

Releases carry Scramble Tools' Developer ID signatures and are
notarized, so Gatekeeper opens a downloaded copy without asking, and
macOS keeps the microphone and audio recording permissions Listen needs
across updates, as every release is signed by the same team. They are
always the organization's certificates, never a personal one: the
workflow checks each signature against the identity its variables name.

`bundle.sh` signs both programs with the hardened runtime, which
notarization asks for, and gives the app the
`com.apple.security.device.audio-input` entitlement
(`packaging/macos/triib.entitlements`), without which the hardened
runtime keeps the microphone from it. It signs with the identity named
in `TRIIB_SIGN_IDENTITY` where it is set, else ad hoc, from the keychain
in `TRIIB_SIGN_KEYCHAIN`, unlocked with `TRIIB_SIGN_KEYCHAIN_PASSWORD`,
and the package with `TRIIB_INSTALLER_IDENTITY`. Given an App Store
Connect API key, `TRIIB_NOTARY_KEY` (its .p8 file), `TRIIB_NOTARY_KEY_ID`
and `TRIIB_NOTARY_ISSUER`, it has Apple's notary service check the
package, waits for it, and staples the ticket to the package.

The release workflow's macOS job runs in the environment `release` and
reads:

| Name | Kind | Holds |
|---|---|---|
| `MACOS_APP_IDENTITY` | variable | `Developer ID Application: <team name> (<team ID>)`, exactly as the certificate names it |
| `MACOS_INSTALLER_IDENTITY` | variable | `Developer ID Installer: <team name> (<team ID>)` |
| `APPLE_API_KEY_ID` | variable | The App Store Connect API key's ID |
| `APPLE_API_ISSUER` | variable | Its issuer ID |
| `MACOS_APP_P12` | secret | The Developer ID Application certificate with its private key, a .p12 file in base64 |
| `MACOS_INSTALLER_P12` | secret | The Developer ID Installer certificate with its private key, likewise |
| `MACOS_P12_PASSWORD` | secret | The password of both .p12 files |
| `APPLE_API_KEY` | secret | The API key's .p8 file in base64 |

Until `MACOS_APP_IDENTITY` is set the job signs ad hoc. Once it is, the
job imports both identities into a keychain of its own for the run,
signs and notarizes, then checks that both programs are signed by
`MACOS_APP_IDENTITY`, the package by `MACOS_INSTALLER_IDENTITY`, that the
package is notarized with its ticket stapled, and that Gatekeeper
accepts the package and the app; any of them failing fails the run.

Only the team's Account Holder can create Developer ID certificates, at
developer.apple.com under Certificates: Developer ID Application and
Developer ID Installer, each with the G2 Sub-CA, from a certificate
signing request whose private key stays with whoever made it. The notary
key is an App Store Connect team key with the Developer role, and its
.p8 file can be downloaded only once. The current certificates expire
on November 15, 2027; a renewed one needs its secret replaced, and its
variable too if the team's name has changed. No App ID or provisioning
profile is needed: the microphone entitlement is not one Apple
restricts.

For development builds a self-signed code signing certificate does, as
macOS keeps the permissions for every build signed by the same
certificate; quit the app before rebuilding it, as one that asks while
its bundle is rewritten loses them.
