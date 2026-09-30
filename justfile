python := if os() == "windows" { "python" } else { "python3" }
executable-suffix := if os() == "windows" { ".exe" } else { "" }

# Build and package all samples supported on the current platform.
default: build

# Build and package all samples supported on the current platform.
build: build-sample-client build-sample-echomain build-sample-reflection build-sample-echomain-stateful build-sample-dummy-self-reconfig build-sample-no-default-features _build-platform-samples

# Build the full Rust workspace.
build-rust:
    cargo build

# Build the PAL crate.
build-pal:
    cargo build -p mssf-pal

# Build the sample client.
build-sample-client:
    cargo build -p samples_client

# Run the sample client.
run-sample-client:
    cargo run -p samples_client

# Generate and format the Rust COM bindings.
generate-rust:
    cargo run -p tools_api
    cargo fmt -p mssf-com

# Remove generated Rust COM bindings.
clean-generated:
    {{ python }} scripts/build_tasks.py clean-generated

# Format handwritten Rust code.
format:
    cargo fmt -p mssf-core
    cargo fmt -p mssf-pal
    cargo fmt -p samples_client
    cargo fmt -p samples_echomain
    cargo fmt -p samples_echomain_stateful
    cargo fmt -p samples_reflection
    cargo fmt -p dummy-self-reconfig

# Build and package the stateless echo sample.
build-sample-echomain: build-pal
    cargo build -p samples_echomain
    just _package-sf-app crates/samples/echomain/manifests build/sf_apps/samples_echomain target/debug/samples_echomain{{ executable-suffix }}

# Build and package the reflection sample.
build-sample-reflection: build-pal
    cargo build -p samples_reflection
    just _package-sf-app crates/samples/reflection/manifests build/sf_apps/samples_reflection target/debug/samples_reflection{{ executable-suffix }}

# Build and package the stateful echo sample.
build-sample-echomain-stateful: build-pal
    cargo build -p samples_echomain_stateful
    just _package-sf-app crates/samples/echomain-stateful/manifests build/sf_apps/samples_echomain_stateful target/debug/samples_echomain_stateful{{ executable-suffix }}

# Build and package the self-reconfiguration sample.
build-sample-dummy-self-reconfig: build-pal
    cargo build -p dummy-self-reconfig
    just _package-sf-app crates/samples/dummy-self-reconfig/manifests build/sf_apps/dummy-self-reconfig target/debug/dummy-self-reconfig{{ executable-suffix }}

# Build the sample that disables default features.
build-sample-no-default-features: build-pal
    cargo build -p no_default_features

# Build and package the Windows-only key-value store sample.
build-sample-kvstore: build-pal
    cargo build -p kvstore
    just _package-sf-app crates/samples/kvstore/manifests build/sf_apps/kvstore target/debug/kvstore{{ executable-suffix }}

_build-platform-samples:
    {{ if os() == "windows" { "just build-sample-kvstore" } else { "echo 'Skipping kvstore sample (Windows only)'" } }}

_package-sf-app manifest-dir output-dir executable:
    {{ python }} scripts/build_tasks.py package "{{ manifest-dir }}" "{{ output-dir }}" "{{ executable }}"
