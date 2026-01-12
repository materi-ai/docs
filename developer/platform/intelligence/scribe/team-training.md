---
title: Scribe Team Training
description: A 45-minute self-paced onboarding for operating and troubleshooting Scribe workflows
relatedPages: []
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Platform Engineering | **Review Cycle:** Quarterly
</Info>

## Goal

Enable DevOps, QA, Documentation, and Platform engineers to operate, monitor, and troubleshoot Scribe.

## Module 1: System overview (10 minutes)

Scribe orchestrates four operational workflows:

1. Specification ingestion
2. Cross-reference validation
3. Documentation publishing
4. Performance SLA reconciliation

## Module 2: Daily monitoring (10 minutes)

### Quick health check

```bash
tail -10 /tmp/scribe_workflow_runs.log

curl -s https://hab.so1.io/api/v1/workflows \
  -H "X-N8N-API-KEY: $N8N_API_KEY" | jq '.[] | {name, active}'

ls -lh /tmp/scribe_metrics/metrics.txt
```

## Module 3: Manual triggers (10 minutes)

Use the webhook trigger for ingestion:

```bash
curl -X POST https://hab.so1.io/webhook/scribe/ingest \
  -H "Content-Type: application/json" \
  -d '{"csv_content":"id,name,description\\n1,REQ-001,Test requirement"}'
```

## Module 4: Troubleshooting (10 minutes)

See: /developer/platform/intelligence/scribe/troubleshooting

## Deliverables

-   Confirm you reviewed the Scribe overview and operator guide.
-   Perform a manual trigger in a safe environment (if available) and capture the execution ID.
-   Identify one improvement opportunity for reliability or observability.
