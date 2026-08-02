#!/usr/bin/env python3
"""Publish an archive packet and receipt through the pinned Hugging Face API."""

from __future__ import annotations

import argparse
import os
from pathlib import Path
from typing import Any


def api_client() -> Any:
    token = os.getenv("HF_TOKEN")
    if not token:
        raise SystemExit("HF_TOKEN is not configured")
    try:
        from huggingface_hub import HfApi
    except ImportError as error:
        raise SystemExit("huggingface_hub is required for publication") from error
    return HfApi(token=token)


def revision(api: Any, repo: str) -> str:
    value = api.repo_info(repo_id=repo, repo_type="dataset").sha
    if not value:
        raise SystemExit(f"Hugging Face did not return a revision for {repo}")
    return str(value)


def parser() -> argparse.ArgumentParser:
    result = argparse.ArgumentParser(description=__doc__)
    subcommands = result.add_subparsers(dest="command", required=True)

    info = subcommands.add_parser("info")
    info.add_argument("--repo", required=True)

    folder = subcommands.add_parser("upload-folder")
    folder.add_argument("--repo", required=True)
    folder.add_argument("--folder", type=Path, required=True)
    folder.add_argument("--message", required=True)

    file = subcommands.add_parser("upload-file")
    file.add_argument("--repo", required=True)
    file.add_argument("--file", type=Path, required=True)
    file.add_argument("--path", required=True)
    file.add_argument("--message", required=True)
    return result


def main() -> None:
    args = parser().parse_args()
    api = api_client()
    if args.command == "upload-folder":
        if not args.folder.is_dir():
            raise SystemExit(f"archive folder does not exist: {args.folder}")
        api.upload_folder(
            repo_id=args.repo,
            repo_type="dataset",
            folder_path=args.folder,
            path_in_repo=".",
            commit_message=args.message,
        )
    elif args.command == "upload-file":
        if not args.file.is_file():
            raise SystemExit(f"receipt file does not exist: {args.file}")
        api.upload_file(
            repo_id=args.repo,
            repo_type="dataset",
            path_or_fileobj=args.file,
            path_in_repo=args.path,
            commit_message=args.message,
        )
    print(revision(api, args.repo))


if __name__ == "__main__":
    main()
