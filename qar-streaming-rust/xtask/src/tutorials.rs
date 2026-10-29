//! `cargo xtask tutorials [--check]` turns the tutorial sources `examples/tutorials/*.md` into
//! the docs site's `.mdx` pages, so the code a tutorial shows is the code the example runs.
//!
//! A source pulls code from an example with mdBook's include syntax, on a line of its own inside
//! a fenced code block: `{{#include ../src/bin/<file>.rs:<anchor>}}`, the path relative to the
//! source. The region is the lines between `// ANCHOR: <anchor>` and `// ANCHOR_END: <anchor>`,
//! with their common indentation removed. Anchors may not nest or overlap, so a region never
//! holds another anchor line. `_category_.json` beside the sources is copied as is.
//!
//! `--check` writes nothing and fails when a page is missing, stale or not generated from any
//! source. Both modes fail on an include that does not resolve, an anchor that is missing,
//! duplicated, nested or unclosed, and prose that MDX would read as JSX or an expression.

use std::collections::{BTreeMap, HashMap};
use std::path::Path;
use std::process::ExitCode;

const SOURCE_DIRECTORY: &str = "examples/tutorials";
const OUTPUT_DIRECTORY: &str = "../docs/website/docs/developer-guide/tutorials/rust";
const CATEGORY_FILE: &str = "_category_.json";

/// Generated file name to content.
type Pages = BTreeMap<String, String>;

/// Anchor name to its region, without anchor lines and common indentation.
type Regions = HashMap<String, String>;

pub fn run(arguments: &[String]) -> ExitCode {
    let check_only = match arguments {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => {
            eprintln!("usage: cargo xtask tutorials [--check]");
            return ExitCode::FAILURE;
        }
    };
    let root = super::workspace_root();
    let expected = match generate_pages(&root) {
        Ok(pages) => pages,
        Err(problems) => {
            for problem in problems {
                eprintln!("{problem}");
            }
            return ExitCode::FAILURE;
        }
    };
    let output_directory = root.join(OUTPUT_DIRECTORY);
    let existing = match read_generated_pages(&output_directory) {
        Ok(pages) => pages,
        Err(problem) => {
            eprintln!("{problem}");
            return ExitCode::FAILURE;
        }
    };
    let differences = compare(&expected, &existing);
    if differences.is_empty() {
        println!("tutorials: {OUTPUT_DIRECTORY} is up to date");
        return ExitCode::SUCCESS;
    }
    if check_only {
        for difference in &differences {
            eprintln!("{OUTPUT_DIRECTORY}/{difference}");
        }
        eprintln!("run `cargo xtask tutorials` and commit the result");
        return ExitCode::FAILURE;
    }
    match write_differences(&output_directory, &expected, &differences) {
        Ok(()) => {
            for difference in &differences {
                match difference {
                    Difference::Missing(name) | Difference::Stale(name) => {
                        println!("tutorials: wrote {OUTPUT_DIRECTORY}/{name}");
                    }
                    Difference::NotGenerated(name) => {
                        println!("tutorials: removed {OUTPUT_DIRECTORY}/{name}");
                    }
                }
            }
            ExitCode::SUCCESS
        }
        Err(problem) => {
            eprintln!("{problem}");
            ExitCode::FAILURE
        }
    }
}

