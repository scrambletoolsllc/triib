#!/usr/bin/env python3
"""triib's website in every language the app speaks.

The English pages in site/ are the source. Their text is found block by
block, a paragraph, heading, list item, table cell, caption or link with
its inline markup, and the alt, title and aria-label of what has them;
each language's site/i18n/LANG.json gives those blocks translated, keyed
by a hash of the English. The build writes English at the root and each
language under LANG/, with a language menu, the page's other languages
for search engines, and right to left where the language reads so.

  site-i18n.py extract           writes site/i18n/en.json, the source
  site-i18n.py check [LANG...]   says what each language lacks or breaks
  site-i18n.py build OUT         writes the pages into OUT

In the catalog, a tag with attributes stands as a short name, such as
<a1>...</a1> for the block's first link, which the build puts back from
the English, and release numbers such as 2.2.2 stand as {v}: so a
translator sees only the words, and a new release or link does not make
the translations of a block stale.
"""

import hashlib
import html
import json
import re
import sys
from html.parser import HTMLParser
from pathlib import Path

ROOT = Path(__file__).resolve().parent.parent
SITE = ROOT / "site"
CATALOGS = SITE / "i18n"
APP_I18N = ROOT / "i18n"
PAGES = ["index.html", "guide.html", "specs.html"]
ORIGIN = "https://triib.run"
# Languages that read right to left.
RIGHT_TO_LEFT = {"ar", "fa", "he", "ur"}
# A language that holds only what it spells differently, and takes the
# rest from US English, as the app's own en-GB does.
SPELLING_ONLY = {"en-GB"}

# Elements whose content is one block of text to translate, unless one
# holds another; links and spans count only outside them.
BLOCKS = {
    "title", "h1", "h2", "h3", "h4", "p", "li", "td", "th", "dt", "dd",
    "figcaption", "summary", "caption", "label", "button", "option",
}
INLINE = {"a", "span"}
VOID = {
    "area", "base", "br", "col", "embed", "hr", "img", "input", "link",
    "meta", "source", "track", "wbr",
}
# Code samples stay as they are.
SKIP = {"script", "style", "pre"}
ATTRIBUTES = {"alt", "title", "aria-label", "placeholder"}
# Not \b, which an underscore defeats, as in triib_1.0.0-1_amd64.deb.
VERSION = re.compile(r"(?<![\d.])\d+\.\d+\.\d+(?![\d.])")
# Extra text the build adds to translated pages, not in the English.
EXTRAS = {
    "extra:policy-note": "This translation is given for convenience; the "
    "English version of this policy is the one that applies.",
}


def key_of(text):
    return hashlib.sha1(text.encode()).hexdigest()[:12]


START_TAG = re.compile(r"<([a-zA-Z][a-zA-Z0-9]*)(\s[^<>]*?)?(/?)>|</([a-zA-Z][a-zA-Z0-9]*)>")


def template(source):
    """The text of a block as the catalog keeps it, on one line with its
    tags' attributes and release numbers left out, and what puts them
    back: the tags by their short names, and the numbers in order."""
    tags = {}
    counts = {}
    open_names = []

    def shorten(match):
        name, attributes, void, closing = match.groups()
        if closing:
            # The short name of the open tag it closes, if it had one.
            for index in range(len(open_names) - 1, -1, -1):
                if open_names[index][0] == closing:
                    short = open_names.pop(index)[1]
                    return f"</{short}>" if short else match.group(0)
            return match.group(0)
        if not attributes or not attributes.strip():
            if name not in VOID and not void:
                open_names.append((name, None))
            return match.group(0)
        counts[name] = counts.get(name, 0) + 1
        short = f"{name}{counts[name]}"
        tags[short] = match.group(0)
        if name not in VOID and not void:
            open_names.append((name, short))
        return f"<{short}>"

    flat = " ".join(source.split())
    text = START_TAG.sub(shorten, flat)
    numbers = VERSION.findall(text)
    return VERSION.sub("{v}", text), tags, numbers


def normalized(source):
    return template(source)[0]


def restored(translated, source):
    """A block's translation with the English block's tags and numbers
    put back."""
    _, tags, numbers = template(source)
    numbers = iter(numbers)

    def put_back(match):
        closing, short = match.group(1), match.group(2)
        if short not in tags:
            return match.group(0)
        if closing:
            return f"</{re.match(r'[a-zA-Z]+', short).group(0)}>"
        return tags[short]

    text = re.sub(r"<(/?)([a-zA-Z]+\d+)>", put_back, translated)
    return re.sub(r"\{v\}", lambda _: next(numbers, "{v}"), text)


