# TASKSET 3 - Batch Migration Framework Phase 1

**Status**: IN PROGRESS - Structured Batch Execution
**Date**: 2026-01-09
**Phase**: 1 of 3 (Critical & Foundation Files)
**Target**: 105 files
**Framework**: Optimized batch processing with Shield/Folio observability integration

---

## Executive Summary

This document outlines the **structured batch migration framework** for executing Phase 1 migration of 105 critical foundation files. Rather than individual file operations, this framework processes files in logical batches:

- **Batch 1**: Architecture Core (15 files)
- **Batch 2**: Operations Core (20 files)
- **Batch 3**: Security Core (10 files)
- **Batch 4**: Service Overviews (15 files)
- **Batch 5**: Key Operational Docs (20 files)
- **Batch 6**: Deployment & Infrastructure (15 files)
- **Batch 7**: Project Status Files (10 files)

**Total**: 7 batches, 105 files
**Execution Pattern**: Batch-by-batch with validation gates
**Observability**: Shield webhooks + Folio metrics + Shredder state tracking

---

## Batch 1: Architecture Core (15 files)

### Files to Migrate

| # | Source File | Destination Path | Status | Notes |
|---|-------------|------------------|--------|-------|
| 1 | `/docs/operations/architecture-overview.md` | `/internal/architecture/system-design/platform-overview.mdx` | ✅ DONE | First migration - frontmatter validated |
| 2 | `/CLAUDE.md` | `/developer/introduction/claude-development-guide.mdx` | ⏳ PENDING | Large file - 400+ lines |
| 3 | `/DOCUMENTATION_CONSOLIDATION_PLAN.md` | `/internal/product/documentation/consolidation-roadmap.mdx` | ⏳ PENDING | Project meta doc |
| 4 | `/.atlas/.spec/L1_STRATEGIC_REQUIREMENTS.md` | `/internal/architecture/specs/l1-strategic-specs.mdx` | ⏳ PENDING | Specifications |
| 5 | `/.atlas/.spec/OBJECTIVES_AND_SCOPE.md` | `/internal/architecture/specs/objectives-scope.mdx` | ⏳ PENDING | Requirements |
| 6 | `/.atlas/.spec/SUCCESS_CRITERIA.md` | `/internal/architecture/specs/success-criteria.mdx` | ⏳ PENDING | Success metrics |
| 7-15 | Additional spec files (8 files) | `/internal/architecture/specs/` | ⏳ PENDING | Requirements index |

**Batch 1 Total**: 15 files | **Estimated Completion**: After all 15 files processed

---

## Batch 2: Operations Core (20 files)

### Files to Migrate

| # | Source File | Destination Path | Status | Notes |
|---|-------------|------------------|--------|-------|
| 1 | `/docs/operations/cicd-runbook.md` | `/developer/operations/deployment/ci-cd-runbook.mdx` | ⏳ PENDING | CI/CD guide |
| 2 | `/docs/operations/deployment-runbook.md` | `/developer/operations/deployment/deployment-runbook.mdx` | ⏳ PENDING | Deployment guide |
| 3 | `/docs/MATERI-GRAFANA-RUNBOOK.md` | `/developer/operations/folio/grafana-runbook.mdx` | ⏳ PENDING | Grafana operations |
| 4 | `/docs/operations/SCRIBE_OBSERVABILITY_SETUP.md` | `/developer/operations/folio/scribe-observability.mdx` | ⏳ PENDING | Scribe metrics |
| 5-20 | Additional operational docs (16 files) | `/developer/operations/` | ⏳ PENDING | Various runbooks |

**Batch 2 Total**: 20 files | **Estimated Completion**: After Batch 1 complete

---

## Batch 3: Security Core (10 files)

### Files to Migrate

| # | Source File | Destination Path | Status | Notes |
|---|-------------|------------------|--------|-------|
| 1 | `/domain/shield/docs/SECURITY.md` | `/enterprise/security/authentication.mdx` | ⏳ PENDING | Shield security |
| 2 | `/platform/intelligence/.scribe/SECURITY_AUDIT_CHECKLIST.json` | `/enterprise/security/security-audit-checklist.mdx` | ⏳ PENDING | Security checklist |
| 3-10 | Additional security docs (8 files) | `/enterprise/security/` | ⏳ PENDING | Security configs |

**Batch 3 Total**: 10 files | **Estimated Completion**: After Batch 2 complete

---

## Batch 4: Service Overviews (15 files)

### Files to Migrate