fn generate_pages(root: &Path) -> Result<Pages, Vec<String>> {
    let source_directory = root.join(SOURCE_DIRECTORY);
    let canonical_root = std::fs::canonicalize(root)
        .map_err(|error| vec![format!("cannot resolve {}: {error}", root.display())])?;
    let mut regions = RegionCache::new(|relative_path: &str| {
        read_included_file(&canonical_root, &source_directory, relative_path)
    });

    let mut pages = Pages::new();
    let mut problems = Vec::new();
    for source_name in source_names(&source_directory).map_err(|problem| vec![problem])? {
        let source_label = format!("{SOURCE_DIRECTORY}/{source_name}");
        let markdown =
            read_text(&source_directory.join(&source_name)).map_err(|problem| vec![problem])?;
        match render_page(&source_name, &markdown, &mut regions) {
            Ok(page) => {
                let stem = source_name.trim_end_matches(".md");
                pages.insert(format!("{stem}.mdx"), page);
            }
            Err(page_problems) => problems.extend(
                page_problems
                    .into_iter()
                    .map(|problem| format!("{source_label}: {problem}")),
            ),
        }
    }
    let category =
        read_text(&source_directory.join(CATEGORY_FILE)).map_err(|problem| vec![problem])?;
    pages.insert(CATEGORY_FILE.to_string(), category);

    if problems.is_empty() {
        Ok(pages)
    } else {
        Err(problems)
    }
}

/// The `.md` files directly in the source directory, sorted.
pub(crate) fn source_names(source_directory: &Path) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(source_directory)
        .map_err(|error| format!("cannot list {}: {error}", source_directory.display()))?;
    let mut names = Vec::new();
    for entry in entries {
        let entry = entry
            .map_err(|error| format!("cannot list {}: {error}", source_directory.display()))?;
        let path = entry.path();
        if path.extension().is_some_and(|extension| extension == "md") && path.is_file() {
            names.push(entry.file_name().to_string_lossy().into_owned());
        }
    }
    names.sort();
    Ok(names)
}

pub(crate) fn read_included_file(
    canonical_root: &Path,
    source_directory: &Path,
    relative_path: &str,
) -> Result<String, String> {
    if Path::new(relative_path).is_absolute() {
        return Err("name the file relative to the tutorial".to_string());
    }
    let path = source_directory.join(relative_path);
    let canonical_path =
        std::fs::canonicalize(&path).map_err(|error| format!("cannot resolve: {error}"))?;
    if !canonical_path.starts_with(canonical_root) {
        return Err("is outside the workspace".to_string());
    }
    read_text(&canonical_path)
}

/// Reads a text file with LF line endings, whatever the checkout's line endings are.
pub(crate) fn read_text(path: &Path) -> Result<String, String> {
    std::fs::read_to_string(path)
        .map(|text| text.replace("\r\n", "\n"))
        .map_err(|error| format!("cannot read {}: {error}", path.display()))
}

/// The files of the output directory, dotfiles (`.DS_Store`) left out.
fn read_generated_pages(output_directory: &Path) -> Result<Pages, String> {
    let mut pages = Pages::new();
    let entries = match std::fs::read_dir(output_directory) {
        Ok(entries) => entries,
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => return Ok(pages),
        Err(error) => {
            return Err(format!(
                "cannot list {}: {error}",
                output_directory.display()
            ));
        }
    };
    for entry in entries {
        let entry = entry
            .map_err(|error| format!("cannot list {}: {error}", output_directory.display()))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        if name.starts_with('.') {
            continue;
        }
        if !entry.path().is_file() {
            return Err(format!(
                "{OUTPUT_DIRECTORY}/{name}: only generated pages belong in this directory"
            ));
        }
        pages.insert(name, read_text(&entry.path())?);
    }
    Ok(pages)
}

#[derive(Debug, PartialEq, Eq)]
enum Difference {
    Missing(String),
    Stale(String),
    NotGenerated(String),
}

impl std::fmt::Display for Difference {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Missing(name) => write!(formatter, "{name}: missing"),
            Self::Stale(name) => write!(formatter, "{name}: stale"),
            Self::NotGenerated(name) => write!(formatter, "{name}: not generated from any source"),
        }
    }
}

