//! ClinicalTrials.gov (API v2) and Europe PMC (REST) literature sources.
//!
//! Both are public and need no token. Discovery returns [`LitHit`]s like the
//! other sources; `orx paper` renders one record as Markdown.
//!   - **ClinicalTrials.gov**: registered studies, keyed by NCT number.
//!   - **Europe PMC**: journal articles, preprints, and open full text. Hits use
//!     the PMID or DOI when there is one, so `orx paper` routes them to PubMed or
//!     OpenAlex; records with neither keep a Europe PMC id (`PMC…`, `PPR…`, or
//!     `SRC:ID`).

use serde_json::Value;

use crate::client::{http, public_get, LitHit};
use crate::error::{anyhow, Result};

#[derive(Clone, Copy)]
pub struct SourceDiscoveryOptions<'a> {
    pub limit: u32,
    pub published_after: Option<&'a str>,
    pub published_before: Option<&'a str>,
    pub prioritize: &'a str,
}

async fn get_json(service: &str, base: &str, url: reqwest::Url) -> Result<Option<Value>> {
    let res = public_get(
        || http().get(url.clone()),
        service,
        |e| anyhow!("Could not reach {service} at {base}: {e}"),
    )
    .await?;
    let status = res.status();
    if status == reqwest::StatusCode::NOT_FOUND {
        return Ok(None);
    }
    if !status.is_success() {
        return Err(anyhow!(
            "{service} request failed ({} {})",
            status.as_u16(),
            status.canonical_reason().unwrap_or("")
        ));
    }
    Ok(Some(res.json().await?))
}

fn validate_date(flag: &str, value: Option<&str>) -> Result<()> {
    let ok = |s: &str| {
        s.len() == 10
            && s.chars().enumerate().all(|(i, c)| {
                if i == 4 || i == 7 {
                    c == '-'
                } else {
                    c.is_ascii_digit()
                }
            })
    };
    match value {
        Some(s) if !ok(s) => Err(anyhow!("--{flag} must be a date like 2024-01-31.")),
        _ => Ok(()),
    }
}

fn str_at<'a>(value: &'a Value, path: &[&str]) -> Option<&'a str> {
    path.iter()
        .try_fold(value, |v, key| v.get(key))
        .and_then(Value::as_str)
        .filter(|s| !s.is_empty())
}

fn strings_at(value: &Value, path: &[&str]) -> Vec<String> {
    path.iter()
        .try_fold(value, |v, key| v.get(key))
        .and_then(Value::as_array)
        .map(|items| {
            items
                .iter()
                .filter_map(Value::as_str)
                .map(str::to_string)
                .collect()
        })
        .unwrap_or_default()
}

/// `NOT_YET_RECRUITING` → `Not yet recruiting`.
fn humanize(code: &str) -> String {
    let lower = code.replace('_', " ").to_lowercase();
    let mut chars = lower.chars();
    match chars.next() {
        Some(first) => first.to_uppercase().chain(chars).collect(),
        None => String::new(),
    }
}

/// `PHASE2` → `Phase 2`, `EARLY_PHASE1` → `Early Phase 1`; `NA` has no phase.
fn phase_label(code: &str) -> Option<String> {
    match code {
        "NA" | "" => None,
        "EARLY_PHASE1" => Some("Early Phase 1".to_string()),
        _ => Some(
            code.strip_prefix("PHASE")
                .map(|n| format!("Phase {n}"))
                .unwrap_or_else(|| humanize(code)),
        ),
    }
}

// --- ClinicalTrials.gov ---------------------------------------------------------

const TRIAL_FIELDS: &str = "NCTId,BriefTitle,BriefSummary,OverallStatus,Phase,StudyType,\
StudyFirstPostDate,EnrollmentCount,Condition";