| # | Source File | Destination Path | Status | Notes |
|---|-------------|------------------|--------|-------|
| 1 | `/domain/shield/README.md` | `/developer/domain/shield/overview.mdx` | ⏳ PENDING | Shield overview |
| 2 | `/domain/relay/docs/ARCHITECTURE.md` | `/developer/domain/relay/architecture.mdx` | ⏳ PENDING | Relay architecture |
| 3 | `/domain/api/docs/README.md` | `/developer/domain/api/overview.mdx` | ⏳ PENDING | API overview |
| 4 | `/domain/manuscript/README.md` | `/developer/domain/manuscript/overview.mdx` | ⏳ PENDING | Manuscript overview |
| 5 | `/domain/printery/README.md` | `/developer/domain/printery/overview.mdx` | ⏳ PENDING | Printery overview |
| 6-15 | Additional service docs (10 files) | `/developer/domain/` | ⏳ PENDING | Service details |

**Batch 4 Total**: 15 files | **Estimated Completion**: After Batch 3 complete

---

## Batch 5: Key Operational Docs (20 files)

### Files to Migrate

| # | Source File | Destination Path | Status | Notes |
|---|-------------|------------------|--------|-------|
| 1 | `/platform/intelligence/.scribe/SCRIBE_MCP_OPERATOR_GUIDE.md` | `/developer/platform/intelligence/scribe/operator-guide.mdx` | ⏳ PENDING | Scribe operators |
| 2 | `/platform/intelligence/.scribe/SCRIBE_TROUBLESHOOTING.md` | `/developer/platform/intelligence/scribe/troubleshooting.mdx` | ⏳ PENDING | Scribe troubleshooting |
| 3 | `/platform/intelligence/.scribe/ALERT_RESPONSE_GUIDE.md` | `/developer/operations/folio/alerting.mdx` | ⏳ PENDING | Alert response |
| 4 | `/platform/intelligence/.scribe/SCRIBE_TEAM_TRAINING.md` | `/developer/platform/intelligence/scribe/team-training.mdx` | ⏳ PENDING | Training material |
| 5-20 | Additional operational docs (16 files) | `/developer/platform/` | ⏳ PENDING | Platform guides |

**Batch 5 Total**: 20 files | **Estimated Completion**: After Batch 4 complete

---

## Batch 6: Deployment & Infrastructure (15 files)

### Files to Migrate

| # | Source File | Destination Path | Status | Notes |
|---|-------------|------------------|--------|-------|
| 1 | `/DEPLOYMENT_ARCHITECTURE_DECISIONS.md` | `/internal/architecture/adrs/adr-deployment-decisions.mdx` | ⏳ PENDING | ADR document |
| 2 | `/operations/gitops/README.md` | `/developer/operations/deployment/gitops.mdx` | ⏳ PENDING | GitOps guide |
| 3 | `/operations/folio/kubernetes/README.md` | `/developer/operations/infrastructure/kubernetes.mdx` | ⏳ PENDING | Kubernetes |
| 4-15 | Additional deployment docs (12 files) | `/developer/operations/` | ⏳ PENDING | Infrastructure docs |

**Batch 6 Total**: 15 files | **Estimated Completion**: After Batch 5 complete

---

## Batch 7: Project Status Files (10 files)

### Files to Archive

| # | Source File | Destination Path | Status | Notes |
|---|-------------|------------------|--------|-------|
| 1 | `/TASKSET_1_ANALYSIS_REPORT.md` | `/internal/product/documentation/taskset-archive/` | ⏳ PENDING | Phase 1 analysis |
| 2 | `/TASKSET_2_INTEGRATION_STRATEGY.md` | `/internal/product/documentation/taskset-archive/` | ⏳ PENDING | Phase 2 strategy |
| 3 | `/TASKSET_3_PHASE1_STATUS.md` | `/internal/product/documentation/taskset-archive/` | ⏳ PENDING | Phase 3 checkpoint |
| 4 | `/platform/intelligence/.scribe/PROJECT_COMPLETION_SUMMARY.md` | `/internal/product/documentation/scribe-completion-report.mdx` | ⏳ PENDING | Scribe summary |
| 5-10 | Additional project docs (6 files) | `/internal/product/documentation/taskset-archive/` | ⏳ PENDING | Reference docs |

**Batch 7 Total**: 10 files | **Estimated Completion**: After Batch 6 complete

---

## Observability & Tracking

### Shield Integration Points

Each batch completion will trigger:

```python
# Pseudo-code for Shield webhook integration
POST /api/v1/webhooks/documentation-migration
{
  "event": "batch_completed",
  "batch_id": "BATCH_1_ARCHITECTURE_CORE",
  "files_processed": 15,
  "files_migrated": 15,
  "timestamp": "2026-01-09T14:30:00Z",
  "status": "success",
  "metrics": {
    "total_lines": 4250,
    "frontmatter_added": 15,
    "links_updated": 42,
    "execution_time_seconds": 180
  }
}
```

