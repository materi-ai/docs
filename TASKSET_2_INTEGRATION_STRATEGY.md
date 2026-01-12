# TASKSET 2: Structure & Foundation - Integration Strategy

**Status**: In Progress
**Date**: 2026-01-08
**Objective**: Design the integration architecture for consolidating 400+ scattered documentation files into the existing Mintlify structure at `/platform/atlas/`

---

## 1. Current State Assessment

### Existing Mintlify Structure (5 Primary Tabs)
```
├── Getting Started (Intro, features, quickstart, architecture)
├── Developer Guide (Domain services, platform, products, operations)
├── API Reference (REST, GraphQL, WebSocket, SDKs, Events)
├── Enterprise (Deployment, security, compliance, operations)
└── Internal (Staff) (Architecture, engineering, operations, product)
```

### Scattered Documentation (400+ files across 12 locations)

**Primary Sources**:
1. `/Users/alexarno/materi/` (root level) - 34 files
2. `/Users/alexarno/materi/docs/` (legacy) - 10 files + archives
3. `/Users/alexarno/materi/.atlas/` - 557 spec files
4. `/Users/alexarno/materi/operations/` - 132 files
5. `/Users/alexarno/materi/domain/` - 2,127 service files
6. `/Users/alexarno/materi/platform/` - 600 files
7. `/Users/alexarno/materi/_ctx/` - 45 context files
8. `/Users/alexarno/materi/lab/` - 1,732 experimental files
9. `/Users/alexarno/materi/api/` - scattered docs
10. `/Users/alexarno/materi/shield/` - scattered docs
11. `/Users/alexarno/materi/relay/` - scattered docs
12. `/Users/alexarno/materi/monitoring/` - scattered docs

---

## 2. Integration Architecture

### Mapping Strategy: 8 Categories → Mintlify Tabs

The 8 documentation categories from Phase 1 will map to the existing Mintlify structure:

| Internal Category | Mintlify Tab | Mintlify Groups | Action |
|-------------------|--------------|-----------------|--------|
| **Architecture** (85 files) | Internal + Developer | System Architecture, ADRs | Consolidate to `internal/architecture/` + `developer/architecture/` |
| **Operations** (145 files) | Developer + Enterprise | Operations groups (Deployment, Folio, Runbooks) | Distribute to existing operations groups |
| **Development** (200 files) | Developer | All domain services + platform + testing | Integrate into existing groups |
| **Security** (65 files) | Enterprise + Internal | Security & Compliance groups | Add to `enterprise/security/` + `internal/security/` |
| **Services** (250 files) | Developer | Domain Services (API, Shield, Relay, etc.) | Consolidate into existing service groups |
| **Observability** (95 files) | Developer + Enterprise | Operations → Folio Observability | Enhance existing folio group |
| **Project Management** (110 files) | Internal | Product + Analytics + Strategy | Consolidate to internal groups |
| **Infrastructure** (140 files) | Developer + Enterprise | Operations → Infrastructure + Enterprise deployment | Enhance existing groups |

---

## 3. Detailed Integration Plan

### 3.1 Architecture Documentation (85 files)

**Current Mintlify Location**:
- `internal/architecture/system-design/` (9 pages)
- `internal/architecture/adrs/` (7 pages)
- `internal/architecture/specs/` (7 pages)

**Source Files to Integrate**:
- `/Users/alexarno/materi/docs/operations/architecture-overview.md`
- `/Users/alexarno/materi/platform/` architecture files
- `/Users/alexarno/materi/domain/` service architecture docs
- `/Users/alexarno/materi/.atlas/.spec/` specification files

**Action Items**:
1. Review all 85 architecture files and extract unique content
2. Consolidate service architecture into existing service groups
3. Enhance ADRs section with additional decision records
4. Expand specs section with requirement traceability
5. Create architecture quick reference guide

**Expected Output**: +15-20 new pages in internal/architecture/

---

### 3.2 Operations Documentation (145 files)