fn trials_discovery_url(
    base: &str,
    query: &str,
    options: SourceDiscoveryOptions<'_>,
) -> Result<reqwest::Url> {
    validate_date("published-after", options.published_after)?;
    validate_date("published-before", options.published_before)?;
    let mut params = vec![
        ("format", "json".to_string()),
        ("query.term", query.to_string()),
        ("pageSize", options.limit.to_string()),
        ("fields", TRIAL_FIELDS.to_string()),
    ];
    // ClinicalTrials.gov ranks by relevance; it has no citation counts, so
    // `popular` keeps that order.
    match options.prioritize {
        "recency" => params.push(("sort", "StudyFirstPostDate:desc".to_string())),
        "historical" => params.push(("sort", "StudyFirstPostDate:asc".to_string())),
        _ => {}
    }
    if options.published_after.is_some() || options.published_before.is_some() {
        params.push((
            "filter.advanced",
            format!(
                "AREA[StudyFirstPostDate]RANGE[{},{}]",
                options.published_after.unwrap_or("MIN"),
                options.published_before.unwrap_or("MAX")
            ),
        ));
    }
    Ok(reqwest::Url::parse_with_params(
        &format!("{}/studies", base.trim_end_matches('/')),
        &params,
    )?)
}

fn trial_overview(study: &Value) -> String {
    let p = &study["protocolSection"];
    let mut meta = Vec::new();
    if let Some(status) = str_at(p, &["statusModule", "overallStatus"]) {
        meta.push(humanize(status));
    }
    let phases: Vec<_> = strings_at(p, &["designModule", "phases"])
        .iter()
        .filter_map(|code| phase_label(code))
        .collect();
    if !phases.is_empty() {
        meta.push(phases.join("/"));
    }
    if let Some(kind) = str_at(p, &["designModule", "studyType"]) {
        meta.push(humanize(kind));
    }
    if let Some(count) = p["designModule"]["enrollmentInfo"]["count"].as_i64() {
        meta.push(format!("{count} participants"));
    }
    let conditions = strings_at(p, &["conditionsModule", "conditions"]);
    if !conditions.is_empty() {
        meta.push(format!("Conditions: {}", conditions.join(", ")));
    }
    meta.join(" · ")
}

fn trial_hit(study: &Value) -> Option<LitHit> {
    let p = &study["protocolSection"];
    let id = str_at(p, &["identificationModule", "nctId"])?.to_string();
    let overview = trial_overview(study);
    let summary = str_at(p, &["descriptionModule", "briefSummary"]).unwrap_or("");
    Some(LitHit {
        source: "clinicaltrials".to_string(),
        id,
        title: str_at(p, &["identificationModule", "briefTitle"])
            .unwrap_or("")
            .to_string(),
        abstract_: match (overview.is_empty(), summary.is_empty()) {
            (true, _) => summary.to_string(),
            (false, true) => overview,
            (false, false) => format!("{overview}\n\n{summary}"),
        },
        publication_date: str_at(p, &["statusModule", "studyFirstPostDateStruct", "date"])
            .map(str::to_string),
        votes: None,
        citations: None,
        snippets: Vec::new(),
    })
}

pub async fn discover_clinicaltrials(
    query: &str,
    options: SourceDiscoveryOptions<'_>,
) -> Result<Vec<LitHit>> {
    let base = crate::config::clinicaltrials_api_url();
    let url = trials_discovery_url(&base, query, options)?;
    let data = get_json("ClinicalTrials.gov", &base, url)
        .await?
        .unwrap_or(Value::Null);
    Ok(data["studies"]
        .as_array()
        .map(|studies| studies.iter().filter_map(trial_hit).collect())
        .unwrap_or_default())
}

/// The NCT number in a bare id or a clinicaltrials.gov URL.
pub fn nct_id(input: &str) -> Option<String> {
    let upper = input.trim().to_ascii_uppercase();
    let start = upper.find("NCT")?;
    let digits: String = upper[start + 3..]
        .chars()
        .take_while(char::is_ascii_digit)
        .collect();
    (digits.len() == 8).then(|| format!("NCT{digits}"))
}

pub async fn fetch_clinical_trial(nct: &str) -> Result<Option<Value>> {
    let base = crate::config::clinicaltrials_api_url();
    let url = reqwest::Url::parse_with_params(
        &format!("{}/studies/{nct}", base.trim_end_matches('/')),
        &[("format", "json")],
    )?;
    get_json("ClinicalTrials.gov", &base, url).await
}

