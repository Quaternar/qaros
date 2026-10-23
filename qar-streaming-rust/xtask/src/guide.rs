//! `cargo xtask guide [--check]` puts a Rust tab into every `<CodeTabs>` of the developer guide
//! that has a C tab, so a reader who picks Rust sees the binding's code beside the C and C# code.
//!
//! A page's tabs come from `examples/guide/<page>.md`: one `## ` heading per C tab of the page, in
//! page order, followed by the tab's content. The heading names the tab in messages and is not
//! copied. Code is pulled from anchored regions with the include syntax of the tutorials and
//! expanded by the same code (`tutorials.rs`); the regions live in `examples/tests/guide/`, which
//! is compiled with the tests.
//!
//! The Rust tab goes last in its `<CodeTabs>`, and a Rust tab already there is replaced, so a
//! second run changes nothing. `--check` writes nothing and fails on a stale page. Both modes fail
//! on a page whose C tabs and source headings differ in number, a source without its page, and a
//! page with a C tab but no source.
//!
//! The check runs in qaros's CI on every change to the developer guide, so its messages are
//! written for a docs writer who does not know the binding: each names the page and the code
//! group (by the heading above it) and ends with what to do.
//!
//! qaros-rust holds no developer guide: there `PAGES_DIRECTORY` is `None` and the command only
//! expands the sources, which checks their includes.

use std::collections::BTreeMap;
use std::path::Path;
use std::process::ExitCode;

use crate::tutorials::{
    Fence, RegionCache, read_included_file, read_text, render_markdown, source_names,
};

const MODULE_DIRECTORY: &str = "qar-streaming-rust";
const SOURCE_DIRECTORY: &str = "examples/guide";
const SNIPPET_DIRECTORY: &str = "examples/tests/guide";
const PAGES_DIRECTORY: Option<&str> = Some("../docs/website/docs/developer-guide");

const TAB_HEADING: &str = "## ";
const TABS_OPENING: &str = "<CodeTabs>";
const TABS_CLOSING: &str = "</CodeTabs>";
const C_TAB_OPENING: &str = r#"<Lang value="c">"#;
const RUST_TAB_OPENING: &str = r#"<Lang value="rust">"#;
const TAB_CLOSING: &str = "</Lang>";

/// One Rust tab: the source heading that names it and its lines, from opening to closing tag.
#[derive(Debug)]
struct RustTab {
    heading: String,
    lines: Vec<String>,
}

/// Page stem (`api-conventions`) to the page's Rust tabs, in page order.
type TabsByPage = BTreeMap<String, Vec<RustTab>>;

/// A failure, with the stem of the page whose Rust tabs it concerns, if any.
struct Failure {
    page_stem: Option<String>,
    message: String,
}

/// A page with its Rust tabs in place, and the code groups whose Rust tab was missing or stale.
#[derive(Debug, PartialEq, Eq)]
struct UpdatedPage {
    text: String,
    outdated_groups: Vec<String>,
}

pub fn run(arguments: &[String]) -> ExitCode {
    let check_only = match arguments {
        [] => false,
        [flag] if flag == "--check" => true,
        _ => {
            eprintln!("usage: cargo xtask guide [--check]");
            return ExitCode::FAILURE;
        }
    };
    let root = super::workspace_root();
    let tabs_by_page = match rust_tabs(&root) {
        Ok(tabs_by_page) => tabs_by_page,
        Err(failures) => return report_failures(&failures),
    };
    let Some(pages_directory) = PAGES_DIRECTORY else {
        println!(
            "guide: every Rust tab of {SOURCE_DIRECTORY} expands; this repository holds no developer guide pages"
        );
        return ExitCode::SUCCESS;
    };
    let pages_label = pages_directory.trim_start_matches("../");
    let stale_pages = match stale_pages(&root.join(pages_directory), pages_label, &tabs_by_page) {
        Ok(stale_pages) => stale_pages,
        Err(failures) => return report_failures(&failures),
    };
    if stale_pages.is_empty() {
        println!("guide: the Rust tabs of {pages_label} are up to date");
        return ExitCode::SUCCESS;
    }
    if check_only {
        let failures: Vec<Failure> = stale_pages
            .iter()
            .flat_map(|(name, updated)| stale_page_failures(pages_label, name, updated))
            .collect();
        return report_failures(&failures);
    }
    for (name, updated) in &stale_pages {
        let path = root.join(pages_directory).join(name);
        if let Err(error) = std::fs::write(&path, &updated.text) {
            eprintln!("cannot write {}: {error}", path.display());
            return ExitCode::FAILURE;
        }
        println!("guide: wrote {pages_label}/{name}");
    }
    ExitCode::SUCCESS
}

