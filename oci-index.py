import argparse
import hashlib
import json
import subprocess


def fetch_json(image: str) -> str:
    return subprocess.check_output(("oras", "manifest", "fetch", image), text=True)


def main() -> None:
    parser = argparse.ArgumentParser()
    parser.add_argument("manifest", nargs="+")
    args = parser.parse_args()

    manifests = []
    for path in args.manifest:
        path, platform, *_ = (*path.rsplit(":", maxsplit=1), None)

        with open(path) as f:
            manifest_json = f.read()
        manifest = json.loads(manifest_json)

        if manifest["mediaType"] == "application/vnd.oci.image.index.v1+json":
            manifests.extend(manifest["manifests"])
        else:
            entry = {
                "mediaType": manifest["mediaType"],
                "digest": (
                    "sha256:" + hashlib.sha256(manifest_json.encode()).hexdigest()
                ),
                "size": len(manifest_json),
            }
            if artifact_type := manifest.get("artifactType"):
                entry["artifactType"] = artifact_type
            if platform:
                os, architecture = platform.split("/", maxsplit=1)
                entry["platform"] = {"os": os, "architecture": architecture}
            manifests.append(entry)

    manifest = {
        "schemaVersion": 2,
        "mediaType": "application/vnd.oci.image.index.v1+json",
        "manifests": manifests,
    }
    print(json.dumps(manifest, sort_keys=True))


if __name__ == "__main__":
    main()
