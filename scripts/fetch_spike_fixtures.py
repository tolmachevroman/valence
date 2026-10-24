#!/usr/bin/env python3
"""Fetch source snippets for the M0 model spike into fixtures/spike/.

A throwaway stand-in for the M1 data layer: pulls a few PubChem PUG-View
sections and the Wikipedia intro and History, and writes one JSON file per
molecule in the snippet shape the app will use.

Usage: python3 scripts/fetch_spike_fixtures.py [name ...]
"""

import json
import re
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

USER_AGENT = "Valence/0.1 (+https://github.com/tolmachevroman/valence)"
OUT_DIR = Path(__file__).resolve().parent.parent / "fixtures" / "spike"

# (PubChem name, Wikipedia title)
MOLECULES = [
    ("caffeine", "Caffeine"),
    ("capsaicin", "Capsaicin"),
    ("aspirin", "Aspirin"),
    ("vanillin", "Vanillin"),
    ("menthol", "Menthol"),
    ("nicotine", "Nicotine"),
    ("benzylpenicillin", "Benzylpenicillin"),
    ("morphine", "Morphine"),
    ("quinine", "Quinine"),
    ("dopamine", "Dopamine"),
    ("serotonin", "Serotonin"),
    ("limonene", "Limonene"),
    ("cinnamaldehyde", "Cinnamaldehyde"),
    ("theobromine", "Theobromine"),
    ("ethylene", "Ethylene"),
    ("nitroglycerin", "Nitroglycerin"),
    ("strychnine", "Strychnine"),
    ("tetrodotoxin", "Tetrodotoxin"),
    ("sucralose", "Sucralose"),
    ("thalidomide", "Thalidomide"),
    ("geosmin", "Geosmin"),
    ("allicin", "Allicin"),
    ("lycopene", "Lycopene"),
    ("artemisinin", "Artemisinin"),
    ("batrachotoxin", "Batrachotoxin"),
    ("ascorbic acid", "Vitamin C"),
    ("carvone", "Carvone"),
    ("indigo", "Indigo dye"),
    ("cubane", "Cubane"),
    ("buckminsterfullerene", "Buckminsterfullerene"),
]

# PUG-View headings and how many entries to keep from each. Manufacturing
# sections are deliberately absent: they never go into card input.
PUBCHEM_SECTIONS = [
    ("Record Description", 5),
    ("Uses", 4),
    ("Natural Pollution Sources", 3),
    ("Toxicity Summary", 2),
]
WIKI_SECTIONS = [("Introduction", 3), ("History", 4)]
# The PRD asks only for the intro and History, but surprising facts often live
# elsewhere (capsaicin's birds are under "Natural function"), so take a little
# from every other section too, except boilerplate and how-to-make content.
WIKI_OTHER_PER_SECTION = 2
WIKI_MAX_SNIPPETS = 20
WIKI_SKIP = re.compile(
    r"see also|references|external links|further reading|notes|bibliography|citations|sources"
    r"|synthesis|production|manufactur|preparation|extraction|isolation|purification",
    re.IGNORECASE,
)

MAX_CHARS = 700
MIN_CHARS = 40
PUBCHEM_DELAY = 0.25  # stay under PubChem's 5 requests/second


def get_json(url):
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            return json.load(resp)
    except urllib.error.HTTPError as e:
        if e.code == 404:  # PubChem: heading not present for this compound
            return None
        raise


def clean(text):
    text = re.sub(r"\s+", " ", text).strip()
    # Wikipedia's plain-text extract drops IPA, leaving "(, commonly )" or "()".
    text = re.sub(r"\s*\(\s*[,;][^()]*\)|\s*\(\s*\)", "", text)
    if len(text) <= MAX_CHARS:
        return text
    cut = text[:MAX_CHARS]
    end = cut.rfind(". ")
    return cut[: end + 1] if end > MIN_CHARS else cut.rstrip() + "…"


def pubchem_cid(name):
    q = urllib.parse.quote(name)
    data = get_json(f"https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/name/{q}/cids/JSON")
    time.sleep(PUBCHEM_DELAY)
    return data["IdentifierList"]["CID"][0]


def pubchem_heading(cid, heading):
    """Yield (text, origin) for every text entry under a PUG-View heading."""
    q = urllib.parse.quote(heading)
    data = get_json(f"https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound/{cid}/JSON?heading={q}")
    time.sleep(PUBCHEM_DELAY)
    if not data:
        return
    record = data["Record"]
    refs = {r["ReferenceNumber"]: r.get("SourceName", "") for r in record.get("Reference", [])}

    def walk(section):
        for info in section.get("Information", []):
            parts = info.get("Value", {}).get("StringWithMarkup", [])
            yield info, " ".join(p.get("String", "") for p in parts), refs.get(info.get("ReferenceNumber"), "")
        for sub in section.get("Section", []):
            yield from walk(sub)

    for section in record.get("Section", []):
        yield from walk(section)