fn stale_page_failures(pages_label: &str, name: &str, updated: &UpdatedPage) -> Vec<Failure> {
    let page_stem = Some(page_stem(name).to_string());
    if updated.outdated_groups.is_empty() {
        return vec![Failure {
            page_stem,
            message: format!("{pages_label}/{name}: its Rust tabs are out of date"),
        }];
    }
    updated
        .outdated_groups
        .iter()
        .map(|group| Failure {
            page_stem: page_stem.clone(),
            message: format!("{pages_label}/{name}: {group}"),
        })
        .collect()
}

/// Prints each failure, and after the last one of a page what to do about that page.
fn report_failures(failures: &[Failure]) -> ExitCode {
    for (index, failure) in failures.iter().enumerate() {
        eprintln!("{}", failure.message);
        let next_page_stem = failures.get(index + 1).map(|next| &next.page_stem);
        if let Some(page_stem) = &failure.page_stem
            && next_page_stem != Some(&failure.page_stem)
        {
            eprintln!("{}", fix_hint(page_stem));
        }
    }
    ExitCode::FAILURE
}

fn fix_hint(page_stem: &str) -> String {
    let snippet_module = page_stem.replace('-', "_");
    format!(
        "  fix: add or update the Rust tab in {MODULE_DIRECTORY}/{SOURCE_DIRECTORY}/{page_stem}.md \
         (one `{}` section per C tab, in page order; its code in \
         {MODULE_DIRECTORY}/{SNIPPET_DIRECTORY}/{snippet_module}.rs), then run `cargo xtask guide` \
         from {MODULE_DIRECTORY}/; or ask the Rust binding's maintainers",
        TAB_HEADING.trim_end()
    )
}

fn rust_tabs(root: &Path) -> Result<TabsByPage, Vec<Failure>> {
    let general_failure = |message: String| {
        vec![Failure {
            page_stem: None,
            message,
        }]
    };
    let source_directory = root.join(SOURCE_DIRECTORY);
    let canonical_root = std::fs::canonicalize(root)
        .map_err(|error| general_failure(format!("cannot resolve {}: {error}", root.display())))?;
    let mut regions = RegionCache::new(|relative_path: &str| {
        read_included_file(&canonical_root, &source_directory, relative_path)
    });

    let mut tabs_by_page = TabsByPage::new();
    let mut failures = Vec::new();
    for source_name in source_names(&source_directory).map_err(general_failure)? {
        let source_label = format!("{SOURCE_DIRECTORY}/{source_name}");
        let markdown = read_text(&source_directory.join(&source_name)).map_err(general_failure)?;
        let stem = source_name.trim_end_matches(".md");
        match render_tabs(&source_label, &markdown, &mut regions) {
            Ok(tabs) => {
                tabs_by_page.insert(stem.to_string(), tabs);
            }
            Err(source_problems) => {
                failures.extend(source_problems.into_iter().map(|problem| Failure {
                    page_stem: Some(stem.to_string()),
                    message: format!("{MODULE_DIRECTORY}/{source_label}: {problem}"),
                }));
            }
        }
    }
    if failures.is_empty() {
        Ok(tabs_by_page)
    } else {
        Err(failures)
    }
}