fn compare(expected: &Pages, existing: &Pages) -> Vec<Difference> {
    let changed = expected
        .iter()
        .filter_map(|(name, content)| match existing.get(name) {
            None => Some(Difference::Missing(name.clone())),
            Some(current) if current.replace("\r\n", "\n") != *content => {
                Some(Difference::Stale(name.clone()))
            }
            Some(_) => None,
        });
    let left_over = existing
        .keys()
        .filter(|name| !expected.contains_key(*name))
        .map(|name| Difference::NotGenerated(name.clone()));
    changed.chain(left_over).collect()
}

fn write_differences(
    output_directory: &Path,
    expected: &Pages,
    differences: &[Difference],
) -> Result<(), String> {
    std::fs::create_dir_all(output_directory)
        .map_err(|error| format!("cannot create {}: {error}", output_directory.display()))?;
    for difference in differences {
        match difference {
            Difference::Missing(name) | Difference::Stale(name) => {
                let path = output_directory.join(name);
                std::fs::write(&path, &expected[name])
                    .map_err(|error| format!("cannot write {}: {error}", path.display()))?;
            }
            Difference::NotGenerated(name) => {
                let path = output_directory.join(name);
                std::fs::remove_file(&path)
                    .map_err(|error| format!("cannot remove {}: {error}", path.display()))?;
            }
        }
    }
    Ok(())
}

/// Parses each included file once, however many includes name it.
pub(crate) struct RegionCache<Load> {
    load: Load,
    files: HashMap<String, Result<Regions, String>>,
}

impl<Load: FnMut(&str) -> Result<String, String>> RegionCache<Load> {
    pub(crate) fn new(load: Load) -> Self {
        Self {
            load,
            files: HashMap::new(),
        }
    }

    fn region(&mut self, path: &str, anchor: &str) -> Result<&str, String> {
        let parsed = self
            .files
            .entry(path.to_string())
            .or_insert_with(|| (self.load)(path).and_then(|code| anchored_regions(&code)));
        match parsed {
            Err(problem) => Err(format!("{path}: {problem}")),
            Ok(regions) => regions
                .get(anchor)
                .map(String::as_str)
                .ok_or_else(|| format!("{path} has no anchor `{anchor}`")),
        }
    }
}

enum AnchorMarker {
    Start(String),
    End(String),
}

/// `// ANCHOR: name` or `// ANCHOR_END: name`, at any indentation.
fn anchor_marker(line: &str) -> Result<Option<AnchorMarker>, String> {
    let Some(comment) = line.trim_start().strip_prefix("//") else {
        return Ok(None);
    };
    let comment = comment.trim_start();
    let (name, marker): (&str, fn(String) -> AnchorMarker) =
        if let Some(name) = comment.strip_prefix("ANCHOR_END:") {
            (name.trim(), AnchorMarker::End)
        } else if let Some(name) = comment.strip_prefix("ANCHOR:") {
            (name.trim(), AnchorMarker::Start)
        } else {
            return Ok(None);
        };
    let starts_with_letter = name.starts_with(|character: char| character.is_ascii_alphabetic());
    let rest_is_word = name
        .chars()
        .all(|character| character.is_ascii_alphanumeric() || character == '_' || character == '-');
    if starts_with_letter && rest_is_word {
        Ok(Some(marker(name.to_string())))
    } else {
        Err(format!(
            "anchor name `{name}` must start with a letter and hold only letters, digits, `_` and `-`"
        ))
    }
}

