# Rust Application Template

This repository is a **CloudOps Works Rust application template**. It gives you:

- a minimal Actix Web HTTP service scaffold
- CloudOps Works CI/CD wiring under `.cloudopsworks/`
- GitHub Actions workflows for build, scan, preview, release, and deployment
- deployment templates for Kubernetes, Lambda, Elastic Beanstalk, App Engine, and Cloud Run
- sample API definition placeholders under `apifiles/`

Use this template when you want a new Rust service that already follows the CloudOps Works delivery model.

---

## What this template includes

### Application scaffold
- `Cargo.toml` — crate metadata, dependencies, and binary name
- `src/main.rs` — Actix Web process entrypoint and server bootstrap
- `src/lib.rs` — library exports for routes and handlers
- `src/routes.rs` — route registration
- `src/handlers/` — sample `hello` and `health` handlers
- `tests/integration_tests.rs` — smoke coverage for the sample endpoints
- `apifiles/` — API definition placeholders and examples

### Delivery scaffold
- `.cloudopsworks/cloudopsworks-ci.yaml` — repository governance and environment mapping
- `.cloudopsworks/vars/inputs-global.yaml` — global build/deploy defaults
- `.cloudopsworks/vars/inputs-*.yaml` — target-specific environment templates
- `.cloudopsworks/gitversion_gitflow.yaml` — reference GitVersion config aligned with the template's default GitFlow release model
- `.cloudopsworks/gitversion_githubflow.yaml` — optional GitVersion reference for teams that explicitly rewire workflows away from GitFlow
- `.github/workflows/` — reusable CI/CD orchestration
- `Makefile` — bootstrap/version targets used by template consumers

---

## Quick start

### 1. Create a repository from this template
Create your new repository from `cloudopsworks/rust-app-template`, then clone it locally.

### 2. Initialize the crate metadata
Run the bootstrap target from the root of the new repository:

```bash
make code/init
```

This target:
- updates `Cargo.toml` package name to the current directory name
- updates the first binary name in `Cargo.toml`
- rewrites Rust crate references from `hello_api::` to your repository name converted to snake case

> Rename the repository directory before running `make code/init` if you want the crate and binary names to match the final service name.

### 3. Update the sample application
At minimum, review and update:
- `Cargo.toml`
- `src/main.rs`
- `src/routes.rs`
- `src/handlers/`
- `tests/integration_tests.rs`
- `apifiles/`

The template starts with two working endpoints so the pipeline has a healthy baseline:
- `GET /` -> `Hello, world!`
- `GET /health` -> `{ "status": "OK" }`

### 4. Verify locally
```bash
cargo test
make version
```

`make version` writes a `VERSION` file using GitVersion semantics and then synchronizes the crate version in `Cargo.toml` with that computed version.

### 5. Run locally
```bash
cargo run
```

By default the service binds to `127.0.0.1:8080`.

Environment variables:
- `HOST` — optional bind host, defaults to `127.0.0.1`
- `PORT` — optional bind port, defaults to `8080`

---

## Required template configuration

### `.cloudopsworks/cloudopsworks-ci.yaml`
This file controls repository behavior and deployment routing.

Update these sections first:

#### `config`
- `branchProtection` — enable branch protection automation
- `gitFlow.enabled` — keep `true` if you use GitFlow branch conventions
- `gitFlow.supportBranches` — enable only if you maintain long-lived support branches
- `requiredReviewers`, `reviewers`, `owners`, `contributors` — repository governance

#### `cd.deployments`
This maps branch/tag flows to deployment environments.

Default mapping in this template:
- `develop` -> `dev`
- `release/**` -> `prod`
- internal `test` stage -> `uat`
- prerelease tags -> `demo`
- `hotfix` -> `hotfix`
- optional `support` mappings by version match

Adjust these names to match your environments and promotion flow.

### `.cloudopsworks/vars/inputs-global.yaml`
This is the main global configuration file used by the workflows.

Set these values before your first pipeline run:
- `organization_name`
- `organization_unit`
- `environment_name`
- `repository_owner`
- `cloud`
- `cloud_type`

Use `cloud: none` and `cloud_type: none` only for repositories that should build and scan without deployment.

Common optional sections:
- `rust` — Rust toolchain version, optional components, target dist/arch, build image variant, and optional `goreleaser: true` flag to activate GoReleaser releases (requires `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` secrets — see **GoReleaser secrets** below)
- `preview` — PR preview environment behavior
- `apis` — API Gateway publishing
- `observability` — tracing and monitoring agent configuration
- `snyk`, `semgrep`, `trivy`, `sonarqube`, `dependencyTrack` — security and quality tooling
- `docker_inline`, `docker_args`, `custom_run_command`, `custom_usergroup` — container customization
- `is_ooss` — mark the generated repository as open source when needed