pub fn render_trial(study: &Value, full: bool) -> String {
    let p = &study["protocolSection"];
    let nct = str_at(p, &["identificationModule", "nctId"]).unwrap_or("");
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    line(format!(
        "# {}",
        str_at(p, &["identificationModule", "briefTitle"]).unwrap_or(nct)
    ));
    if let Some(official) = str_at(p, &["identificationModule", "officialTitle"]) {
        line(format!("_{official}_"));
    }
    line(format!("{nct} · {}", trial_overview(study)));
    let dates: Vec<_> = [
        ("Start", &["statusModule", "startDateStruct", "date"][..]),
        (
            "Primary completion",
            &["statusModule", "primaryCompletionDateStruct", "date"][..],
        ),
        (
            "Completion",
            &["statusModule", "completionDateStruct", "date"][..],
        ),
    ]
    .iter()
    .filter_map(|(label, path)| str_at(p, path).map(|d| format!("{label} {d}")))
    .collect();
    if !dates.is_empty() {
        line(dates.join(" · "));
    }
    if let Some(sponsor) = str_at(p, &["sponsorCollaboratorsModule", "leadSponsor", "name"]) {
        line(format!("Sponsor: {sponsor}"));
    }
    line(format!("Registry: https://clinicaltrials.gov/study/{nct}"));
    if study["hasResults"].as_bool() == Some(true) {
        line(format!(
            "Results posted: https://clinicaltrials.gov/study/{nct}?tab=results"
        ));
    }
    if let Some(interventions) = p["armsInterventionsModule"]["interventions"].as_array() {
        let names: Vec<_> = interventions
            .iter()
            .filter_map(|i| {
                let name = i["name"].as_str()?;
                Some(match i["type"].as_str() {
                    Some(kind) => format!("{name} ({})", humanize(kind)),
                    None => name.to_string(),
                })
            })
            .collect();
        if !names.is_empty() {
            line(format!("Interventions: {}", names.join("; ")));
        }
    }
    line(String::new());
    line(
        str_at(p, &["descriptionModule", "briefSummary"])
            .unwrap_or("(No summary registered.)")
            .to_string(),
    );
    if full {
        if let Some(detail) = str_at(p, &["descriptionModule", "detailedDescription"]) {
            line(String::new());
            line("## Detailed description".to_string());
            line(detail.to_string());
        }
    }
    let outcome_section = |key: &str| -> Vec<String> {
        p["outcomesModule"][key]
            .as_array()
            .map(|outcomes| {
                outcomes
                    .iter()
                    .filter_map(|o| {
                        let measure = o["measure"].as_str()?;
                        Some(match o["timeFrame"].as_str() {
                            Some(time) => format!("- {measure} ({time})"),
                            None => format!("- {measure}"),
                        })
                    })
                    .collect()
            })
            .unwrap_or_default()
    };
    let primary = outcome_section("primaryOutcomes");
    if !primary.is_empty() {
        line(String::new());
        line("## Primary outcomes".to_string());
        primary.into_iter().for_each(&mut line);
    }
    if full {
        let secondary = outcome_section("secondaryOutcomes");
        if !secondary.is_empty() {
            line(String::new());
            line("## Secondary outcomes".to_string());
            secondary.into_iter().for_each(&mut line);
        }
    }
    let e = &p["eligibilityModule"];
    let ages = match (e["minimumAge"].as_str(), e["maximumAge"].as_str()) {
        (Some(min), Some(max)) => Some(format!("{min} to {max}")),
        (Some(min), None) => Some(format!("{min} and older")),
        (None, Some(max)) => Some(format!("up to {max}")),
        (None, None) => None,
    };
    let mut eligibility = Vec::new();
    if let Some(ages) = ages {
        eligibility.push(format!("Ages {ages}"));
    }
    if let Some(sex) = e["sex"].as_str() {
        eligibility.push(format!("Sex: {}", humanize(sex)));
    }
    if e["healthyVolunteers"].as_bool() == Some(true) {
        eligibility.push("Accepts healthy volunteers".to_string());
    }
    if !eligibility.is_empty() || (full && e["eligibilityCriteria"].is_string()) {
        line(String::new());
        line("## Eligibility".to_string());
        if !eligibility.is_empty() {
            line(eligibility.join(" · "));
        }
        if full {
            if let Some(criteria) = e["eligibilityCriteria"].as_str() {
                line(String::new());
                line(criteria.to_string());
            }
        }
    }
    let locations = p["contactsLocationsModule"]["locations"]
        .as_array()
        .map(Vec::len)
        .unwrap_or(0);
    if locations > 0 {
        line(String::new());
        line(format!(
            "{locations} site{}",
            if locations == 1 { "" } else { "s" }
        ));
    }
    if !full {
        line(String::new());
        line("(Pass --full for the detailed description, secondary outcomes, and full eligibility criteria.)".to_string());
    }
    out
}

