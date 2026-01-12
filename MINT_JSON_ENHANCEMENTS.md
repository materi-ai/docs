---
title: "mint.json Navigation Enhancements - TASKSET 2"
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

# mint.json Navigation Enhancements - TASKSET 2

**Purpose**: Document all navigation enhancements required in `mint.json` to accommodate 150-200 new pages from consolidated documentation

**Status**: Planning Complete - Ready for Implementation

---

## Overview

Current `mint.json` structure:
- 5 Primary Tabs (Getting Started, Developer Guide, API Reference, Enterprise, Internal)
- 45 Groups (across all tabs)
- 250+ Pages (current baseline)

Post-consolidation target:
- 5 Primary Tabs (unchanged)
- 50-55 Groups (5-10 new groups)
- 400-450 Pages (150-200 new pages)

---

## Tab 1: Getting Started (No Changes Needed)

**Current State**: 5 groups, ~15 pages
**Changes**: Minor additions only

```json
{
  "tab": "Getting Started",
  "groups": [
    {
      "group": "Introduction",
      "pages": [
        "index",
        "introduction",
        "quickstart",
        "concepts",
        "architecture-overview",
        "ADDED: platform-overview"  // NEW
      ]
    },
    {
      "group": "Platform Essentials",
      "pages": [
        "development",
        "documentation-contract",
        "ADDED: claude-development-guide"  // NEW
      ]
    },
    {
      "group": "MFlow Guides",
      "pages": [
        "mflow/01-mflow-architecture-guide",
        "mflow/02-environment-setup-railway",
        "mflow/03-folio-observability-guide",
        "mflow/04-forge-event-schema-reference",
        "mflow/05-development-workflow-resources"
      ]
    }
  ]
}
```

**New Pages**: +1 (platform-overview, claude-development-guide)

---

## Tab 2: Developer Guide (Major Enhancements)

**Current State**: 15 groups, ~150 pages
**Target**: 20-22 groups, ~220-240 pages
**New Pages**: +70-90

### 2.1 Architecture & Design (NEW GROUP)

```json
{
  "group": "Architecture & Design",
  "icon": "sitemap",
  "pages": [
    "developer/architecture/overview",
    "developer/architecture/system-design",
    "developer/architecture/design-patterns",
    "developer/architecture/decisions",
    "developer/architecture/tech-stack",
    "developer/architecture/adrs"
  ]
}
```

**Source**: Consolidated from `/domain/*/docs/`, `/CLAUDE.md`, architecture overview

### 2.2 Getting Started - ENHANCED

```json
{
  "group": "Getting Started",
  "icon": "code",
  "pages": [
    "developer/introduction/overview",
    "developer/introduction/architecture",
    "developer/introduction/tech-stack",
    "developer/introduction/getting-started",
    "ADDED: developer/introduction/quick-reference",
    "ADDED: developer/introduction/common-workflows"
  ]
}
```

**New Pages**: +2

### 2.3 Contributing - ENHANCED

```json
{
  "group": "Contributing",
  "icon": "code-pull-request",
  "pages": [
    "developer/contributing/overview",
    "developer/contributing/code-standards",
    "developer/contributing/git-workflow",
    "developer/contributing/pr-process",
    "developer/contributing/issue-templates",
    "developer/contributing/documentation",
    "ADDED: developer/contributing/debugging-guide",
    "ADDED: developer/contributing/development-setup"
  ]
}
```

**New Pages**: +2

### 2.4 Domain Services - API - ENHANCED

```json
{
  "group": "Domain Services - API",
  "icon": "server",
  "pages": [
    "developer/domain/api/overview",
    "developer/domain/api/architecture",
    "developer/domain/api/setup",
    "developer/domain/api/endpoints",
    "developer/domain/api/authentication",
    "developer/domain/api/rate-limiting",
    "developer/domain/api/testing",
    "developer/domain/api/deployment",
    "ADDED: developer/domain/api/troubleshooting",
    "ADDED: developer/domain/api/performance-tuning"
  ]
}
```

**New Pages**: +2

### 2.5 Domain Services - Relay - ENHANCED

