#!/usr/bin/env python3
"""The vendor list triib names bridges by, from the IEEE's registries.

Writes crates/triib/src/vendors.tsv: one line per assignment, its prefix
in hex (6, 7 or 9 digits for an MA-L, MA-M, or an MA-S or IAB) and its
organization, sorted by prefix. The MA-L blocks the IEEE keeps for
itself, to hand out as MA-M, MA-S and IAB, are left out, so an address
in one is named by its own block or not at all. CIDs are left out too,
as no globally unique address starts with one.

The registries are kept in target/ieee/. Within a week of asking the
IEEE for them, the list is made from those copies without asking again;
after that, or with --force, the IEEE is asked only for what changed.

  make-vendors.py                makes the list
  make-vendors.py --force        asks the IEEE whatever the copies' age
"""

import csv
import email.utils
import io
import json
import sys
import time
import urllib.error
import urllib.request
from pathlib import Path

REGISTRIES = [
    "https://standards-oui.ieee.org/oui/oui.csv",
    "https://standards-oui.ieee.org/oui28/mam.csv",
    "https://standards-oui.ieee.org/oui36/oui36.csv",
    "https://standards-oui.ieee.org/iab/iab.csv",
]

DIGITS = {"MA-L": 6, "MA-M": 7, "MA-S": 9, "IAB": 9}

ITSELF = "IEEE Registration Authority"

ROOT = Path(__file__).resolve().parent.parent
OUT = ROOT / "crates/triib/src/vendors.tsv"
CACHE = ROOT / "target/ieee"
FRESH = 7 * 24 * 3600


def registry(url, force):
    """The registry's text and the day the IEEE last changed it."""
    copy = CACHE / url.rsplit("/", 1)[1]
    about_path = copy.with_name(copy.name + ".json")
    about = json.loads(about_path.read_text()) if copy.exists() and about_path.exists() else {}
    if about and not force and time.time() - about["asked"] < FRESH:
        print(f"{copy.name}: the copy from {day(about['asked'])}", file=sys.stderr)
        return copy.read_text(encoding="utf-8"), changed(about)
    # The IEEE's firewall turns away Python's own user agent.
    headers = {"User-Agent": "triib-make-vendors/1"}
    if about.get("etag"):
        headers["If-None-Match"] = about["etag"]
    if about.get("last_modified"):
        headers["If-Modified-Since"] = about["last_modified"]
    try:
        with urllib.request.urlopen(urllib.request.Request(url, headers=headers), timeout=120) as response:
            body = response.read()
            about = {"etag": response.headers.get("ETag"), "last_modified": response.headers.get("Last-Modified")}
        CACHE.mkdir(parents=True, exist_ok=True)
        copy.write_bytes(body)
        print(f"{copy.name}: downloaded, {len(body)} bytes", file=sys.stderr)
    except urllib.error.HTTPError as error:
        if error.code != 304 or not about:
            raise
        print(f"{copy.name}: unchanged since the copy", file=sys.stderr)
    about["asked"] = time.time()
    about_path.write_text(json.dumps(about))
    return copy.read_text(encoding="utf-8"), changed(about)


def changed(about):
    if about.get("last_modified"):
        return email.utils.parsedate_to_datetime(about["last_modified"]).strftime("%Y-%m-%d")
    return day(about["asked"])


def day(seconds):
    return time.strftime("%Y-%m-%d", time.gmtime(seconds))


def assignments(text):
    for row in csv.DictReader(io.StringIO(text)):
        kind = row["Registry"]
        prefix = row["Assignment"].strip().upper()
        name = " ".join(row["Organization Name"].split())
        if kind not in DIGITS or not name:
            continue
        if len(prefix) != DIGITS[kind]:
            print(f"skipped {kind} {prefix!r} of {name}: not {DIGITS[kind]} digits", file=sys.stderr)
            continue
        if kind == "MA-L" and name == ITSELF:
            continue
        yield prefix, name


def main(arguments):
    force = arguments == ["--force"]
    if arguments and not force:
        sys.exit(__doc__)
    names = {}
    sources = []
    for url in REGISTRIES:
        text, as_of = registry(url, force)
        sources.append(f"{url}, {as_of}")
        for prefix, name in assignments(text):
            # A few early MA-Ls list more than one organization; the first stands.
            names.setdefault(prefix, name)
    with OUT.open("w", encoding="utf-8", newline="\n") as out:
        out.write("# From the IEEE Registration Authority, each as it last changed:\n")
        for source in sources:
            out.write(f"# {source}\n")
        out.write("# Made by scripts/make-vendors.py; run it again for a newer list.\n")
        for prefix in sorted(names):
            out.write(f"{prefix}\t{names[prefix]}\n")
    print(f"{len(names)} assignments in {OUT}", file=sys.stderr)


if __name__ == "__main__":
    main(sys.argv[1:])
