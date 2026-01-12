---
title: GitOps
description: Deployments managed via Git-driven reconciliation (e.g., ArgoCD), with verification and safety checks
relatedPages:
  - developer/operations/folio/prometheus-advanced.mdx
  - developer/operations/service-doc-10.mdx
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Platform Engineering | **Review Cycle:** Quarterly
</Info>

## Overview

GitOps is the preferred deployment approach: merges to the deployment branch are reconciled into the target environment by the GitOps controller.

## Operational playbook

### Verify application status

```bash
argocd app get materi-platform
```

### Sync manually (break-glass)

```bash
argocd app sync materi-platform
argocd app wait materi-platform --health
```

## Verification checklist

-   Controller reports Healthy/Synced
-   Service health endpoints return success
-   Dashboards show telemetry flowing
-   No crash loops in pods

## Rollback

If the GitOps controller supports it, rollback is a revert of the deployment commit plus reconciliation.

See: /developer/operations/deployment/rollback