def has_words(source):
    text = html.unescape(re.sub(r"<[^>]+>", " ", source))
    return re.search(r"[^\W\d_]", text) is not None


class Segments(HTMLParser):
    """Finds the blocks and attributes of a page, by where they are in
    its source, and text outside any block, which would stay English."""

    def __init__(self, source):
        super().__init__(convert_charrefs=False)
        self.source = source
        self.lines = [0]
        for line in source.splitlines(keepends=True):
            self.lines.append(self.lines[-1] + len(line))
        # (tag, where its content starts, whether it is a block, whether
        # it holds one)
        self.stack = []
        self.blocks = []  # (start, end, source)
        self.attributes = []  # (start, end, value)
        self.stray = []
        self.skipping = 0

    def position(self):
        line, column = self.getpos()
        return self.lines[line - 1] + column

    def in_block(self):
        return any(entry[2] for entry in self.stack)

    def handle_starttag(self, tag, attrs):
        start = self.position()
        text = self.get_starttag_text()
        self.attribute_spans(start, text, tag, attrs)
        if tag in VOID:
            return
        if tag in SKIP:
            self.skipping += 1
        block = tag in BLOCKS or (tag in INLINE and not self.in_block())
        if block:
            for entry in self.stack:
                if entry[2]:
                    entry[3] = True
        self.stack.append([tag, start + len(text), block, False])

    def handle_startendtag(self, tag, attrs):
        self.attribute_spans(self.position(), self.get_starttag_text(), tag, attrs)

    def attribute_spans(self, start, text, tag, attrs):
        names = set(ATTRIBUTES)
        if tag == "meta" and dict(attrs).get("name") == "description":
            names.add("content")
        for name, value in attrs:
            if name not in names or not value or not has_words(value):
                continue
            found = re.search(
                r'\s%s\s*=\s*"([^"]*)"' % re.escape(name), text
            )
            if found:
                self.attributes.append(
                    (start + found.start(1), start + found.end(1), found.group(1))
                )

    def handle_endtag(self, tag):
        if tag in VOID:
            return
        end = self.position()
        while self.stack:
            open_tag, inner, block, holds = self.stack.pop()
            if open_tag in SKIP:
                self.skipping -= 1
            if block and not holds and open_tag == tag:
                content = self.source[inner:end]
                if has_words(content):
                    self.blocks.append((inner, end, content))
            if open_tag == tag:
                return

    def handle_data(self, data):
        if self.skipping or self.in_block() or not data.strip():
            return
        if has_words(data):
            self.stray.append(" ".join(data.split()))


def segments(page):
    source = (SITE / page).read_text()
    parser = Segments(source)
    parser.feed(source)
    parser.close()
    return source, parser


def catalog():
    """Every text of every page, in order, keyed by its hash."""
    texts = {}
    for page in PAGES:
        _, found = segments(page)
        if found.stray:
            sys.exit(f"{page}: text outside any block: {found.stray[:3]}")
        for _, _, content in found.blocks:
            text = normalized(content)
            texts.setdefault(key_of(text), {"page": page, "text": text})
        for _, _, value in found.attributes:
            text = normalized(value)
            texts.setdefault(key_of(text), {"page": page, "text": text, "attribute": True})
    for key, text in EXTRAS.items():
        texts[key] = {"page": "index.html", "text": text}
    return texts


def languages():
    """The app's languages, with their names in themselves."""
    found = {}
    for folder in sorted(APP_I18N.iterdir()):
        ftl = folder / "triib.ftl"
        if not ftl.is_file():
            continue
        values = dict(
            re.findall(r"^(language-name|settings-language) = (.*)$", ftl.read_text(), re.M)
        )
        found[folder.name] = values
    english = found["en"]
    for values in found.values():
        values.setdefault("settings-language", english["settings-language"])
    # The site's English is the app's US English, named plainly.
    found["en"]["language-name"] = "English"
    return found


def translations(language):
    path = CATALOGS / f"{language}.json"
    return json.loads(path.read_text()) if path.is_file() else {}


TAG = re.compile(r"</?([a-zA-Z][a-zA-Z0-9]*)")