```json
{
  "group": "Domain Services - Relay",
  "icon": "bolt",
  "pages": [
    "developer/domain/relay/overview",
    "developer/domain/relay/architecture",
    "developer/domain/relay/operational-transform",
    "developer/domain/relay/websocket-protocol",
    "developer/domain/relay/presence-tracking",
    "developer/domain/relay/testing",
    "developer/domain/relay/deployment",
    "ADDED: developer/domain/relay/debugging",
    "ADDED: developer/domain/relay/performance-optimization"
  ]
}
```

**New Pages**: +2

### 2.6 Domain Services - Shield - ENHANCED

```json
{
  "group": "Domain Services - Shield",
  "icon": "shield",
  "pages": [
    "developer/domain/shield/overview",
    "developer/domain/shield/architecture",
    "developer/domain/shield/authentication",
    "developer/domain/shield/authorization",
    "developer/domain/shield/user-management",
    "developer/domain/shield/oauth-saml",
    "developer/domain/shield/testing",
    "developer/domain/shield/deployment",
    "ADDED: developer/domain/shield/security-hardening",
    "ADDED: developer/domain/shield/troubleshooting"
  ]
}
```

**New Pages**: +2

### 2.7 Domain Services - Manuscript - ENHANCED

```json
{
  "group": "Domain Services - Manuscript",
  "icon": "scroll",
  "pages": [
    "developer/domain/manuscript/overview",
    "developer/domain/manuscript/event-schemas",
    "developer/domain/manuscript/custom-options",
    "developer/domain/manuscript/code-generation",
    "developer/domain/manuscript/versioning",
    "developer/domain/manuscript/msx-inspector",
    "ADDED: developer/domain/manuscript/examples",
    "ADDED: developer/domain/manuscript/troubleshooting"
  ]
}
```

**New Pages**: +2

### 2.8 Domain Services - Printery - ENHANCED

```json
{
  "group": "Domain Services - Printery",
  "icon": "print",
  "pages": [
    "developer/domain/printery/overview",
    "developer/domain/printery/architecture",
    "developer/domain/printery/consumer-groups",
    "developer/domain/printery/handler-registry",
    "developer/domain/printery/resilience-patterns",
    "developer/domain/printery/deployment",
    "ADDED: developer/domain/printery/examples",
    "ADDED: developer/domain/printery/troubleshooting"
  ]
}
```

**New Pages**: +2

### 2.9 Platform Services - Aria AI - ENHANCED

```json
{
  "group": "Platform Services - Aria AI",
  "icon": "brain",
  "pages": [
    "developer/platform/aria/overview",
    "developer/platform/aria/architecture",
    "developer/platform/aria/text-analysis",
    "developer/platform/aria/code-safety",
    "developer/platform/aria/diagram-analysis",
    "developer/platform/aria/enhancement-suggestions",
    "developer/platform/aria/model-integration",
    "developer/platform/aria/graceful-degradation",
    "developer/platform/aria/testing",
    "developer/platform/aria/deployment",
    "ADDED: developer/platform/aria/custom-models",
    "ADDED: developer/platform/aria/performance-metrics"
  ]
}
```

**New Pages**: +2

### 2.10 Platform Services - Intelligence - ENHANCED

```json
{
  "group": "Platform Services - Intelligence",
  "icon": "lightbulb",
  "pages": [
    "developer/platform/intelligence/scribe",
    "developer/platform/intelligence/scribe/overview",
    "developer/platform/intelligence/scribe/operator-guide",
    "developer/platform/intelligence/scribe/troubleshooting",
    "developer/platform/intelligence/scribe/alert-response",
    "developer/platform/intelligence/scribe/team-training",
    "developer/platform/intelligence/scribe/deployment",
    "developer/platform/intelligence/github-integrations",
    "developer/platform/intelligence/analytics",
    "developer/platform/intelligence/MATERI_POSTHOG_CONVEX_UNFAIR_ADVANTAGES",
    "ADDED: developer/platform/intelligence/operational-runbooks"
  ]
}
```

**New Pages**: +1

### 2.11 Products - Canvas - ENHANCED

