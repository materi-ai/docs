---
title: "Stub Rewrite Plan"
description: "Documentation"
icon: "file"
source: "[consolidated]"
sourceRepo: "https://github.com/materi-ai/materi"
lastMigrated: "2026-01-09T16:00:00Z"
status: "migrated"
tags: []
relatedPages:
  []
---

# Stub Rewrite Plan

Generated from the stub audit. This plan defines grouping and minimum section templates to eliminate placeholders while staying aligned with Atlas IA and the Documentation Contract.

## Inputs
- Audit source: `platform/atlas/docs/STUB_AUDIT.txt`
- Manifest: `platform/atlas/docs/stub-manifest.json`

## Totals
- Total stub markers: **298**

### By audience
- internal: 97
- enterprise: 75
- developer: 65
- api: 61

### By planned doc type
- guide: 173
- reference: 61
- runbook: 30
- overview: 22
- adr: 8
- concept: 4

## Minimum templates (required sections)
### overview
- Purpose
- Who it’s for
- How it fits

### concept
- Problem / motivation
- Core model
- Failure modes
- Related references

### guide
- Prerequisites
- Steps
- Expected outcomes
- Troubleshooting

### reference
- Canonical source link
- Versioning policy

### runbook
- Signals
- Playbooks
- Rollback / recovery
- Escalation

### adr
- Context
- Decision
- Consequences
- Alternatives considered

## Recommended execution order
1. API reference pages (spec-linked; avoid hand-written request/response duplication).
2. Internal architecture ADR/SDD pages (tight scope; link to code/specs).
3. Internal operations runbooks (signals/playbooks/escalation; point to dashboards).
4. Enterprise guides (deployment/security/integrations; prereqs + validation checks).

## File inventory (grouped)
### api
- **api/rest** (42)
  - platform/atlas/api/rest/ai/analyze-code.md (→ reference)
  - platform/atlas/api/rest/ai/analyze-diagram.md (→ reference)
  - platform/atlas/api/rest/ai/analyze-text.md (→ reference)
  - platform/atlas/api/rest/ai/enhance-content.md (→ reference)
  - platform/atlas/api/rest/ai/generate-content.md (→ reference)
  - platform/atlas/api/rest/ai/summarize.md (→ reference)
  - platform/atlas/api/rest/auth/login.md (→ reference)
  - platform/atlas/api/rest/auth/logout.md (→ reference)
  - platform/atlas/api/rest/auth/oauth.md (→ reference)
  - platform/atlas/api/rest/auth/refresh-token.md (→ reference)
  - platform/atlas/api/rest/auth/saml.md (→ reference)
  - platform/atlas/api/rest/collaboration/comments.md (→ reference)
  - platform/atlas/api/rest/collaboration/get-presence.md (→ reference)
  - platform/atlas/api/rest/collaboration/get-session.md (→ reference)
  - platform/atlas/api/rest/collaboration/list-sessions.md (→ reference)
  - platform/atlas/api/rest/collaboration/mentions.md (→ reference)
  - platform/atlas/api/rest/documents/create-document.md (→ reference)
  - platform/atlas/api/rest/documents/delete-document.md (→ reference)
  - platform/atlas/api/rest/documents/document-permissions.md (→ reference)
  - platform/atlas/api/rest/documents/document-versions.md (→ reference)
  - platform/atlas/api/rest/documents/get-document.md (→ reference)
  - platform/atlas/api/rest/documents/list-documents.md (→ reference)
  - platform/atlas/api/rest/documents/update-document.md (→ reference)
  - platform/atlas/api/rest/files/delete-file.md (→ reference)
  - platform/atlas/api/rest/files/get-file.md (→ reference)
  - platform/atlas/api/rest/files/list-files.md (→ reference)
  - platform/atlas/api/rest/files/upload-file.md (→ reference)
  - platform/atlas/api/rest/search/search-documents.md (→ reference)
  - platform/atlas/api/rest/search/search-users.md (→ reference)
  - platform/atlas/api/rest/search/search-workspaces.md (→ reference)
  - … +12 more (see stub-manifest.json)
