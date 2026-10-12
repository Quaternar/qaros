// Generates the C# API reference (qar-streaming-sharp) as Docusaurus Markdown.
//
// DocFX (pinned in ../dotnet-tools.json) reads the managed project through
// Roslyn and writes its metadata model as YAML (one ManagedReference file per
// type) into docfx/qar-streaming-sharp/_yaml/. This script renders that model
// into api/qar-streaming-sharp/:
//   - index.md: the landing page, docfx/qar-streaming-sharp/overview.md followed
//     by one table of types per area (docfx/qar-streaming-sharp/areas.json),
//   - one page per public type: summary, declaration, inheritance, then one
//     section per member kind, each member a heading with an explicit id,
//     enums as one table of values,
//   - sidebar-category-docfx-qar-streaming-sharp.json, grouped like the landing page.
// Crefs become relative .md links (Docusaurus resolves them per docs version),
// BCL types link to learn.microsoft.com.
//
// Only the .NET SDK is needed: nothing native is built.
const fs = require("fs");
const path = require("path");
const { execFileSync } = require("child_process");
const yaml = require("js-yaml");

const WEBSITE_DIR = path.join(__dirname, "..");
const REPO_ROOT = path.join(WEBSITE_DIR, "..", "..");
const API_ID = "qar-streaming-sharp";
const CONFIG_DIR = path.join(WEBSITE_DIR, "docfx", API_ID);
const DOCFX_JSON = path.join(CONFIG_DIR, "docfx.json");
const YAML_DIR = path.join(CONFIG_DIR, "_yaml");
const AREAS_JSON = path.join(CONFIG_DIR, "areas.json");
const OVERVIEW_MD = path.join(CONFIG_DIR, "overview.md");
const OUT_DIR = path.join(WEBSITE_DIR, "api", API_ID);
const SIDEBAR_JSON = path.join(WEBSITE_DIR, `sidebar-category-docfx-${API_ID}.json`);
const ASSEMBLY = "QarStreamingSharp.dll";
// `npm run docs:version` pins source links to the commit a release snapshot was taken from.
const SOURCE_URL = `https://github.com/Quaternar/qaros/blob/${process.env.QAROS_SOURCE_REF || "main"}/`;

const TYPE_KINDS = new Set(["Class", "Struct", "Interface", "Enum", "Delegate"]);
const MEMBER_SECTIONS = [
	["Constructor", "Constructors"],
	["Field", "Fields"],
	["Property", "Properties"],
	["Method", "Methods"],
	["Event", "Events"],
	["Operator", "Operators"],
];
// Heading ids the page itself uses; a member anchor must not take one.
const RESERVED_ANCHORS = new Set(["remarks", "examples", "see-also", "values", ...MEMBER_SECTIONS.map(([, s]) => s.toLowerCase())]);
// A section with this many members also gets an overview table.
const OVERVIEW_TABLE_MIN = 3;
const LANGWORDS = new Set(["true", "false", "null"]);

// ---------------------------------------------------------------- generation

function run(args) {
	execFileSync("dotnet", args, { cwd: WEBSITE_DIR, stdio: "inherit" });
}

function generateYaml() {
	// Stale files of removed types would otherwise survive a regenerate.
	fs.rmSync(YAML_DIR, { recursive: true, force: true });
	try {
		run(["tool", "restore"]);
		run(["docfx", "metadata", DOCFX_JSON]);
	} catch (error) {
		console.error(
			"C# API generation failed. It needs the .NET SDK (8 or newer) on PATH; " +
				"`dotnet tool restore` installs the pinned DocFX from dotnet-tools.json."
		);
		throw error;
	}
}

// ---------------------------------------------------------------- model

function slug(text) {
	return text
		.toLowerCase()
		.replace(/[^a-z0-9]+/g, "-")
		.replace(/^-+|-+$/g, "");
}