// --- Europe PMC -------------------------------------------------------------------

fn europepmc_discovery_url(
    base: &str,
    query: &str,
    options: SourceDiscoveryOptions<'_>,
) -> Result<reqwest::Url> {
    validate_date("published-after", options.published_after)?;
    validate_date("published-before", options.published_before)?;
    let mut query = query.to_string();
    if options.published_after.is_some() || options.published_before.is_some() {
        query = format!(
            "({query}) AND FIRST_PDATE:[{} TO {}]",
            options.published_after.unwrap_or("1000-01-01"),
            options.published_before.unwrap_or("3000-12-31")
        );
    }
    let mut params = vec![
        ("query", query),
        ("format", "json".to_string()),
        ("resultType", "core".to_string()),
        ("pageSize", options.limit.to_string()),
    ];
    match options.prioritize {
        "recency" => params.push(("sort", "P_PDATE_D desc".to_string())),
        "historical" => params.push(("sort", "P_PDATE_D asc".to_string())),
        "popular" => params.push(("sort", "CITED desc".to_string())),
        _ => {}
    }
    Ok(reqwest::Url::parse_with_params(
        &format!("{}/search", base.trim_end_matches('/')),
        &params,
    )?)
}

/// Abstract text without Europe PMC's inline HTML (`<h4>`, `<i>`, `<sup>`).
fn strip_tags(html: &str) -> String {
    let mut out = String::with_capacity(html.len());
    let mut in_tag = false;
    for c in html.chars() {
        match c {
            '<' => in_tag = true,
            '>' if in_tag => {
                in_tag = false;
                out.push(' ');
            }
            _ if !in_tag => out.push(c),
            _ => {}
        }
    }
    out.split_whitespace().collect::<Vec<_>>().join(" ")
}

/// The id `orx paper` should route this record by: PMID, then DOI, then a
/// Europe PMC id.
fn europepmc_routing_id(record: &Value) -> Option<String> {
    if let Some(pmid) = str_at(record, &["pmid"]) {
        return Some(pmid.to_string());
    }
    if let Some(doi) = str_at(record, &["doi"]) {
        return Some(doi.to_string());
    }
    if let Some(pmcid) = str_at(record, &["pmcid"]) {
        return Some(pmcid.to_string());
    }
    let id = str_at(record, &["id"])?;
    Some(match str_at(record, &["source"]) {
        Some("PPR") if id.starts_with("PPR") => id.to_string(),
        Some(source) => format!("{source}:{id}"),
        None => id.to_string(),
    })
}

fn europepmc_hit(record: &Value) -> Option<LitHit> {
    Some(LitHit {
        source: "europepmc".to_string(),
        id: europepmc_routing_id(record)?,
        title: strip_tags(str_at(record, &["title"]).unwrap_or("")),
        abstract_: strip_tags(str_at(record, &["abstractText"]).unwrap_or("")),
        publication_date: str_at(record, &["firstPublicationDate"]).map(str::to_string),
        votes: None,
        citations: record["citedByCount"].as_i64(),
        snippets: Vec::new(),
    })
}

pub async fn discover_europepmc(
    query: &str,
    options: SourceDiscoveryOptions<'_>,
) -> Result<Vec<LitHit>> {
    let base = crate::config::europepmc_api_url();
    let url = europepmc_discovery_url(&base, query, options)?;
    let data = get_json("Europe PMC", &base, url)
        .await?
        .unwrap_or(Value::Null);
    Ok(data["resultList"]["result"]
        .as_array()
        .map(|records| records.iter().filter_map(europepmc_hit).collect())
        .unwrap_or_default())
}

/// A Europe PMC id `orx paper` fetches from Europe PMC itself: `PMC…`,
/// `PPR…` preprints, or `SRC:ID` (e.g. `AGR:123`).
pub fn europepmc_id(input: &str) -> Option<String> {
    let s = input.trim();
    let s = s.strip_prefix("europepmc:").unwrap_or(s);
    let numbered = |prefix: &str| {
        s.len() > prefix.len()
            && s[..prefix.len()].eq_ignore_ascii_case(prefix)
            && s[prefix.len()..].chars().all(|c| c.is_ascii_digit())
    };
    if numbered("PMC") || numbered("PPR") {
        return Some(s.to_ascii_uppercase());
    }
    let (source, id) = s.split_once(':')?;
    (source.len() == 3
        && source.chars().all(|c| c.is_ascii_uppercase())
        && !id.is_empty()
        && !id.contains(char::is_whitespace))
    .then(|| s.to_string())
}