**Current Mintlify Location**:
- `developer/operations/infrastructure/` (5 pages)
- `developer/operations/deployment/` (5 pages)
- `developer/operations/folio/` (7 pages)
- `developer/operations/runbooks/` (4 pages)

**Source Files to Integrate**:
- `/Users/alexarno/materi/docs/operations/cicd-runbook.md`
- `/Users/alexarno/materi/docs/operations/deployment-runbook.md`
- `/Users/alexarno/materi/platform/intelligence/.scribe/` (Scribe guides)
- `/Users/alexarno/materi/operations/folio/` (full observability stack)
- `/Users/alexarno/materi/operations/gitops/` (GitOps configs)

**Action Items**:
1. Consolidate runbooks (deployment, incident response, recovery)
2. Expand Folio observability docs with complete stack
3. Add infrastructure-as-code examples
4. Create deployment decision matrix
5. Add operational checklists and procedures

**Expected Output**: +25-30 new pages in developer/operations/

---

### 3.3 Development Documentation (200 files)

**Current Mintlify Location**:
- `developer/contributing/` (6 pages)
- `developer/domain/` (multiple service groups)
- `developer/testing/` (7 pages)
- `developer/platform/` (Aria, Intelligence, Products)

**Source Files to Integrate**:
- `/Users/alexarno/materi/domain/shield/docs/` (auth service)
- `/Users/alexarno/materi/domain/manuscript/` (event schemas)
- `/Users/alexarno/materi/platform/intelligence/` (Scribe, GitHub, Analytics)
- `/Users/alexarno/materi/api/` (API implementation docs)
- All service README files and setup guides

**Action Items**:
1. Create comprehensive service setup guides
2. Add implementation examples and patterns
3. Enhance testing documentation with service-specific tests
4. Create SDK development guides
5. Add debugging and troubleshooting sections

**Expected Output**: +40-50 new pages across developer services

---

### 3.4 Security Documentation (65 files)

**Current Mintlify Location**:
- `enterprise/security/` (11 pages)
- `internal/security/practices/` (6 pages)
- `internal/security/vulnerabilities/` (5 pages)
- `internal/security/compliance/` (5 pages)

**Source Files to Integrate**:
- `/Users/alexarno/materi/domain/shield/docs/SECURITY.md`
- `/Users/alexarno/materi/operations/` security configs
- `/Users/alexarno/materi/platform/intelligence/.scribe/` security/disaster recovery
- Encryption, RBAC, audit trail documentation

**Action Items**:
1. Create security architecture guide
2. Add threat modeling documentation
3. Enhance compliance section with controls mapping
4. Add security incident response procedures
5. Create security audit checklist

**Expected Output**: +10-15 new pages in security groups

---

### 3.5 Services Documentation (250 files)

**Current Mintlify Location**:
- `developer/domain/api/` (8 pages)
- `developer/domain/relay/` (7 pages)
- `developer/domain/shield/` (8 pages)
- `developer/domain/manuscript/` (6 pages)
- `developer/domain/printery/` (6 pages)
- `developer/platform/aria/` (10 pages)
- `developer/platform/intelligence/` (6 pages)

**Source Files to Integrate**:
- Complete domain service documentation
- Platform service documentation (Aria, Intelligence)
- Product documentation (Canvas, Atlas, Specifications)
- Service-specific deployment guides
- Service health checks and monitoring

**Action Items**:
1. Standardize service documentation format
2. Create service ownership matrix
3. Add service interaction diagrams
4. Enhance deployment procedures per service
5. Create service-specific troubleshooting guides

**Expected Output**: +30-40 new pages across service groups

---

### 3.6 Observability Documentation (95 files)

**Current Mintlify Location**:
- `developer/operations/folio/` (7 pages)
- `enterprise/monitoring/` (7 pages)

**Source Files to Integrate**:
- `/Users/alexarno/materi/operations/folio/` (complete stack)
  - Prometheus configs
  - Grafana dashboards
  - Loki log aggregation
  - AlertManager rules
  - Jaeger tracing
