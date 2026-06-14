#!/usr/bin/env python3
"""Build the Cylinder Seal ebook PDF from the current documentation set."""

from __future__ import annotations

import html
import re
from dataclasses import dataclass
from datetime import date
from pathlib import Path

import markdown
import cairosvg
from weasyprint import HTML


ROOT = Path(__file__).resolve().parents[1]
EBOOK_OUT_DIR = ROOT
ASSET_DIR = ROOT / "docs" / "ebook" / "assets"
PDF_PATH = EBOOK_OUT_DIR / "cylinder-seal-ebook.pdf"
HTML_PATH = EBOOK_OUT_DIR / "cylinder-seal-ebook.html"
MD_PATH = EBOOK_OUT_DIR / "cylinder-seal-ebook.md"
RASTER_MAP: dict[Path, Path] = {}


@dataclass(frozen=True)
class BookPart:
    title: str
    path: Path
    note: str = ""


BOOK_TITLE = "Cylinder Seal"
BOOK_SUBTITLE = "Digital IQD, Industrial Dividends, Civic Work, and Iraq's Unified Economic Model"
BOOK_STATUS = "Prototype and policy-design ebook. Not production CBDC infrastructure."


BOOK_PARTS = [
    BookPart("Project Overview", ROOT / "README.md"),
    BookPart("Executive Summary", ROOT / "EXECUTIVE_SUMMARY.md"),
    BookPart("Final Summary", ROOT / "FINAL_SUMMARY.md"),
    BookPart("Economic Assumptions And Source Discipline", ROOT / "docs" / "economic-assumptions.md"),
    BookPart("National Economic Operating Logic", ROOT / "docs" / "national-economic-operating-logic.md"),
    BookPart("National Legal And Institutional Roadmap", ROOT / "docs" / "national-legal-institutional-roadmap.md"),
    BookPart("Project Pipeline And Investment Gates", ROOT / "docs" / "project-pipeline-and-investment-gates.md"),
    BookPart("Political-Economy Transition And Anti-Capture", ROOT / "docs" / "political-economy-transition-and-anti-capture.md"),
    BookPart("Citizen Entitlement Privacy And Appeals", ROOT / "docs" / "citizen-entitlement-privacy-and-appeals.md"),
    BookPart("Cash Formalization And Demonetization Window", ROOT / "docs" / "cash-formalization-and-demonetization-window.md"),
    BookPart("Federalism Governorate Equity And Local Compacts", ROOT / "docs" / "federalism-governorate-equity-and-local-compacts.md"),
    BookPart("Environmental Social Water And Cultural Safeguards", ROOT / "docs" / "environmental-social-cultural-safeguards.md"),
    BookPart("Macro Monetary Inflation And FX Stability", ROOT / "docs" / "macro-monetary-fx-stability.md"),
    BookPart("Fiscal Stress And Contingent Liability", ROOT / "docs" / "fiscal-stress-and-contingent-liability-model.md"),
    BookPart("National Program Sequencing And Dependency Control", ROOT / "docs" / "national-program-sequencing-and-dependency-control.md"),
    BookPart("Procurement Integrity And Market Discipline", ROOT / "docs" / "procurement-integrity-and-market-discipline.md"),
    BookPart("Benefit Realization And Claim Audit", ROOT / "docs" / "benefit-realization-and-claim-audit.md"),
    BookPart("Iraq Integrated Growth Impact Model", ROOT / "docs" / "iraq-integrated-growth-impact-model.md"),
    BookPart("Iraq Comprehensive Benefits Model", ROOT / "docs" / "iraq-comprehensive-benefits-model.md"),
    BookPart("Iraq Quantified Affordability And Cashflow Model", ROOT / "docs" / "iraq-quantified-affordability-model.md"),
    BookPart("System And Financial Flow Diagrams", ROOT / "docs" / "system-and-financial-flow-diagrams.md"),
    BookPart("Business Value Chain Charts", ROOT / "docs" / "business-value-chain-charts.md"),
    BookPart("Unified Economic Model", ROOT / "docs" / "unified-economic-model.md"),
    BookPart("National Dividend Holding Company", ROOT / "docs" / "national-dividend-holding-company.md"),
    BookPart("INDHC Ten-Year Plan", ROOT / "docs" / "indhc-10-year-plan.md"),
    BookPart("Import, Services, And Diaspora Expansion", ROOT / "docs" / "import-services-diaspora-expansion.md"),
    BookPart("Facility Recycling And Capital Markets", ROOT / "docs" / "facility-recycling-and-capital-markets.md"),
    BookPart("Digitally Governed Industrial Champions", ROOT / "docs" / "digitally-governed-industrial-champions.md"),
    BookPart("National Civic Work System", ROOT / "docs" / "national-civic-work-system.md"),
    BookPart("Ministry Transition Roadmap", ROOT / "docs" / "ministry-transition-roadmap.md"),
    BookPart("Security Model", ROOT / "SECURITY.md"),
    BookPart("Current Implementation Status", ROOT / "IMPLEMENTATION_STATUS.md"),
    BookPart("Technical Primitives", ROOT / "docs" / "technical-primitives.md"),
]


