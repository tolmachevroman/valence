//! The local SQLite cache: everything fetched for a molecule, and the cards
//! written from it. Schema from the PRD, with the changes noted in `MIGRATIONS`.

use std::path::Path;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use rusqlite::{Connection, OptionalExtension, params};

use crate::card::Snippet;
use crate::ghs::Ghs;
use crate::pubchem::Compound;

/// Cached data older than this is refetched in the background when viewed.
pub const FRESH_FOR: Duration = Duration::from_secs(90 * 24 * 60 * 60);

/// One entry per schema version; `PRAGMA user_version` records how many ran.
///
/// Changes from the PRD schema: `molar_mass` is TEXT as PubChem formats it
/// ("180.20" must not become 180.2); `molecule` adds `cas`, `iupac_name` and
/// `png_2d` (the 2D drawing, so it works offline); `snippet` adds `origin`
/// (the data provider) and `position` (the order cards were measured with);
/// times are Unix seconds. The GHS snippet is stored like any other so a
/// Danger card can cite it.
const MIGRATIONS: &[&str] = &["
CREATE TABLE molecule (
  cid            INTEGER PRIMARY KEY,
  name           TEXT NOT NULL,
  formula        TEXT NOT NULL,
  molar_mass     TEXT NOT NULL,
  iupac_name     TEXT,
  cas            TEXT,
  synonyms_json  TEXT NOT NULL,
  sdf_3d         TEXT,
  png_2d         BLOB,
  ghs_json       TEXT,
  fetched_at     INTEGER NOT NULL
);

CREATE TABLE snippet (
  id         INTEGER PRIMARY KEY,
  cid        INTEGER NOT NULL REFERENCES molecule(cid) ON DELETE CASCADE,
  position   INTEGER NOT NULL,
  source     TEXT NOT NULL,
  section    TEXT NOT NULL,
  url        TEXT NOT NULL,
  origin     TEXT NOT NULL,
  text       TEXT NOT NULL,
  hash       TEXT NOT NULL
);
CREATE INDEX snippet_cid ON snippet(cid, position);

CREATE TABLE card (
  id           INTEGER PRIMARY KEY,
  cid          INTEGER NOT NULL REFERENCES molecule(cid) ON DELETE CASCADE,
  kind         TEXT NOT NULL,
  title        TEXT NOT NULL,
  body         TEXT NOT NULL,
  model        TEXT NOT NULL,
  created_at   INTEGER NOT NULL
);

CREATE TABLE card_source (
  card_id     INTEGER NOT NULL REFERENCES card(id) ON DELETE CASCADE,
  snippet_id  INTEGER NOT NULL REFERENCES snippet(id) ON DELETE CASCADE,
  PRIMARY KEY (card_id, snippet_id)
);

CREATE TABLE user_molecule (
  cid          INTEGER PRIMARY KEY REFERENCES molecule(cid) ON DELETE CASCADE,
  favorite     INTEGER NOT NULL DEFAULT 0,
  last_viewed  INTEGER
);
"];

/// Everything fetched for one molecule.
#[derive(Debug, Clone, PartialEq)]
pub struct Molecule {
    pub compound: Compound,
    pub sdf_3d: Option<String>,
    pub png_2d: Option<Vec<u8>>,
    pub ghs: Option<Ghs>,
    /// In card-input order. Loaded snippets carry their row id as `id`.
    pub snippets: Vec<Snippet>,
    /// Unix seconds.
    pub fetched_at: u64,
}

impl Molecule {
    pub fn is_fresh(&self, now: SystemTime) -> bool {
        let age = unix_seconds(now).saturating_sub(self.fetched_at);
        age < FRESH_FOR.as_secs()
    }
}

pub fn unix_seconds(t: SystemTime) -> u64 {
    t.duration_since(UNIX_EPOCH).map_or(0, |d| d.as_secs())
}

/// A stable 64-bit FNV-1a hash of the snippet text, as 16 hex digits. Stable
/// across Rust versions, unlike `DefaultHasher`, so stored hashes stay valid.
pub fn text_hash(text: &str) -> String {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in text.as_bytes() {
        h ^= u64::from(*b);
        h = h.wrapping_mul(0x0000_0100_0000_01b3);
    }
    format!("{h:016x}")
}

/// A card to store, citing snippets by row id.
#[derive(Debug, Clone)]
pub struct NewCard<'a> {
    pub cid: u64,
    pub kind: &'a str,
    pub title: &'a str,
    pub body: &'a str,
    /// Model file name, for regeneration when the model changes.
    pub model: &'a str,
    pub snippet_ids: &'a [i64],
    /// Unix seconds.
    pub created_at: u64,
}