---

## Choose one deployment target per environment

Each active environment should have exactly one matching deployment-target file under `.cloudopsworks/vars/`. Files that contain `ENV` in the name are placeholders: copy or rename them for each real environment as part of repository setup.

### Kubernetes / EKS / AKS / GKE
Use `inputs-KUBERNETES-ENV.yaml`.

Key fields:
- `container_registry`
- `cluster_name`
- `namespace`
- target-cloud credentials/settings
- optional Helm, secret, and external-secret overrides

### AWS Lambda
Use `inputs-LAMBDA-ENV.yaml`.

Key fields:
- `versions_bucket`
- `aws.region`
- Lambda runtime/handler settings
- IAM, VPC, and trigger configuration

### AWS Elastic Beanstalk
Use `inputs-BEANSTALK-ENV.yaml`.

Key fields:
- `versions_bucket`
- `container_registry`
- `aws.region`
- Beanstalk platform, instance, port, and extra settings

Use this file as a starting point only: review the Beanstalk platform defaults carefully and replace them with the runtime strategy your Rust service actually uses.

`runner_set` is optional and only needed when you use self-hosted runners.

### Google App Engine
Use `inputs-APPENGINE.yaml`.

Key fields:
- `versions_bucket`
- `container_registry`
- `gcp.region`
- `gcp.project_id`
- `appengine.runtime` — use `custom` for Rust App Engine deployments
- `appengine.type` — use `flexible` for Rust custom runtimes
- `appengine.entrypoint_shell` — startup command App Engine should execute, for example `./your-service-binary`

For Rust services on App Engine, prefer the flexible environment with a custom runtime.

### Google Cloud Run
Use `inputs-CLOUDRUN.yaml`.

Key fields:
- `versions_bucket`
- `container_registry`
- `gcp.region`
- `gcp.project_id`
- `cloudrun.type`

---

## Preview environments

Preview environments are configured from:
- `.cloudopsworks/vars/preview/inputs.yaml`
- `.cloudopsworks/vars/preview/values.yaml`

Enable them in `inputs-global.yaml`:

```yaml
preview:
  enabled: true
```

Use preview environments when pull requests from `feature/**` or `hotfix/**` should deploy an isolated review environment.

---

## GitHub Actions workflow model

Important workflows in this template:

- `main-build.yml` — build, test, containerize, scan, and release/deploy on branch/tag events
- `pr-build.yml` — PR validation and optional preview deployment
- `deploy-container.yml` — push application container artifacts
- `deploy.yml` — standard deployment flow
- `deploy-blue-green.yml` — blue/green deployment flow
- `scan.yml` — SAST/SCA/DAST orchestration
- `environment-unlock.yml` / `environment-destroy.yml` — environment operations
- `automerge.yml`, slash-command workflows, Jira integration workflows — repo automation

This template also includes dedicated GitVersion reference files for both GitFlow and GitHub Flow release models. The default repository behavior remains GitFlow. Treat the GitHub Flow file as reference-only unless you also update workflow triggers, deployment mapping, and branch policy to match it. If your generated repository wants to use one of these files directly, wire it explicitly in your generator or build logic rather than assuming automatic selection.

---

## Secrets and variables expected by workflows

The workflows expect GitHub repository or organization configuration for build, preview, and deploy credentials.

Typical examples:
- `BOT_TOKEN`
- `BUILD_AWS_ACCESS_KEY_ID` / `BUILD_AWS_SECRET_ACCESS_KEY`
- `DEPLOYMENT_AWS_ACCESS_KEY_ID` / `DEPLOYMENT_AWS_SECRET_ACCESS_KEY`
- `BUILD_GCP_CREDENTIALS` / `DEPLOYMENT_GCP_CREDENTIALS`
- `BUILD_AZURE_SERVICE_ID` / `BUILD_AZURE_SERVICE_SECRET`
- `DEPLOYMENT_AZURE_SERVICE_ID` / `DEPLOYMENT_AZURE_SERVICE_SECRET`
- runner, registry, region, and state configuration variables

Review the `with:` and `secrets:` blocks in the workflow files and align your repository settings before enabling deployments.

### GoReleaser secrets (required when goreleaser is enabled)

When you set `goreleaser: true` under the `rust:` section of `.cloudopsworks/vars/inputs-global.yaml`, the `main-build.yml` pipeline activates a GoReleaser release step. This step signs artifacts using GPG and requires two additional secrets to be present at the **repository or organization level** before the first pipeline run:

| Secret | Description |
|--------|-------------|
| `GPG_PRIVATE_KEY` | ASCII-armored GPG private key used to sign release artifacts. Export with `gpg --armor --export-secret-keys <KEY_ID>`. |
| `GPG_PASSPHRASE` | Passphrase that unlocks the private key above. |

> **Note:** When goreleaser is active, standard cloud deployment jobs (`deploy.yml` and `deploy-blue-green.yml`) are skipped automatically. Set `cloud_type: none` in `inputs-global.yaml` to make this intent explicit and avoid configuration drift.

---

## Release and versioning expectations

This template repository follows semantic versioning.

- documentation/template-only fixes -> patch release
- backward-compatible template capability additions -> minor release
- breaking workflow or template contract changes -> major release

Version calculation is GitVersion-based, and release automation relies on commit/PR annotations such as:
- `+semver: patch`
- `+semver: fix`
- `+semver: minor`
- `+semver: feature`
- `+semver: major`
- `+semver: hotfix`

If you use the CloudOps Works release workflow, keep changes grouped by release intent so the generated version bump stays predictable.

---

## Recommended first-pass checklist for new repositories

- [ ] Create repo from template
- [ ] Run `make code/init`
- [ ] Rename/update the sample Actix handlers and routes
- [ ] Review crate metadata and dependencies in `Cargo.toml`
- [ ] Update `.cloudopsworks/cloudopsworks-ci.yaml`
- [ ] Update `.cloudopsworks/vars/inputs-global.yaml`
- [ ] Configure exactly one target file per active environment
- [ ] Configure preview settings if needed
- [ ] Add required GitHub secrets and variables (include `GPG_PRIVATE_KEY` and `GPG_PASSPHRASE` if goreleaser is enabled)
- [ ] Run `cargo test`
- [ ] Open a PR and verify `pr-build.yml`
- [ ] Merge and verify the first environment deployment

---

## Notes

- `.omx/`, `.claude/`, `.opencode/`, and similar agent/tooling directories are intentionally ignored and are not part of the application template contract.
- The template is designed for CloudOps Works blueprint-backed automation; if you remove that integration, also prune the related workflows and `.cloudopsworks/` configuration.

---

## AI-assisted upgrade of `.cloudopsworks/vars` configuration files

This section is a machine-readable protocol for AI agents performing a seamless, non-destructive upgrade of all configuration files under `.cloudopsworks/vars/` when a new template version is released. Follow the steps below in order.

### Upgrade overview

The template version locked into this repository is recorded in `.cloudopsworks/_VERSION`. The canonical upstream source is the GitHub repository `cloudopsworks/rust-app-template`, pinned to the tag that matches the content of `_VERSION`.

An upgrade merges new keys, updated comments, and structural changes from the upstream template into local files **without overwriting values the operator has already set**.

---

### Step 1 — determine current and target versions

1. Read `.cloudopsworks/_VERSION` to get the **current locked version** (e.g., `v1.4.15`).
2. The **target version** is either supplied by the operator or is the latest release tag on `cloudopsworks/rust-app-template`.
3. Fetch any upstream file from GitHub using the pattern:
   ```
   https://raw.githubusercontent.com/cloudopsworks/rust-app-template/<version>/<path>
   ```
   Example:
   ```
   https://raw.githubusercontent.com/cloudopsworks/rust-app-template/v1.4.15/.cloudopsworks/vars/inputs-global.yaml
   ```

---

### Step 2 — identify the deployment type for each environment file

Each `inputs-<name>.yaml` file under `.cloudopsworks/vars/` maps to a specific upstream template. Determine the type using the following priority order:

**Priority 1 — `Agents:` header comment**

If the file contains an `# Agents:` line in its header block, read `cloud` and `cloud_type` directly from it:

```yaml
# Agents: cloud=aws ; cloud_type=lambda
```

Multiple valid combinations may be listed separated by `|`:

```yaml
# Agents: cloud=aws|gcp|azure ; cloud_type=kubernetes
```

**Priority 2 — fallback to `inputs-global.yaml`**

If no `# Agents:` line is present, read the active `cloud` and `cloud_type` values from `.cloudopsworks/vars/inputs-global.yaml` and apply the mapping table below.

**`cloud` / `cloud_type` → upstream template file:**