def problems(language, source_texts):
    """What LANG's catalog lacks or breaks: missing texts, texts the
    English no longer has, and inline markup or {v} that does not match."""
    found = translations(language)
    said = []
    for key, entry in source_texts.items():
        if key not in found:
            if language not in SPELLING_ONLY:
                said.append(f"missing {key}: {entry['text'][:60]}")
            continue
        translated = found[key]
        if sorted(TAG.findall(translated)) != sorted(TAG.findall(entry["text"])):
            said.append(f"markup differs in {key}: {translated[:60]}")
        if translated.count("{v}") != entry["text"].count("{v}"):
            said.append(f"{{v}} differs in {key}")
        for code in re.findall(r"<code>(.*?)</code>", entry["text"]):
            if f"<code>{code}</code>" not in translated and code.isascii():
                said.append(f"<code>{code}</code> changed in {key}")
    for key in found:
        if key not in source_texts:
            said.append(f"no longer in the English: {key}")
    return said


def asset(url):
    """Whether a relative URL is a file beside the pages, which a page one
    folder down reaches with ../, rather than another page."""
    if re.match(r"^([a-z]+:|#|/)", url):
        return False
    path = url.split("#")[0]
    return path not in ("", "./") and not path.endswith(".html")


def language_menu(page, current, names):
    label = html.escape(names[current]["settings-language"])
    items = []
    for code, values in names.items():
        href = page_url(code, page, relative_from=current)
        name = html.escape(values["language-name"])
        here = ' aria-current="page"' if code == current else ""
        items.append(f'<li><a href="{href}" hreflang="{code}" lang="{code}"{here}>{name}</a></li>')
    globe = (
        '<svg viewBox="0 0 24 24" width="20" height="20" aria-hidden="true">'
        '<path fill="currentColor" d="M12 2a10 10 0 1 0 0 20 10 10 0 0 0 0-20Zm6.9 6h-2.9a15.7 '
        "15.7 0 0 0-1.4-3.6A8 8 0 0 1 18.9 8ZM12 4c.8 1.2 1.5 2.5 1.9 4h-3.8c.4-1.5 1.1-2.8 "
        "1.9-4ZM4.3 14a8.2 8.2 0 0 1 0-4h3.4a16.5 16.5 0 0 0 0 4H4.3Zm.8 2h3a15.7 15.7 0 0 0 1.3 "
        "3.6A8 8 0 0 1 5.1 16Zm3-8h-3a8 8 0 0 1 4.3-3.6A15.7 15.7 0 0 0 8.1 8ZM12 20c-.8-1.2-1.5-"
        "2.5-1.9-4h3.8c-.4 1.5-1.1 2.8-1.9 4Zm2.3-6H9.7a14.7 14.7 0 0 1 0-4h4.6a14.7 14.7 0 0 1 0 "
        "4Zm.3 5.6c.6-1.1 1-2.3 1.3-3.6h3a8 8 0 0 1-4.3 3.6Zm1.7-5.6a16.5 16.5 0 0 0 0-4h3.4a8.2 "
        '8.2 0 0 1 0 4h-3.4Z"/></svg>'
    )
    return (
        f'<details class="languages"><summary aria-label="{label}">{globe}'
        f'<span>{html.escape(names[current]["language-name"])}</span></summary>'
        f'<ul>{"".join(items)}</ul></details>'
        # Closes the menu on a click or Escape elsewhere, as a menu does.
        "<script>(function () { var menu = document.currentScript.previousElementSibling;"
        ' document.addEventListener("click", function (event) {'
        " if (!menu.contains(event.target)) menu.open = false; });"
        ' document.addEventListener("keydown", function (event) {'
        ' if (event.key === "Escape") menu.open = false; }); })();</script>'
    )


def page_url(language, page, relative_from=None):
    """Where PAGE in LANGUAGE is: absolute, or from a page in another."""
    folder = "" if language == "en" else f"{language}/"
    name = "" if page == "index.html" else page
    if relative_from is None:
        return f"{ORIGIN}/{folder}{name}"
    up = "" if relative_from == "en" else "../"
    return f"{up}{folder}{name}" or "./"


# On an English page reached from another site, a reader whose browser
# prefers one of the site's languages is taken to that one. A reader
# who chose English in the menu came from the site itself, so stays.
REDIRECT = """<script>
(function () {
  var languages = %s;
  try {
    if (document.referrer && new URL(document.referrer).host === location.host) return;
  } catch (e) {}
  var wanted = navigator.languages || [navigator.language || ""];
  for (var i = 0; i < wanted.length; i++) {
    var tag = wanted[i].toLowerCase();
    var match = languages[tag] || languages[tag.split("-")[0]];
    // English other than British stays here.
    if (!match && tag.split("-")[0] === "en") return;
    if (match) {
      location.replace(match + "/" + location.pathname.split("/").pop() + location.hash);
      return;
    }
  }
})();
</script>"""


