---
title: Scribe (MCP)
description: Operational workflow orchestration for specs validation, publishing, and SLA reconciliation
relatedPages:
  - developer/products/canvas/architecture.md
  - developer/introduction/architecture.md
  - developer/platform/intelligence/scribe/architecture.mdx
  - developer/domain/shield/authentication.md
  - developer/domain/shield/database-schema.mdx
---

<Info>
**SDD Classification:** L3-Technical | **Authority:** Platform Engineering | **Review Cycle:** Quarterly
</Info>

Scribe is Materi’s orchestration system for running operational workflows (n8n) with consistent monitoring, alerts, and runbooks.

## What Scribe does

Scribe provides:

-   Automated workflow execution (scheduled + on-demand)
-   Execution monitoring and log retrieval
-   Metrics export for Grafana/Prometheus
-   Alerting to team channels

## Where the source of truth lives

-   Code: `platform/intelligence/.scribe`
-   Workflow runtime: n8n instance (see Operator Guide)

## Guides

-   Operator guide: /developer/platform/intelligence/scribe/operator-guide
-   Troubleshooting: /developer/platform/intelligence/scribe/troubleshooting
-   Alert response: /developer/platform/intelligence/scribe/alert-response
-   Team training: /developer/platform/intelligence/scribe/team-training