- **api/events** (8)
  - platform/atlas/api/events/aria-events.md (→ reference)
  - platform/atlas/api/events/audit-events.md (→ reference)
  - platform/atlas/api/events/collaboration-events.md (→ reference)
  - platform/atlas/api/events/document-events.md (→ reference)
  - platform/atlas/api/events/notification-events.md (→ reference)
  - platform/atlas/api/events/overview.md (→ reference)
  - platform/atlas/api/events/user-events.md (→ reference)
  - platform/atlas/api/events/workspace-events.md (→ reference)
- **api/sdks** (6)
  - platform/atlas/api/sdks/go.md (→ reference)
  - platform/atlas/api/sdks/java.md (→ reference)
  - platform/atlas/api/sdks/javascript-typescript.md (→ reference)
  - platform/atlas/api/sdks/overview.md (→ reference)
  - platform/atlas/api/sdks/python.md (→ reference)
  - platform/atlas/api/sdks/rust.md (→ reference)
- **api/graphql** (5)
  - platform/atlas/api/graphql/mutations.md (→ reference)
  - platform/atlas/api/graphql/overview.md (→ reference)
  - platform/atlas/api/graphql/queries.md (→ reference)
  - platform/atlas/api/graphql/schema.md (→ reference)
  - platform/atlas/api/graphql/subscriptions.md (→ reference)

### developer
- **developer/products** (17)
  - platform/atlas/developer/products/atlas/contributing.md (→ guide)
  - platform/atlas/developer/products/atlas/mdx-components.md (→ guide)
  - platform/atlas/developer/products/atlas/overview.md (→ guide)
  - platform/atlas/developer/products/atlas/structure.md (→ guide)
  - platform/atlas/developer/products/canvas/api-integration.md (→ guide)
  - platform/atlas/developer/products/canvas/component-library.md (→ guide)
  - platform/atlas/developer/products/canvas/deployment.md (→ guide)
  - platform/atlas/developer/products/canvas/overview.md (→ guide)
  - platform/atlas/developer/products/canvas/real-time-sync.md (→ guide)
  - platform/atlas/developer/products/canvas/setup.md (→ guide)
  - platform/atlas/developer/products/canvas/state-management.md (→ guide)
  - platform/atlas/developer/products/canvas/storybook.md (→ guide)
  - platform/atlas/developer/products/canvas/testing.md (→ guide)
  - platform/atlas/developer/products/specifications/overview.md (→ guide)
  - platform/atlas/developer/products/specifications/requirement-types.md (→ guide)
  - platform/atlas/developer/products/specifications/traceability.md (→ guide)
  - platform/atlas/developer/products/specifications/verification.md (→ guide)
- **developer/domain** (12)
  - platform/atlas/developer/domain/manuscript/code-generation.md (→ guide)
  - platform/atlas/developer/domain/manuscript/custom-options.md (→ guide)
  - platform/atlas/developer/domain/manuscript/event-schemas.md (→ guide)
  - platform/atlas/developer/domain/manuscript/msx-inspector.md (→ guide)
  - platform/atlas/developer/domain/manuscript/overview.md (→ guide)
  - platform/atlas/developer/domain/manuscript/versioning.md (→ guide)
  - platform/atlas/developer/domain/printery/architecture.md (→ guide)
  - platform/atlas/developer/domain/printery/consumer-groups.md (→ guide)
  - platform/atlas/developer/domain/printery/deployment.md (→ guide)
  - platform/atlas/developer/domain/printery/handler-registry.md (→ guide)
  - platform/atlas/developer/domain/printery/overview.md (→ guide)
  - platform/atlas/developer/domain/printery/resilience-patterns.md (→ guide)
- **developer/operations** (11)
  - platform/atlas/developer/operations/folio/dashboards.md (→ runbook)
  - platform/atlas/developer/operations/folio/logging.md (→ runbook)
  - platform/atlas/developer/operations/folio/metrics.md (→ runbook)
  - platform/atlas/developer/operations/folio/tracing.md (→ runbook)
  - platform/atlas/developer/operations/infrastructure/kubernetes.md (→ runbook)
  - platform/atlas/developer/operations/infrastructure/networking.md (→ runbook)
  - platform/atlas/developer/operations/infrastructure/overview.md (→ runbook)
  - platform/atlas/developer/operations/infrastructure/security.md (→ runbook)
  - platform/atlas/developer/operations/infrastructure/terraform.md (→ runbook)
  - platform/atlas/developer/operations/runbooks/backup-restore.md (→ runbook)
  - platform/atlas/developer/operations/runbooks/scaling.md (→ runbook)