```json
{
  "group": "Products - Canvas",
  "icon": "palette",
  "pages": [
    "developer/products/canvas/overview",
    "developer/products/canvas/architecture",
    "developer/products/canvas/setup",
    "developer/products/canvas/component-library",
    "developer/products/canvas/state-management",
    "developer/products/canvas/api-integration",
    "developer/products/canvas/real-time-sync",
    "developer/products/canvas/testing",
    "developer/products/canvas/storybook",
    "developer/products/canvas/deployment",
    "ADDED: developer/products/canvas/examples",
    "ADDED: developer/products/canvas/troubleshooting"
  ]
}
```

**New Pages**: +2

### 2.12 Products - Atlas Docs - UNCHANGED

(Already comprehensive)

### 2.13 Products - Specifications - ENHANCED

```json
{
  "group": "Products - Specifications",
  "icon": "clipboard-check",
  "pages": [
    "developer/products/specifications/overview",
    "developer/products/specifications/traceability",
    "developer/products/specifications/requirement-types",
    "developer/products/specifications/verification",
    "ADDED: developer/products/specifications/requirement-index",
    "ADDED: developer/products/specifications/examples"
  ]
}
```

**New Pages**: +2

### 2.14 Event-Driven Architecture - ENHANCED

```json
{
  "group": "Event-Driven Architecture",
  "icon": "diagram-project",
  "pages": [
    "developer/events/overview",
    "developer/events/event-envelope",
    "developer/events/redis-streams",
    "developer/events/publishing",
    "developer/events/consuming",
    "developer/events/idempotency",
    "developer/events/retry-patterns",
    "developer/events/dead-letter-queue",
    "ADDED: developer/events/examples",
    "ADDED: developer/events/troubleshooting"
  ]
}
```

**New Pages**: +2

### 2.15 Testing - ENHANCED

```json
{
  "group": "Testing",
  "icon": "vial",
  "pages": [
    "developer/testing/overview",
    "developer/testing/unit-tests",
    "developer/testing/integration-tests",
    "developer/testing/e2e-tests",
    "developer/testing/load-tests",
    "developer/testing/contract-tests",
    "developer/testing/chaos-engineering",
    "ADDED: developer/testing/service-specific-tests",
    "ADDED: developer/testing/test-patterns"
  ]
}
```

**New Pages**: +2

### 2.16 Operations - Infrastructure - ENHANCED

```json
{
  "group": "Operations - Infrastructure",
  "icon": "cloud",
  "pages": [
    "developer/operations/infrastructure/overview",
    "developer/operations/infrastructure/terraform",
    "developer/operations/infrastructure/kubernetes",
    "developer/operations/infrastructure/networking",
    "developer/operations/infrastructure/security",
    "ADDED: developer/operations/infrastructure/gitops",
    "ADDED: developer/operations/infrastructure/capacity-planning",
    "ADDED: developer/operations/infrastructure/cost-optimization"
  ]
}
```

**New Pages**: +3

### 2.17 Operations - Deployment - ENHANCED

```json
{
  "group": "Operations - Deployment",
  "icon": "rocket",
  "pages": [
    "developer/operations/deployment/overview",
    "developer/operations/deployment/gitops",
    "developer/operations/deployment/ci-cd",
    "developer/operations/deployment/ci-cd-runbook",
    "developer/operations/deployment/deployment-runbook",
    "developer/operations/deployment/blue-green",
    "developer/operations/deployment/rollback",
    "ADDED: developer/operations/deployment/service-specific-deployment",
    "ADDED: developer/operations/deployment/performance-tuning",
    "ADDED: developer/operations/deployment/troubleshooting"
  ]
}
```

**New Pages**: +3

### 2.18 Operations - Folio Observability - ENHANCED

```json
{
  "group": "Operations - Folio Observability",
  "icon": "chart-line",
  "pages": [
    "developer/operations/folio/overview",
    "developer/operations/folio/metrics",
    "developer/operations/folio/logging",
    "developer/operations/folio/tracing",
    "developer/operations/folio/alerting",
    "developer/operations/folio/dashboards",
    "developer/operations/folio/grafana-runbook",
    "ADDED: developer/operations/folio/prometheus-setup",
    "ADDED: developer/operations/folio/alert-response-procedures",
    "ADDED: developer/operations/folio/slo-sli-sla",
    "ADDED: developer/operations/folio/performance-baseline"
  ]
}
```

**New Pages**: +4

### 2.19 Operations - Runbooks - ENHANCED