DIAGRAMS = [
    ("Software System Architecture", ROOT / "docs" / "diagrams" / "software-system-architecture.svg"),
    ("Unified Economic Model", ROOT / "docs" / "diagrams" / "unified-economic-model.svg"),
    ("Transaction Lifecycle", ROOT / "docs" / "diagrams" / "transaction-lifecycle.svg"),
    ("Financial Flow Combinations", ROOT / "docs" / "diagrams" / "financial-flow-combinations.svg"),
    ("Transaction Combination Matrix", ROOT / "docs" / "diagrams" / "transaction-combination-matrix.svg"),
    ("National Dividend Holding Company", ROOT / "docs" / "diagrams" / "national-dividend-holding-company.svg"),
    ("National Civic Work System", ROOT / "docs" / "diagrams" / "national-civic-work-system.svg"),
    ("Business Value Chain Overview", ROOT / "docs" / "diagrams" / "business-value-chain-overview.svg"),
    ("Sector Value Chain Matrix", ROOT / "docs" / "diagrams" / "sector-value-chain-matrix.svg"),
    ("Capital And Repayment Lanes", ROOT / "docs" / "diagrams" / "capital-and-repayment-lanes.svg"),
    ("Society And Economy Feedback Loop", ROOT / "docs" / "diagrams" / "society-economy-feedback-loop.svg"),
]


LEGACY_POLICY_NOTE = """
# Legacy Policy Paper Boundary

The long-form `docs/policy-paper.md` is intentionally not included as a main
ebook chapter. It is a legacy scenario workbook and narrative archive, not the
implementation status of the repository and not an externally validated
forecast.

Do not quote its dollar ranges, adoption rates, sovereign-rating paths,
timelines, employment figures, or GDP paths as validated project claims. Use the
README, implementation status, economic assumptions, unified economic model,
security model, and newer institutional-design documents as the front-door
position.
"""


def slug(text: str) -> str:
    value = re.sub(r"[^a-z0-9]+", "-", text.lower()).strip("-")
    return value or "section"


def rel_url(path: Path) -> str:
    return path.resolve().relative_to(ROOT).as_posix()


def rasterize_diagrams() -> None:
    ASSET_DIR.mkdir(parents=True, exist_ok=True)
    svg_paths = {path.resolve() for _, path in DIAGRAMS}
    for part in BOOK_PARTS:
        text = part.path.read_text(encoding="utf-8")
        for match in re.finditer(r"!\[[^\]]*\]\(([^)]+)\)", text):
            target = match.group(1).strip().split()[0]
            if re.match(r"^[a-z]+://", target) or target.startswith("#"):
                continue
            candidate = (part.path.parent / target).resolve()
            if candidate.suffix.lower() == ".svg" and candidate.exists():
                svg_paths.add(candidate)

    for svg_path in sorted(svg_paths):
        png_path = ASSET_DIR / f"{svg_path.stem}.png"
        cairosvg.svg2png(url=str(svg_path), write_to=str(png_path), output_width=2400)
        RASTER_MAP[svg_path] = png_path


def rewrite_image_paths(markdown_text: str, source_path: Path) -> str:
    source_dir = source_path.parent

    def replace(match: re.Match[str]) -> str:
        alt = match.group("alt")
        target = match.group("target").strip()
        if re.match(r"^[a-z]+://", target) or target.startswith("#"):
            return match.group(0)
        target_only = target.split()[0]
        resolved = (source_dir / target_only).resolve()
        if not str(resolved).startswith(str(ROOT.resolve())):
            return match.group(0)
        rewritten_path = RASTER_MAP.get(resolved, resolved)
        rewritten = rel_url(rewritten_path)
        return f"![{alt}]({rewritten})"

    return re.sub(r"!\[(?P<alt>[^\]]*)\]\((?P<target>[^)]+)\)", replace, markdown_text)