- **developer/platform** (11)
  - platform/atlas/developer/platform/aria/architecture.md (→ guide)
  - platform/atlas/developer/platform/aria/code-safety.md (→ guide)
  - platform/atlas/developer/platform/aria/deployment.md (→ guide)
  - platform/atlas/developer/platform/aria/diagram-analysis.md (→ guide)
  - platform/atlas/developer/platform/aria/enhancement-suggestions.md (→ guide)
  - platform/atlas/developer/platform/aria/graceful-degradation.md (→ guide)
  - platform/atlas/developer/platform/aria/overview.md (→ guide)
  - platform/atlas/developer/platform/aria/testing.md (→ guide)
  - platform/atlas/developer/platform/aria/text-analysis.md (→ guide)
  - platform/atlas/developer/platform/intelligence/analytics.md (→ guide)
  - platform/atlas/developer/platform/intelligence/github-integrations.md (→ guide)
- **developer/events** (8)
  - platform/atlas/developer/events/consuming.md (→ guide)
  - platform/atlas/developer/events/dead-letter-queue.md (→ guide)
  - platform/atlas/developer/events/event-envelope.md (→ guide)
  - platform/atlas/developer/events/idempotency.md (→ guide)
  - platform/atlas/developer/events/overview.md (→ guide)
  - platform/atlas/developer/events/publishing.md (→ guide)
  - platform/atlas/developer/events/redis-streams.md (→ guide)
  - platform/atlas/developer/events/retry-patterns.md (→ guide)
- **developer/testing** (4)
  - platform/atlas/developer/testing/chaos-engineering.md (→ guide)
  - platform/atlas/developer/testing/contract-tests.md (→ guide)
  - platform/atlas/developer/testing/load-tests.md (→ guide)
  - platform/atlas/developer/testing/unit-tests.md (→ guide)
- **developer/introduction** (2)
  - platform/atlas/developer/introduction/getting-started.md (→ guide)
  - platform/atlas/developer/introduction/tech-stack.md (→ guide)

### enterprise
- **enterprise/deployment** (15)
  - platform/atlas/enterprise/deployment/cloud-dedicated/custom-domains.md (→ guide)
  - platform/atlas/enterprise/deployment/cloud-dedicated/monitoring.md (→ guide)
  - platform/atlas/enterprise/deployment/cloud-dedicated/overview.md (→ guide)
  - platform/atlas/enterprise/deployment/cloud-dedicated/provisioning.md (→ guide)
  - platform/atlas/enterprise/deployment/cloud-dedicated/vpc-peering.md (→ guide)
  - platform/atlas/enterprise/deployment/hybrid/architecture.md (→ guide)
  - platform/atlas/enterprise/deployment/hybrid/compliance.md (→ guide)
  - platform/atlas/enterprise/deployment/hybrid/data-residency.md (→ guide)
  - platform/atlas/enterprise/deployment/hybrid/overview.md (→ guide)
  - platform/atlas/enterprise/deployment/self-hosted/backup-restore.md (→ guide)
  - platform/atlas/enterprise/deployment/self-hosted/configuration.md (→ guide)
  - platform/atlas/enterprise/deployment/self-hosted/docker-compose.md (→ guide)
  - platform/atlas/enterprise/deployment/self-hosted/kubernetes.md (→ guide)
  - platform/atlas/enterprise/deployment/self-hosted/requirements.md (→ guide)
  - platform/atlas/enterprise/deployment/self-hosted/upgrading.md (→ guide)
