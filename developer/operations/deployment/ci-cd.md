---
title: CI/CD
description: How Materi builds, tests, scans, and ships changes (pipelines + operational playbooks)
relatedPages:
  - developer/operations/folio/prometheus-advanced.mdx
  - developer/operations/service-doc-10.mdx
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Platform Engineering | **Review Cycle:** Quarterly
</Info>

## Overview

This runbook covers how CI/CD is expected to work across Materi services, and how to operate and troubleshoot the pipeline system.

## Pipeline architecture

At a high level:

-   Per-service CI runs on PRs and main pushes
-   Integration tests run after service CI gates
-   Security scans and policy gates run before deploy
-   Deployment is triggered via approved workflows (manual or GitOps)

## Workflow reference (baseline)

| Workflow                | Trigger  | Purpose                                |
| ----------------------- | -------- | -------------------------------------- |
| `ci.yml`                | PR, push | Service CI (build/test/lint)           |
| `integration-tests.yml` | after CI | Cross-service integration validation   |
| `security-tests.yml`    | PR, push | SAST + dependency + container scanning |
| `deploy.yml`            | manual   | Unified deployment to env              |

## Triggering pipelines

### Manual dispatch

```bash
gh workflow run deploy.yml \
	-f environment=staging \
	-f version=v1.2.3
```

## Monitoring

Track pipeline health using:

-   GitHub Actions run history
-   Deployment health checks in the target environment
-   Observability dashboards (if configured)

## Rollback procedures

Rollback guidance lives in: /developer/operations/deployment/rollback

## Troubleshooting

Common classes:

-   Toolchain mismatch (Go/Python/Rust versions)
-   Secrets missing (registry, kubeconfig)
-   Security gate failures
-   Flaky integration tests

When debugging, capture:

-   Workflow name + run URL
-   failing job name
-   commit SHA
