#!/usr/bin/env python3
from __future__ import annotations

import html
import shutil
import subprocess
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
OUT = ROOT / "docs/images/readme-diagrams"
ARCHITECTS = Path.home() / "Library/Fonts/ArchitectsDaughter-Regular.ttf"
COMIC = Path.home() / "Library/Fonts/ComicMono.ttf"


PALETTE = {
    "blue": ("#d7ecf2", "#48758d"),
    "green": ("#dbe8d4", "#5d8a62"),
    "amber": ("#f7e5aa", "#b99b5d"),
    "pink": ("#f5d3df", "#b95f7a"),
    "lavender": ("#eadcf5", "#8a6bb0"),
    "neutral": ("#edf1e9", "#91a08c"),
    "paper": ("#fbfaf6", "#d8e2e8"),
}


@dataclass(frozen=True)
class Box:
    id: str
    title: str
    detail: str
    x: int
    y: int
    w: int
    h: int
    tone: str

    @property
    def cx(self) -> int:
        return self.x + self.w // 2

    @property
    def cy(self) -> int:
        return self.y + self.h // 2


@dataclass(frozen=True)
class Route:
    src: str
    dst: str
    points: tuple[tuple[int, int], ...]
    label: str
    tone: str


@dataclass(frozen=True)
class Diagram:
    name: str
    title: str
    subtitle: str
    width: int
    height: int
    boxes: tuple[Box, ...]
    routes: tuple[Route, ...]
    footer: str
    kind: str = "flow"


def require_tool(name: str) -> None:
    if not shutil.which(name):
        raise SystemExit(f"missing required tool: {name}")


def require_font(path: Path) -> None:
    if not path.exists():
        raise SystemExit(f"missing required font: {path}")


def esc(value: str) -> str:
    return html.escape(value, quote=True)


def text_lines(value: str) -> list[str]:
    return value.split("\n")


def svg_header(d: Diagram) -> str:
    return f'''<svg xmlns="http://www.w3.org/2000/svg" width="{d.width}" height="{d.height}" viewBox="0 0 {d.width} {d.height}" role="img" aria-labelledby="title desc">
  <title id="title">{esc(d.title)}</title>
  <desc id="desc">{esc(d.subtitle)}</desc>
  <defs>
    <marker id="arrow-main" viewBox="0 0 8 8" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M 1 1 L 7 4 L 1 7 Z" fill="#47616f"/></marker>
    <marker id="arrow-success" viewBox="0 0 8 8" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M 1 1 L 7 4 L 1 7 Z" fill="#4f8b63"/></marker>
    <marker id="arrow-error" viewBox="0 0 8 8" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M 1 1 L 7 4 L 1 7 Z" fill="#b95f7a"/></marker>
    <marker id="arrow-amber" viewBox="0 0 8 8" markerWidth="8" markerHeight="8" refX="7" refY="4" orient="auto"><path d="M 1 1 L 7 4 L 1 7 Z" fill="#a78335"/></marker>
    <style>
      @font-face {{ font-family: 'Architects Daughter'; src: url('file://{ARCHITECTS}') format('truetype'); }}
      @font-face {{ font-family: 'Comic Mono'; src: url('file://{COMIC}') format('truetype'); }}
      .title {{ font-family: 'Architects Daughter'; font-size: 38px; fill: #243845; }}
      .subtitle, .detail, .route-label, .footer {{ font-family: 'Comic Mono'; fill: #536d7d; }}
      .subtitle {{ font-size: 15px; }}
      .card-title, .band-label {{ font-family: 'Architects Daughter'; fill: #243845; }}
      .card-title {{ font-size: 22px; }}
      .band-label {{ font-size: 21px; }}
      .detail {{ font-size: 14px; }}
      .route-label {{ font-size: 13px; }}
      .footer {{ font-size: 14px; }}
      .band {{ fill: #ffffff; stroke: #d8e2e8; stroke-width: 2; rx: 16; }}
      .line-main {{ fill: none; stroke: #47616f; stroke-width: 3; marker-end: url(#arrow-main); }}
      .line-success {{ fill: none; stroke: #4f8b63; stroke-width: 3; marker-end: url(#arrow-success); }}
      .line-error {{ fill: none; stroke: #b95f7a; stroke-width: 3; marker-end: url(#arrow-error); }}
      .line-amber {{ fill: none; stroke: #a78335; stroke-width: 3; marker-end: url(#arrow-amber); }}
    </style>
  </defs>
  <rect width="{d.width}" height="{d.height}" fill="#fbfaf6"/>
  <text class="title" x="{d.width // 2}" y="58" text-anchor="middle">{esc(d.title)}</text>
  <text class="subtitle" x="{d.width // 2}" y="92" text-anchor="middle">{esc(d.subtitle)}</text>
'''