- **enterprise/security** (11)
  - platform/atlas/enterprise/security/audit-logs.md (→ guide)
  - platform/atlas/enterprise/security/authentication.md (→ guide)
  - platform/atlas/enterprise/security/compliance-certifications.md (→ guide)
  - platform/atlas/enterprise/security/data-residency.md (→ guide)
  - platform/atlas/enterprise/security/encryption.md (→ guide)
  - platform/atlas/enterprise/security/network-security.md (→ guide)
  - platform/atlas/enterprise/security/overview.md (→ guide)
  - platform/atlas/enterprise/security/penetration-testing.md (→ guide)
  - platform/atlas/enterprise/security/scim-provisioning.md (→ guide)
  - platform/atlas/enterprise/security/sso-saml.md (→ guide)
  - platform/atlas/enterprise/security/vulnerability-disclosure.md (→ guide)
- **enterprise/integration** (8)
  - platform/atlas/enterprise/integration/api-gateway.md (→ guide)
  - platform/atlas/enterprise/integration/azure-ad.md (→ guide)
  - platform/atlas/enterprise/integration/custom-sso.md (→ guide)
  - platform/atlas/enterprise/integration/google-workspace.md (→ guide)
  - platform/atlas/enterprise/integration/ldap-ad.md (→ guide)
  - platform/atlas/enterprise/integration/okta.md (→ guide)
  - platform/atlas/enterprise/integration/overview.md (→ guide)
  - platform/atlas/enterprise/integration/webhooks.md (→ guide)
- **enterprise/admin** (7)
  - platform/atlas/enterprise/admin/billing.md (→ overview)
  - platform/atlas/enterprise/admin/license-management.md (→ overview)
  - platform/atlas/enterprise/admin/overview.md (→ overview)
  - platform/atlas/enterprise/admin/support.md (→ overview)
  - platform/atlas/enterprise/admin/usage-analytics.md (→ overview)
  - platform/atlas/enterprise/admin/user-management.md (→ overview)
  - platform/atlas/enterprise/admin/workspace-management.md (→ overview)
- **enterprise/monitoring** (7)
  - platform/atlas/enterprise/monitoring/alerting.md (→ guide)
  - platform/atlas/enterprise/monitoring/capacity-planning.md (→ guide)
  - platform/atlas/enterprise/monitoring/health-checks.md (→ guide)
  - platform/atlas/enterprise/monitoring/logging.md (→ guide)
  - platform/atlas/enterprise/monitoring/metrics.md (→ guide)
  - platform/atlas/enterprise/monitoring/overview.md (→ guide)
  - platform/atlas/enterprise/monitoring/performance-tuning.md (→ guide)
- **enterprise/ha** (6)
  - platform/atlas/enterprise/ha/architecture.md (→ guide)
  - platform/atlas/enterprise/ha/backup-strategy.md (→ guide)
  - platform/atlas/enterprise/ha/disaster-recovery.md (→ guide)
  - platform/atlas/enterprise/ha/failover.md (→ guide)
  - platform/atlas/enterprise/ha/load-balancing.md (→ guide)
  - platform/atlas/enterprise/ha/overview.md (→ guide)
- **enterprise/migration** (6)
  - platform/atlas/enterprise/migration/data-import.md (→ overview)
  - platform/atlas/enterprise/migration/from-cloud.md (→ overview)
  - platform/atlas/enterprise/migration/from-competitors.md (→ overview)
  - platform/atlas/enterprise/migration/overview.md (→ overview)
  - platform/atlas/enterprise/migration/user-migration.md (→ overview)
  - platform/atlas/enterprise/migration/validation.md (→ overview)
- **enterprise/scalability** (6)
  - platform/atlas/enterprise/scalability/cache-strategy.md (→ guide)
  - platform/atlas/enterprise/scalability/cdn-configuration.md (→ guide)
  - platform/atlas/enterprise/scalability/database-scaling.md (→ guide)
  - platform/atlas/enterprise/scalability/horizontal-scaling.md (→ guide)
  - platform/atlas/enterprise/scalability/overview.md (→ guide)
  - platform/atlas/enterprise/scalability/performance-benchmarks.md (→ guide)
- **enterprise/support** (6)
  - platform/atlas/enterprise/support/contact.md (→ overview)
  - platform/atlas/enterprise/support/escalation.md (→ overview)
  - platform/atlas/enterprise/support/overview.md (→ overview)
  - platform/atlas/enterprise/support/sla.md (→ overview)
  - platform/atlas/enterprise/support/support-tiers.md (→ overview)
  - platform/atlas/enterprise/support/training.md (→ overview)