fn anchored_regions(code: &str) -> Result<Regions, String> {
    let lines: Vec<&str> = code.lines().collect();
    let mut regions = Regions::new();
    let mut open_anchor: Option<(String, usize)> = None;
    for (index, line) in lines.iter().enumerate() {
        let line_number = index + 1;
        let marker =
            anchor_marker(line).map_err(|problem| format!("line {line_number}: {problem}"))?;
        match (marker, open_anchor.take()) {
            (None, still_open) => open_anchor = still_open,
            (Some(AnchorMarker::Start(name)), Some((open_name, _))) => {
                return Err(format!(
                    "line {line_number}: anchor `{name}` starts inside anchor `{open_name}`; anchors may not nest or overlap"
                ));
            }
            (Some(AnchorMarker::Start(name)), None) => {
                if regions.contains_key(&name) {
                    return Err(format!(
                        "line {line_number}: anchor `{name}` is defined twice"
                    ));
                }
                open_anchor = Some((name, index));
            }
            (Some(AnchorMarker::End(name)), None) => {
                return Err(format!(
                    "line {line_number}: ANCHOR_END `{name}` without a matching ANCHOR"
                ));
            }
            (Some(AnchorMarker::End(name)), Some((open_name, start_index))) => {
                if name != open_name {
                    return Err(format!(
                        "line {line_number}: ANCHOR_END `{name}` while anchor `{open_name}` is open; anchors may not nest or overlap"
                    ));
                }
                let body = &lines[start_index + 1..index];
                if body.iter().all(|body_line| body_line.trim().is_empty()) {
                    return Err(format!("line {line_number}: anchor `{name}` is empty"));
                }
                regions.insert(name, without_common_indentation(body));
            }
        }
    }
    match open_anchor {
        Some((name, start_index)) => Err(format!(
            "line {}: anchor `{name}` has no ANCHOR_END",
            start_index + 1
        )),
        None => Ok(regions),
    }
}

fn without_common_indentation(lines: &[&str]) -> String {
    let indentation_of = |line: &&str| line.len() - line.trim_start_matches([' ', '\t']).len();
    let common = lines
        .iter()
        .filter(|line| !line.trim().is_empty())
        .map(indentation_of)
        .min()
        .unwrap_or(0);
    lines
        .iter()
        .map(|line| {
            if line.trim().is_empty() {
                ""
            } else {
                &line[common..]
            }
        })
        .collect::<Vec<_>>()
        .join("\n")
}

/// An open fenced code block: its marker character and run length.
pub(crate) struct Fence {
    marker: char,
    length: usize,
}

impl Fence {
    pub(crate) fn opened_by(line: &str) -> Option<Self> {
        let trimmed = line.trim_start();
        let marker = trimmed
            .chars()
            .next()
            .filter(|first| matches!(first, '`' | '~'))?;
        let length = trimmed
            .chars()
            .take_while(|&character| character == marker)
            .count();
        (length >= 3).then_some(Self { marker, length })
    }

    pub(crate) fn is_closed_by(&self, line: &str) -> bool {
        let trimmed = line.trim();
        trimmed.len() >= self.length && trimmed.chars().all(|character| character == self.marker)
    }
}

struct Include<'a> {
    indentation: &'a str,
    path: &'a str,
    anchor: &'a str,
}

const INCLUDE_PREFIX: &str = "{{#include ";

/// `None` for a line that is not an include; an error for a malformed one.
fn include_directive(line: &str) -> Option<Result<Include<'_>, String>> {
    let trimmed = line.trim();
    let target = trimmed.strip_prefix(INCLUDE_PREFIX)?;
    let indentation = &line[..line.len() - line.trim_start().len()];
    let Some(target) = target.strip_suffix("}}") else {
        return Some(Err(
            "an include ends with `}}` and stands on a line of its own".to_string(),
        ));
    };
    let Some((path, anchor)) = target.trim().rsplit_once(':') else {
        return Some(Err(format!(
            "`{target}` names no anchor; write `{{{{#include <path>:<anchor>}}}}`"
        )));
    };
    Some(Ok(Include {
        indentation,
        path,
        anchor,
    }))
}

/// The first character of a prose line that MDX would read as JSX (`<`) or an expression
/// (`{`), outside code spans and backslash escapes, with its 1-based column.
fn mdx_hazard(line: &str) -> Option<(usize, char)> {
    let characters: Vec<char> = line.chars().collect();
    let mut position = 0;
    while position < characters.len() {
        match characters[position] {
            '\\' => position += 2,
            '`' => position = after_code_span(&characters, position),
            hazard @ ('<' | '{') => return Some((position + 1, hazard)),
            _ => position += 1,
        }
    }
    None
}

