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
| `packaging/macos/` | triib.app and its installer package, made by `bundle.sh --pkg` around `distribution.xml`, with the capture access helper |
| `.github/workflows/release.yml` | Builds all of them for a tag and drafts the GitHub release |
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
   artifact, or with "Packages to build" only one system's, such as
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

The app and package are signed ad hoc, so Gatekeeper blocks a downloaded
copy until the user chooses Open Anyway in System Settings, Privacy &
Security. A Developer ID Application certificate, with a Developer ID
Installer certificate for the package (`productsign`), and notarization
with `xcrun notarytool submit --wait` and `xcrun stapler staple`, remove
the warning.

An ad hoc signature also changes with every build, and macOS asks again
for the microphone and audio recording permissions Listen needs after
each update. `bundle.sh` signs with the identity named in
`TRIIB_SIGN_IDENTITY` where it is set, from the keychain in
`TRIIB_SIGN_KEYCHAIN`, unlocked with `TRIIB_SIGN_KEYCHAIN_PASSWORD`: the
Developer ID Application identity for a release. For development builds
a self-signed code signing certificate does, as macOS keeps the
permissions for every build signed by the same certificate; quit the app
before rebuilding it, as one that asks while its bundle is rewritten
loses them.
