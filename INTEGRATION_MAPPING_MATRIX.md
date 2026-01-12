---
title: "Integration Mapping Matrix - TASKSET 2 Reference"
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

# Integration Mapping Matrix - TASKSET 2 Reference

**Purpose**: Master reference for mapping all 400+ scattered documentation files to their destination in the Mintlify structure at `/platform/atlas/`

**Format**: Source File → Destination Path (with priority and integration strategy)

---

## PHASE 1: CRITICAL & FOUNDATION FILES (105 files)

### Architecture Core (15 files)

| Source File | Category | Destination Path | Priority | Integration Strategy |
|-------------|----------|------------------|----------|----------------------|
| `/docs/operations/architecture-overview.md` | Architecture | `/internal/architecture/system-design/platform-overview.mdx` | CRITICAL | Merge with existing overview |
| `/CLAUDE.md` | Architecture | `/developer/introduction/claude-development-guide.mdx` | HIGH | New page - development reference |
| `/DOCUMENTATION_CONSOLIDATION_PLAN.md` | Project Mgmt | `/internal/product/documentation/consolidation-roadmap.mdx` | HIGH | Reference doc - archive after migration |
| `/.atlas/.spec/...` (95 files) | Requirements | `/internal/architecture/specs/requirements-traceability.mdx` | HIGH | Index all requirements, link to originals |

### Operations Core (20 files)

| Source File | Category | Destination Path | Priority | Integration Strategy |
|-------------|----------|------------------|----------|----------------------|
| `/docs/operations/cicd-runbook.md` | Operations | `/developer/operations/deployment/ci-cd-runbook.mdx` | CRITICAL | Merge with existing CI/CD page |
| `/docs/operations/deployment-runbook.md` | Operations | `/developer/operations/deployment/deployment-runbook.mdx` | CRITICAL | Merge with existing deployment page |
| `/docs/MATERI-GRAFANA-RUNBOOK.md` | Observability | `/developer/operations/folio/grafana-runbook.mdx` | CRITICAL | Already listed, enhance with full content |
| `/.atlas/.spec/__requirements__/system-requirements.yaml` | Requirements | `/internal/architecture/specs/l1-strategic-specs.mdx` | HIGH | Extract and document |

### Security Core (10 files)

| Source File | Category | Destination Path | Priority | Integration Strategy |
|-------------|----------|------------------|----------|----------------------|
| `/domain/shield/docs/SECURITY.md` | Security | `/enterprise/security/authentication.mdx` | HIGH | Merge with existing auth page |
| `/platform/intelligence/.scribe/SECURITY_AUDIT_CHECKLIST.json` | Security | `/enterprise/security/security-audit-checklist.mdx` | HIGH | New page - security checklists |
| `/operations/security/...` | Security | `/enterprise/security/network-security.mdx` | HIGH | Consolidate security configs |

### Service Overviews (15 files)

| Source File | Category | Destination Path | Priority | Integration Strategy |
|-------------|----------|------------------|----------|----------------------|
| `/domain/shield/README.md` | Services | `/developer/domain/shield/overview.mdx` | HIGH | Enhance existing overview |
| `/domain/relay/docs/ARCHITECTURE.md` | Services | `/developer/domain/relay/architecture.mdx` | HIGH | Merge with existing architecture |
| `/domain/api/docs/...` | Services | `/developer/domain/api/architecture.mdx` | HIGH | Consolidate API docs |
| `/platform/intelligence/.scribe/README.md` | Services | `/developer/platform/intelligence/scribe/overview.mdx` | HIGH | Already listed |

### Key Operational Docs (20 files)

| Source File | Category | Destination Path | Priority | Integration Strategy |
|-------------|----------|------------------|----------|----------------------|
| `/platform/intelligence/.scribe/SCRIBE_MCP_OPERATOR_GUIDE.md` | Operations | `/developer/platform/intelligence/scribe/operator-guide.mdx` | CRITICAL | Already listed, verify content |
| `/platform/intelligence/.scribe/SCRIBE_TROUBLESHOOTING.md` | Operations | `/developer/platform/intelligence/scribe/troubleshooting.mdx` | CRITICAL | Already listed |
| `/platform/intelligence/.scribe/ALERT_RESPONSE_GUIDE.md` | Observability | `/developer/operations/folio/alerting.mdx` | HIGH | Enhance with alert response |
| `/platform/intelligence/.scribe/SCRIBE_TEAM_TRAINING.md` | Development | `/developer/platform/intelligence/scribe/team-training.mdx` | HIGH | New training material |

### Deployment & Infrastructure (15 files)

| Source File | Category | Destination Path | Priority | Integration Strategy |
|-------------|----------|------------------|----------|----------------------|
| `/DEPLOYMENT_ARCHITECTURE_DECISIONS.md` | Architecture | `/internal/architecture/adrs/adr-deployment-decisions.mdx` | HIGH | New ADR document |
| `/operations/gitops/...` | Infrastructure | `/developer/operations/infrastructure/gitops.mdx` | HIGH | Enhance with GitOps details |
| `/operations/folio/kubernetes/...` | Infrastructure | `/developer/operations/infrastructure/kubernetes.mdx` | HIGH | Merge K8s configs and docs |

