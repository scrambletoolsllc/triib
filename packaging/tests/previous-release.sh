#!/usr/bin/env bash
# Downloads the last release's package of the same kind as PACKAGE into
# DIR, for the tests to upgrade from, and prints its path. Prints nothing
# when the last release is this version or has no such package. Needs gh
# signed in, as GH_TOKEN in CI.
#
#   packaging/tests/previous-release.sh dist/triib_1.2.0-1_amd64.deb previous
set -euo pipefail

package=$(basename "${1:?usage: previous-release.sh PACKAGE DIR}")
dir=${2:?usage: previous-release.sh PACKAGE DIR}
root=$(cd "$(dirname "$0")/../.." && pwd)
version=$(sed -n 's/^version = "\(.*\)"/\1/p' "$root/Cargo.toml" | head -1)

tag=$(gh release view --json tagName --jq .tagName 2>/dev/null) || exit 0
[ "$tag" != "v$version" ] || exit 0
pattern=${package//"$version"/*}
gh release download "$tag" --pattern "$pattern" --dir "$dir" --clobber >/dev/null 2>&1 || exit 0
find "$dir" -maxdepth 1 -name "$pattern" | head -1