def redirect_script(names):
    table = {}
    for code in names:
        if code == "en":
            continue
        table[code.lower()] = code
        base = code.split("-")[0].lower()
        # A bare language goes to its first variant: pt to pt-BR, zh to
        # zh-CN; bare English is the site's own.
        if base != "en":
            table.setdefault(base, code)
    # English spelled as in Britain.
    for region in ("au", "nz", "ie", "za", "in"):
        table[f"en-{region}"] = "en-GB"
    table["zh-hk"] = table["zh-mo"] = table.get("zh-tw", "zh-TW")
    table["zh-hant"] = "zh-TW"
    table["zh-hans"] = "zh-CN"
    # Norwegian as browsers also name it.
    table["no"] = table["nn"] = "nb"
    return REDIRECT % json.dumps(table, sort_keys=True)


def build_page(page, language, names, source_texts):
    source, found = segments(page)
    found_translations = {} if language == "en" else translations(language)
    replacements = []
    for start, end, content in found.blocks + found.attributes:
        text = normalized(content)
        translated = found_translations.get(key_of(text))
        if translated is not None:
            replacement = restored(translated, content)
            if (start, end) in {(a, b) for a, b, _ in found.attributes}:
                replacement = html.escape(html.unescape(replacement), quote=True)
            replacements.append((start, end, replacement))
    out = source
    for start, end, replacement in sorted(replacements, reverse=True):
        out = out[:start] + replacement + out[end:]

    direction = ' dir="rtl"' if language in RIGHT_TO_LEFT else ""
    out = out.replace('<html lang="en">', f'<html lang="{language}"{direction}>', 1)
    if language != "en":
        out = re.sub(
            r'(\s(?:src|href)=")([^"]+)"',
            lambda m: f'{m.group(1)}../{m.group(2)}"' if asset(m.group(2)) else m.group(0),
            out,
        )
        # Each of a srcset's images, before its density.
        out = re.sub(
            r'(\ssrcset=")([^"]+)"',
            lambda m: m.group(1)
            + ", ".join(
                f"../{item}" if asset(item.split()[0]) else item
                for item in map(str.strip, m.group(2).split(","))
            )
            + '"',
            out,
        )
    alternates = "".join(
        f'<link rel="alternate" hreflang="{code}" href="{page_url(code, page)}">\n'
        for code in names
    )
    alternates += f'<link rel="alternate" hreflang="x-default" href="{page_url("en", page)}">\n'
    head_extra = alternates + (redirect_script(names) + "\n" if language == "en" else "")
    out = out.replace("</head>", head_extra + "</head>", 1)
    out = re.sub(
        r"(<header class=\"top\">.*?)(\s*</nav>)",
        lambda m: m.group(1) + "\n    " + language_menu(page, language, names) + m.group(2),
        out,
        count=1,
        flags=re.S,
    )
    note = found_translations.get("extra:policy-note")
    if note and page == "index.html":
        out = re.sub(
            r'(<section id="code-signing" class="policy">\s*<h2>.*?</h2>)',
            lambda m: m.group(1) + f'\n    <p class="small">{note}</p>',
            out,
            count=1,
            flags=re.S,
        )
    return out


def main(argv):
    command = argv[1] if len(argv) > 1 else ""
    if command == "extract":
        CATALOGS.mkdir(exist_ok=True)
        texts = catalog()
        (CATALOGS / "en.json").write_text(
            json.dumps(texts, ensure_ascii=False, indent=1) + "\n"
        )
        print(f"{len(texts)} texts in site/i18n/en.json")
    elif command == "check":
        texts = catalog()
        wanted = argv[2:] or [code for code in languages() if code != "en"]
        failed = False
        for language in wanted:
            said = problems(language, texts)
            if said:
                failed = True
                print(f"{language}: {len(said)} problems")
                for line in said[:20]:
                    print(f"  {line}")
            else:
                print(f"{language}: ok")
        sys.exit(1 if failed else 0)
    elif command == "build" and len(argv) == 3:
        out = Path(argv[2])
        names = languages()
        texts = catalog()
        for language in names:
            folder = out if language == "en" else out / language
            folder.mkdir(parents=True, exist_ok=True)
            for page in PAGES:
                (folder / page).write_text(build_page(page, language, names, texts))
        print(f"Built {len(PAGES)} pages in {len(names)} languages")
    else:
        sys.exit(__doc__)


if __name__ == "__main__":
    main(sys.argv)
