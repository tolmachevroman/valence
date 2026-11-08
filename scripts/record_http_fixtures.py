#!/usr/bin/env python3
"""Record every PubChem and Wikipedia reply the fetcher needs, for offline tests.

Writes fixtures/http/molecules/<name>/ with one file per request. A request
that 404s is simply not saved; the test server answers 404 for missing files.

Usage: python3 scripts/record_http_fixtures.py [name ...]
"""

import json
import sys
import time
import urllib.error
import urllib.parse
import urllib.request
from pathlib import Path

USER_AGENT = "Valence/0.1.0 (+https://github.com/tolmachevroman/valence)"
OUT = Path(__file__).resolve().parent.parent / "fixtures" / "http" / "molecules"
PUG = "https://pubchem.ncbi.nlm.nih.gov/rest/pug/compound/cid"
VIEW = "https://pubchem.ncbi.nlm.nih.gov/rest/pug_view/data/compound"
WIKI = "https://en.wikipedia.org/w/api.php"

# Chosen for coverage: a full record, a "Danger" signal from company reports,
# several ECHA notification groups, no 3D conformer and missing sections, and
# two linked Wikipedia articles.
MOLECULES = {
    "caffeine": 2519,
    "capsaicin": 1548943,
    "menthol": 1254,
    "buckminsterfullerene": 123591,
    "ascorbic-acid": 54670067,
}
HEADINGS = [
    "Record Description",
    "Uses",
    "Natural Pollution Sources",
    "Toxicity Summary",
    "GHS Classification",
    "Wikipedia",
]


def slug(text):
    return text.lower().replace(" ", "-")


def get(url):
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    time.sleep(0.25)  # PubChem allows 5 requests/second
    try:
        with urllib.request.urlopen(req, timeout=15) as resp:
            return resp.read()
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return None
        raise


def record(name, cid):
    out = OUT / name
    out.mkdir(parents=True, exist_ok=True)
    for old in out.iterdir():
        old.unlink()

    def save(file, body):
        if body is not None:
            (out / file).write_bytes(body)

    props = "Title,MolecularFormula,MolecularWeight,IUPACName"
    save("properties.json", get(f"{PUG}/{cid}/property/{props}/JSON"))
    save("synonyms.json", get(f"{PUG}/{cid}/synonyms/JSON"))
    sdf = get(f"{PUG}/{cid}/record/SDF?record_type=3d")
    save("sdf-3d.sdf", sdf)
    if sdf is None:
        save("png.png", get(f"{PUG}/{cid}/PNG?image_size=500x500"))
    for heading in HEADINGS:
        q = urllib.parse.quote(heading)
        save(f"view-{slug(heading)}.json", get(f"{VIEW}/{cid}/JSON?heading={q}"))

    # Every article PubChem links, so the test can follow the same choice.
    links = out / "view-wikipedia.json"
    titles = []
    if links.exists():
        def walk(section):
            for info in section.get("Information", []):
                url = info.get("URL", "")
                if url.startswith("https://en.wikipedia.org/wiki/"):
                    titles.append(urllib.parse.unquote(url.rsplit("/", 1)[1]).replace("_", " "))
            for sub in section.get("Section", []):
                walk(sub)
        for section in json.loads(links.read_text())["Record"]["Section"]:
            walk(section)
    for title in titles:
        q = urllib.parse.urlencode({
            "action": "query", "prop": "extracts", "explaintext": 1, "redirects": 1,
            "titles": title, "format": "json", "formatversion": 2,
        })
        save(f"wikipedia-{slug(title)}.json", get(f"{WIKI}?{q}"))
    files = sorted(p.name for p in out.iterdir())
    print(f"{name:22} cid={cid:<10} {len(files)} files: {', '.join(files)}")


def main():
    wanted = set(sys.argv[1:])
    for name, cid in MOLECULES.items():
        if not wanted or name in wanted:
            record(name, cid)


if __name__ == "__main__":
    main()
