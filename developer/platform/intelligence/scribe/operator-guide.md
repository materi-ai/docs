---
title: Scribe MCP Operator Guide
description: "How to operate Scribe: architecture, daily checks, monitoring, and emergency procedures"
relatedPages: []
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Platform Engineering | **Review Cycle:** Monthly
</Info>

## Overview

Scribe is a Model-Context-Protocol (MCP) infrastructure that centralizes n8n workflow orchestration for Materi.

## System architecture

### High-level data flow

```
External trigger (webhook/cron)
  → n8n workflow execution
  → execution monitor
  → metrics + alerts + logs
```

## Daily operations

### Morning startup checklist

```bash
# 1) Check n8n health
curl -s https://hab.so1.io/api/v1/workflows \
  -H "X-N8N-API-KEY: $N8N_API_KEY" | jq '.length'

# 2) Verify cron jobs are installed
crontab -l | grep scribe | wc -l

# 3) Check recent executions
tail -20 /tmp/scribe_workflow_runs.log

# 4) Check disk space
df -h /tmp | awk 'NR==2 {print $5}'
```

### Scheduled execution times (UTC)

-   03:00 — Documentation publishing
-   04:00 — Cross-reference validation
-   12:00 — Performance SLA reconciliation
-   Every 60s — Metrics collection

### Manual workflow trigger

```bash
curl -X POST https://hab.so1.io/webhook/scribe/ingest \
  -H "Content-Type: application/json" \
  -d '{
    "csv_content": "id,name,description\\n1,REQ-001,Feature request\\n2,REQ-002,Bug fix"
  }'
```

## Monitoring & alerts

### Key metrics

| Metric                                       | Expected | Action if violated |
| -------------------------------------------- | -------- | ------------------ |
| `scribe_workflow_active`                     | 4        | Check n8n if <4    |
| `scribe_workflow_execution_duration_seconds` | <600s    | Investigate logs   |
| `scribe_workflow_errors_total`               | 0 new    | Review alerts      |

## Troubleshooting

See: /developer/platform/intelligence/scribe/troubleshooting

## Emergency procedures

-   If a workflow is stuck: identify the execution ID, retrieve logs, and stop/kill via the n8n UI or API.
-   Document the incident and follow your incident response process.