- Monitoring guides and dashboards
- Alert response procedures
- Metrics and SLO documentation

**Action Items**:
1. Create Folio stack architecture guide
2. Add dashboard setup procedures
3. Create alert runbooks (mapped by service)
4. Add metrics dictionary (all available metrics)
5. Create SLO/SLI/SLA documentation
6. Add performance baseline documentation

**Expected Output**: +20-25 new pages in operations/folio and enterprise/monitoring

---

### 3.7 Project Management Documentation (110 files)

**Current Mintlify Location**:
- `internal/product/strategy/` (4 pages)
- `internal/product/development/` (4 pages)
- `internal/analytics/` (5 pages)

**Source Files to Integrate**:
- `/Users/alexarno/materi/.atlas/.spec/__requirements__/` (95+ requirements)
- Project status reports and checklists
- Product roadmap and vision
- OKRs and planning documents
- Team capacity and resource planning

**Action Items**:
1. Create requirements traceability matrix
2. Consolidate roadmap and planning documents
3. Create OKR tracking dashboard docs
4. Add project template library
5. Create resource planning guides

**Expected Output**: +15-20 new pages in internal/product and internal/analytics

---

### 3.8 Infrastructure Documentation (140 files)

**Current Mintlify Location**:
- `developer/operations/infrastructure/` (5 pages)
- `enterprise/deployment/` (multiple groups, 15+ pages)
- `enterprise/ha/` (6 pages)
- `enterprise/scalability/` (6 pages)

**Source Files to Integrate**:
- `/Users/alexarno/materi/operations/folio/kubernetes/` (K8s manifests + docs)
- `/Users/alexarno/materi/operations/gitops/` (ArgoCD, Kustomize)
- `/Users/alexarno/materi/operations/` CI/CD workflows
- Infrastructure-as-code documentation
- Network policies and security groups
- Database and Redis configuration
- Load testing and performance tuning

**Action Items**:
1. Create infrastructure decision matrix
2. Add IaC examples (Terraform, Kustomize, etc.)
3. Create deployment scenario guides
4. Add infrastructure troubleshooting
5. Create capacity planning procedures
6. Add cost optimization guide

**Expected Output**: +25-30 new pages in infrastructure groups

---

## 4. Migration Approach

### Phase-Based Rollout

**Phase 1 - Foundation (TASKSET 3)**
- Core architecture and operations docs
- Critical security and compliance docs
- Service overview and getting started
- ~105 files

**Phase 2 - Core Services (TASKSET 4)**
- Domain services documentation (Shield, Relay, API, etc.)
- Platform services documentation (Aria, Intelligence)
- Complete development guides
- ~250 files

**Phase 3 - Supporting Systems (TASKSET 5)**
- Infrastructure and deployment documentation
- Observability and monitoring complete stack
- Project management and planning docs
- ~160 files

---

## 5. Cross-Cutting Concerns

### 5.1 Navigation Integration

**Strategy**: Update `mint.json` to include new pages in existing groups
- Each group in mint.json will have new pages added
- New subsections created where appropriate (e.g., service-specific guides)
- Breadcrumb navigation maintained through existing structure

### 5.2 Internal Linking

**Strategy**: Create a linking convention that works across migrated docs
```
Pattern 1: Same Tab - [text](../related-page)
Pattern 2: Different Tab - [text](/developer/service/specific-page)
Pattern 3: External - [text](https://external.url)
```

### 5.3 Search Optimization

**Strategy**: Enhance metadata and tagging
- Add frontmatter with keywords for each page
- Create cross-references for related content
- Implement search taxonomy (role-based, task-based, service-based)

### 5.4 Version Management

**Strategy**: No versioning changes needed
- All content integrated into primary docs.json
- Legacy versions stay in `/Users/alexarno/materi/docs/` (reference only)
- Git history preserved for attribution

---