```json
{
  "group": "Operations - Runbooks",
  "icon": "book",
  "pages": [
    "developer/operations/runbooks/incident-response",
    "developer/operations/runbooks/scaling",
    "developer/operations/runbooks/backup-restore",
    "developer/operations/runbooks/disaster-recovery",
    "ADDED: developer/operations/runbooks/common-issues",
    "ADDED: developer/operations/runbooks/service-specific-runbooks"
  ]
}
```

**New Pages**: +2

### 2.20 Developer Recipes - ENHANCED

```json
{
  "group": "Developer Recipes",
  "icon": "flask",
  "pages": [
    "developer/recipes/verify-webhook-signature",
    "developer/recipes/test-shield-github-actions-webhook",
    "developer/recipes/send-pipeline-metrics-webhook",
    "ADDED: developer/recipes/service-integration-patterns",
    "ADDED: developer/recipes/event-patterns",
    "ADDED: developer/recipes/deployment-patterns"
  ]
}
```

**New Pages**: +3

**Tab 2 Summary**:
- Current: 15 groups, ~150 pages
- New: 22 groups, ~240 pages
- **Additions**: +7 groups, +90 pages

---

## Tab 3: API Reference (Minor Additions)

**Current State**: 11 groups, ~60 pages
**Target**: 12-13 groups, ~75-80 pages
**New Pages**: +15-20

### 3.1 Introduction - ENHANCED

```json
{
  "group": "Introduction",
  "icon": "circle-info",
  "pages": [
    "api/introduction/overview",
    "api/introduction/authentication",
    "api/introduction/rate-limits",
    "api/introduction/errors",
    "api/introduction/pagination",
    "api/introduction/versioning",
    "api/introduction/webhooks",
    "ADDED: api/introduction/sdks-guide",
    "ADDED: api/introduction/best-practices"
  ]
}
```

**New Pages**: +2

### 3.2 REST API - Additional Endpoints - NEW GROUP

```json
{
  "group": "REST API - Additional Endpoints",
  "icon": "plug",
  "pages": [
    "api/rest/notifications/overview",
    "api/rest/audit/overview",
    "api/rest/analytics/overview"
  ]
}
```

**New Pages**: +3

### 3.3 Event Schemas - ENHANCED

```json
{
  "group": "Event Schemas",
  "icon": "diagram-project",
  "pages": [
    "api/events/overview",
    "api/events/user-events",
    "api/events/document-events",
    "api/events/collaboration-events",
    "api/events/workspace-events",
    "api/events/aria-events",
    "api/events/notification-events",
    "api/events/audit-events",
    "ADDED: api/events/event-examples",
    "ADDED: api/events/publishing-guide",
    "ADDED: api/events/consuming-guide"
  ]
}
```

**New Pages**: +3

**Tab 3 Summary**:
- Current: 11 groups, ~60 pages
- New: 13 groups, ~78 pages
- **Additions**: +2 groups, +18 pages

---

## Tab 4: Enterprise (Major Enhancements)

**Current State**: 12 groups, ~70 pages
**Target**: 14-15 groups, ~100-110 pages
**New Pages**: +30-40

### 4.1 Security & Compliance - ENHANCED

```json
{
  "group": "Security & Compliance",
  "icon": "shield-halved",
  "pages": [
    "enterprise/security/overview",
    "enterprise/security/authentication",
    "enterprise/security/sso-saml",
    "enterprise/security/scim-provisioning",
    "enterprise/security/encryption",
    "enterprise/security/network-security",
    "enterprise/security/audit-logs",
    "enterprise/security/data-residency",
    "enterprise/security/compliance-certifications",
    "enterprise/security/penetration-testing",
    "enterprise/security/vulnerability-disclosure",
    "ADDED: enterprise/security/security-audit-checklist",
    "ADDED: enterprise/security/threat-modeling",
    "ADDED: enterprise/security/incident-response-security"
  ]
}
```

**New Pages**: +3

### 4.2 Monitoring & Operations - ENHANCED