### Folio Metrics Pushed

```
# Prometheus metrics format
cicd_batch_migration_files_total{batch="BATCH_1",status="migrated"} 15
cicd_batch_migration_duration_seconds{batch="BATCH_1"} 180
cicd_batch_migration_lines_total{batch="BATCH_1"} 4250
cicd_batch_migration_frontmatter_added{batch="BATCH_1"} 15
```

### Shredder State Tracking

```yaml
# Batch state persisted to Shredder
migration_batch_state:
  batch_id: "BATCH_1_ARCHITECTURE_CORE"
  phase: 1
  total_files: 15
  files_completed: 15
  files_failed: 0
  timestamp_started: "2026-01-09T14:00:00Z"
  timestamp_completed: "2026-01-09T14:30:00Z"
  status: "COMPLETED"
  validation_passed: true
```

---

## Batch Execution Gates

### Pre-Batch Gate
Each batch processes only if:
- ✅ Previous batch status = COMPLETED
- ✅ All source files verified to exist
- ✅ Destination directories created
- ✅ mint.json backup exists
- ✅ Frontmatter template ready

### During-Batch Operations
For each file in batch:
1. Read source file (with error handling)
2. Add frontmatter metadata
3. Write to destination path
4. Verify file created successfully
5. Log operation with correlation ID
6. Update Shredder state

### Post-Batch Validation
After each batch completes:
- ✅ Count migrated files (must equal batch size)
- ✅ Validate frontmatter consistency
- ✅ Check destination file sizes match source (±10%)
- ✅ Verify all links are properly formatted
- ✅ Generate batch report

### Quality Gate Thresholds
- **Success**: 100% of files migrated successfully
- **Warning**: 95-99% success rate (log and retry failed files)
- **Failure**: <95% success rate (pause, investigate, retry after fix)

---

## mint.json Updates by Batch

### Phase 1 mint.json Additions

After each batch completes, corresponding mint.json entries will be added:

**Batch 1 → Developer Guide > Architecture & Design**
```json
{
  "group": "Architecture & Design",
  "pages": [
    "developer/architecture/platform-overview",
    "developer/architecture/claude-development-guide",
    "internal/architecture/consolidation-roadmap",
    "internal/architecture/specs/requirements",
    ...
  ]
}
```

**Batch 2 → Developer Guide > Operations**
```json
{
  "group": "Operations & Deployment",
  "pages": [
    "developer/operations/deployment/ci-cd-runbook",
    "developer/operations/deployment/deployment-runbook",
    "developer/operations/folio/grafana-runbook",
    ...
  ]
}
```

And so on for Batches 3-7...

---

## Execution Timeline

| Batch | Files | Start | Est. Duration | Status |
|-------|-------|-------|---------------|--------|
| 1 | 15 | Now | 15 min | ⏳ NEXT |
| 2 | 20 | After B1 | 20 min | ⏳ PENDING |
| 3 | 10 | After B2 | 10 min | ⏳ PENDING |
| 4 | 15 | After B3 | 15 min | ⏳ PENDING |
| 5 | 20 | After B4 | 20 min | ⏳ PENDING |
| 6 | 15 | After B5 | 15 min | ⏳ PENDING |
| 7 | 10 | After B6 | 10 min | ⏳ PENDING |
| **TOTAL** | **105** | | **~105 min** | ⏳ IN PROGRESS |

---

## Validation Checklist

After all 7 batches complete, verify:

- [ ] All 105 files migrated to destination paths
- [ ] All files have proper frontmatter with metadata
- [ ] All internal links are formatted correctly
- [ ] No broken links in migrated content
- [ ] mint.json updated with all 105 file references
- [ ] Directory structure matches plan
- [ ] No duplicate files in destination
- [ ] File sizes and line counts documented
- [ ] Shield webhooks logged all batch completions
- [ ] Folio metrics show all batches completed
- [ ] Shredder state tracking shows COMPLETED for all batches

---

## Success Criteria

✅ **Phase 1 Migration Complete** when:
1. All 105 files successfully migrated
2. 100% frontmatter consistency across all files
3. 0 broken internal links
4. mint.json successfully updated with Phase 1 references
5. All quality gates passed

✅ **Ready for Phase 2** when:
1. Phase 1 validation passed
2. Phase 1 completion report generated
3. User confirms readiness: "GO TASKSET 4"

---

## Next Action

**Batch 1 is next**: 15 Architecture Core files

Ready to proceed with batch execution framework.

---

**Document Created**: 2026-01-09 14:15 UTC
**Framework**: Optimized batch processing
**Observability**: Shield + Folio + Shredder integrated
**Status**: Ready for Phase 1 execution