/// Where scanning goes on after the backtick run at `start`: past its closing run of the same
/// length, or past the run alone when none closes it (then the backticks are literal text).
fn after_code_span(characters: &[char], start: usize) -> usize {
    let run_length = |from: usize| {
        characters[from..]
            .iter()
            .take_while(|&&character| character == '`')
            .count()
    };
    let opening = run_length(start);
    let mut position = start + opening;
    while position < characters.len() {
        if characters[position] == '`' {
            let closing = run_length(position);
            if closing == opening {
                return position + closing;
            }
            position += closing;
        } else {
            position += 1;
        }
    }
    start + opening
}

/// The frontmatter block (both `---` lines included) and the rest.
fn split_frontmatter(markdown: &str) -> Option<(Vec<&str>, Vec<&str>)> {
    let lines: Vec<&str> = markdown.lines().collect();
    if lines.first() != Some(&"---") {
        return None;
    }
    let closing = lines.iter().skip(1).position(|line| *line == "---")? + 1;
    Some((lines[..=closing].to_vec(), lines[closing + 1..].to_vec()))
}

fn render_page<Load: FnMut(&str) -> Result<String, String>>(
    source_name: &str,
    markdown: &str,
    regions: &mut RegionCache<Load>,
) -> Result<String, Vec<String>> {
    let Some((frontmatter, body)) = split_frontmatter(markdown) else {
        return Err(vec![
            "starts with no `---` frontmatter block (sidebar_position, title, description)"
                .to_string(),
        ]);
    };
    let mut page: Vec<String> = frontmatter.iter().map(ToString::to_string).collect();
    page.push(String::new());
    page.push(format!(
        "{{/* Generated by cargo xtask tutorials from {SOURCE_DIRECTORY}/{source_name} of the Rust binding. Edit that file, not this page. */}}"
    ));
    page.push(String::new());

    let body_start = body
        .iter()
        .position(|line| !line.trim().is_empty())
        .unwrap_or(body.len());
    let first_rendered_line_number = frontmatter.len() + 1 + body_start;
    page.extend(render_markdown(
        &body[body_start..],
        first_rendered_line_number,
        regions,
    )?);
    Ok(page.join("\n") + "\n")
}

/// Expands the includes of `lines`, the source's lines from `first_line_number` on, and checks the
/// prose for MDX hazards. Problems name their source line.
pub(crate) fn render_markdown<Load: FnMut(&str) -> Result<String, String>>(
    lines: &[&str],
    first_line_number: usize,
    regions: &mut RegionCache<Load>,
) -> Result<Vec<String>, Vec<String>> {
    let mut rendered = Vec::new();
    let mut problems = Vec::new();
    let mut open_fence: Option<Fence> = None;
    for (offset, line) in lines.iter().enumerate() {
        let line_number = first_line_number + offset;
        let mut report = |problem: String| problems.push(format!("line {line_number}: {problem}"));
        match &open_fence {
            Some(fence) if fence.is_closed_by(line) => {
                open_fence = None;
                rendered.push((*line).to_string());
            }
            Some(fence) => match include_directive(line) {
                Some(Ok(include)) => match regions.region(include.path, include.anchor) {
                    Ok(region)
                        if region
                            .lines()
                            .any(|region_line| fence.is_closed_by(region_line)) =>
                    {
                        report(format!(
                            "{}:{} holds a line that would close the code fence",
                            include.path, include.anchor
                        ));
                    }
                    Ok(region) => rendered.extend(region.lines().map(|region_line| {
                        if region_line.is_empty() {
                            String::new()
                        } else {
                            format!("{}{region_line}", include.indentation)
                        }
                    })),
                    Err(problem) => report(problem),
                },
                Some(Err(problem)) => report(problem),
                None if line.contains("{{#") => {
                    report("`{{#` starts a directive; only `{{#include <path>:<anchor>}}` on a line of its own is supported".to_string());
                }
                None => rendered.push((*line).to_string()),
            },
            None => {
                if let Some(fence) = Fence::opened_by(line) {
                    open_fence = Some(fence);
                } else if line.contains("{{#") {
                    report("an include belongs inside a fenced code block".to_string());
                } else if let Some((column, hazard)) = mdx_hazard(line) {
                    report(format!(
                        "column {column}: MDX reads `{hazard}` outside code as JSX or an expression; put it in backticks or escape it as `\\{hazard}`"
                    ));
                }
                rendered.push((*line).to_string());
            }
        }
    }
    if open_fence.is_some() {
        problems.push("a fenced code block is never closed".to_string());
    }

    if problems.is_empty() {
        Ok(rendered)
    } else {
        Err(problems)
    }
}

