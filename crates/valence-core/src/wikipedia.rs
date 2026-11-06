//! English Wikipedia: the article PubChem links for a compound, split into
//! section snippets.

use std::sync::LazyLock;

use regex::Regex;
use reqwest::Url;
use serde_json::Value;

use crate::card::Snippet;
use crate::error::Error;
use crate::http::{Host, Http};
use crate::pubchem::PubChem;
use crate::text::{MIN_CHARS, clean};

pub const API: &str = "https://en.wikipedia.org/w/api.php";
const ARTICLE_BASE: &str = "https://en.wikipedia.org/wiki/";

/// Sections always taken, and how many paragraphs from each.
const SECTIONS: [(&str, usize); 2] = [("Introduction", 3), ("History", 4)];
/// The PRD asks only for the intro and History, but surprising facts often
/// live elsewhere (capsaicin's birds are under "Natural function"), so a
/// little comes from every other section too.
const OTHER_PER_SECTION: usize = 2;
const MAX_SNIPPETS: usize = 20;

/// Boilerplate, and how-it's-made sections that must never reach a card.
static SKIP: LazyLock<Regex> = LazyLock::new(|| {
    Regex::new(
        r"(?i)see also|references|external links|further reading|notes|bibliography|citations|sources|synthesis|production|manufactur|preparation|extraction|isolation|purification",
    )
    .unwrap()
});
static TOP_HEADING: LazyLock<Regex> = LazyLock::new(|| Regex::new(r"^== ([^=].*?) ==$").unwrap());

/// An article's title and plain text, as the API returns them.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Article {
    pub title: String,
    pub text: String,
}

#[derive(Clone)]
pub struct Wikipedia {
    http: Http,
    api: Url,
}

impl Wikipedia {
    pub fn new(http: Http) -> Self {
        Self::with_api(http, API)
    }

    pub fn with_api(http: Http, api: &str) -> Self {
        Self {
            http,
            api: Url::parse(api).expect("valid API URL"),
        }
    }

    /// The plain text of `title`, following redirects; `None` if there is no
    /// such article.
    pub async fn article(&self, title: &str) -> Result<Option<Article>, Error> {
        let mut url = self.api.clone();
        url.query_pairs_mut()
            .append_pair("action", "query")
            .append_pair("prop", "extracts")
            .append_pair("explaintext", "1")
            .append_pair("redirects", "1")
            .append_pair("titles", title)
            .append_pair("format", "json")
            .append_pair("formatversion", "2");
        let body = self.http.get(Host::Wikipedia, url.as_str()).await?;
        let v: Value =
            serde_json::from_slice(&body).map_err(|_| Error::Format("Wikipedia JSON reply"))?;
        parse_article(&v)
    }
}

fn parse_article(v: &Value) -> Result<Option<Article>, Error> {
    let page = v
        .pointer("/query/pages/0")
        .ok_or(Error::Format("Wikipedia page list"))?;
    if page["missing"].as_bool() == Some(true) {
        return Ok(None);
    }
    Ok(Some(Article {
        title: page["title"]
            .as_str()
            .ok_or(Error::Format("Wikipedia page"))?
            .to_string(),
        text: page["extract"].as_str().unwrap_or_default().to_string(),
    }))
}

/// Article titles from PubChem's "Wikipedia" heading, in its order.
pub fn linked_titles(view: &Value) -> Vec<String> {
    let mut titles: Vec<String> = Vec::new();
    let mut stack: Vec<&Value> = view["Record"]["Section"]
        .as_array()
        .into_iter()
        .flatten()
        .rev()
        .collect();
    while let Some(section) = stack.pop() {
        for info in section["Information"].as_array().into_iter().flatten() {
            let title = info["URL"]
                .as_str()
                .and_then(|u| u.strip_prefix(ARTICLE_BASE))
                .map(|t| percent_decode(t).replace('_', " "));
            if let Some(t) = title.filter(|t| !titles.contains(t)) {
                titles.push(t);
            }
        }
        stack.extend(section["Section"].as_array().into_iter().flatten().rev());
    }
    titles
}

fn percent_decode(s: &str) -> String {
    let bytes = s.as_bytes();
    let mut out = Vec::with_capacity(bytes.len());
    let mut i = 0;
    while i < bytes.len() {
        let hex = bytes
            .get(i + 1..i + 3)
            .and_then(|h| std::str::from_utf8(h).ok())
            .and_then(|h| u8::from_str_radix(h, 16).ok());
        match (bytes[i], hex) {
            (b'%', Some(b)) => {
                out.push(b);
                i += 3;
            }
            (b, _) => {
                out.push(b);
                i += 1;
            }
        }
    }
    String::from_utf8_lossy(&out).into_owned()
}

/// The article split into top-level sections, in order. Subsections stay
/// with their parent; the text before the first heading is "Introduction".
fn sections(text: &str) -> Vec<(String, Vec<&str>)> {
    let mut out = vec![("Introduction".to_string(), Vec::new())];
    for line in text.lines() {
        if let Some(c) = TOP_HEADING.captures(line.trim()) {
            out.push((c[1].to_string(), Vec::new()));
        } else if !line.trim().is_empty() && !line.starts_with('=') {
            out.last_mut().expect("starts with the intro").1.push(line);
        }
    }
    out
}