```json
{
  "group": "Monitoring & Operations",
  "icon": "chart-line",
  "pages": [
    "enterprise/monitoring/overview",
    "enterprise/monitoring/health-checks",
    "enterprise/monitoring/metrics",
    "enterprise/monitoring/logging",
    "enterprise/monitoring/alerting",
    "enterprise/monitoring/performance-tuning",
    "enterprise/monitoring/capacity-planning",
    "ADDED: enterprise/monitoring/alert-runbooks",
    "ADDED: enterprise/monitoring/performance-baseline",
    "ADDED: enterprise/monitoring/cost-optimization"
  ]
}
```

**New Pages**: +3

### 4.3 High Availability - ENHANCED

```json
{
  "group": "High Availability",
  "icon": "database",
  "pages": [
    "enterprise/ha/overview",
    "enterprise/ha/architecture",
    "enterprise/ha/failover",
    "enterprise/ha/load-balancing",
    "enterprise/ha/disaster-recovery",
    "enterprise/ha/backup-strategy",
    "ADDED: enterprise/ha/recovery-procedures",
    "ADDED: enterprise/ha/rto-rpo-targets"
  ]
}
```

**New Pages**: +2

### 4.4 Scalability - ENHANCED

```json
{
  "group": "Scalability",
  "icon": "arrows-up-down",
  "pages": [
    "enterprise/scalability/overview",
    "enterprise/scalability/horizontal-scaling",
    "enterprise/scalability/database-scaling",
    "enterprise/scalability/cache-strategy",
    "enterprise/scalability/cdn-configuration",
    "enterprise/scalability/performance-benchmarks",
    "ADDED: enterprise/scalability/bottleneck-identification",
    "ADDED: enterprise/scalability/tuning-guide"
  ]
}
```

**New Pages**: +2

### 4.5 Migration - ENHANCED

```json
{
  "group": "Migration",
  "icon": "truck-moving",
  "pages": [
    "enterprise/migration/overview",
    "enterprise/migration/from-cloud",
    "enterprise/migration/from-competitors",
    "enterprise/migration/data-import",
    "enterprise/migration/user-migration",
    "enterprise/migration/validation",
    "ADDED: enterprise/migration/auth-migration"
  ]
}
```

**New Pages**: +1

### 4.6 Enterprise Support - ENHANCED

```json
{
  "group": "Enterprise Support",
  "icon": "life-ring",
  "pages": [
    "enterprise/support/overview",
    "enterprise/support/support-tiers",
    "enterprise/support/contact",
    "enterprise/support/sla",
    "enterprise/support/escalation",
    "enterprise/support/training",
    "ADDED: enterprise/support/incident-communication",
    "ADDED: enterprise/support/knowledge-base"
  ]
}
```

**New Pages**: +2

### 4.7 Deployment - ENHANCED

```json
{
  "group": "Self-Hosted Deployment",
  "icon": "server",
  "pages": [
    "enterprise/deployment/self-hosted/overview",
    "enterprise/deployment/self-hosted/requirements",
    "enterprise/deployment/self-hosted/kubernetes",
    "enterprise/deployment/self-hosted/docker-compose",
    "enterprise/deployment/self-hosted/configuration",
    "enterprise/deployment/self-hosted/upgrading",
    "enterprise/deployment/self-hosted/backup-restore",
    "ADDED: enterprise/deployment/self-hosted/troubleshooting",
    "ADDED: enterprise/deployment/self-hosted/post-deployment"
  ]
}
```

**New Pages**: +2

**Tab 4 Summary**:
- Current: 12 groups, ~70 pages
- New: 14 groups, ~105 pages
- **Additions**: +2 groups, +35 pages

---

## Tab 5: Internal (Staff) (Major Enhancements)

**Current State**: 14 groups, ~80 pages
**Target**: 18-20 groups, ~130-140 pages
**New Pages**: +50-60

### 5.1 System Architecture - ENHANCED

```json
{
  "group": "System Architecture",
  "icon": "sitemap",
  "pages": [
    "internal/architecture/system-design/overview",
    "internal/architecture/system-design/domain-services",
    "internal/architecture/system-design/platform-services",
    "internal/architecture/system-design/product-architecture",
    "internal/architecture/system-design/operations-architecture",
    "internal/architecture/system-design/cross-segment-integration",
    "internal/architecture/system-design/event-driven-architecture",
    "internal/architecture/system-design/data-models",
    "internal/architecture/system-design/office-frontend",
    "ADDED: internal/architecture/system-design/service-interaction-map",
    "ADDED: internal/architecture/system-design/deployment-topology"
  ]
}
```