#[cfg(test)]
mod tests {
    use super::{
        Difference, Pages, RegionCache, anchored_regions, compare, mdx_hazard, render_page,
    };

    const REGIONS: &str = include_str!("../fixtures/tutorials/regions.rs");
    const INDENTED_REGION: &str = include_str!("../fixtures/tutorials/indented-region.txt");
    const PAGE_SOURCE: &str = include_str!("../fixtures/tutorials/page.md");
    const PAGE_GENERATED: &str = include_str!("../fixtures/tutorials/page.mdx");
    const NESTED_ANCHORS: &str = include_str!("../fixtures/tutorials/nested-anchors.rs");
    const OVERLAPPING_ANCHORS: &str = include_str!("../fixtures/tutorials/overlapping-anchors.rs");
    const DUPLICATE_ANCHOR: &str = include_str!("../fixtures/tutorials/duplicate-anchor.rs");
    const UNCLOSED_ANCHOR: &str = include_str!("../fixtures/tutorials/unclosed-anchor.rs");
    const FENCE_IN_REGION: &str = include_str!("../fixtures/tutorials/fence-in-region.rs");
    const BROKEN_PAGE: &str = include_str!("../fixtures/tutorials/broken-page.md");

    fn fixture_file(path: &str) -> Result<String, String> {
        match path {
            "../src/regions.rs" => Ok(REGIONS.to_string()),
            "../src/fence-in-region.rs" => Ok(FENCE_IN_REGION.to_string()),
            _ => Err("no such fixture".to_string()),
        }
    }

    fn rendered(source: &str) -> Result<String, Vec<String>> {
        render_page("page.md", source, &mut RegionCache::new(fixture_file))
    }

    #[test]
    fn region_drops_anchor_lines_and_common_indentation() {
        let regions = anchored_regions(REGIONS);
        assert_eq!(
            regions
                .as_ref()
                .map(|found| found.get("indented_region").cloned()),
            Ok(Some(INDENTED_REGION.trim_end_matches('\n').to_string()))
        );
        assert_eq!(
            regions.map(|found| found.get("top_level_region").cloned()),
            Ok(Some("fn second_fixture_function() {}".to_string()))
        );
    }

    #[test]
    fn page_expands_includes_and_carries_frontmatter() {
        assert_eq!(rendered(PAGE_SOURCE), Ok(PAGE_GENERATED.to_string()));
    }

    #[test]
    fn crlf_source_renders_like_lf() {
        let crlf_source = PAGE_SOURCE.replace('\n', "\r\n");
        assert_eq!(rendered(&crlf_source), Ok(PAGE_GENERATED.to_string()));
    }

    #[test]
    fn nested_anchors_are_refused() {
        assert_eq!(
            anchored_regions(NESTED_ANCHORS).map(|_| ()),
            Err("line 4: anchor `inner_region` starts inside anchor `outer_region`; anchors may not nest or overlap".to_string())
        );
    }