| `cloud`                  | `cloud_type`                   | Upstream template file         |
|--------------------------|--------------------------------|--------------------------------|
| `aws`                    | `eks` or `kubernetes`          | `inputs-KUBERNETES-ENV.yaml`   |
| `azure`                  | `aks` or `kubernetes`          | `inputs-KUBERNETES-ENV.yaml`   |
| `gcp`                    | `gke` or `kubernetes`          | `inputs-KUBERNETES-ENV.yaml`   |
| `aws`                    | `lambda`                       | `inputs-LAMBDA-ENV.yaml`       |
| `aws`                    | `beanstalk`                    | `inputs-BEANSTALK-ENV.yaml`    |
| `gcp`                    | `appengine`                    | `inputs-APPENGINE.yaml`        |
| `gcp`                    | `cloudrun`                     | `inputs-CLOUDRUN.yaml`         |
| `aws` / `gcp` / `azure`  | `none` or library mode         | `inputs-LIB-ENV.yaml`          |

`inputs-global.yaml` always maps to the upstream `inputs-global.yaml` regardless of cloud type.

---

### Step 3 — upgrade deployment target files

The deployment target files identified by the Step 2 mapping table — such as `inputs-KUBERNETES-ENV.yaml`, `inputs-LAMBDA-ENV.yaml`, `inputs-BEANSTALK-ENV.yaml`, `inputs-APPENGINE.yaml`, `inputs-CLOUDRUN.yaml`, `inputs-LIB-ENV.yaml`, and mobile equivalents such as `inputs-ANDROID-ENV.yaml` and `inputs-XCODE-ENV.yaml` — are **scaffolding templates**. They provide placeholder structures and documented examples, not finalized operator configuration.

**Do not merge these files. Overwrite them.**

Upgrade procedure for each deployment target file:

1. **Before overwriting** — inspect the local file and record any operator-configured values (keys that have been uncommented and set to non-placeholder values).
2. **Replace the file** — overwrite the local file entirely with the upstream template version.
3. **Re-apply operator values** — after overwriting, set each previously recorded operator-configured value at its corresponding key in the new file.
4. **Copy in absent files** — if a deployment target file is present in the upstream template but absent locally, copy it in from the upstream template as a new file.

---

### Step 4 — merge `inputs-global.yaml`

`inputs-global.yaml` requires special handling because it contains mandatory operator identity fields alongside a large body of optional commented-out sections.

Merge procedure:

1. **Retain the four mandatory identity fields** verbatim at the top of the file:
   ```yaml
   organization_name: "..."
   organization_unit: "..."
   environment_name: "..."
   repository_owner: "..."
   ```
2. **Retain `cloud` and `cloud_type`** exactly as the operator set them.
3. **For every optional commented-out section** in the upstream template, check the local file:
   - If the operator **has uncommented and configured it** — keep the operator's values; update only surrounding comment text if it changed upstream.
   - If the section **is still fully commented out locally** — replace the entire commented block with the upstream version, capturing any new fields or updated documentation within it.
4. **Append new optional sections** that appear in the upstream template but are entirely absent locally, in fully commented-out form, preserving their upstream position and comments.

---

### Step 5 — upgrade subdirectory files

Apply the merge rules from Step 4 to every file in the following subdirectories, matching each local file to its corresponding upstream file at the same relative path:

- `.cloudopsworks/vars/preview/inputs.yaml`
- `.cloudopsworks/vars/preview/values.yaml`
- `.cloudopsworks/vars/apigw/apis-global.yaml`
- `.cloudopsworks/vars/apigw/apis-dev.yaml`
- `.cloudopsworks/vars/apigw/apis-uat.yaml`
- `.cloudopsworks/vars/apigw/apis-prod.yaml`
- `.cloudopsworks/vars/helm/values-dev.yaml`
- `.cloudopsworks/vars/helm/values-uat.yaml`
- `.cloudopsworks/vars/helm/values-prod.yaml`

---

### Step 6 — update `_VERSION`

After all merges are verified correct, write the target version string (e.g., `v1.4.16`) to `.cloudopsworks/_VERSION`. This is the final step.

---

### Upgrade invariants

An agent performing this upgrade must **never**:

- Overwrite a field the operator has explicitly set to a non-placeholder value.
- Remove a commented-out operator value without first reporting it.
- Change the YAML structure of any active (uncommented) operator section.
- Alter a file's opening description comment (`# This file contains...`) unless the upstream version changed it.
- Modify `.cloudopsworks/cloudopsworks-ci.yaml`, `gitversion_*.yaml`, or any file under `.github/workflows/` as part of a vars upgrade — those follow their own upgrade path.
- Update `_VERSION` before all file merges are complete.

---

### Conflict resolution

When a merge cannot be resolved automatically (for example, the upstream template restructured a section that the operator has customized):

1. Emit a diff showing both the upstream template block and the local operator block side by side.
2. Pause and present the conflict to the operator, asking which version to keep or whether a manual merge is needed.
3. Never silently choose one side.
