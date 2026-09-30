import argparse
import shutil
from pathlib import Path
from xml.etree import ElementTree


REPO_ROOT = Path(__file__).resolve().parent.parent
GENERATED_COM_DIRECTORIES = (
    "crates/libs/com/src/Microsoft/ServiceFabric/FabricClient",
    "crates/libs/com/src/Microsoft/ServiceFabric/FabricCommon",
    "crates/libs/com/src/Microsoft/ServiceFabric/FabricRuntime",
    "crates/libs/com/src/Microsoft/ServiceFabric/FabricTypes",
    "crates/libs/com/src/Windows/ServiceFabric/FabricClient",
    "crates/libs/com/src/Windows/ServiceFabric/FabricCommon",
    "crates/libs/com/src/Windows/ServiceFabric/FabricRuntime",
    "crates/libs/com/src/Windows/ServiceFabric/FabricTypes",
)


def clean_generated() -> None:
    for relative_path in GENERATED_COM_DIRECTORIES:
        path = REPO_ROOT / relative_path
        if path.exists():
            shutil.rmtree(path)


def get_service_manifest_name(manifest_dir: Path) -> str:
    application_manifest = manifest_dir / "ApplicationManifest.xml"
    root = ElementTree.parse(application_manifest).getroot()
    for element in root.iter():
        service_manifest_name = element.get("ServiceManifestName")
        if service_manifest_name:
            return service_manifest_name
    raise ValueError(
        f"Cannot find ServiceManifestName in {application_manifest}"
    )


def package_sf_app(
    manifest_dir: Path, output_dir: Path, executable: Path
) -> None:
    service_manifest_name = get_service_manifest_name(manifest_dir)
    executable_name = (
        executable.stem if executable.suffix.lower() == ".exe" else executable.name
    )
    code_dir = output_dir / service_manifest_name / "Code"

    code_dir.mkdir(parents=True, exist_ok=True)
    shutil.copytree(manifest_dir, output_dir, dirs_exist_ok=True)
    shutil.copy2(executable, code_dir / f"{executable_name}.exe")


def main() -> None:
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers(dest="command", required=True)
    subparsers.add_parser("clean-generated")

    package_parser = subparsers.add_parser("package")
    package_parser.add_argument("manifest_dir", type=Path)
    package_parser.add_argument("output_dir", type=Path)
    package_parser.add_argument("executable", type=Path)

    args = parser.parse_args()
    if args.command == "clean-generated":
        clean_generated()
    else:
        package_sf_app(args.manifest_dir, args.output_dir, args.executable)


if __name__ == "__main__":
    main()