    #[test]
    fn overlapping_anchors_are_refused() {
        assert_eq!(
            anchored_regions(OVERLAPPING_ANCHORS).map(|_| ()),
            Err("line 3: anchor `second_region` starts inside anchor `first_region`; anchors may not nest or overlap".to_string())
        );
    }

    #[test]
    fn duplicate_anchor_is_refused() {
        assert_eq!(
            anchored_regions(DUPLICATE_ANCHOR).map(|_| ()),
            Err("line 5: anchor `repeated_region` is defined twice".to_string())
        );
    }

    #[test]
    fn unclosed_anchor_is_refused() {
        assert_eq!(
            anchored_regions(UNCLOSED_ANCHOR).map(|_| ()),
            Err("line 2: anchor `unclosed_region` has no ANCHOR_END".to_string())
        );
    }

    #[test]
    fn end_without_start_is_refused() {
        assert_eq!(
            anchored_regions("fn fixture() {}\n// ANCHOR_END: stray_region\n").map(|_| ()),
            Err("line 2: ANCHOR_END `stray_region` without a matching ANCHOR".to_string())
        );
    }

    #[test]
    fn broken_page_reports_every_problem_with_its_line() {
        assert_eq!(
            rendered(BROKEN_PAGE),
            Err(vec![
                "line 7: an include belongs inside a fenced code block".to_string(),
                "line 10: ../src/regions.rs has no anchor `missing_region`".to_string(),
                "line 14: ../src/absent.rs: no such fixture".to_string(),
                "line 18: ../src/fence-in-region.rs:fenced_region holds a line that would close the code fence".to_string(),
                "line 21: column 23: MDX reads `<` outside code as JSX or an expression; put it in backticks or escape it as `\\<`".to_string(),
                "line 23: column 18: MDX reads `{` outside code as JSX or an expression; put it in backticks or escape it as `\\{`".to_string(),
                "line 26: `{{#` starts a directive; only `{{#include <path>:<anchor>}}` on a line of its own is supported".to_string(),
                "line 27: `../src/regions.rs` names no anchor; write `{{#include <path>:<anchor>}}`".to_string(),
                "a fenced code block is never closed".to_string(),
            ])
        );
    }

    #[test]
    fn page_without_frontmatter_is_refused() {
        assert_eq!(
            rendered("## A heading without frontmatter\n"),
            Err(vec![
                "starts with no `---` frontmatter block (sidebar_position, title, description)"
                    .to_string()
            ])
        );
    }

    #[test]
    fn mdx_hazards_outside_code_only() {
        assert_eq!(mdx_hazard("plain prose with `a < b` in code"), None);
        assert_eq!(
            mdx_hazard("a ``code span with ` inside`` and {x}"),
            Some((35, '{'))
        );
        assert_eq!(mdx_hazard(r"escaped \< and \{ stay text"), None);
        assert_eq!(
            mdx_hazard("an unclosed ` backtick, then <tag>"),
            Some((30, '<'))
        );
        assert_eq!(mdx_hazard("`<example>.onboarding-id.txt`"), None);
    }

    #[test]
    fn compare_finds_missing_stale_and_left_over_pages() {
        let expected = Pages::from([
            ("kept.mdx".to_string(), "same\n".to_string()),
            ("changed.mdx".to_string(), "new\n".to_string()),
            ("added.mdx".to_string(), "fresh\n".to_string()),
        ]);
        let existing = Pages::from([
            ("kept.mdx".to_string(), "same\r\n".to_string()),
            ("changed.mdx".to_string(), "old\n".to_string()),
            ("removed.mdx".to_string(), "gone\n".to_string()),
        ]);
        assert_eq!(
            compare(&expected, &existing),
            vec![
                Difference::Missing("added.mdx".to_string()),
                Difference::Stale("changed.mdx".to_string()),
                Difference::NotGenerated("removed.mdx".to_string()),
            ]
        );
    }
}