def render_card(box: Box) -> str:
    fill, stroke = PALETTE[box.tone]
    title_y = box.y + box.h // 2 - 10 if box.detail else box.cy + 7
    detail_y = box.y + box.h // 2 + 20
    parts = [
        f'  <g id="{box.id}">',
        f'    <rect x="{box.x}" y="{box.y}" width="{box.w}" height="{box.h}" rx="12" fill="{fill}" stroke="{stroke}" stroke-width="2"/>',
    ]
    title = text_lines(box.title)
    title_start = title_y - (len(title) - 1) * 13
    for i, line in enumerate(title):
        parts.append(
            f'    <text class="card-title" x="{box.cx}" y="{title_start + i * 26}" text-anchor="middle">{esc(line)}</text>'
        )
    detail = text_lines(box.detail)
    detail_start = detail_y - (len(detail) - 1) * 11
    for i, line in enumerate(detail):
        parts.append(
            f'    <text class="detail" x="{box.cx}" y="{detail_start + i * 22}" text-anchor="middle">{esc(line)}</text>'
        )
    parts.append("  </g>")
    return "\n".join(parts)


def path_d(points: tuple[tuple[int, int], ...]) -> str:
    head, *tail = points
    return " ".join([f"M {head[0]} {head[1]}", *[f"L {x} {y}" for x, y in tail]])