**New Pages**: +2

### 5.2 Technical Specifications - ENHANCED

```json
{
  "group": "Technical Specifications",
  "icon": "file-contract",
  "pages": [
    "internal/architecture/specs/concept-of-operations",
    "internal/architecture/specs/technology-strategy",
    "internal/architecture/specs/taxonomy-index",
    "internal/architecture/specs/l1-strategic-specs",
    "internal/architecture/specs/l2-tactical-specs",
    "internal/architecture/specs/requirement-traceability",
    "internal/architecture/specs/verification-matrix",
    "ADDED: internal/architecture/specs/requirements-index",
    "ADDED: internal/architecture/specs/specification-standards"
  ]
}
```

**New Pages**: +2

### 5.3 Engineering Workflow - ENHANCED

```json
{
  "group": "Engineering Workflow",
  "icon": "code-branch",
  "pages": [
    "internal/engineering/workflow/overview",
    "internal/engineering/workflow/issue-templates",
    "internal/engineering/workflow/pr-process",
    "internal/engineering/workflow/code-review",
    "internal/engineering/workflow/testing-strategy",
    "internal/engineering/workflow/deployment-process",
    "ADDED: internal/engineering/workflow/local-development-setup",
    "ADDED: internal/engineering/workflow/debugging-strategies"
  ]
}
```

**New Pages**: +2

### 5.4 Engineering Standards - ENHANCED

```json
{
  "group": "Engineering Standards",
  "icon": "check-double",
  "pages": [
    "internal/engineering/standards/overview",
    "internal/engineering/standards/go-standards",
    "internal/engineering/standards/rust-standards",
    "internal/engineering/standards/python-standards",
    "internal/engineering/standards/typescript-standards",
    "internal/engineering/standards/proto-standards",
    "ADDED: internal/engineering/standards/documentation-standards",
    "ADDED: internal/engineering/standards/testing-standards",
    "ADDED: internal/engineering/standards/security-standards"
  ]
}
```

**New Pages**: +3

### 5.5 Team Ownership - UNCHANGED

(Adequate coverage)

### 5.6 Performance Engineering - ENHANCED

```json
{
  "group": "Performance Engineering",
  "icon": "gauge-high",
  "pages": [
    "internal/engineering/performance/overview",
    "internal/engineering/performance/benchmarking",
    "internal/engineering/performance/profiling",
    "internal/engineering/performance/optimization-guide",
    "internal/engineering/performance/slo-sli-sla",
    "ADDED: internal/engineering/performance/baseline-metrics",
    "ADDED: internal/engineering/performance/performance-regression-testing"
  ]
}
```

**New Pages**: +2

### 5.7 Incident Management - ENHANCED

```json
{
  "group": "Incident Management",
  "icon": "triangle-exclamation",
  "pages": [
    "internal/operations/incidents/overview",
    "internal/operations/incidents/severity-levels",
    "internal/operations/incidents/response-process",
    "internal/operations/incidents/communication",
    "internal/operations/incidents/post-mortems",
    "internal/operations/incidents/blameless-culture",
    "ADDED: internal/operations/incidents/runbooks-by-service",
    "ADDED: internal/operations/incidents/escalation-procedures"
  ]
}
```

**New Pages**: +2

### 5.8 Release Management - ENHANCED

```json
{
  "group": "Release Management",
  "icon": "tags",
  "pages": [
    "internal/operations/releases/overview",
    "internal/operations/releases/versioning",
    "internal/operations/releases/release-process",
    "internal/operations/releases/rollback-procedures",
    "internal/operations/releases/change-management",
    "ADDED: internal/operations/releases/release-checklist",
    "ADDED: internal/operations/releases/post-release-validation"
  ]
}
```

**New Pages**: +2

### 5.9 Capacity Planning - ENHANCED

```json
{
  "group": "Capacity Planning",
  "icon": "chart-pie",
  "pages": [
    "internal/operations/capacity/overview",
    "internal/operations/capacity/metrics",
    "internal/operations/capacity/forecasting",
    "internal/operations/capacity/scaling-strategy",
    "ADDED: internal/operations/capacity/resource-allocation",
    "ADDED: internal/operations/capacity/cost-forecasting"
  ]
}
```

