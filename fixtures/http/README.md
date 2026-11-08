# Recorded HTTP replies

Real PubChem and Wikipedia replies, so the data layer's tests run without the network (CI never calls either service).

- **`molecules/<name>/`:** every reply one full fetch needs, for 5 molecules, recorded by `scripts/record_http_fixtures.py` (November 2026). A request PubChem answers with 404 has no file. Chosen for coverage:
  - caffeine: a full record
  - capsaicin: a "Danger" signal word from company reports
  - menthol: several ECHA notification groups
  - buckminsterfullerene: no 3D conformer, no toxicity summary
  - ascorbic acid: not classified, and two linked Wikipedia articles
- **`pubchem/` and `wikipedia/`:** single replies for parser and client tests, including error replies.

Rerun the script to refresh. Some tests compare against the M0 spike fixtures, so a large upstream edit can make them fail; that means the recordings changed, not the code.

## Attribution

- Wikipedia text is © Wikipedia contributors, licensed under [CC BY-SA 4.0](https://creativecommons.org/licenses/by-sa/4.0/).
- PubChem data comes from the National Center for Biotechnology Information and the depositors each record names; see [PubChem's data policies](https://pubchem.ncbi.nlm.nih.gov/docs/about).
