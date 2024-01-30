import argparse
import hashlib
import itertools
import json
import subprocess


def fetch_json(image: str) -> str:
    return subprocess.check_output(("oras", "manifest", "fetch", image), text=True)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("base")
    args = parser.parse_args()

    manifests = []
    for os, arch, kind in itertools.product(
        ("linux",),
        ("amd64",),
        ("docker", "oras"),
    ):
        manifest_json = fetch_json(f"{args.base}-{os}-{arch}-{kind}")
        manifest = json.loads(manifest_json)
        if manifest["mediaType"] == "application/vnd.oci.image.index.v1+json":
            manifests.extend(manifest["manifests"])
        else:
            manifests.append(
                {
                    "mediaType": manifest["mediaType"],
                    "artifactType": manifest.get("artifactType"),
                    "digest": (
                        "sha256:" + hashlib.sha256(manifest_json.encode()).hexdigest()
                    ),
                    "size": len(manifest_json),
                    "platform": {
                        "architecture": arch,
                        "os": os,
                    },
                },
            )

    manifest = {
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.index.v1+json",
        "manifests": manifests,
    }
    print(json.dumps(manifest))


if __name__ == "__main__":
    main()
