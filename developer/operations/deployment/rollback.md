---
title: Rollback
description: How to roll back safely (app, GitOps, and database-aware rollbacks)
relatedPages:
  - developer/operations/folio/prometheus-advanced.mdx
  - developer/operations/service-doc-10.mdx
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Platform Engineering | **Review Cycle:** Quarterly
</Info>

## Triage

Rollback when:

-   availability is degraded
-   error rate spikes
-   a deployment introduces incorrect behavior with no safe runtime mitigation

## Application rollback (Kubernetes)

```bash
kubectl rollout history deployment/api -n materi
kubectl rollout undo deployment/api -n materi
kubectl rollout status deployment/api -n materi
```

## GitOps rollback

If using GitOps, rollback is usually:

1. Revert the deployment commit
2. Allow controller reconciliation
3. Verify health + telemetry

## Database-aware rollback

-   If migrations are **backwards compatible**, an app rollback is usually safe.
-   If migrations are **breaking**, you need a downgrade plan (or restore) before rolling back the app.

## Verification checklist

-   Health checks pass
-   Error rate returns to baseline
-   Key dashboards stable
-   Incident summary captured