function baseId(item) {
	return item.id.replace(/\(.*$/, "");
}

function loadModel() {
	const items = new Map();
	const refs = new Map();
	const types = [];
	const files = fs.readdirSync(YAML_DIR).filter((f) => f.endsWith(".yml") && f !== "toc.yml");
	for (const file of files) {
		const doc = yaml.load(fs.readFileSync(path.join(YAML_DIR, file), "utf8"));
		for (const item of doc.items ?? []) {
			items.set(item.uid, item);
			if (TYPE_KINDS.has(item.type)) {
				types.push({ item, page: file.replace(/\.yml$/, ""), members: [] });
			}
		}
		for (const ref of doc.references ?? []) {
			if (!refs.has(ref.uid)) refs.set(ref.uid, ref);
		}
	}
	const pageOf = new Map(types.map((t) => [t.item.uid, t.page]));
	const anchorOf = new Map();
	const overloadTarget = new Map();
	for (const type of types) {
		type.members = (type.item.children ?? []).map((uid) => items.get(uid)).filter(Boolean);
		type.members.sort((a, b) => (a.source?.startLine ?? 0) - (b.source?.startLine ?? 0));
		assignAnchors(type.members, anchorOf);
		for (const member of type.members) {
			if (member.overload && !overloadTarget.has(member.overload)) overloadTarget.set(member.overload, member.uid);
		}
	}
	types.sort((a, b) => a.item.name.localeCompare(b.item.name));
	return { items, refs, types, pageOf, anchorOf, overloadTarget };
}

// A member's anchor is its name; overloads add their parameter types.
function assignAnchors(members, anchorOf) {
	const countByBase = new Map();
	members.forEach((m) => countByBase.set(baseId(m), (countByBase.get(baseId(m)) ?? 0) + 1));
	const used = new Set(RESERVED_ANCHORS);
	for (const member of members) {
		const overloaded = countByBase.get(baseId(member)) > 1;
		let anchor = slug(overloaded ? `${baseId(member)} ${member.name.replace(/^[^(]*/, "")}` : baseId(member)) || "member";
		if (used.has(anchor)) anchor = `${anchor}-member`;
		for (let n = 2; used.has(anchor); n++) anchor = `${anchor.replace(/-\d+$/, "")}-${n}`;
		used.add(anchor);
		anchorOf.set(member.uid, anchor);
	}
}

// ---------------------------------------------------------------- markdown helpers

function escapeText(text) {
	return text.replace(/[\\`*_[\]<>|]/g, (c) => ({ "<": "&lt;", ">": "&gt;" })[c] ?? `\\${c}`);
}

function decodeEntities(text) {
	return text
		.replace(/&lt;/g, "<")
		.replace(/&gt;/g, ">")
		.replace(/&quot;/g, '"')
		.replace(/&#39;/g, "'")
		.replace(/&amp;/g, "&");
}

function codeSpan(text) {
	const plain = decodeEntities(text);
	const fence = plain.includes("`") ? "``" : "`";
	return `${fence}${plain}${fence}`;
}

function link(text, url) {
	return url ? `[${text}](${/[\s()]/.test(url) ? `<${url}>` : url})` : text;
}

function cell(markdown) {
	return markdown.replace(/\s*\n\s*/g, " ").replace(/(?<!\\)\|/g, "\\|").trim();
}

function firstParagraph(markdown) {
	return markdown.trim().split(/\n\s*\n/)[0] ?? "";
}

function plainText(markdown) {
	return decodeEntities(
		markdown
			.replace(/\[([^\]]*)\]\([^)]*\)/g, "$1")
			.replace(/[`*]/g, "")
			.replace(/\\(.)/g, "$1")
			.replace(/\s+/g, " ")
	).trim();
}

function kindLabel(item) {
	return item.type.toLowerCase();
}

// ---------------------------------------------------------------- links

class Linker {
	constructor(model, page) {
		this.model = model;
		this.page = page;
	}

	// URL of a uid as seen from this page, or undefined.
	url(rawUid) {
		const { items, refs, pageOf, anchorOf, overloadTarget } = this.model;
		const uid = overloadTarget.get(rawUid) ?? rawUid;
		if (pageOf.has(uid)) return `${pageOf.get(uid)}.md`;
		const item = items.get(uid);
		if (item && pageOf.has(item.parent) && anchorOf.has(uid)) {
			const target = pageOf.get(item.parent);
			return `${target === this.page ? "" : `${target}.md`}#${anchorOf.get(uid)}`;
		}
		const ref = refs.get(uid);
		if (ref?.href && /^https?:/.test(ref.href)) return ref.href;
		if (ref?.definition && ref.definition !== uid) return this.url(ref.definition);
		return undefined;
	}

