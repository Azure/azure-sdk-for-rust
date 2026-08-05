#!/usr/bin/env python3
"""Polls a Cosmos DB account-read endpoint and records every distinct payload.

Answers the open questions the in-memory emulator's dynamic-topology work could
not settle from source alone:

  Q1  When does a newly added region appear in readableLocations, and what
      orders the array?
  Q4  Is `_etag` populated on the account read, and does it change on a
      topology change? (Load-bearing: the driver short-circuits
      sync_account_properties on an unchanged etag.)
  Q5  What happens to region ids / session tokens across remove + re-add?

Usage:
  capture_topology.py <account-name> <master-key> [--seconds N] [--interval S]
                      [--out FILE]
"""

import argparse
import base64
import datetime
import hashlib
import hmac
import json
import ssl
import sys
import time
import urllib.parse
import urllib.request

try:
    import certifi

    SSL_CONTEXT = ssl.create_default_context(cafile=certifi.where())
except ImportError:  # pragma: no cover - fall back to the system store
    SSL_CONTEXT = ssl.create_default_context()


def auth_header(verb: str, resource_type: str, resource_link: str, key: str, date: str) -> str:
    text = f"{verb.lower()}\n{resource_type.lower()}\n{resource_link}\n{date.lower()}\n\n"
    signature = base64.b64encode(
        hmac.new(base64.b64decode(key), text.encode("utf-8"), hashlib.sha256).digest()
    ).decode()
    return urllib.parse.quote(f"type=master&ver=1.0&sig={signature}", safe="")


def read_account(endpoint: str, key: str):
    date = datetime.datetime.now(datetime.timezone.utc).strftime("%a, %d %b %Y %H:%M:%S GMT")
    request = urllib.request.Request(
        endpoint,
        headers={
            "Authorization": auth_header("get", "", "", key, date),
            "x-ms-date": date,
            "x-ms-version": "2018-12-31",
        },
    )
    with urllib.request.urlopen(request, timeout=30, context=SSL_CONTEXT) as response:
        return json.loads(response.read()), dict(response.headers)


def summarize(payload: dict) -> dict:
    """The topology-relevant projection, used to detect distinct states."""
    return {
        "readableLocations": [l["name"] for l in payload.get("readableLocations", [])],
        "writableLocations": [l["name"] for l in payload.get("writableLocations", [])],
        "enableMultipleWriteLocations": payload.get("enableMultipleWriteLocations"),
        "enablePerPartitionFailoverBehavior": payload.get("enablePerPartitionFailoverBehavior"),
        "_etag": payload.get("_etag"),
    }


def main() -> int:
    parser = argparse.ArgumentParser()
    parser.add_argument("account")
    parser.add_argument("key")
    parser.add_argument("--seconds", type=int, default=0, help="0 = single sample")
    parser.add_argument("--interval", type=float, default=5.0)
    parser.add_argument("--out")
    args = parser.parse_args()

    endpoint = f"https://{args.account}.documents.azure.com/"
    captured = []
    last_summary = None
    deadline = time.time() + args.seconds

    while True:
        stamp = datetime.datetime.now(datetime.timezone.utc).isoformat()
        try:
            payload, headers = read_account(endpoint, args.key)
        except Exception as exc:  # noqa: BLE001 - capture tool, report and keep polling
            print(f"{stamp} ERROR {exc}", flush=True)
            if time.time() >= deadline:
                break
            time.sleep(args.interval)
            continue

        current = summarize(payload)
        if current != last_summary:
            entry = {
                "timestamp": stamp,
                "etag_header": headers.get("etag") or headers.get("ETag"),
                "summary": current,
                "payload": payload,
            }
            captured.append(entry)
            print(f"{stamp} {json.dumps(current)}", flush=True)
            last_summary = current

        if time.time() >= deadline:
            break
        time.sleep(args.interval)

    if args.out:
        with open(args.out, "w", encoding="utf-8") as handle:
            json.dump(captured, handle, indent=2)
        print(f"wrote {len(captured)} distinct state(s) to {args.out}", flush=True)
    return 0


if __name__ == "__main__":
    sys.exit(main())
