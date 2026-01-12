---
title: Blue-Green
description: Progressive and blue-green deployment strategies for higher-risk changes
relatedPages:
  - developer/operations/folio/prometheus-advanced.mdx
  - developer/operations/service-doc-10.mdx
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Platform Engineering | **Review Cycle:** Quarterly
</Info>

## When to use blue-green / progressive rollout

Use progressive rollout patterns when:

-   The change is high-risk (auth, collaboration, migrations)
-   The blast radius is large
-   You need fast rollback with minimal downtime

## Canary deployment (example)

```bash
gh workflow run canary-deploy.yml \
	-f version=v1.2.3 \
	-f canary_percent=10
```

## Promotion and rollback

-   Promote only if error rate/latency remain within SLO guardrails.
-   Roll back immediately on sustained SLO violations.

See rollback runbook: /developer/operations/deployment/rollback
