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
- `rust` — Rust toolchain version, optional components, target dist/arch, and build image variant
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
- [ ] Add required GitHub secrets and variables
- [ ] Run `cargo test`
- [ ] Open a PR and verify `pr-build.yml`
- [ ] Merge and verify the first environment deployment

---

## Notes

- `.omx/`, `.claude/`, `.opencode/`, and similar agent/tooling directories are intentionally ignored and are not part of the application template contract.
- The template is designed for CloudOps Works blueprint-backed automation; if you remove that integration, also prune the related workflows and `.cloudopsworks/` configuration.