## 6. Deliverables for TASKSET 2

### 6.1 Updated mint.json
- **Action**: Add new pages to existing groups (no new tabs, only new pages in groups)
- **File**: `/Users/alexarno/materi/platform/atlas/mint.json`
- **Scope**: Add 150-200 page references in appropriate groups

### 6.2 Integration Mapping Document
- **File**: `/Users/alexarno/materi/platform/atlas/INTEGRATION_MAPPING.md`
- **Content**:
  - Source → Destination mappings for all 400+ files
  - File categorization and priority
  - Deduplication strategy
  - Archive strategy for legacy content

### 6.3 Content Consolidation Guide
- **File**: `/Users/alexarno/materi/platform/atlas/CONSOLIDATION_GUIDE.md`
- **Content**:
  - Step-by-step procedures for each category
  - File naming conventions
  - Frontmatter template
  - Link rewriting rules

### 6.4 Section Enhancement Documents
- **Files**:
  - `/Users/alexarno/materi/platform/atlas/developer/SECTION_ROADMAP.md`
  - `/Users/alexarno/materi/platform/atlas/enterprise/SECTION_ROADMAP.md`
  - `/Users/alexarno/materi/platform/atlas/internal/SECTION_ROADMAP.md`
- **Content**: Detailed content gaps and planned additions per section

### 6.5 Quality Assurance Checklist
- **File**: `/Users/alexarno/materi/platform/atlas/QA_CHECKLIST.md`
- **Content**:
  - Link validation procedures
  - Formatting standards
  - SEO requirements
  - Accessibility requirements

---

## 7. Success Criteria for TASKSET 2

✅ Complete mapping of all 400+ source files to destination paths in Mintlify
✅ Updated mint.json with 150-200 new page references
✅ Integration mapping document (source → destination)
✅ Consolidation guide with procedures
✅ Section roadmaps for Developer, Enterprise, Internal tabs
✅ QA checklist for content validation
✅ Decision on archive strategy (what stays in `/Users/alexarno/materi/docs/`)
✅ Plan for handling duplicate/conflicting content
✅ Guidelines for frontmatter and metadata

---

## 8. Key Decisions to Make

### 8.1 Duplicate Content Handling
**Decision Required**: When same content exists in multiple locations, which is the "source of truth"?

**Options**:
- A: Use most recent modification date
- B: Use most detailed/comprehensive version
- C: Use existing Mintlify version as authoritative
- D: Merge/synthesize all versions

**Recommendation**: Option C (existing Mintlify as authority) + merge supplementary content

### 8.2 Archive Strategy
**Decision Required**: What to do with legacy docs in `/Users/alexarno/materi/docs/`?

**Options**:
- A: Delete after migration (content preserved in Git)
- B: Move to `/Users/alexarno/materi/platform/atlas/docs/archive/`
- C: Keep both (reference + live)
- D: Create redirect symlinks

**Recommendation**: Option B (move to archive subfolder for reference)

### 8.3 Section Organization
**Decision Required**: Create new subsections or integrate into existing groups?

**Options**:
- A: Flat integration (add pages to existing groups only)
- B: Create new subsections (new groups for major categories)
- C: Hybrid (new groups for large categories only)

**Recommendation**: Option C (hybrid - new subsections for very large content areas)

---

## 9. Next Steps

### Immediate (TASKSET 2 completion)
1. Finalize key decisions above
2. Create complete integration mapping (all 400+ files)
3. Update mint.json structure
4. Create consolidation procedures

### Subsequent (TASKSET 3-5)
1. Migrate Phase 1 content (105 files)
2. Migrate Phase 2 content (250 files)
3. Migrate Phase 3 content (160 files)

### Final (TASKSET 6-8)
1. Cross-reference management
2. Index and navigation
3. Verification and QA

---

**Document Status**: Strategy Design Complete - Ready for Implementation
**Last Updated**: 2026-01-08 21:45 UTC
**Next Action**: Finalize decisions and create integration mapping
