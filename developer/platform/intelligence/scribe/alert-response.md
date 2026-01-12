---
title: Scribe Alert Response
description: How to respond to Scribe alerts (success, failure, timeout) with SLAs and playbooks
relatedPages: []
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Operations / On-call | **Review Cycle:** Monthly
</Info>

## Signals

Scribe emits Discord alerts for workflow events. Each alert includes an execution ID you can use to pull logs.

## Triage

-   If **failure**: treat as P2 unless impact indicates higher severity.
-   If **timeout**: investigate quickly; may indicate backlog or stuck workflow.
-   If **success**: acknowledge; no action unless anomalous.

## Playbooks

### Workflow success

**Action**: Acknowledge in the channel; no remediation.

### Workflow failure

**Response target**: 15 minutes (P2)

**Diagnosis**

```bash
curl -s https://hab.so1.io/api/v1/executions/$EXEC_ID/logs \
  -H "X-N8N-API-KEY: $N8N_API_KEY" | jq '.logs[-20:]'
```

**Mitigation**

-   External dependency failure → confirm upstream status, retry when stable.
-   Configuration error → correct config/credentials, redeploy workflow.
-   Logic error / data format error → capture failing input, patch workflow/code.

### Workflow timeout

**Response target**: 10 minutes (P2)

**Diagnosis**

```bash
curl -s https://hab.so1.io/api/v1/executions/$EXEC_ID \
  -H "X-N8N-API-KEY: $N8N_API_KEY" | jq '{status, startedAt, completedAt}'
```

**Mitigation**

-   If stuck: terminate execution via n8n UI/API.
-   If slow: adjust timeout or optimize data processing.

## Post-incident updates

-   Record workflow name + execution ID, root cause, remediation, and time-to-recover.
-   If the issue recurs, update: /developer/platform/intelligence/scribe/troubleshooting