	// Display name of a cref: types by name, members as Type.Member.
	crefName(uid) {
		const { items, refs, overloadTarget } = this.model;
		const target = items.get(overloadTarget.get(uid) ?? uid) ?? refs.get(uid);
		if (!target) return uid.replace(/\(.*$/, "").split(".").pop();
		const name = TYPE_KINDS.has(target.type) ? target.name : target.nameWithType ?? target.name;
		return name.replace(/\(.*\)$/, "");
	}

	cref(uid, text) {
		return link(codeSpan(text || this.crefName(uid)), this.url(uid));
	}

	// A type reference (`Result{PeerId}`, `System.String`, `{T}`) as linked text.
	type(uid) {
		if (!uid) return "";
		if (/^\{.*\}$/.test(uid)) return escapeText(uid.slice(1, -1));
		const ref = this.model.refs.get(uid);
		const spec = ref?.["spec.csharp"];
		if (spec) {
			return spec.map((part) => (part.uid ? link(escapeText(part.name), this.url(part.uid)) : escapeText(part.name))).join("");
		}
		const name = ref?.name ?? this.model.items.get(uid)?.name ?? uid;
		return link(escapeText(name), this.url(uid));
	}

	// XML doc comment content (DocFX hands it over as Markdown with some HTML) as Markdown.
	doc(text) {
		if (!text) return "";
		return text
			.replace(/<pre><code(?: class="lang-([^"]*)")?>([\s\S]*?)<\/code><\/pre>/g, (_, lang, code) => `\n\n\`\`\`${lang ?? "csharp"}\n${decodeEntities(code).replace(/\n$/, "")}\n\`\`\`\n\n`)
			.replace(/<xref href="([^"]+)"[^>]*>([\s\S]*?)<\/xref>/g, (_, uid, inner) => this.cref(decodeURIComponent(uid), inner.trim()))
			.replace(/<code(?: class="[^"]*")?>([\s\S]*?)<\/code>/g, (_, code) => codeSpan(code))
			.replace(/<a href="([^"]+)"[^>]*>([\s\S]*?)<\/a>/g, (_, href, inner) => (LANGWORDS.has(inner) ? codeSpan(inner) : link(inner, href)))
			.replace(/<\/?(?:b|strong)>/g, "**")
			.replace(/<\/?(?:i|em)>/g, "*")
			.replace(/<ul>|<ol>/g, "\n\n")
			.replace(/<\/ul>|<\/ol>/g, "\n\n")
			.replace(/<li>/g, "\n- ")
			.replace(/<\/li>/g, "")
			.replace(/<\/?p>/g, "\n\n")
			.replace(/<br\s*\/?>/g, "\n\n")
			.replace(/\n{3,}/g, "\n\n")
			.trim();
	}
}

// ---------------------------------------------------------------- type pages

function sourceLink(item) {
	const source = item.source;
	if (!source?.path) return undefined;
	const relative = path.relative(REPO_ROOT, source.path).split(path.sep).join("/");
	if (relative.startsWith("..")) return undefined;
	return link(escapeText(path.posix.basename(relative)), `${SOURCE_URL}${relative}#L${(source.startLine ?? 0) + 1}`);
}

function frontmatter(fields) {
	const lines = Object.entries(fields)
		.filter(([, value]) => value !== undefined && value !== "")
		.map(([key, value]) => `${key}: ${JSON.stringify(value)}`);
	return ["---", ...lines, "---", "", ""].join("\n");
}

function codeBlock(content) {
	return ["```csharp", content, "```", ""];
}

function parametersTable(L, title, params, typeKey) {
	if (!params?.length) return [];
	const rows = params.map((p) => `| ${codeSpan(p.id)} | ${cell(typeKey ? L.type(p.type) : "")} | ${cell(L.doc(p.description))} |`);
	const head = typeKey ? ["| Name | Type | Description |", "|---|---|---|"] : ["| Name | | Description |", "|---|---|---|"];
	return [`**${title}**`, "", ...head, ...rows, ""];
}

function typeParametersTable(L, params) {
	if (!params?.length) return [];
	return ["**Type parameters**", "", "| Name | Description |", "|---|---|", ...params.map((p) => `| ${codeSpan(p.id)} | ${cell(L.doc(p.description))} |`), ""];
}