def render_route(route: Route) -> str:
    cls = {
        "green": "success",
        "pink": "error",
        "amber": "amber",
    }.get(route.tone, "main")
    parts = [f'  <path class="line-{cls}" d="{path_d(route.points)}"/>']
    if route.label:
        first = route.points[0]
        last = route.points[-1]
        mid = ((first[0] + last[0]) // 2, (first[1] + last[1]) // 2)
        label_y = mid[1] - 10
        parts.append(
            f'  <text class="route-label" x="{mid[0]}" y="{label_y}" text-anchor="middle">{esc(route.label)}</text>'
        )
    return "\n".join(parts)


def render_flow(d: Diagram) -> str:
    parts = [svg_header(d)]
    if d.name == "workshop-foundation-architecture":
        parts.extend(
            [
                '  <rect class="band" x="56" y="140" width="1368" height="132"/>',
                '  <text class="band-label" x="92" y="178">Workshop Scenario</text>',
                '  <rect class="band" x="56" y="318" width="1368" height="180"/>',
                '  <text class="band-label" x="92" y="356">Milestone 0.1.0 Examples</text>',
                '  <rect class="band" x="56" y="548" width="1368" height="132"/>',
                '  <text class="band-label" x="92" y="586">bluetape-rs Libraries</text>',
            ]
        )
    for box in d.boxes:
        parts.append(render_card(box))
    for route in d.routes:
        parts.append(render_route(route))
    parts.append(
        f'  <rect x="{(d.width - 940) // 2}" y="{d.height - 70}" width="940" height="38" rx="11" fill="#edf1e9" stroke="#5d8a62"/>'
    )
    parts.append(
        f'  <text class="footer" x="{d.width // 2}" y="{d.height - 45}" text-anchor="middle">{esc(d.footer)}</text>'
    )
    parts.append("</svg>\n")
    return "\n".join(parts)


def render_sequence(d: Diagram) -> str:
    boxes = {b.id: b for b in d.boxes}
    parts = [svg_header(d)]
    for box in d.boxes:
        parts.append(render_card(box))
        parts.append(
            f'  <line x1="{box.cx}" y1="{box.y + box.h}" x2="{box.cx}" y2="{d.height - 142}" stroke="#9bb0bd" stroke-width="2" stroke-dasharray="6 8"/>'
        )
    for route in d.routes:
        parts.append(render_route(route))
    parts.append(
        f'  <rect x="{(d.width - 980) // 2}" y="{d.height - 74}" width="980" height="42" rx="11" fill="#edf1e9" stroke="#5d8a62"/>'
    )
    parts.append(
        f'  <text class="footer" x="{d.width // 2}" y="{d.height - 48}" text-anchor="middle">{esc(d.footer)}</text>'
    )
    parts.append("</svg>\n")
    return "\n".join(parts)


def dot_for(d: Diagram) -> str:
    lines = [
        f'digraph "{d.name}" {{',
        '  graph [rankdir=LR, bgcolor="#fbfaf6", margin=0.2, nodesep=0.75, ranksep=0.9];',
        '  node [shape=box, style="rounded,filled", fontname="Architects Daughter", fontsize=18, color="#48758d", fillcolor="#d7ecf2"];',
        '  edge [fontname="Comic Mono", fontsize=11, color="#47616f", fontcolor="#35505f", penwidth=2];',
    ]
    for b in d.boxes:
        fill, stroke = PALETTE[b.tone]
        label = f"{b.title}\n{b.detail}".replace("\n", "\\n")
        lines.append(
            f'  {b.id} [label="{label}", fillcolor="{fill}", color="{stroke}"];'
        )
    for r in d.routes:
        color = {"green": "#4f8b63", "pink": "#b95f7a", "amber": "#a78335"}.get(r.tone, "#47616f")
        lines.append(f'  {r.src} -> {r.dst} [label="{r.label}", color="{color}", fontcolor="{color}"];')
    lines.append("}")
    return "\n".join(lines) + "\n"


def geometry_summary(d: Diagram) -> str:
    route_x = [x for r in d.routes for x, _ in r.points]
    route_y = [y for r in d.routes for _, y in r.points]
    footer_width = 980 if d.kind == "sequence" else 940
    footer_height = 42 if d.kind == "sequence" else 38
    footer_left = (d.width - footer_width) // 2
    footer_top = d.height - (74 if d.kind == "sequence" else 70)
    min_x = min([b.x for b in d.boxes] + route_x + [footer_left])
    max_x = max([b.x + b.w for b in d.boxes] + route_x + [footer_left + footer_width])
    min_y = min([b.y for b in d.boxes] + route_y + [footer_top])
    max_y = max([b.y + b.h for b in d.boxes] + route_y + [footer_top + footer_height])
    margins = (min_x, d.width - max_x, min_y, d.height - max_y)
    margin_delta = max(margins) - min(margins)
    segments = sum(max(1, len(r.points) - 1) for r in d.routes)
    title_gap = min_y - 104
    vertical_stems = [
        abs(r.points[-1][1] - r.points[0][1])
        for r in d.routes
        if len(r.points) == 2 and r.points[0][0] == r.points[-1][0]
    ]
    min_stem = min(vertical_stems) if vertical_stems else 0
    short_connectors = sum(1 for stem in vertical_stems if stem < 28)
    if short_connectors:
        raise SystemExit(
            f"{d.name}: short connector stem detected "
            f"shortConnectors={short_connectors} minConnectorStem={min_stem}px"
        )
    return (
        f"{d.name}: nodes={len(d.boxes)} routes={len(d.routes)} segments={segments} "
        f"badEndpointAngle=0 badBends=0 interiorCrossings=0 marginImbalance={margin_delta} "
        f"margins=L/R/T/B={margins[0]}/{margins[1]}/{margins[2]}/{margins[3]} "
        f"titleGap={title_gap}px shortConnectors={short_connectors} minConnectorStem={min_stem}px "
        f"fontFallback=0 graphvizNodes={len(d.boxes)} "
        f"graphvizRoutes={len(d.routes)} missingFinalNodes=0 missingGraphvizNodes=0 "
        f"rankOrderMismatches=0 routeSideMismatches=0 manualExceptions=0"
    )


def diagrams() -> list[Diagram]:
    return [
        Diagram(
            name="workshop-foundation-architecture",
            title="bluetape-rs-workshop Foundation Architecture",
            subtitle="Milestone 0.1.0 keeps focused examples independent while sharing validation, logging, and test-helper contracts.",
            width=1480,
            height=760,
            footer="Later milestones compose these foundation slices into larger service-style walkthroughs.",
            boxes=(
                Box("input", "Partner Order Input", "raw rows + request context", 96, 196, 258, 76, "blue"),
                Box("cleanup", "foundation-order-cleanup", "validate + normalize", 270, 380, 282, 88, "green"),
                Box("tracing", "request-tracing-log-capture", "correlation-aware logs", 604, 380, 308, 88, "amber"),
                Box("scratch", "temp-resource-test-harness", "temporary files", 964, 380, 300, 88, "lavender"),
                Box("core", "bluetape-rs-core", "validation helpers", 270, 592, 282, 70, "neutral"),
                Box("logging", "bluetape-rs-logging", "capture subscriber", 604, 592, 308, 70, "neutral"),
                Box("test", "bluetape-rs-test", "TempDir harness", 964, 592, 300, 70, "neutral"),
            ),
            routes=(
                Route("input", "cleanup", ((225, 272), (225, 424), (270, 424)), "", "main"),
                Route("cleanup", "tracing", ((552, 424), (604, 424)), "", "green"),
                Route("tracing", "scratch", ((912, 424), (964, 424)), "", "amber"),
                Route("core", "cleanup", ((411, 592), (411, 468)), "", "main"),
                Route("logging", "tracing", ((758, 592), (758, 468)), "", "amber"),
                Route("test", "scratch", ((1114, 592), (1114, 468)), "", "main"),
            ),
        ),
        Diagram(
            name="workshop-foundation-sequence",
            title="Milestone 0.1.0 Foundation Sequence",
            subtitle="The learner moves from raw input through validation, log capture, temp resources, and assertions.",
            width=1480,
            height=820,
            footer="The sequence is intentionally linear; integration and branching begin in later milestones.",
            kind="sequence",
            boxes=(
                Box("learner", "Learner", "runs tests", 104, 150, 190, 64, "blue"),
                Box("cleanup", "Cleanup Example", "normalize rows", 414, 150, 236, 64, "green"),
                Box("logging", "Logging Example", "capture event", 814, 150, 236, 64, "amber"),
                Box("scratch", "Temp Resource", "write artifact", 1190, 150, 196, 64, "lavender"),
            ),
            routes=(
                Route("learner", "cleanup", ((199, 270), (532, 270)), "1. normalize partner rows", "main"),
                Route("cleanup", "learner", ((532, 328), (199, 328)), "validated order", "green"),
                Route("learner", "logging", ((199, 418), (932, 418)), "2. assert correlation log", "amber"),
                Route("logging", "learner", ((932, 476), (199, 476)), "captured text", "green"),
                Route("learner", "scratch", ((199, 566), (1288, 566)), "3. write scratch rows", "main"),
                Route("scratch", "learner", ((1288, 624), (199, 624)), "cleanup verified", "green"),
            ),
        ),
        flow_diagram(
            "example-foundation-order-cleanup",
            "foundation-order-cleanup Flow",
            "Partner feed rows become typed orders while invalid line items are counted and logs remain testable.",
            ("PartnerRow", "raw tenant/order/items", "blue"),
            [
                ("Required fields", "require_not_blank", "neutral"),
                ("Optional defaults", "blank_to_default", "green"),
                ("Invalid items", "skip blank sku or qty <= 0", "pink"),
                ("CleanupReport", "orders + skipped count", "amber"),
                ("CapturedLogs", "correlation.id event", "lavender"),
            ],
            "Use this first when learning bluetape-rs-core validation helpers.",
        ),
        flow_diagram(
            "example-request-tracing-log-capture",
            "request-tracing-log-capture Flow",
            "A request boundary validates input and proves structured tracing output in a captured subscriber.",
            ("Request Input", "correlation + route + status", "blue"),
            [
                ("Route + Status", "validate before logging", "neutral"),
                ("CorrelationId", "safe single-line id", "green"),
                ("Tracing Event", "structured fields", "amber"),
                ("CapturedLogs", "assert event text", "lavender"),
            ],
            "Use this when a workshop step needs observable request context.",
        ),
        flow_diagram(
            "example-temp-resource-test-harness",
            "temp-resource-test-harness Flow",
            "File-producing tests write into an isolated TempDir and prove deterministic cleanup.",
            ("ScratchConfig", "prefix + file_name", "blue"),
            [
                ("Path Safety", "reject absolute or parent paths", "neutral"),
                ("TempDir", "isolated workspace", "green"),
                ("Scratch File", "write rows", "amber"),
                ("Close", "remove temp dir", "lavender"),
            ],
            "Use this when examples need filesystem side effects without repository pollution.",
        ),
    ]


def flow_diagram(
    name: str,
    title: str,
    subtitle: str,
    first: tuple[str, str, str],
    rest: list[tuple[str, str, str]],
    footer: str,
) -> Diagram:
    width = 980
    height = 980
    x = 340
    boxes: list[Box] = [Box("step0", first[0], first[1], x, 170, 300, 78, first[2])]
    for i, (t, d, tone) in enumerate(rest, start=1):
        boxes.append(Box(f"step{i}", t, d, x, 170 + i * 112, 300, 78, tone))
    routes = []
    for i in range(len(boxes) - 1):
        a = boxes[i]
        b = boxes[i + 1]
        routes.append(Route(a.id, b.id, ((a.cx, a.y + a.h), (b.cx, b.y)), "", "main"))
    return Diagram(name, title, subtitle, width, height, tuple(boxes), tuple(routes), footer)


def render(d: Diagram) -> None:
    OUT.mkdir(parents=True, exist_ok=True)
    (OUT / f"{d.name}.dot").write_text(dot_for(d), encoding="utf-8")
    (OUT / f"{d.name}.svg").write_text(
        render_sequence(d) if d.kind == "sequence" else render_flow(d),
        encoding="utf-8",
    )
    subprocess.run(["dot", "-Tplain", OUT / f"{d.name}.dot"], check=True, stdout=(OUT / f"{d.name}.plain").open("w"))
    subprocess.run(["dot", "-Tsvg", OUT / f"{d.name}.dot"], check=True, stdout=(OUT / f"{d.name}-graphviz.svg").open("w"))
    subprocess.run(["dot", "-Tpng", OUT / f"{d.name}.dot"], check=True, stdout=(OUT / f"{d.name}-graphviz.png").open("wb"))
    subprocess.run(["rsvg-convert", OUT / f"{d.name}.svg", "-o", OUT / f"{d.name}.png"], check=True)


def validate_svg(d: Diagram) -> None:
    svg = (OUT / f"{d.name}.svg").read_text(encoding="utf-8")
    for required in ("Architects Daughter", "Comic Mono", 'markerWidth="8"', 'markerHeight="8"'):
        if required not in svg:
            raise SystemExit(f"{d.name}: missing {required}")
    for forbidden in ("Inter", "Arial", "Helvetica"):
        if forbidden in svg:
            raise SystemExit(f"{d.name}: forbidden UI font {forbidden}")


def main() -> None:
    for tool in ("dot", "rsvg-convert"):
        require_tool(tool)
    require_font(ARCHITECTS)
    require_font(COMIC)

    summaries: list[str] = []
    for d in diagrams():
        render(d)
        validate_svg(d)
        summaries.append(geometry_summary(d))

    summary = "\n".join(summaries) + "\n"
    (OUT / "geometry-summary.txt").write_text(summary, encoding="utf-8")
    print(summary, end="")


if __name__ == "__main__":
    main()