fn europepmc_lookup_query(id: &str) -> String {
    if id.starts_with("PMC") {
        format!("PMCID:{id}")
    } else if id.starts_with("PPR") {
        format!("EXT_ID:{id} AND SRC:PPR")
    } else {
        let (source, ext) = id.split_once(':').unwrap_or(("MED", id));
        format!("EXT_ID:{ext} AND SRC:{source}")
    }
}

pub async fn fetch_europepmc(id: &str) -> Result<Option<Value>> {
    let base = crate::config::europepmc_api_url();
    let url = reqwest::Url::parse_with_params(
        &format!("{}/search", base.trim_end_matches('/')),
        &[
            ("query", europepmc_lookup_query(id)),
            ("format", "json".to_string()),
            ("resultType", "core".to_string()),
            ("pageSize", "1".to_string()),
        ],
    )?;
    Ok(get_json("Europe PMC", &base, url)
        .await?
        .and_then(|data| data["resultList"]["result"].get(0).cloned()))
}

pub fn render_europepmc(record: &Value, full: bool) -> String {
    let mut out = String::new();
    let mut line = |s: String| {
        out.push_str(&s);
        out.push('\n');
    };
    line(format!(
        "# {}",
        strip_tags(str_at(record, &["title"]).unwrap_or("(untitled)"))
    ));
    if let Some(authors) = str_at(record, &["authorString"]) {
        line(authors.to_string());
    }
    let mut meta = Vec::new();
    if let Some(journal) = str_at(record, &["journalInfo", "journal", "title"]) {
        meta.push(journal.to_string());
    } else if str_at(record, &["source"]) == Some("PPR") {
        meta.push("Preprint".to_string());
    }
    if let Some(date) = str_at(record, &["firstPublicationDate"]) {
        meta.push(date.to_string());
    }
    if let Some(cited) = record["citedByCount"].as_i64() {
        meta.push(format!("{cited} citations"));
    }
    if !meta.is_empty() {
        line(meta.join(" · "));
    }
    if let (Some(source), Some(id)) = (str_at(record, &["source"]), str_at(record, &["id"])) {
        line(format!(
            "Europe PMC: https://europepmc.org/article/{source}/{id}"
        ));
    }
    if let Some(pmid) = str_at(record, &["pmid"]) {
        line(format!("PubMed: https://pubmed.ncbi.nlm.nih.gov/{pmid}/"));
    }
    if let Some(doi) = str_at(record, &["doi"]) {
        line(format!("DOI: https://doi.org/{doi}"));
    }
    if let Some(urls) = record["fullTextUrlList"]["fullTextUrl"].as_array() {
        for u in urls.iter().filter(|u| u["availabilityCode"] == "OA") {
            if let Some(url) = u["url"].as_str() {
                let style = u["documentStyle"].as_str().unwrap_or("full text");
                line(format!("Open access ({style}): {url}"));
            }
        }
    }
    line(String::new());
    let abstract_ = strip_tags(str_at(record, &["abstractText"]).unwrap_or(""));
    line(if abstract_.is_empty() {
        "(No abstract available from Europe PMC.)".to_string()
    } else {
        abstract_
    });
    if full {
        eprintln!("Europe PMC returns metadata + abstract here — open the open-access link above for full text.");
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn options(prioritize: &'static str) -> SourceDiscoveryOptions<'static> {
        SourceDiscoveryOptions {
            limit: 5,
            published_after: Some("2020-01-01"),
            published_before: None,
            prioritize,
        }
    }

    #[test]
    fn trials_url_carries_query_sort_and_open_date_range() {
        let url = trials_discovery_url("https://ct.test/api/v2", "hypospadias", options("recency"))
            .unwrap();
        let pairs: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(url.path(), "/api/v2/studies");
        assert_eq!(pairs["query.term"], "hypospadias");
        assert_eq!(pairs["pageSize"], "5");
        assert_eq!(pairs["sort"], "StudyFirstPostDate:desc");
        assert_eq!(
            pairs["filter.advanced"],
            "AREA[StudyFirstPostDate]RANGE[2020-01-01,MAX]"
        );
        let popular = trials_discovery_url("https://ct.test", "x", options("popular")).unwrap();
        assert!(!popular.query_pairs().any(|(k, _)| k == "sort"));
    }

    #[test]
    fn rejects_malformed_dates() {
        let bad = SourceDiscoveryOptions {
            published_after: Some("2020/01/01"),
            ..options("default")
        };
        assert!(trials_discovery_url("https://ct.test", "x", bad).is_err());
        assert!(europepmc_discovery_url("https://epmc.test", "x", bad).is_err());
    }

    fn sample_study() -> Value {
        json!({
            "hasResults": true,
            "protocolSection": {
                "identificationModule": {"nctId": "NCT07191938", "briefTitle": "Penile vs pudendal block", "officialTitle": "A randomized comparison"},
                "statusModule": {
                    "overallStatus": "NOT_YET_RECRUITING",
                    "startDateStruct": {"date": "2025-09"},
                    "studyFirstPostDateStruct": {"date": "2025-09-25"}
                },
                "descriptionModule": {"briefSummary": "Compares two blocks.", "detailedDescription": "Long detail."},
                "conditionsModule": {"conditions": ["Postoperative Analgesia", "Posthectomy"]},
                "designModule": {"phases": ["PHASE2", "PHASE3"], "studyType": "INTERVENTIONAL", "enrollmentInfo": {"count": 240}},
                "sponsorCollaboratorsModule": {"leadSponsor": {"name": "Caen University Hospital"}},
                "armsInterventionsModule": {"interventions": [{"type": "PROCEDURE", "name": "Penile block"}]},
                "outcomesModule": {
                    "primaryOutcomes": [{"measure": "Pain score", "timeFrame": "24 hours"}],
                    "secondaryOutcomes": [{"measure": "Block time"}]
                },
                "eligibilityModule": {"minimumAge": "6 Months", "maximumAge": "17 Years", "sex": "MALE", "eligibilityCriteria": "Inclusion: elective surgery"},
                "contactsLocationsModule": {"locations": [{}, {}]}
            }
        })
    }

    #[test]
    fn trial_hit_summarizes_status_phase_and_conditions() {
        let hit = trial_hit(&sample_study()).unwrap();
        assert_eq!(hit.source, "clinicaltrials");
        assert_eq!(hit.id, "NCT07191938");
        assert_eq!(hit.publication_date.as_deref(), Some("2025-09-25"));
        assert!(hit.abstract_.starts_with(
            "Not yet recruiting · Phase 2/Phase 3 · Interventional · 240 participants · Conditions: Postoperative Analgesia, Posthectomy\n\nCompares two blocks."
        ));
    }

    #[test]
    fn renders_trial_with_and_without_full_detail() {
        let brief = render_trial(&sample_study(), false);
        assert!(brief.contains("# Penile vs pudendal block"));
        assert!(brief.contains("Sponsor: Caen University Hospital"));
        assert!(brief.contains("Registry: https://clinicaltrials.gov/study/NCT07191938"));
        assert!(brief.contains("Results posted:"));
        assert!(brief.contains("Interventions: Penile block (Procedure)"));
        assert!(brief.contains("- Pain score (24 hours)"));
        assert!(brief.contains("Ages 6 Months to 17 Years · Sex: Male"));
        assert!(brief.contains("2 sites"));
        assert!(!brief.contains("Long detail."));
        assert!(!brief.contains("Inclusion: elective surgery"));
        let full = render_trial(&sample_study(), true);
        assert!(full.contains("## Detailed description\nLong detail."));
        assert!(full.contains("## Secondary outcomes\n- Block time"));
        assert!(full.contains("Inclusion: elective surgery"));
    }

    #[test]
    fn nct_ids_come_from_bare_ids_and_urls() {
        assert_eq!(nct_id("NCT07191938").as_deref(), Some("NCT07191938"));
        assert_eq!(nct_id("nct07191938").as_deref(), Some("NCT07191938"));
        assert_eq!(
            nct_id("https://clinicaltrials.gov/study/NCT07191938?tab=results").as_deref(),
            Some("NCT07191938")
        );
        assert_eq!(nct_id("NCT123"), None);
        assert_eq!(nct_id("38308006"), None);
    }

    #[test]
    fn europepmc_url_adds_date_window_and_sort() {
        let url =
            europepmc_discovery_url("https://epmc.test/rest", "hypospadias", options("popular"))
                .unwrap();
        let pairs: std::collections::HashMap<_, _> = url.query_pairs().into_owned().collect();
        assert_eq!(url.path(), "/rest/search");
        assert_eq!(
            pairs["query"],
            "(hypospadias) AND FIRST_PDATE:[2020-01-01 TO 3000-12-31]"
        );
        assert_eq!(pairs["sort"], "CITED desc");
        assert_eq!(pairs["resultType"], "core");
        let relevance = europepmc_discovery_url(
            "https://epmc.test",
            "x",
            SourceDiscoveryOptions {
                published_after: None,
                ..options("default")
            },
        )
        .unwrap();
        let pairs: std::collections::HashMap<_, _> = relevance.query_pairs().into_owned().collect();
        assert_eq!(pairs["query"], "x");
        assert!(!pairs.contains_key("sort"));
    }

    #[test]
    fn europepmc_hits_route_by_pmid_then_doi_then_europepmc_id() {
        let article = json!({"id": "37662771", "source": "MED", "pmid": "37662771", "doi": "10.1/x", "title": "A <i>title</i>", "abstractText": "<h4>Background</h4>Text here.", "firstPublicationDate": "2023-08-25", "citedByCount": 167});
        let hit = europepmc_hit(&article).unwrap();
        assert_eq!(hit.id, "37662771");
        assert_eq!(hit.title, "A title");
        assert_eq!(hit.abstract_, "Background Text here.");
        assert_eq!(hit.citations, Some(167));
        let doi_only = json!({"id": "X1", "source": "AGR", "doi": "10.2/y", "title": "t"});
        assert_eq!(europepmc_hit(&doi_only).unwrap().id, "10.2/y");
        let preprint = json!({"id": "PPR123456", "source": "PPR", "title": "t"});
        assert_eq!(europepmc_hit(&preprint).unwrap().id, "PPR123456");
        let other = json!({"id": "123", "source": "AGR", "title": "t"});
        assert_eq!(europepmc_hit(&other).unwrap().id, "AGR:123");
    }

    #[test]
    fn europepmc_ids_and_lookup_queries() {
        assert_eq!(europepmc_id("PMC10472068").as_deref(), Some("PMC10472068"));
        assert_eq!(europepmc_id("pmc10472068").as_deref(), Some("PMC10472068"));
        assert_eq!(europepmc_id("PPR123456").as_deref(), Some("PPR123456"));
        assert_eq!(
            europepmc_id("europepmc:AGR:123").as_deref(),
            Some("AGR:123")
        );
        assert_eq!(europepmc_id("38308006"), None);
        assert_eq!(europepmc_id("10.1038/nature14539"), None);
        assert_eq!(europepmc_lookup_query("PMC1"), "PMCID:PMC1");
        assert_eq!(europepmc_lookup_query("PPR9"), "EXT_ID:PPR9 AND SRC:PPR");
        assert_eq!(europepmc_lookup_query("AGR:123"), "EXT_ID:123 AND SRC:AGR");
    }

    #[test]
    fn renders_europepmc_record_links() {
        let record = json!({
            "id": "37662771", "source": "MED", "pmid": "37662771", "doi": "10.1016/j.x",
            "title": "Pollutants", "authorString": "Shetty SS, D D.",
            "journalInfo": {"journal": {"title": "Heliyon"}},
            "firstPublicationDate": "2023-08-25", "citedByCount": 167,
            "abstractText": "<b>Abstract</b> body.",
            "fullTextUrlList": {"fullTextUrl": [
                {"availabilityCode": "OA", "documentStyle": "pdf", "url": "https://x/pdf"},
                {"availabilityCode": "S", "documentStyle": "html", "url": "https://x/paywall"}
            ]}
        });
        let md = render_europepmc(&record, false);
        assert!(
            md.starts_with("# Pollutants\nShetty SS, D D.\nHeliyon · 2023-08-25 · 167 citations\n")
        );
        assert!(md.contains("Europe PMC: https://europepmc.org/article/MED/37662771"));
        assert!(md.contains("PubMed: https://pubmed.ncbi.nlm.nih.gov/37662771/"));
        assert!(md.contains("Open access (pdf): https://x/pdf"));
        assert!(!md.contains("paywall"));
        assert!(md.ends_with("Abstract body.\n"));
    }
}