def pubchem_snippets(cid, heading, limit):
    url = f"https://pubchem.ncbi.nlm.nih.gov/compound/{cid}#section={heading.replace(' ', '-')}"
    out = []
    for info, text, origin in pubchem_heading(cid, heading):
        if info.get("Name") == "See Also":
            continue
        text = clean(text)
        if len(text) < MIN_CHARS or any(text == s["text"] for s in out):
            continue
        out.append({"source": "pubchem_view", "section": heading, "url": url, "origin": origin, "text": text})
        if len(out) == limit:
            break
    return out


def ghs_snippets(cid):
    """Signal word, pictograms and hazard statements from the first listed authority."""
    url = f"https://pubchem.ncbi.nlm.nih.gov/compound/{cid}#section=GHS-Classification"
    by_origin = {}
    for info, text, origin in pubchem_heading(cid, "GHS Classification"):
        name = info.get("Name")
        entry = by_origin.setdefault(origin, {})
        if name == "Pictogram(s)":
            marks = info.get("Value", {}).get("StringWithMarkup", [{}])[0].get("Markup", [])
            entry["pictograms"] = [m["Extra"] for m in marks if m.get("Extra")]
        elif name == "Signal":
            entry["signal"] = text.strip()
        elif name == "GHS Hazard Statements":
            entry["statements"] = [s.get("String", "") for s in info["Value"]["StringWithMarkup"]]
    for origin, entry in by_origin.items():
        if entry.get("statements"):
            lines = []
            if entry.get("signal"):
                lines.append(f"Signal word: {entry['signal']}.")
            if entry.get("pictograms"):
                lines.append(f"Pictograms: {', '.join(entry['pictograms'])}.")
            lines.extend(entry["statements"])
            return [{"source": "pubchem_view", "section": "GHS Classification", "url": url, "origin": origin, "text": " ".join(lines)}]
    return []


def wikipedia_snippets(title):
    q = urllib.parse.urlencode({
        "action": "query", "prop": "extracts", "explaintext": 1, "redirects": 1,
        "titles": title, "format": "json", "formatversion": 2,
    })
    page = get_json(f"https://en.wikipedia.org/w/api.php?{q}")["query"]["pages"][0]
    if page.get("missing"):
        return []
    page_url = "https://en.wikipedia.org/wiki/" + urllib.parse.quote(page["title"].replace(" ", "_"))

    # Split the plain-text extract into top-level sections; subsections stay with their parent.
    sections, current = {"Introduction": []}, "Introduction"
    for line in page.get("extract", "").splitlines():
        top = re.fullmatch(r"== ([^=].*?) ==", line.strip())
        if top:
            current = top.group(1)
            sections.setdefault(current, [])
        elif line.strip() and not line.startswith("="):
            sections[current].append(line)

    plan = list(WIKI_SECTIONS)
    named = {name for name, _ in WIKI_SECTIONS}
    plan += [(name, WIKI_OTHER_PER_SECTION) for name in sections if name not in named and not WIKI_SKIP.search(name)]

    out = []
    for name, limit in plan:
        anchor = "" if name == "Introduction" else "#" + name.replace(" ", "_")
        kept = 0
        for para in sections.get(name, []):
            text = clean(para)
            if len(text) < MIN_CHARS:
                continue
            out.append({"source": "wikipedia", "section": name, "url": page_url + anchor, "origin": "Wikipedia", "text": text})
            kept += 1
            if kept == limit or len(out) == WIKI_MAX_SNIPPETS:
                break
        if len(out) == WIKI_MAX_SNIPPETS:
            break
    return out


def fetch(name, wiki_title):
    cid = pubchem_cid(name)
    snippets = []
    for heading, limit in PUBCHEM_SECTIONS[:1]:
        snippets += pubchem_snippets(cid, heading, limit)
    snippets += wikipedia_snippets(wiki_title)
    for heading, limit in PUBCHEM_SECTIONS[1:]:
        snippets += pubchem_snippets(cid, heading, limit)
    snippets += ghs_snippets(cid)
    # The same text can appear under several PubChem headings.
    seen = set()
    snippets = [s for s in snippets if not (s["text"] in seen or seen.add(s["text"]))]
    for i, s in enumerate(snippets, 1):
        s["id"] = f"s{i}"
    return {"molecule": name, "cid": cid, "snippets": [{"id": s.pop("id"), **s} for s in snippets]}


def main():
    wanted = set(sys.argv[1:])
    OUT_DIR.mkdir(parents=True, exist_ok=True)
    for name, wiki_title in MOLECULES:
        if wanted and name not in wanted:
            continue
        fixture = fetch(name, wiki_title)
        path = OUT_DIR / f"{name.replace(' ', '-')}.json"
        path.write_text(json.dumps(fixture, indent=2, ensure_ascii=False) + "\n")
        counts = {}
        for s in fixture["snippets"]:
            counts[s["section"]] = counts.get(s["section"], 0) + 1
        print(f"{name:22} cid={fixture['cid']:<10} {len(fixture['snippets']):>2} snippets  {counts}")


if __name__ == "__main__":
    main()