function remarksBlock(L, item, heading) {
	const out = [];
	if (item.remarks) out.push(heading("Remarks"), "", L.doc(item.remarks), "");
	for (const example of item.example ?? []) out.push(heading("Example"), "", L.doc(example), "");
	if (item.seealso?.length) {
		out.push(heading("See also"), "");
		item.seealso.forEach((s) => out.push(`- ${s.linkType === "HRef" ? link(escapeText(s.altText || s.linkId), s.linkId) : L.cref(s.linkId, s.altText)}`));
		out.push("");
	}
	return out;
}

function memberHeading(member, overloaded) {
	const name = overloaded || member.type === "Constructor" ? member.name : member.name.replace(/\(.*$/, "");
	return escapeText(name);
}

function renderMember(L, member, anchor, overloaded) {
	const syntax = member.syntax ?? {};
	const out = [`### ${memberHeading(member, overloaded)} {#${anchor}}`, ""];
	const summary = L.doc(member.summary);
	if (summary) out.push(summary, "");
	if (syntax.content) out.push(...codeBlock(syntax.content));
	out.push(...typeParametersTable(L, syntax.typeParameters));
	out.push(...parametersTable(L, "Parameters", syntax.parameters, true));
	const valueKind = { Property: "Value", Field: "Value", Event: "Event type" }[member.type];
	if (syntax.return && member.type !== "Constructor") {
		const description = L.doc(syntax.return.description);
		const label = valueKind ?? "Returns";
		out.push(`**${label}:** ${L.type(syntax.return.type)}${description ? ` — ${description}` : ""}`, "");
	}
	if (member.exceptions?.length) {
		out.push("**Exceptions**", "", "| Exception | Condition |", "|---|---|");
		member.exceptions.forEach((e) => out.push(`| ${cell(L.type(e.type))} | ${cell(L.doc(e.description))} |`));
		out.push("");
	}
	out.push(...remarksBlock(L, member, (title) => `**${title}**`));
	return out;
}

function overviewTable(L, members, model) {
	const rows = members.map((m) => {
		const summary = cell(firstParagraph(L.doc(m.summary)));
		return `| [${escapeText(m.name)}](#${model.anchorOf.get(m.uid)}) | ${summary} |`;
	});
	return ["| Member | Description |", "|---|---|", ...rows, ""];
}

function renderEnumValues(L, members, model) {
	const rows = members.map((m) => {
		const [name, value] = (m.syntax?.content ?? m.name).split(/\s*=\s*/);
		return `| <a id="${model.anchorOf.get(m.uid)}"></a>${codeSpan(name)} | ${codeSpan(value ?? "")} | ${cell(L.doc(m.summary))} |`;
	});
	return ["## Values", "", "| Name | Value | Description |", "|---|---|---|", ...rows, ""];
}

function relations(L, item) {
	const out = [];
	const inheritance = (item.inheritance ?? []).filter((uid) => !["System.Object", "System.ValueType", "System.Enum"].includes(uid));
	if (inheritance.length) out.push(`**Inherits:** ${[...inheritance.map((uid) => L.type(uid)), escapeText(item.name)].join(" → ")}`);
	if (item.derivedClasses?.length) out.push(`**Derived:** ${item.derivedClasses.map((uid) => L.type(uid)).join(", ")}`);
	if (item.implements?.length) out.push(`**Implements:** ${item.implements.map((uid) => L.type(uid)).join(", ")}`);
	return out.length ? [out.join("  \n"), ""] : [];
}

function renderType(model, type) {
	const { item, page, members } = type;
	const L = new Linker(model, page);
	const summary = L.doc(item.summary);
	const title = `${item.name} ${kindLabel(item)}`;
	const out = [
		frontmatter({ title, sidebar_label: item.name, description: plainText(firstParagraph(summary)).slice(0, 200) }),
		`# ${escapeText(item.name)} ${kindLabel(item)}`,
		"",
	];
	if (summary) out.push(summary, "");
	const meta = [`**Namespace:** ${link("Qar", "index.md")}`, `**Assembly:** ${ASSEMBLY}`];
	const source = sourceLink(item);
	if (source) meta.push(`**Source:** ${source}`);
	out.push(meta.join(" · "), "");
	out.push(...codeBlock(item.syntax?.content ?? ""));
	out.push(...typeParametersTable(L, item.syntax?.typeParameters));
	out.push(...relations(L, item));
	out.push(...remarksBlock(L, item, (title) => `## ${title === "Example" ? "Examples" : title}`));

	if (item.type === "Enum") {
		out.push(...renderEnumValues(L, members, model));
	} else {
		for (const [kind, section] of MEMBER_SECTIONS) {
			const inSection = members.filter((m) => m.type === kind);
			if (!inSection.length) continue;
			out.push(`## ${section}`, "");
			if (inSection.length >= OVERVIEW_TABLE_MIN) out.push(...overviewTable(L, inSection, model));
			const counts = new Map();
			inSection.forEach((m) => counts.set(baseId(m), (counts.get(baseId(m)) ?? 0) + 1));
			for (const member of inSection) {
				out.push(...renderMember(L, member, model.anchorOf.get(member.uid), counts.get(baseId(member)) > 1));
			}
		}
	}
	return out.join("\n").replace(/\n{3,}/g, "\n\n");
}

// ---------------------------------------------------------------- landing page and sidebar

function groupByArea(types) {
	const { areas } = JSON.parse(fs.readFileSync(AREAS_JSON, "utf8"));
	const groups = areas.map((area) => ({ ...area, types: [] }));
	const byName = new Map();
	areas.forEach((area, i) => (area.types ?? []).forEach((name) => byName.set(name, i)));
	const other = { label: "Other", description: "Types no area in areas.json claims yet.", types: [] };
	for (const type of types) {
		const name = type.item.name;
		let index = byName.get(name);
		if (index === undefined) {
			let best = 0;
			areas.forEach((area, i) =>
				(area.prefixes ?? []).forEach((prefix) => {
					if (name.startsWith(prefix) && prefix.length > best) {
						best = prefix.length;
						index = i;
					}
				})
			);
		}
		(index === undefined ? other : groups[index]).types.push(type);
	}
	if (other.types.length) {
		console.warn(`C# API: no area in ${path.relative(WEBSITE_DIR, AREAS_JSON)} claims ${other.types.map((t) => t.item.name).join(", ")}; listed under "Other".`);
		groups.push(other);
	}
	return groups.filter((group) => group.types.length);
}

function renderLanding(model, groups) {
	const L = new Linker(model, "index");
	const overview = fs.readFileSync(OVERVIEW_MD, "utf8").replace(/<!--[\s\S]*?-->/g, "").trim();
	const out = [
		frontmatter({
			title: "QAR Streaming C# API",
			sidebar_label: "Overview",
			slug: `/${API_ID}/`,
			description: "Reference of the QAROS .NET binding: the public types of the Qar namespace, grouped by area.",
		}),
		"# QAR Streaming C# API",
		"",
		overview,
		"",
	];
	for (const group of groups) {
		out.push(`## ${group.label}`, "");
		if (group.description) out.push(group.description, "");
		out.push("| Type | Kind | Description |", "|---|---|---|");
		for (const { item, page } of group.types) {
			out.push(`| ${link(codeSpan(item.name), `${page}.md`)} | ${kindLabel(item)} | ${cell(firstParagraph(L.doc(item.summary)))} |`);
		}
		out.push("");
	}
	return out.join("\n");
}

function buildSidebar(groups) {
	const sidebar = {
		type: "category",
		label: "QAR Streaming C# API",
		link: { type: "doc", id: `${API_ID}/index` },
		collapsed: false,
		items: groups.map((group) => ({
			type: "category",
			label: group.label,
			collapsed: true,
			items: group.types.map(({ item, page }) => ({ type: "doc", id: `${API_ID}/${page}`, label: item.name })),
		})),
	};
	fs.writeFileSync(SIDEBAR_JSON, JSON.stringify(sidebar, null, 2) + "\n");
}

// ---------------------------------------------------------------- main

generateYaml();
const model = loadModel();
fs.rmSync(OUT_DIR, { recursive: true, force: true });
fs.mkdirSync(OUT_DIR, { recursive: true });
for (const type of model.types) {
	fs.writeFileSync(path.join(OUT_DIR, `${type.page}.md`), renderType(model, type));
}
const groups = groupByArea(model.types);
fs.writeFileSync(path.join(OUT_DIR, "index.md"), renderLanding(model, groups));
buildSidebar(groups);
console.log(`C# API: ${model.types.length} type pages written to ${path.relative(WEBSITE_DIR, OUT_DIR)}`);