- **enterprise/overview** (3)
  - platform/atlas/enterprise/overview/feature-comparison.md (→ overview)
  - platform/atlas/enterprise/overview/pricing.md (→ overview)
  - platform/atlas/enterprise/overview/sla.md (→ overview)

### internal
- **internal/engineering** (21)
  - platform/atlas/internal/engineering/ownership/domain-team.md (→ guide)
  - platform/atlas/internal/engineering/ownership/on-call-rotation.md (→ guide)
  - platform/atlas/internal/engineering/ownership/operations-team.md (→ guide)
  - platform/atlas/internal/engineering/ownership/overview.md (→ guide)
  - platform/atlas/internal/engineering/ownership/platform-team.md (→ guide)
  - platform/atlas/internal/engineering/ownership/product-team.md (→ guide)
  - platform/atlas/internal/engineering/performance/benchmarking.md (→ guide)
  - platform/atlas/internal/engineering/performance/optimization-guide.md (→ guide)
  - platform/atlas/internal/engineering/performance/profiling.md (→ guide)
  - platform/atlas/internal/engineering/standards/go-standards.md (→ guide)
  - platform/atlas/internal/engineering/standards/overview.md (→ guide)
  - platform/atlas/internal/engineering/standards/proto-standards.md (→ guide)
  - platform/atlas/internal/engineering/standards/python-standards.md (→ guide)
  - platform/atlas/internal/engineering/standards/rust-standards.md (→ guide)
  - platform/atlas/internal/engineering/standards/typescript-standards.md (→ guide)
  - platform/atlas/internal/engineering/workflow/code-review.md (→ guide)
  - platform/atlas/internal/engineering/workflow/deployment-process.md (→ guide)
  - platform/atlas/internal/engineering/workflow/issue-templates.md (→ guide)
  - platform/atlas/internal/engineering/workflow/overview.md (→ guide)
  - platform/atlas/internal/engineering/workflow/pr-process.md (→ guide)
  - platform/atlas/internal/engineering/workflow/testing-strategy.md (→ guide)
- **internal/operations** (19)
  - platform/atlas/internal/operations/capacity/forecasting.md (→ runbook)
  - platform/atlas/internal/operations/capacity/metrics.md (→ runbook)
  - platform/atlas/internal/operations/capacity/overview.md (→ runbook)
  - platform/atlas/internal/operations/capacity/scaling-strategy.md (→ runbook)
  - platform/atlas/internal/operations/cost/budgeting.md (→ runbook)
  - platform/atlas/internal/operations/cost/optimization.md (→ runbook)
  - platform/atlas/internal/operations/cost/overview.md (→ runbook)
  - platform/atlas/internal/operations/cost/tracking.md (→ runbook)
  - platform/atlas/internal/operations/incidents/blameless-culture.md (→ runbook)
  - platform/atlas/internal/operations/incidents/communication.md (→ runbook)
  - platform/atlas/internal/operations/incidents/overview.md (→ runbook)
  - platform/atlas/internal/operations/incidents/post-mortems.md (→ runbook)
  - platform/atlas/internal/operations/incidents/response-process.md (→ runbook)
  - platform/atlas/internal/operations/incidents/severity-levels.md (→ runbook)
  - platform/atlas/internal/operations/releases/change-management.md (→ runbook)
  - platform/atlas/internal/operations/releases/overview.md (→ runbook)
  - platform/atlas/internal/operations/releases/release-process.md (→ runbook)
  - platform/atlas/internal/operations/releases/rollback-procedures.md (→ runbook)
  - platform/atlas/internal/operations/releases/versioning.md (→ runbook)