### Project Status Files (10 files)

| Source File | Category | Destination Path | Priority | Integration Strategy |
|-------------|----------|------------------|----------|----------------------|
| `/TASKSET_*.md` (8 files) | Project Mgmt | `/internal/product/documentation/taskset-archive/` | MEDIUM | Archive completed tasksets |
| `/platform/intelligence/.scribe/PROJECT_COMPLETION_SUMMARY.md` | Project Mgmt | `/internal/product/documentation/scribe-completion-report.mdx` | MEDIUM | Archive as reference |

---

## PHASE 2: CORE SERVICES & DEVELOPMENT (250 files)

### Shield Service (40 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/domain/shield/docs/*` | `/developer/domain/shield/*` | Consolidate all docs, enhance architecture |
| `/domain/shield/tests/*` | Reference only - not in docs | Link to repo in test page |
| `/domain/shield/SECURITY.md` | `/enterprise/security/authentication.mdx` | Merge security content |
| Shield migration guides | `/enterprise/migration/from-competitors.mdx` | Add auth migration guide |

### Relay Service (35 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/domain/relay/docs/*` | `/developer/domain/relay/*` | Consolidate docs |
| `/domain/relay/src/` OT docs | `/developer/domain/relay/operational-transform.mdx` | Extract and enhance |
| Relay performance tuning | `/developer/operations/deployment/performance-tuning.mdx` | Add performance section |

### API Service (30 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/api/docs/*` | `/developer/domain/api/*` | Consolidate with existing |
| `/api/README.md` | `/developer/domain/api/overview.mdx` | Merge into overview |
| API deployment guides | `/developer/domain/api/deployment.mdx` | Enhance deployment section |
| API rate limiting | `/api/introduction/rate-limits.mdx` | Already exists, verify |

### Manuscript Service (25 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/domain/manuscript/docs/*` | `/developer/domain/manuscript/*` | Consolidate docs |
| Event schema examples | `/developer/domain/manuscript/event-schemas.mdx` | Enhance with real examples |
| Code generation | `/developer/domain/manuscript/code-generation.mdx` | Add comprehensive guide |

### Printery Service (20 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/domain/printery/docs/*` | `/developer/domain/printery/*` | Consolidate existing docs |
| Consumer group examples | `/developer/domain/printery/consumer-groups.mdx` | Enhance with patterns |
| Resilience patterns | `/developer/domain/printery/resilience-patterns.mdx` | Add comprehensive guide |

### Aria AI Platform (25 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/platform/aria/docs/*` | `/developer/platform/aria/*` | Consolidate existing |
| Model integration guide | `/developer/platform/aria/model-integration.mdx` | Enhance with custom models |
| Graceful degradation | `/developer/platform/aria/graceful-degradation.mdx` | Add patterns and examples |

### Intelligence Platform (30 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/platform/intelligence/.scribe/*` | `/developer/platform/intelligence/scribe/*` | Already partially listed |
| GitHub integrations | `/developer/platform/intelligence/github-integrations.mdx` | Already listed |
| Analytics | `/developer/platform/intelligence/analytics.mdx` | Already listed |
| Scribe deployment | `/developer/operations/deployment/scribe-deployment.mdx` | Add operational deployment |

### Canvas Product (20 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/platform/canvas/docs/*` | `/developer/products/canvas/*` | Consolidate existing |
| Component library | `/developer/products/canvas/component-library.mdx` | Enhance with inventory |
| Real-time sync | `/developer/products/canvas/real-time-sync.mdx` | Add detailed guide |

### Atlas Docs Product (15 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/platform/atlas/docs/*` | `/developer/products/atlas/*` | Self-referential - use as is |
| Structure guide | `/developer/products/atlas/structure.mdx` | Already exists |
| Contributing guide | `/developer/products/atlas/contributing.mdx` | Already exists |

### Specifications Product (15 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/.atlas/.spec/` | `/developer/products/specifications/*` | Link to existing spec system |
| Requirement types | `/developer/products/specifications/requirement-types.mdx` | Already exists |
| Verification matrix | `/developer/products/specifications/verification.mdx` | Already exists |

### Development Practices (25 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/CLAUDE.md` development patterns | `/developer/contributing/code-standards.mdx` | Merge dev standards |
| Testing guides from services | `/developer/testing/*` | Consolidate across services |
| Code examples from all services | `/developer/recipes/*` | Create new recipe pages |
| Debugging guides | `/developer/contributing/debugging-guide.mdx` | New page - debugging |

---

## PHASE 3: INFRASTRUCTURE & SUPPORTING SYSTEMS (160 files)

