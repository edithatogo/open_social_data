"""Run long-term maintenance checks for Open Social Datasets."""

from __future__ import annotations

import argparse
import re
import subprocess
import sys
import urllib.error
import urllib.request
from dataclasses import dataclass
from pathlib import Path


ROOT = Path(__file__).resolve().parents[1]
DATASET_ROOT = ROOT / "datasets"
URL_RE = re.compile(r"https?://[^\s)\]>]+")
PLACEHOLDER_RE = re.compile(
    r"\$\(date|YYYY-MM-DD|Previous Date|repository issues page or maintainer contact|ABS website URL",
    re.IGNORECASE,
)
REQUIRED_DATASET_FILES = (
    "README.md",
    "SESSION_LOG.md",
)


@dataclass
class CheckResult:
    name: str
    ok: bool
    detail: str


def dataset_dirs() -> list[Path]:
    return [
        path
        for source in ("abs", "stats_nz", "aihw")
        for path in sorted((DATASET_ROOT / source).glob("*"))
        if path.is_dir()
    ]


def check_dataset_pack(dataset: Path) -> list[CheckResult]:
    results: list[CheckResult] = []
    rel = dataset.relative_to(ROOT)
    for filename in REQUIRED_DATASET_FILES:
        path = dataset / filename
        results.append(
            CheckResult(
                f"{rel}/{filename}",
                path.is_file(),
                "present" if path.is_file() else "missing",
            )
        )

    docs = dataset / "docs"
    data_dictionary = any(docs.glob("data_dictionary*.md")) if docs.is_dir() else False
    accessible_guide = any(docs.glob("accessible_guide*.md")) if docs.is_dir() else False
    scripts = dataset / "scripts"
    access_script = any(scripts.glob("*.py")) if scripts.is_dir() else False
    results.extend(
        [
            CheckResult(f"{rel}/docs/data_dictionary*.md", data_dictionary, "present" if data_dictionary else "missing"),
            CheckResult(f"{rel}/docs/accessible_guide*.md", accessible_guide, "present" if accessible_guide else "missing"),
            CheckResult(f"{rel}/scripts/*.py", access_script, "present" if access_script else "missing"),
        ]
    )
    return results


def markdown_files() -> list[Path]:
    roots = [ROOT, ROOT / "datasets", ROOT / "docs", ROOT / "conductor"]
    files: set[Path] = set()
    for root in roots:
        files.update(root.glob("*.md"))
        if root != ROOT:
            files.update(root.rglob("*.md"))
    return sorted(files)


def check_placeholders() -> list[CheckResult]:
    results: list[CheckResult] = []
    allowed = {
        ROOT / "templates" / "dataset_readme_template.md",
        ROOT / "templates" / "data_dictionary_template.md",
        ROOT / "templates" / "accessible_guide_template.md",
        ROOT / "SESSION_LOG_TEMPLATE.md",
    }
    for path in markdown_files():
        if path in allowed:
            continue
        text = path.read_text(encoding="utf-8", errors="replace")
        matches = sorted(set(match.group(0) for match in PLACEHOLDER_RE.finditer(text)))
        if matches:
            results.append(
                CheckResult(
                    str(path.relative_to(ROOT)),
                    False,
                    f"placeholder markers: {', '.join(matches)}",
                )
            )
    return results


def collect_urls() -> dict[str, set[str]]:
    urls: dict[str, set[str]] = {}
    for path in markdown_files():
        text = path.read_text(encoding="utf-8", errors="replace")
        found = {url.rstrip(".,") for url in URL_RE.findall(text)}
        if found:
            urls[str(path.relative_to(ROOT))] = found
    return urls


def check_url_presence() -> list[CheckResult]:
    results: list[CheckResult] = []
    for dataset in dataset_dirs():
        readme = dataset / "README.md"
        rel = readme.relative_to(ROOT)
        if not readme.exists():
            continue
        urls = URL_RE.findall(readme.read_text(encoding="utf-8", errors="replace"))
        results.append(
            CheckResult(str(rel), bool(urls), f"{len(urls)} URL(s)" if urls else "no source URLs found")
        )
    return results


def live_check_url(url: str, timeout: float) -> tuple[bool, str]:
    request = urllib.request.Request(
        url,
        method="HEAD",
        headers={"User-Agent": "open-social-data-maintenance/0.1"},
    )
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response:
            return 200 <= response.status < 400, f"HTTP {response.status}"
    except urllib.error.HTTPError as error:
        if error.code == 405:
            get_request = urllib.request.Request(
                url,
                headers={"User-Agent": "open-social-data-maintenance/0.1"},
            )
            with urllib.request.urlopen(get_request, timeout=timeout) as response:
                return 200 <= response.status < 400, f"HTTP {response.status}"
        return False, f"HTTP {error.code}"
    except Exception as error:  # noqa: BLE001 - maintenance report should keep going.
        return False, str(error)


def check_live_urls(timeout: float) -> list[CheckResult]:
    results: list[CheckResult] = []
    for file_path, urls in collect_urls().items():
        for url in sorted(urls):
            ok, detail = live_check_url(url, timeout)
            results.append(CheckResult(f"{file_path}: {url}", ok, detail))
    return results


def run_command(name: str, command: list[str]) -> CheckResult:
    completed = subprocess.run(
        command,
        cwd=ROOT,
        text=True,
        stdout=subprocess.PIPE,
        stderr=subprocess.STDOUT,
    )
    detail = completed.stdout.strip().splitlines()
    summary = detail[-1] if detail else "no output"
    return CheckResult(name, completed.returncode == 0, summary)


def emit(results: list[CheckResult]) -> int:
    failed = False
    for result in results:
        status = "OK" if result.ok else "FAIL"
        print(f"{status} {result.name}: {result.detail}")
        failed = failed or not result.ok
    return 1 if failed else 0


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("--live", action="store_true", help="Also perform live HTTP checks for discovered URLs.")
    parser.add_argument("--timeout", type=float, default=10.0, help="Per-URL timeout for --live checks.")
    args = parser.parse_args()

    results: list[CheckResult] = []
    for dataset in dataset_dirs():
        results.extend(check_dataset_pack(dataset))
    results.extend(check_url_presence())
    results.extend(check_placeholders())
    results.append(run_command("python compileall scripts datasets", [sys.executable, "-m", "compileall", "-q", "scripts", "datasets"]))
    if args.live:
        results.extend(check_live_urls(args.timeout))

    raise SystemExit(emit(results))


if __name__ == "__main__":
    main()