/// Snippets from the intro, History, and a little from every other section
/// except boilerplate and how-it's-made. Ids are left empty.
pub fn snippets(article: &Article) -> Vec<Snippet> {
    let mut page = Url::parse(ARTICLE_BASE).expect("valid base");
    page.path_segments_mut()
        .expect("has a path")
        .pop()
        .push(&article.title.replace(' ', "_"));
    let all = sections(&article.text);
    let mut plan: Vec<(&str, usize)> = SECTIONS.to_vec();
    plan.extend(
        all.iter()
            .map(|(name, _)| name.as_str())
            .filter(|name| !SECTIONS.iter().any(|(n, _)| n == name) && !SKIP.is_match(name))
            .map(|name| (name, OTHER_PER_SECTION)),
    );

    let mut out = Vec::new();
    for (name, limit) in plan {
        let anchor = if name == "Introduction" {
            String::new()
        } else {
            format!("#{}", name.replace(' ', "_"))
        };
        let paragraphs = all
            .iter()
            .find(|(n, _)| n == name)
            .map(|(_, p)| p.as_slice());
        let mut kept = 0;
        for para in paragraphs.unwrap_or_default() {
            let text = clean(para);
            if text.chars().count() < MIN_CHARS {
                continue;
            }
            out.push(Snippet {
                id: String::new(),
                source: "wikipedia".into(),
                section: name.to_string(),
                url: format!("{page}{anchor}"),
                origin: "Wikipedia".into(),
                text,
            });
            kept += 1;
            if kept == limit || out.len() == MAX_SNIPPETS {
                break;
            }
        }
        if out.len() == MAX_SNIPPETS {
            break;
        }
    }
    out
}

/// Snippets from the first article PubChem links for `cid` that exists, or
/// none when it links no article.
pub async fn wikipedia_snippets(
    pubchem: &PubChem,
    wikipedia: &Wikipedia,
    cid: u64,
) -> Result<Vec<Snippet>, Error> {
    let Some(view) = pubchem.view(cid, "Wikipedia").await? else {
        return Ok(vec![]);
    };
    for title in linked_titles(&view) {
        if let Some(article) = wikipedia.article(&title).await? {
            return Ok(snippets(&article));
        }
    }
    Ok(vec![])
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::*;

    fn fixture(path: &str) -> Value {
        let path = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../fixtures/http")
            .join(path);
        serde_json::from_slice(&std::fs::read(path).unwrap()).unwrap()
    }

    fn caffeine() -> Article {
        parse_article(&fixture("wikipedia/extract-Caffeine.json"))
            .unwrap()
            .unwrap()
    }

    #[test]
    fn titles_from_pubchem_links() {
        assert_eq!(
            linked_titles(&fixture("pubchem/view-2519-wikipedia.json")),
            ["Caffeine"]
        );
        assert_eq!(
            linked_titles(&fixture("pubchem/view-54670067-wikipedia.json")),
            ["Ascorbic acid", "Vitamin C"]
        );
        assert_eq!(percent_decode("Caf%C3%A9_(drink)"), "Café_(drink)");
        assert_eq!(percent_decode("100%_sure%"), "100%_sure%");
    }

    #[test]
    fn missing_articles_are_none() {
        assert_eq!(
            parse_article(&fixture("wikipedia/extract-No_such_molecule_page_xyz.json")).unwrap(),
            None
        );
    }

    #[test]
    fn intro_and_history_first_then_other_sections() {
        let s = snippets(&caffeine());
        assert!(s.len() <= MAX_SNIPPETS);
        assert_eq!(s[0].section, "Introduction");
        assert_eq!(s[0].url, "https://en.wikipedia.org/wiki/Caffeine");
        let intro = s.iter().filter(|s| s.section == "Introduction").count();
        assert!((1..=3).contains(&intro));
        let history: Vec<_> = s.iter().filter(|s| s.section == "History").collect();
        assert!(!history.is_empty() && history.len() <= 4);
        assert_eq!(
            history[0].url,
            "https://en.wikipedia.org/wiki/Caffeine#History"
        );
        assert!(s.iter().all(|s| !SKIP.is_match(&s.section)));
        assert!(s.iter().all(|s| s.text.chars().count() >= MIN_CHARS));
    }

    /// Against the real services: `cargo test -p valence-core -- --ignored live`.
    #[tokio::test]
    #[ignore = "network"]
    async fn live_capsaicin() {
        let http = Http::new();
        let pubchem = PubChem::new(http.clone());
        let s = wikipedia_snippets(&pubchem, &Wikipedia::new(http), 1548943)
            .await
            .unwrap();
        assert!(s.iter().any(|s| s.section == "History"), "{s:#?}");
        // Capsaicin's birds live under "Natural function".
        assert!(s.iter().any(|s| s.text.contains("dispersed predominantly by birds")), "{s:#?}");
    }

    /// Same paragraphs as the spike fixture wherever the article hasn't been
    /// edited since: compare the sections both picked.
    #[test]
    fn mostly_matches_the_spike_fixture_for_caffeine() {
        let spike = fixture("../spike/caffeine.json");
        let spike: Vec<&str> = spike["snippets"]
            .as_array()
            .unwrap()
            .iter()
            .filter(|s| s["source"] == "wikipedia")
            .map(|s| s["text"].as_str().unwrap())
            .collect();
        let ours: Vec<String> = snippets(&caffeine()).into_iter().map(|s| s.text).collect();
        let same = spike
            .iter()
            .filter(|t| ours.iter().any(|o| o == *t))
            .count();
        assert!(
            same * 4 >= spike.len() * 3,
            "{same} of {} unchanged",
            spike.len()
        );
    }
}