/// The content lines under each `## ` heading of a source, as index ranges into `lines`.
fn tab_sections(lines: &[&str]) -> Result<Vec<(usize, usize)>, Vec<String>> {
    let mut starts = Vec::new();
    let mut problems = Vec::new();
    let mut open_fence: Option<Fence> = None;
    for (index, line) in lines.iter().enumerate() {
        if let Some(fence) = &open_fence {
            if fence.is_closed_by(line) {
                open_fence = None;
            }
        } else if let Some(fence) = Fence::opened_by(line) {
            open_fence = Some(fence);
        } else if line.starts_with(TAB_HEADING) {
            starts.push(index);
        }
        let before_first_heading = starts.is_empty() && !line.trim().is_empty();
        if before_first_heading {
            problems.push(format!(
                "line {}: text before the first `{}` heading; every Rust tab starts with one",
                index + 1,
                TAB_HEADING.trim_end()
            ));
        }
    }
    if starts.is_empty() && problems.is_empty() {
        problems.push(format!(
            "no `{}` heading, so no Rust tab",
            TAB_HEADING.trim_end()
        ));
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    let ends = starts.iter().skip(1).copied().chain([lines.len()]);
    Ok(starts
        .iter()
        .zip(ends)
        .map(|(heading, end)| (heading + 1, end))
        .collect())
}

fn render_tabs<Load: FnMut(&str) -> Result<String, String>>(
    source_label: &str,
    markdown: &str,
    regions: &mut RegionCache<Load>,
) -> Result<Vec<RustTab>, Vec<String>> {
    let lines: Vec<&str> = markdown.lines().collect();
    let mut tabs = Vec::new();
    let mut problems = Vec::new();
    for (start, end) in tab_sections(&lines)? {
        let heading = lines[start - 1];
        let content = match render_markdown(&lines[start..end], start + 1, regions) {
            Ok(content) => content,
            Err(section_problems) => {
                problems.extend(section_problems);
                continue;
            }
        };
        let first = content.iter().position(|line| !line.trim().is_empty());
        let last = content.iter().rposition(|line| !line.trim().is_empty());
        let (Some(first), Some(last)) = (first, last) else {
            problems.push(format!("line {start}: the tab under `{heading}` is empty"));
            continue;
        };
        let content = &content[first..=last];
        if let Some(markup) = content
            .iter()
            .find(|line| [TABS_OPENING, TABS_CLOSING, TAB_CLOSING].contains(&line.trim()))
        {
            problems.push(format!(
                "line {start}: the tab under `{heading}` holds a `{}` line, which would end the tab on the page",
                markup.trim()
            ));
            continue;
        }
        let mut lines = vec![
            RUST_TAB_OPENING.to_string(),
            String::new(),
            format!(
                "{{/* Generated by cargo xtask guide from {source_label} of the Rust binding. Edit that file, not this tab. */}}"
            ),
            String::new(),
        ];
        lines.extend(content.iter().cloned());
        lines.extend([String::new(), TAB_CLOSING.to_string()]);
        tabs.push(RustTab {
            heading: heading.to_string(),
            lines,
        });
    }
    if problems.is_empty() {
        Ok(tabs)
    } else {
        Err(problems)
    }
}

/// The pages whose Rust tabs differ from the sources, with their expected content.
fn stale_pages(
    pages_directory: &Path,
    pages_label: &str,
    tabs_by_page: &TabsByPage,
) -> Result<BTreeMap<String, UpdatedPage>, Vec<Failure>> {
    let page_names = page_names(pages_directory).map_err(|message| {
        vec![Failure {
            page_stem: None,
            message,
        }]
    })?;
    let mut failures: Vec<Failure> = tabs_by_page
        .keys()
        .filter(|stem| {
            !page_names
                .iter()
                .any(|name| page_stem(name) == stem.as_str())
        })
        .map(|stem| Failure {
            page_stem: Some(stem.clone()),
            message: format!(
                "{MODULE_DIRECTORY}/{SOURCE_DIRECTORY}/{stem}.md: no page {stem}.md or {stem}.mdx in {pages_label}; rename or remove the source to follow the page"
            ),
        })
        .collect();
    let mut stale = BTreeMap::new();
    for name in page_names {
        let stem = page_stem(&name).to_string();
        let tabs = tabs_by_page.get(&stem).map_or(&[][..], Vec::as_slice);
        let page = read_text(&pages_directory.join(&name)).map_err(|message| {
            vec![Failure {
                page_stem: None,
                message,
            }]
        })?;
        match with_rust_tabs(&page, tabs) {
            Ok(updated) if updated.text != page => {
                stale.insert(name, updated);
            }
            Ok(_) => {}
            Err(problems) => failures.extend(problems.into_iter().map(|problem| Failure {
                page_stem: Some(stem.clone()),
                message: format!("{pages_label}/{name}: {problem}"),
            })),
        }
    }
    if failures.is_empty() {
        Ok(stale)
    } else {
        Err(failures)
    }
}

fn page_stem(name: &str) -> &str {
    name.strip_suffix(".mdx")
        .or_else(|| name.strip_suffix(".md"))
        .unwrap_or(name)
}

/// The `.md` and `.mdx` files directly in the pages directory, sorted.
fn page_names(pages_directory: &Path) -> Result<Vec<String>, String> {
    let entries = std::fs::read_dir(pages_directory)
        .map_err(|error| format!("cannot list {}: {error}", pages_directory.display()))?;
    let mut names = Vec::new();
    for entry in entries {
        let entry =
            entry.map_err(|error| format!("cannot list {}: {error}", pages_directory.display()))?;
        let name = entry.file_name().to_string_lossy().into_owned();
        let is_page = Path::new(&name)
            .extension()
            .is_some_and(|extension| extension == "md" || extension == "mdx");
        if is_page && entry.path().is_file() {
            names.push(name);
        }
    }
    names.sort();
    Ok(names)
}

/// Where a line of a page stands, for [`with_rust_tabs`].
#[derive(Clone, Copy, PartialEq, Eq)]
enum PagePosition {
    OutsideCodeTabs,
    InsideCodeTabs { holds_c_tab: bool },
    InsideRustTab { holds_c_tab: bool },
}

/// A `<CodeTabs>` of a page, named in messages by the heading above it.
struct CodeGroup {
    heading: Option<String>,
    line_number: usize,
    rust_tab: Option<Vec<String>>,
}

impl CodeGroup {
    fn describe(&self) -> String {
        match &self.heading {
            Some(heading) => format!("code group under \"{heading}\" (line {})", self.line_number),
            None => format!("code group at line {}", self.line_number),
        }
    }
}

fn is_markdown_heading(line: &str) -> bool {
    line.starts_with('#') && line.trim_start_matches('#').starts_with(' ')
}

/// The page with `tabs` as the last tab of each `<CodeTabs>` that has a C tab, in order, and
/// without the Rust tabs it had.
fn with_rust_tabs(page: &str, tabs: &[RustTab]) -> Result<UpdatedPage, Vec<String>> {
    let mut output: Vec<String> = Vec::new();
    let mut outdated_groups = Vec::new();
    let mut next_tabs = tabs.iter();
    let mut groups_with_c_tab: Vec<CodeGroup> = Vec::new();
    let mut current_group: Option<CodeGroup> = None;
    let mut rust_tab_on_page: Vec<String> = Vec::new();
    let mut heading: Option<String> = None;
    let mut position = PagePosition::OutsideCodeTabs;
    let mut open_fence: Option<Fence> = None;
    for (index, line) in page.lines().enumerate() {
        let line_number = index + 1;
        let was_in_rust_tab = matches!(position, PagePosition::InsideRustTab { .. });
        if let Some(fence) = &open_fence {
            if fence.is_closed_by(line) {
                open_fence = None;
            }
        } else if let Some(fence) = Fence::opened_by(line) {
            open_fence = Some(fence);
        } else {
            if position == PagePosition::OutsideCodeTabs && is_markdown_heading(line) {
                heading = Some(line.trim().to_string());
            }
            position = match (position, line.trim()) {
                (PagePosition::InsideRustTab { holds_c_tab }, TAB_CLOSING) => {
                    PagePosition::InsideCodeTabs { holds_c_tab }
                }
                (PagePosition::InsideRustTab { .. }, _) => position,
                (PagePosition::OutsideCodeTabs, TABS_OPENING) => {
                    current_group = Some(CodeGroup {
                        heading: heading.clone(),
                        line_number,
                        rust_tab: None,
                    });
                    PagePosition::InsideCodeTabs { holds_c_tab: false }
                }
                (PagePosition::InsideCodeTabs { .. }, TABS_OPENING) => {
                    return Err(vec![format!(
                        "line {line_number}: `{TABS_OPENING}` inside another"
                    )]);
                }
                (PagePosition::InsideCodeTabs { .. }, C_TAB_OPENING) => {
                    PagePosition::InsideCodeTabs { holds_c_tab: true }
                }
                (PagePosition::InsideCodeTabs { holds_c_tab }, RUST_TAB_OPENING) => {
                    rust_tab_on_page.clear();
                    PagePosition::InsideRustTab { holds_c_tab }
                }
                (PagePosition::InsideCodeTabs { holds_c_tab }, TABS_CLOSING) => {
                    if let Some(group) = current_group.take() {
                        if holds_c_tab {
                            if let Some(tab) = next_tabs.next() {
                                output.extend(tab.lines.iter().cloned());
                                if let Some(problem) = outdated_rust_tab(&group, tab) {
                                    outdated_groups.push(problem);
                                }
                            }
                            groups_with_c_tab.push(group);
                        } else if group.rust_tab.is_some() {
                            outdated_groups
                                .push(format!("{}: has a Rust tab but no C tab", group.describe()));
                        }
                    }
                    PagePosition::OutsideCodeTabs
                }
                (PagePosition::OutsideCodeTabs, TABS_CLOSING) => {
                    return Err(vec![format!(
                        "line {line_number}: `{TABS_CLOSING}` without `{TABS_OPENING}`"
                    )]);
                }
                (unchanged, _) => unchanged,
            };
        }
        let is_in_rust_tab = matches!(position, PagePosition::InsideRustTab { .. });
        if was_in_rust_tab || is_in_rust_tab {
            rust_tab_on_page.push(line.to_string());
        } else {
            output.push(line.to_string());
        }
        let rust_tab_ended = was_in_rust_tab && !is_in_rust_tab;
        if rust_tab_ended && let Some(group) = current_group.as_mut() {
            group.rust_tab = Some(std::mem::take(&mut rust_tab_on_page));
        }
    }
    if open_fence.is_some() {
        return Err(vec!["a fenced code block is never closed".to_string()]);
    }
    if position != PagePosition::OutsideCodeTabs {
        return Err(vec![format!("a `{TABS_OPENING}` is never closed")]);
    }
    if groups_with_c_tab.len() != tabs.len() {
        return Err(count_mismatch_problems(&groups_with_c_tab, tabs));
    }
    Ok(UpdatedPage {
        text: output.join("\n") + "\n",
        outdated_groups,
    })
}

fn outdated_rust_tab(group: &CodeGroup, tab: &RustTab) -> Option<String> {
    match &group.rust_tab {
        None => Some(format!(
            "{}: its Rust tab is missing; its source \"{}\" is there",
            group.describe(),
            tab.heading
        )),
        Some(lines) if *lines != tab.lines => Some(format!(
            "{}: its Rust tab differs from its source \"{}\"",
            group.describe(),
            tab.heading
        )),
        Some(_) => None,
    }
}

/// Names the groups without a Rust tab, or the source tabs that match no group, when the page's
/// groups with a C tab and the source's tabs differ in number.
fn count_mismatch_problems(groups: &[CodeGroup], tabs: &[RustTab]) -> Vec<String> {
    let named: Vec<String> = if groups.len() > tabs.len() {
        groups
            .iter()
            .filter(|group| group.rust_tab.is_none())
            .map(|group| format!("{}: has a C tab but no Rust tab", group.describe()))
            .collect()
    } else {
        tabs.iter()
            .filter(|tab| {
                !groups
                    .iter()
                    .any(|group| group.rust_tab.as_ref() == Some(&tab.lines))
            })
            .map(|tab| {
                format!(
                    "the Rust tab \"{}\" matches no code group with a C tab; remove it from the source if its group is gone",
                    tab.heading
                )
            })
            .collect()
    };
    if named.is_empty() {
        vec![format!(
            "{} code groups with a C tab, but {} Rust tabs in the source; it needs one per group, in page order",
            groups.len(),
            tabs.len()
        )]
    } else {
        named
    }
}

#[cfg(test)]
mod tests {
    use super::{RegionCache, RustTab, fix_hint, render_tabs, with_rust_tabs};

    const REGIONS: &str = include_str!("../fixtures/guide/regions.rs");
    const TABS_SOURCE: &str = include_str!("../fixtures/guide/tabs.md");
    const PAGE: &str = include_str!("../fixtures/guide/page.mdx");
    const PAGE_WITH_RUST_TABS: &str = include_str!("../fixtures/guide/page-with-rust-tabs.mdx");

    fn fixture_file(path: &str) -> Result<String, String> {
        match path {
            "../src/regions.rs" => Ok(REGIONS.to_string()),
            _ => Err("no such fixture".to_string()),
        }
    }

    fn fixture_tabs() -> Vec<RustTab> {
        render_tabs(
            "examples/guide/tabs.md",
            TABS_SOURCE,
            &mut RegionCache::new(fixture_file),
        )
        .unwrap_or_else(|problems| panic!("the fixture tabs render: {problems:?}"))
    }

    fn page_text(page: &str, tabs: &[RustTab]) -> Result<String, Vec<String>> {
        with_rust_tabs(page, tabs).map(|updated| updated.text)
    }

    #[test]
    fn rust_tab_goes_last_in_each_code_tabs_with_a_c_tab() {
        assert_eq!(
            page_text(PAGE, &fixture_tabs()),
            Ok(PAGE_WITH_RUST_TABS.to_string())
        );
    }

    #[test]
    fn missing_rust_tabs_are_named_by_group() {
        assert_eq!(
            with_rust_tabs(PAGE, &fixture_tabs()).map(|updated| updated.outdated_groups),
            Ok(vec![
                "code group under \"# Fixture Guide Page\" (line 12): its Rust tab is missing; its source \"## First tab group\" is there"
                    .to_string(),
                "code group under \"# Fixture Guide Page\" (line 48): its Rust tab is missing; its source \"## Second tab group\" is there"
                    .to_string(),
            ])
        );
    }

    #[test]
    fn second_run_changes_nothing() {
        let updated = with_rust_tabs(PAGE_WITH_RUST_TABS, &fixture_tabs())
            .unwrap_or_else(|problems| panic!("the fixture page updates: {problems:?}"));
        assert_eq!(updated.text, PAGE_WITH_RUST_TABS);
        assert_eq!(updated.outdated_groups, Vec::<String>::new());
    }

    #[test]
    fn stale_rust_tab_is_replaced_and_named() {
        let stale_page = PAGE_WITH_RUST_TABS.replace(
            "Prose for the second Rust tab, which holds no code.",
            "An outdated line of the second Rust tab.",
        );
        assert_ne!(stale_page, PAGE_WITH_RUST_TABS);
        assert_eq!(
            with_rust_tabs(&stale_page, &fixture_tabs()),
            Ok(super::UpdatedPage {
                text: PAGE_WITH_RUST_TABS.to_string(),
                outdated_groups: vec![
                    "code group under \"# Fixture Guide Page\" (line 59): its Rust tab differs from its source \"## Second tab group\""
                        .to_string()
                ],
            })
        );
    }

    #[test]
    fn crlf_page_renders_like_lf() {
        let crlf_page = PAGE.replace('\n', "\r\n");
        assert_eq!(
            page_text(&crlf_page, &fixture_tabs()),
            Ok(PAGE_WITH_RUST_TABS.to_string())
        );
    }

    #[test]
    fn new_c_group_without_a_rust_tab_is_named() {
        let page = format!(
            "{PAGE_WITH_RUST_TABS}\n## A fixture section added later\n\n<CodeTabs>\n<Lang value=\"c\">\n\nA new fixture C tab.\n\n</Lang>\n</CodeTabs>\n"
        );
        let new_group_line = PAGE_WITH_RUST_TABS.lines().count() + 4;
        assert_eq!(
            page_text(&page, &fixture_tabs()),
            Err(vec![format!(
                "code group under \"## A fixture section added later\" (line {new_group_line}): has a C tab but no Rust tab"
            )])
        );
    }

    #[test]
    fn rust_tab_of_a_removed_group_is_named() {
        let first_group_end = PAGE_WITH_RUST_TABS
            .find("A group without a C tab")
            .unwrap_or_else(|| panic!("the fixture page has its second group"));
        let page_with_first_group_only = &PAGE_WITH_RUST_TABS[..first_group_end];
        assert_eq!(
            page_text(page_with_first_group_only, &fixture_tabs()),
            Err(vec![
                "the Rust tab \"## Second tab group\" matches no code group with a C tab; remove it from the source if its group is gone"
                    .to_string()
            ])
        );
    }

    #[test]
    fn fewer_rust_tabs_than_c_tabs_names_the_groups_without_one() {
        assert_eq!(
            page_text(PAGE, &fixture_tabs()[..1]),
            Err(vec![
                "code group under \"# Fixture Guide Page\" (line 12): has a C tab but no Rust tab"
                    .to_string(),
                "code group under \"# Fixture Guide Page\" (line 48): has a C tab but no Rust tab"
                    .to_string(),
            ])
        );
    }

    #[test]
    fn fix_hint_names_the_source_and_the_snippet_module() {
        assert_eq!(
            fix_hint("api-conventions"),
            "  fix: add or update the Rust tab in qar-streaming-rust/examples/guide/api-conventions.md \
             (one `##` section per C tab, in page order; its code in \
             qar-streaming-rust/examples/tests/guide/api_conventions.rs), then run `cargo xtask guide` \
             from qar-streaming-rust/; or ask the Rust binding's maintainers"
        );
    }

    #[test]
    fn text_before_the_first_heading_is_refused() {
        let source = format!("A stray fixture line.\n\n{TABS_SOURCE}");
        assert_eq!(
            render_tabs("tabs.md", &source, &mut RegionCache::new(fixture_file)).map(|_| ()),
            Err(vec![
                "line 1: text before the first `##` heading; every Rust tab starts with one"
                    .to_string()
            ])
        );
    }

    #[test]
    fn tab_markup_inside_a_tab_is_refused() {
        let source = "## A fixture tab\n\n```text\n</Lang>\n```\n";
        assert_eq!(
            render_tabs("tabs.md", source, &mut RegionCache::new(fixture_file)).map(|_| ()),
            Err(vec![
                "line 1: the tab under `## A fixture tab` holds a `</Lang>` line, which would end the tab on the page"
                    .to_string()
            ])
        );
    }
}