**New Pages**: +2

### 5.10 Cost Management - ENHANCED

```json
{
  "group": "Cost Management",
  "icon": "dollar-sign",
  "pages": [
    "internal/operations/cost/overview",
    "internal/operations/cost/tracking",
    "internal/operations/cost/optimization",
    "internal/operations/cost/budgeting",
    "ADDED: internal/operations/cost/cost-allocation",
    "ADDED: internal/operations/cost/rightsizing-guide"
  ]
}
```

**New Pages**: +2

### 5.11 Product Strategy - ENHANCED

```json
{
  "group": "Product Strategy",
  "icon": "chess",
  "pages": [
    "internal/product/strategy/vision",
    "internal/product/strategy/roadmap",
    "internal/product/strategy/okrs",
    "internal/product/strategy/competitive-analysis",
    "ADDED: internal/product/strategy/market-research",
    "ADDED: internal/product/strategy/product-requirements"
  ]
}
```

**New Pages**: +2

### 5.12 Product Development - UNCHANGED

(Adequate coverage)

### 5.13 User Research - ENHANCED

```json
{
  "group": "User Research",
  "icon": "user-magnifying-glass",
  "pages": [
    "internal/product/research/overview",
    "internal/product/research/user-personas",
    "internal/product/research/interviews",
    "internal/product/research/surveys",
    "internal/product/research/analytics",
    "ADDED: internal/product/research/user-feedback",
    "ADDED: internal/product/research/usability-testing"
  ]
}
```

**New Pages**: +2

### 5.14 Data & Analytics - NEW GROUP

```json
{
  "group": "Data & Analytics",
  "icon": "chart-bar",
  "pages": [
    "internal/analytics/overview",
    "internal/analytics/metrics-dashboard",
    "internal/analytics/user-analytics",
    "internal/analytics/system-analytics",
    "internal/analytics/business-intelligence",
    "ADDED: internal/analytics/event-analytics",
    "ADDED: internal/analytics/performance-analytics"
  ]
}
```

**New Pages**: +2

### 5.15 Documentation & Knowledge - NEW GROUP

```json
{
  "group": "Documentation & Knowledge",
  "icon": "book-open",
  "pages": [
    "internal/documentation/consolidation-roadmap",
    "internal/documentation/scribe-completion-report",
    "internal/documentation/documentation-standards",
    "internal/documentation/taskset-archive"
  ]
}
```

**New Pages**: +4 (NEW GROUP)

**Tab 5 Summary**:
- Current: 14 groups, ~80 pages
- New: 16 groups, ~135 pages
- **Additions**: +2 groups, +55 pages

---

## Summary of All Changes

| Tab | Current Groups | Current Pages | New Groups | New Pages | Total Change |
|-----|----------------|---------------|-----------|-----------|--------------|
| Getting Started | 3 | ~15 | 0 | +2 | +2 pages |
| Developer Guide | 15 | ~150 | +7 | +90 | +97 pages |
| API Reference | 11 | ~60 | +2 | +18 | +20 pages |
| Enterprise | 12 | ~70 | +2 | +35 | +37 pages |
| Internal (Staff) | 14 | ~80 | +2 | +55 | +57 pages |
| **TOTALS** | **55** | **~375** | **+13** | **+200** | **+213 pages** |

---

## Implementation Checklist

- [ ] Backup existing mint.json ✅
- [ ] Create new pages in destination directories
- [ ] Add new page references to mint.json
- [ ] Update group icons and descriptions
- [ ] Add frontmatter metadata to all new pages
- [ ] Test Mintlify preview: `mint dev`
- [ ] Validate broken links: `mint broken-links`
- [ ] Commit changes to git
- [ ] Generate updated mint.json for production

---

## Next Steps - TASKSET 3

1. Create destination directory structure for all new pages
2. Begin content migration Phase 1 (105 files)
3. Update mint.json with new page references
4. Test Mintlify build and navigation
5. Validate all links work correctly

---

**Document Status**: Enhancement Plan Complete - Ready for Phase 1 Implementation
**Last Updated**: 2026-01-08 22:15 UTC
**Next Action**: Execute TASKSET 3 - Create Directory Structure & Begin Content Migration