def strip_mermaid(markdown_text: str) -> str:
    notice = (
        "\n\n> Mermaid source omitted from the ebook body. "
        "Use the rendered SVG diagram atlas or the repository Markdown for source.\n\n"
    )
    return re.sub(r"```mermaid\n.*?```", notice, markdown_text, flags=re.DOTALL)


def demote_headings(markdown_text: str) -> str:
    return re.sub(r"^(#{1,5})(\s+)", r"#\1\2", markdown_text, flags=re.MULTILINE)


def preprocess_part(part: BookPart) -> str:
    text = part.path.read_text(encoding="utf-8")
    text = rewrite_image_paths(text, part.path)
    text = strip_mermaid(text)
    text = demote_headings(text)
    if part.note:
        text = f"> {part.note}\n\n{text}"
    return text


def build_markdown() -> str:
    today = date.today().isoformat()
    lines = [
        f"# {BOOK_TITLE}",
        "",
        BOOK_SUBTITLE,
        "",
        f"**Status:** {BOOK_STATUS}",
        "",
        f"**Generated:** {today}",
        "",
        "This ebook is generated from the repository documentation. It preserves the",
        "prototype boundary: Cylinder Seal is suitable for technical review, policy",
        "exploration, and demo workflows, but it is not production-ready payment",
        "infrastructure and is not an official Central Bank of Iraq project.",
        "",
        "# Contents",
        "",
    ]
    for idx, part in enumerate(BOOK_PARTS, 1):
        lines.append(f"{idx}. {part.title}")
    lines.append(f"{len(BOOK_PARTS) + 1}. Legacy Policy Paper Boundary")
    lines.append("")
    lines.append("# Diagram Atlas")
    lines.append("")
    for title, path in DIAGRAMS:
        lines.append(f"## {title}")
        lines.append("")
        image_path = RASTER_MAP.get(path.resolve(), path)
        lines.append(f"![{title}]({rel_url(image_path)})")
        lines.append("")
    for idx, part in enumerate(BOOK_PARTS, 1):
        lines.append(f"# Part {idx}: {part.title}")
        lines.append("")
        lines.append(preprocess_part(part))
        lines.append("")
    lines.append(LEGACY_POLICY_NOTE.strip())
    lines.append("")
    return "\n".join(lines)