- **internal/security** (15)
  - platform/atlas/internal/security/compliance/gdpr.md (→ guide)
  - platform/atlas/internal/security/compliance/hipaa.md (→ guide)
  - platform/atlas/internal/security/compliance/iso27001.md (→ guide)
  - platform/atlas/internal/security/compliance/overview.md (→ guide)
  - platform/atlas/internal/security/compliance/soc2.md (→ guide)
  - platform/atlas/internal/security/practices/code-review-security.md (→ guide)
  - platform/atlas/internal/security/practices/dependency-management.md (→ guide)
  - platform/atlas/internal/security/practices/secrets-management.md (→ guide)
  - platform/atlas/internal/security/practices/secure-coding.md (→ guide)
  - platform/atlas/internal/security/practices/threat-modeling.md (→ guide)
  - platform/atlas/internal/security/vulnerabilities/disclosure.md (→ guide)
  - platform/atlas/internal/security/vulnerabilities/overview.md (→ guide)
  - platform/atlas/internal/security/vulnerabilities/remediation.md (→ guide)
  - platform/atlas/internal/security/vulnerabilities/reporting.md (→ guide)
  - platform/atlas/internal/security/vulnerabilities/scanning.md (→ guide)
- **internal/architecture** (13)
  - platform/atlas/internal/architecture/adrs/adr-001-microservices-architecture.mdx (→ adr)
  - platform/atlas/internal/architecture/adrs/adr-002-event-driven-communication.md (→ adr)
  - platform/atlas/internal/architecture/adrs/adr-003-operational-transform.md (→ adr)
  - platform/atlas/internal/architecture/adrs/adr-004-protocol-buffers.md (→ adr)
  - platform/atlas/internal/architecture/adrs/adr-005-redis-streams.md (→ adr)
  - platform/atlas/internal/architecture/adrs/adr-006-polyglot-services.md (→ adr)
  - platform/atlas/internal/architecture/adrs/adr-007-graceful-degradation.md (→ adr)
  - platform/atlas/internal/architecture/adrs/overview.md (→ adr)
  - platform/atlas/internal/architecture/specs/requirement-traceability.md (→ guide)
  - platform/atlas/internal/architecture/system-design/domain-services.md (→ concept)
  - platform/atlas/internal/architecture/system-design/event-driven-architecture.md (→ concept)
  - platform/atlas/internal/architecture/system-design/operations-architecture.md (→ concept)
  - platform/atlas/internal/architecture/system-design/product-architecture.md (→ concept)
- **internal/product** (12)
  - platform/atlas/internal/product/development/discovery.md (→ guide)
  - platform/atlas/internal/product/development/execution.md (→ guide)
  - platform/atlas/internal/product/development/launch.md (→ guide)
  - platform/atlas/internal/product/development/planning.md (→ guide)
  - platform/atlas/internal/product/research/analytics.md (→ guide)
  - platform/atlas/internal/product/research/interviews.md (→ guide)
  - platform/atlas/internal/product/research/overview.md (→ guide)
  - platform/atlas/internal/product/research/surveys.md (→ guide)
  - platform/atlas/internal/product/research/user-personas.md (→ guide)
  - platform/atlas/internal/product/strategy/competitive-analysis.md (→ guide)
  - platform/atlas/internal/product/strategy/okrs.md (→ guide)
  - platform/atlas/internal/product/strategy/roadmap.md (→ guide)
- **internal/analytics** (5)
  - platform/atlas/internal/analytics/business-intelligence.md (→ guide)
  - platform/atlas/internal/analytics/metrics-dashboard.md (→ guide)
  - platform/atlas/internal/analytics/overview.md (→ guide)
  - platform/atlas/internal/analytics/system-analytics.md (→ guide)
  - platform/atlas/internal/analytics/user-analytics.md (→ guide)
- **internal/hr** (5)
  - platform/atlas/internal/hr/benefits.md (→ guide)
  - platform/atlas/internal/hr/compensation.md (→ guide)
  - platform/atlas/internal/hr/engineering-levels.md (→ guide)
  - platform/atlas/internal/hr/performance-reviews.md (→ guide)
  - platform/atlas/internal/hr/remote-work.md (→ guide)
- **internal/legal** (4)
  - platform/atlas/internal/legal/contracts.md (→ guide)
  - platform/atlas/internal/legal/intellectual-property.md (→ guide)
  - platform/atlas/internal/legal/privacy-policy.md (→ guide)
  - platform/atlas/internal/legal/terms-of-service.md (→ guide)
- **internal/finance** (2)
  - platform/atlas/internal/finance/budget.md (→ guide)
  - platform/atlas/internal/finance/expense-policy.md (→ guide)
- **internal/overview** (1)
  - platform/atlas/internal/overview/communication.md (→ guide)