### Infrastructure & Deployment (60 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/operations/folio/kubernetes/` | `/developer/operations/infrastructure/kubernetes.mdx` | Consolidate K8s docs |
| `/operations/folio/terraform/` | `/developer/operations/infrastructure/terraform.mdx` | Consolidate IaC |
| `/operations/gitops/` | `/developer/operations/deployment/gitops.mdx` | Add GitOps details |
| CI/CD workflows | `/developer/operations/deployment/ci-cd.mdx` | Enhance with examples |
| Blue-green deployment | `/developer/operations/deployment/blue-green.mdx` | Already exists |
| Rollback procedures | `/developer/operations/deployment/rollback.mdx` | Already exists |

### Observability Stack (50 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/operations/folio/prometheus/` | `/developer/operations/folio/metrics.mdx` | Consolidate Prometheus |
| `/operations/folio/grafana/` | `/developer/operations/folio/dashboards.mdx` | Consolidate dashboards |
| `/operations/folio/loki/` | `/developer/operations/folio/logging.mdx` | Consolidate logging |
| `/operations/folio/alertmanager/` | `/developer/operations/folio/alerting.mdx` | Consolidate alerts |
| `/operations/folio/jaeger/` | `/developer/operations/folio/tracing.mdx` | Consolidate tracing |
| Observability runbooks | `/enterprise/monitoring/*` | Merge with enterprise monitoring |

### Operational Runbooks (30 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| Incident response | `/developer/operations/runbooks/incident-response.mdx` | Already exists |
| Scaling procedures | `/developer/operations/runbooks/scaling.mdx` | Already exists |
| Backup/restore | `/developer/operations/runbooks/backup-restore.mdx` | Already exists |
| Disaster recovery | `/developer/operations/runbooks/disaster-recovery.mdx` | Already exists |
| Additional runbooks | `/developer/operations/runbooks/` | Create new pages |

### Security & Compliance (20 files)

| Source Category | Destination Path | Integration Strategy |
|-----------------|------------------|----------------------|
| `/operations/security/` configs | `/enterprise/security/` groups | Consolidate security |
| Compliance documentation | `/internal/security/compliance/*` | Already mostly mapped |
| SOC2/ISO27001 evidence | `/internal/security/compliance/` | Merge certifications |

---

## DEDUPLICATION & CONFLICT RESOLUTION

### Known Duplicates (Priority: Remove or Merge)

| Files | Issue | Resolution |
|-------|-------|-----------|
| Multiple SCRIBE guides | Same content in `/docs/` and `/platform/intelligence/.scribe/` | Keep `/platform/intelligence/.scribe/` version (more current) |
| Multiple architecture docs | Overlapping system architecture docs | Consolidate into single `/internal/architecture/system-design/` |
| Multiple security policies | Different versions of security policies | Use most recent from `/platform/intelligence/.scribe/` |
| Multiple deployment guides | Multiple deployment runbooks | Merge into master runbook, add service-specific sections |

### Resolution Strategy

**Rule 1**: Check modification date - use most recent
**Rule 2**: Check comprehensiveness - use most detailed
**Rule 3**: Check Mintlify version - use as baseline, enhance with external content
**Rule 4**: Check service-specificity - keep service-specific, merge general

---

## METADATA & FRONTMATTER TEMPLATE

All migrated documents should include frontmatter:

```yaml
---
title: "Document Title"
description: "Brief description"
icon: "icon-name"
source: "original/path/to/file.md"
sourceRepo: "https://github.com/materi-ai/repo"
lastMigrated: "2026-01-08"
status: "migrated"
consolidatedFrom: ["file1.md", "file2.md"]  # If merged
relatedPages:
  - "page-path-1"
  - "page-path-2"
tags:
  - "category-tag"
  - "service-tag"
  - "role-tag"
---
```

---

## PROGRESS TRACKING

### Files Mapped
- Phase 1 (Critical): 105/105 files
- Phase 2 (Core Services): 250/250 files
- Phase 3 (Infrastructure): 160/160 files
- **Total**: 515/515 files planned

### Status Legend
- ✅ Mapped (exists in matrix)
- ⏳ To be mapped (identified but not yet positioned)
- 🔄 Merged (multiple sources consolidated)
- 🗂️ Archived (legacy, reference only)
- ❓ Requires decision (conflicting sources)

---

## NEXT STEPS - TASKSET 3

1. **Validate Mapping**: Review matrix for accuracy and completeness
2. **Identify Gaps**: Check for unmapped files in source directories
3. **Resolve Duplicates**: Make final decisions on conflicting content
4. **Create Destination Structure**: Set up directory structure in `/platform/atlas/`
5. **Begin Migration**: Start with Phase 1 files
6. **Update mint.json**: Add new page references to navigation

---

**Document Status**: Mapping Complete - Ready for Phase 1 Migration
**Last Updated**: 2026-01-08 22:00 UTC
**Next Action**: Execute TASKSET 3 - Content Migration Phase 1