def css() -> str:
    return """
@page {
  size: A4;
  margin: 17mm 15mm 19mm;
  @bottom-center {
    content: "Cylinder Seal - " counter(page);
    color: #667085;
    font-size: 9pt;
  }
}
@page:first {
  margin: 0;
  @bottom-center { content: ""; }
}
html {
  color: #101828;
  font-family: "DejaVu Sans", "Noto Sans", Arial, sans-serif;
  font-size: 10.4pt;
  line-height: 1.48;
}
body { margin: 0; }
.cover {
  align-items: flex-start;
  background: #0f172a;
  color: #f8fafc;
  display: flex;
  flex-direction: column;
  height: 297mm;
  justify-content: center;
  padding: 0 26mm;
}
.cover .kicker {
  color: #93c5fd;
  font-size: 12pt;
  letter-spacing: .08em;
  margin-bottom: 20mm;
  text-transform: uppercase;
}
.cover h1 {
  color: #fff;
  font-size: 46pt;
  line-height: 1.02;
  margin: 0 0 8mm;
}
.cover .subtitle {
  color: #dbeafe;
  font-size: 18pt;
  line-height: 1.35;
  max-width: 155mm;
}
.cover .status {
  border-top: 1px solid rgba(255,255,255,.28);
  color: #cbd5e1;
  font-size: 11pt;
  margin-top: 25mm;
  padding-top: 8mm;
}
main { padding: 0; }
h1, h2, h3, h4 {
  color: #111827;
  line-height: 1.22;
  page-break-after: avoid;
}
h1 {
  border-bottom: 1.4pt solid #d0d5dd;
  font-size: 25pt;
  margin: 0 0 9mm;
  padding-bottom: 4mm;
  page-break-before: always;
}
h2 {
  color: #12355b;
  font-size: 17pt;
  margin: 9mm 0 4mm;
}
h3 {
  color: #344054;
  font-size: 13pt;
  margin: 7mm 0 3mm;
}
h4 {
  color: #475467;
  font-size: 11.5pt;
  margin: 5mm 0 2mm;
}
p { margin: 0 0 3.3mm; }
a { color: #175cd3; text-decoration: none; }
blockquote {
  border-left: 4px solid #98a2b3;
  color: #344054;
  margin: 4mm 0;
  padding: 2mm 0 2mm 5mm;
}
code {
  background: #f2f4f7;
  border-radius: 3px;
  color: #111827;
  font-family: "DejaVu Sans Mono", "Noto Sans Mono", monospace;
  font-size: 8.7pt;
  padding: 0.2mm 1mm;
}
pre {
  background: #0f172a;
  border-radius: 5px;
  color: #f8fafc;
  font-family: "DejaVu Sans Mono", "Noto Sans Mono", monospace;
  font-size: 7.4pt;
  line-height: 1.35;
  margin: 4mm 0;
  overflow-wrap: break-word;
  padding: 4mm;
  white-space: pre-wrap;
}
pre code {
  background: transparent;
  color: inherit;
  padding: 0;
}
table {
  border-collapse: collapse;
  font-size: 8.2pt;
  margin: 4mm 0 6mm;
  page-break-inside: avoid;
  width: 100%;
}
th {
  background: #e0f2fe;
  color: #0f172a;
  font-weight: 700;
}
td, th {
  border: 0.7pt solid #cbd5e1;
  padding: 1.9mm 2.1mm;
  vertical-align: top;
}
tr:nth-child(even) td { background: #f8fafc; }
ul, ol { margin: 1mm 0 4mm 7mm; padding: 0; }
li { margin-bottom: 1.3mm; }
img {
  display: block;
  margin: 5mm auto 7mm;
  max-height: 230mm;
  max-width: 100%;
  object-fit: contain;
}
.diagram-atlas img {
  max-height: 176mm;
}
.source-note {
  background: #fff7e6;
  border: 1px solid #fedf89;
  border-radius: 5px;
  color: #7a4b00;
  margin: 5mm 0;
  padding: 4mm;
}
.contents ol {
  margin-top: 5mm;
}
.contents li { break-inside: avoid; }
"""


def build_html(markdown_text: str) -> str:
    body_html = markdown.markdown(
        markdown_text,
        extensions=["extra", "tables", "fenced_code", "sane_lists", "toc"],
        output_format="html5",
    )
    # Add classes to the generated contents and diagram atlas sections.
    body_html = body_html.replace("<h1 id=\"contents\">Contents</h1>", "<h1 id=\"contents\">Contents</h1><div class=\"contents\">", 1)
    body_html = body_html.replace("<h1 id=\"diagram-atlas\">Diagram Atlas</h1>", "</div><section class=\"diagram-atlas\"><h1 id=\"diagram-atlas\">Diagram Atlas</h1>", 1)
    body_html = body_html.replace("<h1 id=\"part-1-project-overview\">", "</section><h1 id=\"part-1-project-overview\">", 1)
    generated = date.today().isoformat()
    return f"""<!doctype html>
<html lang="en">
<head>
  <meta charset="utf-8">
  <title>{html.escape(BOOK_TITLE)} Ebook</title>
  <style>{css()}</style>
</head>
<body>
  <section class="cover">
    <div class="kicker">Cylinder Seal Ebook</div>
    <h1>{html.escape(BOOK_TITLE)}</h1>
    <div class="subtitle">{html.escape(BOOK_SUBTITLE)}</div>
    <div class="status">{html.escape(BOOK_STATUS)}<br>Generated {generated}</div>
  </section>
  <main>
    {body_html}
  </main>
</body>
</html>
"""


def main() -> None:
    EBOOK_OUT_DIR.mkdir(parents=True, exist_ok=True)
    rasterize_diagrams()
    markdown_text = build_markdown()
    html_text = build_html(markdown_text)
    MD_PATH.write_text(markdown_text, encoding="utf-8")
    HTML_PATH.write_text(html_text, encoding="utf-8")
    HTML(filename=str(HTML_PATH), base_url=str(ROOT)).write_pdf(str(PDF_PATH))
    print(PDF_PATH)


if __name__ == "__main__":
    main()
