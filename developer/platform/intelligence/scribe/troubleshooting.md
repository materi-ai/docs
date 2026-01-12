---
title: Scribe MCP Troubleshooting
description: "Diagnose and resolve common Scribe failures: alerts, cron, n8n connectivity, and metrics"
relatedPages: []
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Platform Engineering | **Review Cycle:** Monthly
</Info>

## Triage

If you only have 2 minutes:

1. Identify the symptom (alerts missing, workflow not running, workflow failing, metrics stopped).
2. Confirm n8n connectivity.
3. Pull the last 20 lines of execution logs for the failing execution.

## Common failure modes

### No Discord alerts

**Signals**

-   Workflows run successfully but no Discord notifications arrive.

**Diagnosis**

```bash
source .env.n8n
echo "$DISCORD_WEBHOOK_URL"

curl -X POST "$DISCORD_WEBHOOK_URL" \
  -H "Content-Type: application/json" \
  -d '{"embeds":[{"title":"Test","description":"Webhook test","color":3066993}]}'
```

**Mitigation**

-   Rotate the webhook URL and update `.env.n8n`.

### Workflow didn’t run on schedule

**Signals**

-   No new entries in `/tmp/scribe_workflow_runs.log`.

**Diagnosis**

```bash
crontab -l | grep scribe
date -u
```

**Mitigation**

-   Reinstall the crontab from the Scribe repo’s cron file if needed.

### Workflow failed

**Diagnosis**

```bash
curl -s https://hab.so1.io/api/v1/executions/$EXEC_ID \
  -H "X-N8N-API-KEY: $N8N_API_KEY" | jq '{status, startedAt, completedAt, error}'

curl -s https://hab.so1.io/api/v1/executions/$EXEC_ID/logs \
  -H "X-N8N-API-KEY: $N8N_API_KEY" | jq '.logs[-20:]'
```

**Mitigation**

-   Classify: dependency outage vs config error vs logic error.
-   Apply smallest safe fix, then retry.

## Escalation

-   Escalate to the Platform/Operations owner if the issue is cross-cutting (n8n outage, credentials, infra).
-   Use your incident process for SEV-1/SEV-2 symptoms.