pub struct Cache {
    conn: Connection,
}

impl Cache {
    pub fn open(path: &Path) -> rusqlite::Result<Self> {
        Self::init(Connection::open(path)?)
    }

    pub fn in_memory() -> rusqlite::Result<Self> {
        Self::init(Connection::open_in_memory()?)
    }

    fn init(conn: Connection) -> rusqlite::Result<Self> {
        conn.pragma_update(None, "foreign_keys", true)?;
        conn.pragma_update(None, "journal_mode", "WAL")?;
        let mut cache = Self { conn };
        cache.migrate()?;
        Ok(cache)
    }

    fn migrate(&mut self) -> rusqlite::Result<()> {
        let done: usize = self
            .conn
            .pragma_query_value(None, "user_version", |r| r.get(0))?;
        for (i, sql) in MIGRATIONS.iter().enumerate().skip(done) {
            let tx = self.conn.transaction()?;
            tx.execute_batch(sql)?;
            tx.pragma_update(None, "user_version", i + 1)?;
            tx.commit()?;
        }
        Ok(())
    }

    /// Store a fetch, replacing the previous one. Snippets whose text is
    /// unchanged keep their row id, so cards citing them stay; cards citing a
    /// snippet that changed or disappeared are deleted, to be written again.
    pub fn save(&mut self, m: &Molecule) -> rusqlite::Result<()> {
        let tx = self.conn.transaction()?;
        let c = &m.compound;
        tx.execute(
            "INSERT INTO molecule (cid, name, formula, molar_mass, iupac_name, cas, synonyms_json,
                                   sdf_3d, png_2d, ghs_json, fetched_at)
             VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10, ?11)
             ON CONFLICT(cid) DO UPDATE SET
               name = excluded.name, formula = excluded.formula, molar_mass = excluded.molar_mass,
               iupac_name = excluded.iupac_name, cas = excluded.cas,
               synonyms_json = excluded.synonyms_json, sdf_3d = excluded.sdf_3d,
               png_2d = excluded.png_2d, ghs_json = excluded.ghs_json,
               fetched_at = excluded.fetched_at",
            params![
                c.cid,
                c.title,
                c.formula,
                c.molar_mass,
                c.iupac_name,
                c.cas,
                serde_json::to_string(&c.synonyms).expect("strings serialize"),
                m.sdf_3d,
                m.png_2d,
                m.ghs
                    .as_ref()
                    .map(|g| serde_json::to_string(g).expect("GHS serializes")),
                m.fetched_at,
            ],
        )?;

        // Old snippets by (source, section, hash); each can be kept once.
        let mut old: Vec<(i64, String, String, String)> = {
            let mut stmt =
                tx.prepare("SELECT id, source, section, hash FROM snippet WHERE cid = ?1")?;
            stmt.query_map([c.cid], |r| {
                Ok((r.get(0)?, r.get(1)?, r.get(2)?, r.get(3)?))
            })?
            .collect::<rusqlite::Result<_>>()?
        };
        for (position, s) in m.snippets.iter().enumerate() {
            let hash = text_hash(&s.text);
            let kept = old.iter().position(|(_, source, section, h)| {
                *source == s.source && *section == s.section && *h == hash
            });
            match kept {
                Some(i) => {
                    let (id, ..) = old.swap_remove(i);
                    tx.execute(
                        "UPDATE snippet SET position = ?1, url = ?2, origin = ?3 WHERE id = ?4",
                        params![position, s.url, s.origin, id],
                    )?;
                }
                None => {
                    tx.execute(
                        "INSERT INTO snippet (cid, position, source, section, url, origin, text, hash)
                         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8)",
                        params![c.cid, position, s.source, s.section, s.url, s.origin, s.text, hash],
                    )?;
                }
            }
        }
        for (id, ..) in old {
            tx.execute(
                "DELETE FROM card WHERE id IN (SELECT card_id FROM card_source WHERE snippet_id = ?1)",
                [id],
            )?;
            tx.execute("DELETE FROM snippet WHERE id = ?1", [id])?;
        }
        tx.commit()
    }

    pub fn molecule(&self, cid: u64) -> rusqlite::Result<Option<Molecule>> {
        let row = self
            .conn
            .query_row(
                "SELECT name, formula, molar_mass, iupac_name, cas, synonyms_json, sdf_3d, png_2d,
                        ghs_json, fetched_at
                 FROM molecule WHERE cid = ?1",
                [cid],
                |r| {
                    let synonyms: String = r.get(5)?;
                    let ghs: Option<String> = r.get(8)?;
                    Ok(Molecule {
                        compound: Compound {
                            cid,
                            title: r.get(0)?,
                            formula: r.get(1)?,
                            molar_mass: r.get(2)?,
                            iupac_name: r.get(3)?,
                            cas: r.get(4)?,
                            synonyms: serde_json::from_str(&synonyms).unwrap_or_default(),
                        },
                        sdf_3d: r.get(6)?,
                        png_2d: r.get(7)?,
                        ghs: ghs.and_then(|g| serde_json::from_str(&g).ok()),
                        snippets: vec![],
                        fetched_at: r.get(9)?,
                    })
                },
            )
            .optional()?;
        let Some(mut m) = row else { return Ok(None) };
        let mut stmt = self.conn.prepare(
            "SELECT id, source, section, url, origin, text FROM snippet
             WHERE cid = ?1 ORDER BY position",
        )?;
        m.snippets = stmt
            .query_map([cid], |r| {
                Ok(Snippet {
                    id: r.get::<_, i64>(0)?.to_string(),
                    source: r.get(1)?,
                    section: r.get(2)?,
                    url: r.get(3)?,
                    origin: r.get(4)?,
                    text: r.get(5)?,
                })
            })?
            .collect::<rusqlite::Result<_>>()?;
        Ok(Some(m))
    }

    /// Store a card. Returns its id.
    pub fn save_card(&mut self, card: &NewCard) -> rusqlite::Result<i64> {
        let tx = self.conn.transaction()?;
        tx.execute(
            "INSERT INTO card (cid, kind, title, body, model, created_at) VALUES (?1, ?2, ?3, ?4, ?5, ?6)",
            params![card.cid, card.kind, card.title, card.body, card.model, card.created_at],
        )?;
        let id = tx.last_insert_rowid();
        for snippet in card.snippet_ids {
            tx.execute(
                "INSERT OR IGNORE INTO card_source (card_id, snippet_id) VALUES (?1, ?2)",
                [id, *snippet],
            )?;
        }
        tx.commit()?;
        Ok(id)
    }

    /// Card ids for a molecule, oldest first.
    pub fn card_ids(&self, cid: u64) -> rusqlite::Result<Vec<i64>> {
        let mut stmt = self
            .conn
            .prepare("SELECT id FROM card WHERE cid = ?1 ORDER BY id")?;
        stmt.query_map([cid], |r| r.get(0))?.collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn snippet(section: &str, text: &str) -> Snippet {
        Snippet {
            id: String::new(),
            source: "wikipedia".into(),
            section: section.into(),
            url: format!("https://en.wikipedia.org/wiki/Caffeine#{section}"),
            origin: "Wikipedia".into(),
            text: text.into(),
        }
    }

    fn caffeine(snippets: Vec<Snippet>, fetched_at: u64) -> Molecule {
        Molecule {
            compound: Compound {
                cid: 2519,
                title: "Caffeine".into(),
                formula: "C8H10N4O2".into(),
                molar_mass: "194.19".into(),
                iupac_name: Some("1,3,7-trimethylpurine-2,6-dione".into()),
                cas: Some("58-08-2".into()),
                synonyms: vec!["Guaranine".into(), "Theine".into()],
            },
            sdf_3d: Some("2519\n...M  END\n$$$$\n".into()),
            png_2d: None,
            ghs: Some(Ghs::from_snippet_text(
                "Signal word: Warning. Pictograms: Irritant. H302: Harmful if swallowed [Warning Acute toxicity, oral]",
            )),
            snippets,
            fetched_at,
        }
    }

    #[test]
    fn round_trips_a_molecule_in_order() {
        let mut cache = Cache::in_memory().unwrap();
        assert_eq!(cache.molecule(2519).unwrap(), None);
        let m = caffeine(
            vec![
                snippet("Introduction", "Caffeine is a stimulant."),
                snippet("History", "Runge isolated it in 1819."),
            ],
            1_700_000_000,
        );
        cache.save(&m).unwrap();
        let loaded = cache.molecule(2519).unwrap().unwrap();
        assert_eq!(loaded.compound, m.compound);
        assert_eq!((loaded.sdf_3d, loaded.ghs), (m.sdf_3d, m.ghs));
        let texts: Vec<_> = loaded.snippets.iter().map(|s| s.text.as_str()).collect();
        assert_eq!(
            texts,
            ["Caffeine is a stimulant.", "Runge isolated it in 1819."]
        );
        assert!(loaded.snippets.iter().all(|s| s.id.parse::<i64>().is_ok()));
    }

    #[test]
    fn refetch_keeps_unchanged_snippets_and_drops_stale_cards() {
        let mut cache = Cache::in_memory().unwrap();
        let intro = snippet("Introduction", "Caffeine is a stimulant.");
        let history = snippet("History", "Runge isolated it in 1819.");
        cache
            .save(&caffeine(vec![intro.clone(), history.clone()], 1))
            .unwrap();
        let ids: Vec<i64> = cache
            .molecule(2519)
            .unwrap()
            .unwrap()
            .snippets
            .iter()
            .map(|s| s.id.parse().unwrap())
            .collect();
        let card = |kind, cited| NewCard {
            cid: 2519,
            kind,
            title: "t",
            body: "b",
            model: "qwen",
            snippet_ids: cited,
            created_at: 2,
        };
        let keeps = cache.save_card(&card("where", &ids[..1])).unwrap();
        let goes = cache.save_card(&card("origin", &ids[1..])).unwrap();

        // History was edited upstream; the intro moved down a place.
        let edited = snippet("History", "Runge first isolated it in 1819.");
        cache.save(&caffeine(vec![edited, intro], 3)).unwrap();

        let loaded = cache.molecule(2519).unwrap().unwrap();
        assert_eq!(loaded.fetched_at, 3);
        assert_eq!(
            loaded.snippets[1].id,
            ids[0].to_string(),
            "unchanged snippet keeps its id"
        );
        assert_ne!(loaded.snippets[0].id, ids[1].to_string());
        assert_eq!(cache.card_ids(2519).unwrap(), [keeps]);
        assert_ne!(keeps, goes);
    }

    #[test]
    fn freshness_and_stable_hashes() {
        let m = caffeine(vec![], 1_000);
        let day = 24 * 60 * 60;
        assert!(m.is_fresh(UNIX_EPOCH + Duration::from_secs(1_000 + 89 * day)));
        assert!(!m.is_fresh(UNIX_EPOCH + Duration::from_secs(1_000 + 90 * day)));
        // FNV-1a test vectors.
        assert_eq!(text_hash(""), "cbf29ce484222325");
        assert_eq!(text_hash("a"), "af63dc4c8601ec8c");
    }

    #[test]
    fn migrations_run_once_on_a_file() {
        let dir = std::env::temp_dir().join(format!("valence-cache-test-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let path = dir.join("cache.sqlite");
        let _ = std::fs::remove_file(&path);
        Cache::open(&path)
            .unwrap()
            .save(&caffeine(vec![], 1))
            .unwrap();
        let reopened = Cache::open(&path).unwrap();
        assert!(reopened.molecule(2519).unwrap().is_some());
        let version: usize = reopened
            .conn
            .pragma_query_value(None, "user_version", |r| r.get(0))
            .unwrap();
        assert_eq!(version, MIGRATIONS.len());
        drop(reopened);
        let _ = std::fs::remove_dir_all(&dir);
    }
}
