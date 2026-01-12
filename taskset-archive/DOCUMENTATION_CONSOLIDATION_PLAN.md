# Documentation Consolidation Plan
## Platform Atlas Centralization Strategy

**Date Created**: 2026-01-08
**Project Owner**: Claude Code
**Status**: Ready for Execution
**Total Scope**: 400+ documentation files across 12 locations → Unified `/docs/atlas/` collection

---

## 📋 TABLE OF CONTENTS

1. [Executive Summary](#executive-summary)
2. [Current State Analysis](#current-state-analysis)
3. [Implementation Strategy](#implementation-strategy)
4. [TASKSET 1: Analysis & Baseline](#taskset-1-analysis--baseline)
5. [TASKSET 2: Structure & Foundation](#taskset-2-structure--foundation)
6. [TASKSET 3: Content Migration - Phase 1](#taskset-3-content-migration---phase-1)
7. [TASKSET 4: Content Migration - Phase 2](#taskset-4-content-migration---phase-2)
8. [TASKSET 5: Content Migration - Phase 3](#taskset-5-content-migration---phase-3)
9. [TASKSET 6: Cross-Reference & Link Management](#taskset-6-cross-reference--link-management)
10. [TASKSET 7: Index & Navigation](#taskset-7-index--navigation)
11. [TASKSET 8: Verification & Quality Assurance](#taskset-8-verification--quality-assurance)
12. [Execution Roadmap](#execution-roadmap)
13. [Success Criteria](#success-criteria)

---

## EXECUTIVE SUMMARY

The Materi platform contains **400+ documentation files** distributed across **12 major locations** (root directory, `/docs/`, `/atlas/`, `/domain/`, `/clari/`, `/sparki/`, `/scribe/`, service directories, and more), totaling **6.5MB+ of content** with **105,000+ lines of documentation**.

This fragmentation creates:
- **Findability Issues**: Users don't know where to find documentation
- **Maintenance Burden**: Updates scattered across multiple locations
- **Duplicate Content**: Same information in multiple places
- **Inconsistent Standards**: Different naming, formatting, linking conventions
- **Navigation Gaps**: No clear path from one related document to another

### Solution Approach

**Consolidate all documentation into a unified `/docs/atlas/` collection** organized by:
- **8 primary categories** (Architecture, Operations, Development, Testing, Deployment, Security, Project Management, Service-Specific)
- **Hierarchical structure** with clear parent-child relationships
- **Master index system** for discoverability
- **Standardized metadata** for searchability and maintenance
- **Cross-reference network** connecting related documents
- **Quick reference materials** for different user roles

### Key Outcomes

✅ Single source of truth for all platform documentation
✅ Improved discoverability through master indices and search
✅ Consistent standards across all documents
✅ Clear navigation paths for different user roles
✅ Simplified maintenance and updates
✅ Unified contribution guidelines

---

## CURRENT STATE ANALYSIS

### Documentation Distribution

| Location | File Count | Size | Purpose |
|----------|-----------|------|---------|
| Root (`/materi/`) | 32 | — | Platform overview, deployment, standards |
| `/docs/` | 10 | 188KB | Scribe MCP, operations, quick references |
| `/docs/operations/` | 4 | — | Deployment, CI/CD, production readiness |
| `/atlas/TODO/` | 280+ | 3.6MB | Phase reports, implementation guides, testing |
| `/domain/docs/` | 150+ | 1.7MB | API, Shield, Relay, Manuscript service docs |
| `/clari/` | 30+ | — | Phase completion reports |
| `/sparki/docs/` | 10 core | 256KB | Operations, deployment, architecture |
| `/scribe/` | 4 | — | Scribe MCP operations guides |
| Service READMEs | 7 | — | API, Shield, Relay, Manuscript, Printery, Sparki |
| Other (`/platform/`, `/lab/`, etc.) | 20+ | — | Miscellaneous documentation |
| **TOTAL** | **400+** | **6.5MB** | **105,000+ lines** |

### Documentation Categories

**Architecture & Design** (40+ files)
- System architecture, C4 diagrams, design journals, decision logs

**Operations & Runbooks** (50+ files)
- Deployment procedures, incident response, troubleshooting, monitoring

**Development Guides** (60+ files)
- Implementation strategies, development setup, API references, testing

**Project Management** (100+ files)
- Taskset reports, phase completion, progress summaries

**Service-Specific** (120+ files)
- API, Shield, Relay, Manuscript, Printery, Sparki documentation

**Scribe MCP & Automation** (15+ files)
- Workflow orchestration, team training, operator guides

**Testing & Quality** (40+ files)
- Test architecture, infrastructure, coverage analysis

**Deployment & Infrastructure** (50+ files)
- Deployment strategies, Railway, Kubernetes, CI/CD pipelines

### Key Documentation Entry Points (Current)

**For Developers**:
- `/.claude/CLAUDE.md`
- `/README.md`
- `/.atlas/TODO/START_HERE.md`

**For Operations**:
- `/docs/SCRIBE_MCP_OPERATOR_GUIDE.md`
- `/.sparki/docs/operator-guide.md`
- `/docs/operations/deployment-runbook.md`

**For Architecture**:
- `/.sparki/docs/architecture-overview.md` (100KB)
- `/domain/docs/api/architecture/EVENT-SYSTEM.md`
- `/domain/shield/docs/ARCHITECTURE.md`

---

## IMPLEMENTATION STRATEGY

### Principles

1. **Centralization**: All documentation flows into `/docs/atlas/`
2. **Organization**: Clear hierarchical structure by category and service
3. **Discoverability**: Master indices, search metadata, cross-references
4. **Consistency**: Standardized naming, formatting, linking conventions
5. **Maintainability**: Clear ownership, update procedures, versioning
6. **Scalability**: Structure accommodates future documentation additions
7. **Accessibility**: Quick references for different user roles and tasks

### Target Architecture

```
/docs/atlas/
├── _config.json                    # Global configuration
├── _metadata.json                  # Atlas metadata
├── INDEX.md                        # Master table of contents
├── QUICK_START.md                  # Quick navigation guide
├── DOCUMENTATION_STANDARDS.md      # Contribution guidelines
│
├── architecture/                   # System & service architecture
│   ├── _metadata.json
│   ├── INDEX.md
│   ├── ARCHITECTURE_OVERVIEW.md
│   ├── C4_DIAGRAMS.md
│   ├── DESIGN_DECISIONS.md
│   └── [architecture docs]
│
├── operations/                     # Operations & runbooks
│   ├── _metadata.json
│   ├── INDEX.md
│   ├── OPERATIONS_HANDBOOK.md
│   ├── deployment/
│   ├── incident-response/
│   ├── troubleshooting/
│   ├── monitoring/
│   ├── scribe-mcp/
│   └── [operations docs]
│
├── development/                    # Development guides
│   ├── _metadata.json
│   ├── INDEX.md
│   ├── DEVELOPMENT_HANDBOOK.md
│   ├── api-reference/
│   ├── implementation-guides/
│   ├── quick-start/
│   └── [development docs]
│
├── testing/                        # Testing & quality
│   ├── _metadata.json
│   ├── INDEX.md
│   ├── TESTING_STANDARDS.md
│   ├── architecture/
│   ├── coverage-analysis/
│   └── [testing docs]
│
├── deployment/                     # Deployment & infrastructure
│   ├── _metadata.json
│   ├── INDEX.md
│   ├── DEPLOYMENT_HANDBOOK.md
│   ├── railway/
│   ├── kubernetes/
│   ├── ci-cd/
│   ├── disaster-recovery/
│   └── [deployment docs]
│
├── security/                       # Security & compliance
│   ├── _metadata.json
│   ├── INDEX.md
│   ├── SECURITY_HANDBOOK.md
│   ├── rbac/
│   ├── encryption/
│   ├── audit-trails/
│   └── [security docs]
│
├── services/                       # Service-specific documentation
│   ├── api/
│   │   ├── _metadata.json
│   │   ├── INDEX.md
│   │   ├── GUIDE.md
│   │   └── [API service docs]
│   ├── shield/
│   │   ├── _metadata.json
│   │   ├── INDEX.md
│   │   ├── GUIDE.md
│   │   └── [Shield service docs]
│   ├── relay/
│   │   ├── _metadata.json
│   │   ├── INDEX.md
│   │   ├── GUIDE.md
│   │   └── [Relay service docs]
│   ├── manuscript/
│   ├── printery/
│   ├── sparki/
│   └── [other services]
│
├── project-archive/                # Project management docs
│   ├── _metadata.json
│   ├── INDEX.md
│   ├── phases/
│   │   ├── phase-1-completion.md
│   │   ├── phase-2-completion.md
│   │   └── [phase reports]
│   ├── tasksets/
│   │   └── [taskset reports]
│   └── roadmap/
│
└── learning/                       # Tutorials, onboarding, team materials
    ├── _metadata.json
    ├── INDEX.md
    ├── onboarding/
    ├── tutorials/
    ├── team/
    └── [learning materials]
```

### Execution Timeline

**Total Duration**: 16-24 hours of focused execution

**Phases**:
- TASKSET 1: 2-3 hours
- TASKSET 2: 1-2 hours
- TASKSET 3: 2-3 hours (105 files)
- TASKSET 4: 2-3 hours (250 files)
- TASKSET 5: 2-3 hours (160 files)
- TASKSET 6: 2-3 hours (400+ files)
- TASKSET 7: 2-3 hours (8 indices)
- TASKSET 8: 2-3 hours (comprehensive audit)

---

## TASKSET 1: ANALYSIS & BASELINE

**Objective**: Create definitive inventory and design the target architecture

**Estimated Duration**: 2-3 hours

### Task 1.1: Generate Complete Documentation Inventory

**Actions**:
1. Scan all 12 documentation locations
2. Create structured inventory with:
   - File path (source location)
   - Filename
   - File size
   - Line count
   - Purpose/description
   - Category assignment
   - Status (active/deprecated/archive)
3. Identify all duplicates (same content in multiple locations)
4. Map cross-references (which documents reference which)

**Output**: `DOCUMENTATION_INVENTORY.json`
```json
[
  {
    "source_path": "/Users/alexarno/materi/docs/SCRIBE_MCP_OPERATOR_GUIDE.md",
    "filename": "SCRIBE_MCP_OPERATOR_GUIDE.md",
    "size_bytes": 12288,
    "line_count": 245,
    "purpose": "Daily operations guide for Scribe MCP",
    "category": "operations",
    "subcategory": "scribe-mcp",
    "status": "active",
    "target_path": "/docs/atlas/operations/scribe-mcp/OPERATOR_GUIDE.md",
    "duplicates": [],
    "references": ["ALERT_RESPONSE_GUIDE.md", "SCRIBE_TROUBLESHOOTING.md"],
    "referenced_by": ["INDEX.md", "QUICK_START.md"]
  }
]
```

### Task 1.2: Classify All Documents

**Actions**:
1. Assign all 400+ files to 8 categories (or subcategories)
2. Create classification matrix showing:
   - Category
   - File count
   - Total size
   - Active vs. deprecated
   - Dependencies
3. Identify archival candidates (old phase reports, deprecated implementations)
4. Flag documents needing updates

**Output**: `DOCUMENTATION_CLASSIFICATION.json`
```json
{
  "categories": {
    "architecture": {
      "count": 42,
      "size_mb": 1.2,
      "subcategories": ["system", "services", "design-decisions"],
      "active_files": 40,
      "deprecated_files": 2
    }
  }
}
```

### Task 1.3: Design Atlas Collection Structure

**Actions**:
1. Define 8 primary categories and subcategories
2. Create directory hierarchy (shown in Implementation Strategy above)
3. Define naming conventions:
   - Directory names: lowercase-with-hyphens
   - File names: UPPERCASE_WITH_UNDERSCORES.md
   - Index files: INDEX.md
   - Metadata files: _metadata.json
4. Plan URL/path schemes for consistent references

**Output**: `ATLAS_STRUCTURE_DESIGN.md`
```markdown
# Atlas Structure Design

## Directory Naming
- lowercase-with-hyphens for folders
- UPPERCASE_WITH_UNDERSCORES for files
- _metadata.json for category metadata
- INDEX.md for category table of contents

## Path Examples
- /docs/atlas/operations/scribe-mcp/OPERATOR_GUIDE.md
- /docs/atlas/services/api/ARCHITECTURE_GUIDE.md
- /docs/atlas/deployment/railway/SETUP_GUIDE.md
```

### Task 1.4: Create Migration Mapping

**Actions**:
1. Create comprehensive mapping of source → destination for all 400+ files
2. Identify content to consolidate (multiple sources = one destination)
3. Plan handling of duplicates (keep latest, merge, archive)
4. Mark files for deletion/archival

**Output**: `MIGRATION_MAPPING.csv`
```csv
source_path,filename,destination_path,action,notes
/docs/SCRIBE_MCP_OPERATOR_GUIDE.md,SCRIBE_MCP_OPERATOR_GUIDE.md,/docs/atlas/operations/scribe-mcp/OPERATOR_GUIDE.md,migrate,consolidate with sparki operator guide
/sparki/docs/operator-guide.md,operator-guide.md,/docs/atlas/operations/sparki/OPERATOR_GUIDE.md,migrate,merge into unified operations handbook
/.atlas/TODO/DEPLOYMENT_RUNBOOK.md,DEPLOYMENT_RUNBOOK.md,/docs/atlas/deployment/DEPLOYMENT_HANDBOOK.md,consolidate,merge with other deployment docs
```

### Task 1.5: Create Index of Deliverables

**Deliverables for TASKSET 1**:
- ✅ `DOCUMENTATION_INVENTORY.json` - Complete structured inventory (all 400+ files)
- ✅ `DOCUMENTATION_CLASSIFICATION.json` - Category breakdown and status
- ✅ `ATLAS_STRUCTURE_DESIGN.md` - Target architecture and directory hierarchy
- ✅ `MIGRATION_MAPPING.csv` - Source-to-destination mapping for all files
- ✅ `TASKSET_1_COMPLETION_REPORT.md` - Summary of findings and planning

**Location**: `/Users/alexarno/materi/docs/atlas/planning/`

---

## TASKSET 2: STRUCTURE & FOUNDATION

**Objective**: Build the centralized Platform Atlas collection framework

**Estimated Duration**: 1-2 hours

### Task 2.1: Create Atlas Directory Structure

**Actions**:
1. Create base directory: `/docs/atlas/`
2. Create 8 category directories:
   - `architecture/`
   - `operations/`
   - `development/`
   - `testing/`
   - `deployment/`
   - `security/`
   - `services/` (with subdirectories for each service)
   - `project-archive/`
3. Create `learning/` directory for onboarding/training
4. Create `planning/` directory for this consolidation project
5. Create `_old/` directory for deprecated/archived docs (with README)

**Commands**:
```bash
mkdir -p /docs/atlas/{architecture,operations,development,testing,deployment,security,services,project-archive,learning,planning,_old}
mkdir -p /docs/atlas/services/{api,shield,relay,manuscript,printery,sparki}
mkdir -p /docs/atlas/operations/{scribe-mcp,deployment,incident-response,troubleshooting,monitoring}
```

### Task 2.2: Establish Metadata & Configuration

**Actions**:
1. Create global `_config.json` with:
   - Atlas version and last updated date
   - Root navigation structure
   - Search configuration
   - Link resolver settings
2. Create category-level `_metadata.json` files with:
   - Category name and description
   - Owner/maintainer
   - Last updated date
   - File count
   - Related categories
3. Create `.gitignore` for Atlas (if using git)

**Output Files**:

`/docs/atlas/_config.json`:
```json
{
  "atlas": {
    "version": "1.0.0",
    "last_updated": "2026-01-08",
    "title": "Platform Atlas - Centralized Documentation",
    "description": "Unified documentation collection for Materi platform"
  },
  "navigation": {
    "root_categories": [
      "architecture", "operations", "development", "testing",
      "deployment", "security", "services", "project-archive", "learning"
    ]
  },
  "search": {
    "enabled": true,
    "index_type": "markdown-frontmatter",
    "fields": ["title", "description", "tags", "keywords"]
  },
  "link_resolver": {
    "base_path": "/docs/atlas/",
    "use_relative_paths": true,
    "auto_breadcrumbs": true
  }
}
```

`/docs/atlas/operations/_metadata.json`:
```json
{
  "category": "operations",
  "title": "Operations & Runbooks",
  "description": "Operational procedures, runbooks, and incident response guides",
  "owner": "Platform Operations Team",
  "last_updated": "2026-01-08",
  "file_count": 45,
  "subcategories": ["scribe-mcp", "deployment", "incident-response", "troubleshooting", "monitoring"],
  "related_categories": ["deployment", "security", "development"]
}
```

### Task 2.3: Build Master Index & Navigation

**Actions**:
1. Create main `INDEX.md` with:
   - Platform Atlas introduction
   - 9-category navigation (8 categories + learning)
   - Service directory
   - Quick links by role
   - Search instructions
2. Create category landing pages (8 pages)
   - One for each category
   - List all files in category
   - Cross-references to related categories
   - Purpose and use cases

**Output Files**:

`/docs/atlas/INDEX.md` - Master table of contents

`/docs/atlas/architecture/INDEX.md` - Architecture category landing page

`/docs/atlas/operations/INDEX.md` - Operations category landing page

(and 6 more for other categories)

### Task 2.4: Set Up Documentation Standards

**Actions**:
1. Create `DOCUMENTATION_STANDARDS.md` defining:
   - File naming conventions
   - Frontmatter/metadata format
   - Markdown style guide
   - Code block formatting
   - Link conventions (relative vs. absolute)
   - Table of contents requirements
   - Cross-reference patterns
   - Version control strategy
2. Create template files:
   - `_TEMPLATE_GUIDE.md` - For how-to guides
   - `_TEMPLATE_REFERENCE.md` - For reference documentation
   - `_TEMPLATE_RUNBOOK.md` - For operational runbooks
   - `_TEMPLATE_ARCHITECTURE.md` - For architecture documentation
3. Create contribution guidelines
4. Link to Scribe MCP documentation standards

**Output Files**:
- `DOCUMENTATION_STANDARDS.md`
- `_TEMPLATE_GUIDE.md`
- `_TEMPLATE_REFERENCE.md`
- `_TEMPLATE_RUNBOOK.md`
- `_TEMPLATE_ARCHITECTURE.md`
- `CONTRIBUTION_GUIDELINES.md`

### Task 2.5: Create Atlas Foundation Summary

**Deliverables for TASKSET 2**:
- ✅ Complete `/docs/atlas/` directory structure created
- ✅ `_config.json` and category `_metadata.json` files
- ✅ Master `INDEX.md` with searchable structure
- ✅ 8 category landing pages
- ✅ `DOCUMENTATION_STANDARDS.md`
- ✅ Template files for all documentation types
- ✅ `TASKSET_2_COMPLETION_REPORT.md`

**Location**: `/docs/atlas/` (all files)

---

## TASKSET 3: CONTENT MIGRATION - PHASE 1

**Objective**: Migrate 40% of high-priority documentation (105 files)

**Estimated Duration**: 2-3 hours

**Focus Areas**:
- Architecture & Design documentation (40 files)
- Operations & Runbooks (35 files)
- Scribe MCP documentation (15 files)
- Quality: 105 files total

### Task 3.1: Migrate Architecture & Design Docs

**Actions**:
1. Move all architecture documentation to `/docs/atlas/architecture/`
2. Consolidate from sources:
   - `/.sparki/docs/architecture-overview.md` → `/docs/atlas/architecture/ARCHITECTURE_OVERVIEW.md`
   - `/domain/docs/api/architecture/` → `/docs/atlas/architecture/services/`
   - `/domain/shield/docs/ARCHITECTURE.md` → `/docs/atlas/architecture/services/shield/`
   - Design journals from `/.atlas/TODO/` → `/docs/atlas/architecture/DESIGN_DECISIONS.md` (consolidated)
3. Create consolidated `ARCHITECTURE_OVERVIEW.md` if multiple exist
4. Create C4 diagram reference document
5. Create design decision log

**Files to Migrate**: ~40 files
- C4 diagrams and architecture visualizations
- Design decision documents
- Architecture decision records (ADRs)
- Service-specific architecture docs
- System design documents

### Task 3.2: Migrate Operations & Runbooks

**Actions**:
1. Move all operations documentation to `/docs/atlas/operations/`
2. Consolidate from sources:
   - `/docs/SCRIBE_MCP_OPERATOR_GUIDE.md` → `/docs/atlas/operations/scribe-mcp/OPERATOR_GUIDE.md`
   - `/.sparki/docs/operator-guide.md` → Merge with above
   - `/docs/operations/deployment-runbook.md` → `/docs/atlas/deployment/DEPLOYMENT_RUNBOOK.md`
   - `/docs/MATERI-GRAFANA-RUNBOOK.md` → `/docs/atlas/operations/monitoring/GRAFANA_RUNBOOK.md`
3. Create unified `OPERATIONS_HANDBOOK.md`
4. Organize subcategories: deployment, incident-response, troubleshooting, monitoring

**Files to Migrate**: ~35 files
- Operator guides
- Monitoring and alerting procedures
- Troubleshooting guides
- Incident response procedures
- On-call runbooks
- Escalation procedures

### Task 3.3: Consolidate Scribe MCP Documentation

**Actions**:
1. Move all Scribe MCP docs to `/docs/atlas/operations/scribe-mcp/`
2. Consolidate from sources:
   - `/docs/SCRIBE_MCP_OPERATOR_GUIDE.md`
   - `/docs/ALERT_RESPONSE_GUIDE.md`
   - `/docs/SCRIBE_TROUBLESHOOTING.md`
   - `/docs/SCRIBE_TEAM_TRAINING.md`
   - `/platform/intelligence/.scribe/PHASE_*_REPORT.md` (8 files)
3. Create consolidated `SCRIBE_MCP_DEPLOYMENT_REPORT.md` (merge all phase reports)
4. Organize by: Guides, Alerts, Troubleshooting, Training, Deployment History

**Files to Migrate**: ~15 files
- Operator guide
- Alert response procedures
- Troubleshooting guide
- Team training materials
- 8 Phase completion reports → 1 consolidated report

### Task 3.4: Update All Cross-References

**Actions**:
1. Scan all migrated files for internal links
2. Update links to point to new locations in `/docs/atlas/`
3. Create mapping of old paths to new paths
4. Test navigation to ensure links work
5. Document any external references

**Verification Checklist**:
- [ ] All internal links updated
- [ ] No broken links within migrated files
- [ ] Cross-references between migrated and non-migrated docs work
- [ ] Relative path links are consistent

### Task 3.5: Create Migration Report

**Deliverables for TASKSET 3**:
- ✅ 40 architecture files migrated
- ✅ 35 operations files migrated
- ✅ 15 Scribe MCP files migrated
- ✅ All cross-references updated (verified)
- ✅ `/docs/atlas/architecture/` populated
- ✅ `/docs/atlas/operations/` populated
- ✅ `/docs/atlas/operations/scribe-mcp/` populated
- ✅ `TASKSET_3_COMPLETION_REPORT.md` with verification results

**Location**: `/docs/atlas/`

---

## TASKSET 4: CONTENT MIGRATION - PHASE 2

**Objective**: Migrate 40% of remaining documentation (250 files)

**Estimated Duration**: 2-3 hours

**Focus Areas**:
- Development & Implementation (60 files)
- Testing & Quality (40 files)
- Project Management (100 files)
- Deployment & Infrastructure (50 files)

### Task 4.1: Migrate Development & Implementation Docs

**Actions**:
1. Move development docs to `/docs/atlas/development/`
2. Consolidate from sources:
   - `/.atlas/TODO/` implementation guides → `/docs/atlas/development/IMPLEMENTATION_GUIDES/`
   - API references from `/domain/docs/api/` → `/docs/atlas/services/api/`
   - Quick starts from `/atlas/TODO/QUICK_START*.md` → `/docs/atlas/development/QUICK_START/`
   - Development setup from `/domain/docs/` → `/docs/atlas/development/SETUP_GUIDE.md`
3. Create `DEVELOPMENT_HANDBOOK.md`
4. Organize by subcategories: api-reference, implementation-guides, quick-start, setup

**Files to Migrate**: ~60 files
- API references and documentation
- Implementation guides
- Development setup instructions
- Quick start guides
- Code examples and tutorials

### Task 4.2: Migrate Testing & Quality Documentation

**Actions**:
1. Move testing docs to `/docs/atlas/testing/`
2. Consolidate from sources:
   - `/.atlas/TODO/TESTING_INFRASTRUCTURE*.md` → `/docs/atlas/testing/TESTING_STANDARDS.md`
   - `/domain/docs/*/testing/` → `/docs/atlas/testing/`
   - Test coverage analysis from `/.atlas/TODO/` → `/docs/atlas/testing/COVERAGE_ANALYSIS.md`
3. Create unified `TESTING_STANDARDS.md`
4. Organize test reports by service

**Files to Migrate**: ~40 files
- Test architecture and infrastructure
- Testing standards and guidelines
- Test coverage analysis
- Testing implementation reports
- Quality assurance procedures

### Task 4.3: Organize Project Management Documentation

**Actions**:
1. Move phase/taskset reports to `/docs/atlas/project-archive/`
2. Create subdirectories:
   - `/phases/` - Phase completion reports
   - `/tasksets/` - Taskset completion reports
   - `/roadmap/` - Roadmap and planning documents
3. Create summary index for each phase
4. Archive older sprint reports

**Files to Migrate**: ~100 files
- Phase 1-8 completion reports
- Taskset 1-9 completion reports
- Roadmap documents
- Sprint summaries
- Project status documents

### Task 4.4: Migrate Deployment & Infrastructure

**Actions**:
1. Move deployment docs to `/docs/atlas/deployment/`
2. Consolidate from sources:
   - `DEPLOYMENT_QUICK_START.md` (root) → `/docs/atlas/deployment/QUICK_START.md`
   - `/.atlas/TODO/RAILWAY_DEPLOYMENT*.md` → `/docs/atlas/deployment/railway/`
   - `/.atlas/TODO/CI_CD*.md` → `/docs/atlas/deployment/ci-cd/`
   - Kubernetes docs → `/docs/atlas/deployment/kubernetes/`
   - Disaster recovery → `/docs/atlas/security/DISASTER_RECOVERY.md`
3. Create unified `DEPLOYMENT_HANDBOOK.md`
4. Consolidate duplicate runbooks

**Files to Migrate**: ~50 files
- Deployment strategies and guides
- Railway deployment documentation
- Kubernetes manifests and guides
- CI/CD pipeline documentation
- Disaster recovery procedures
- Infrastructure as code documentation

### Task 4.5: Create Migration Report

**Deliverables for TASKSET 4**:
- ✅ 60 development files migrated
- ✅ 40 testing files migrated
- ✅ 100 project management files migrated
- ✅ 50 deployment files migrated
- ✅ `/docs/atlas/development/` populated
- ✅ `/docs/atlas/testing/` populated
- ✅ `/docs/atlas/deployment/` populated
- ✅ `/docs/atlas/project-archive/` populated
- ✅ All cross-references updated (verified)
- ✅ `TASKSET_4_COMPLETION_REPORT.md` with verification results

**Location**: `/docs/atlas/`

---

## TASKSET 5: CONTENT MIGRATION - PHASE 3

**Objective**: Migrate remaining documentation (160 files)

**Estimated Duration**: 2-3 hours

**Focus Areas**:
- Service-Specific Documentation (120 files)
- Security & Compliance (15 files)
- Miscellaneous (25 files)

### Task 5.1: Migrate Service-Specific Documentation

**Actions**:
1. Create service-specific directories under `/docs/atlas/services/`
2. Organize by service:
   - **API Service** (`/docs/atlas/services/api/`)
     - Move from `/domain/docs/api/` and root API docs
     - Create `API_GUIDE.md` consolidating READMEs
     - Organize by subcategories: architecture, testing, deployment
   - **Shield Service** (`/docs/atlas/services/shield/`)
     - Move from `/domain/shield/docs/`
     - Create `SHIELD_GUIDE.md`
   - **Relay Service** (`/docs/atlas/services/relay/`)
     - Move from `/domain/relay/docs/`
     - Create `RELAY_GUIDE.md`
   - **Manuscript, Printery, Sparki** (similar structure)

3. Create service landing pages with:
   - Service overview
   - Architecture diagram
   - API/interface documentation
   - Deployment procedures
   - Known issues and limitations

**Files to Migrate**: ~120 files
- Service architecture documentation
- API references
- Integration guides
- Service-specific deployment docs
- Service-specific testing docs
- Service-specific troubleshooting

### Task 5.2: Migrate Security & Compliance Documentation

**Actions**:
1. Move security docs to `/docs/atlas/security/`
2. Consolidate from sources:
   - RBAC documentation → `/docs/atlas/security/RBAC.md`
   - Encryption docs → `/docs/atlas/security/ENCRYPTION.md`
   - Audit trail docs → `/docs/atlas/security/AUDIT_TRAILS.md`
   - Disaster recovery → `/docs/atlas/security/DISASTER_RECOVERY.md`
   - Compliance docs → `/docs/atlas/security/COMPLIANCE.md`
3. Create unified `SECURITY_HANDBOOK.md`

**Files to Migrate**: ~15 files
- RBAC and access control
- Encryption and TLS
- Audit logging
- Disaster recovery procedures
- Compliance documentation
- Security standards

### Task 5.3: Migrate Miscellaneous Documentation

**Actions**:
1. Organize remaining documents:
   - **Team & Hiring** → `/docs/atlas/learning/team/`
     - Interview scripts and materials
     - Team guides
     - Hiring documentation
   - **Roadmap & Planning** → `/docs/atlas/project-archive/roadmap/`
     - Roadmap documents
     - Strategic planning
     - Long-term planning
   - **Demos & Tutorials** → `/docs/atlas/learning/tutorials/`
     - Code examples
     - Demo walkthroughs
     - Educational materials
   - **Miscellaneous** → `/docs/atlas/learning/` or archive as needed

**Files to Migrate**: ~25 files
- Team and hiring materials
- Onboarding documentation
- Training materials
- Roadmap documents
- Demo and tutorial materials

### Task 5.4: Set Up Service Documentation Indices

**Actions**:
1. Create `INDEX.md` for each service:
   - API service index
   - Shield service index
   - Relay service index
   - Other service indices
2. Create service relationship map
   - Which services depend on which
   - Integration points
   - Data flow between services
3. Cross-reference between service docs

**Output Files**:
- `/docs/atlas/services/INDEX.md` - Main services index
- `/docs/atlas/services/api/INDEX.md` - API service index
- `/docs/atlas/services/shield/INDEX.md` - Shield service index
- (and similar for other services)

### Task 5.5: Create Migration Report

**Deliverables for TASKSET 5**:
- ✅ 120 service-specific files migrated
- ✅ 15 security files migrated
- ✅ 25 miscellaneous files migrated
- ✅ `/docs/atlas/services/` fully populated with 7 service directories
- ✅ `/docs/atlas/security/` populated
- ✅ `/docs/atlas/learning/` populated
- ✅ Service indices created
- ✅ Service relationship map created
- ✅ All cross-references updated (verified)
- ✅ `TASKSET_5_COMPLETION_REPORT.md` with verification results

**Location**: `/docs/atlas/`

---

## TASKSET 6: CROSS-REFERENCE & LINK MANAGEMENT

**Objective**: Ensure all documentation is properly interconnected and navigable

**Estimated Duration**: 2-3 hours

### Task 6.1: Audit All Internal Links

**Actions**:
1. Create script to scan all files for internal links
2. Extract all links and their targets
3. Create mapping of:
   - Source file → linked files
   - Broken links (targets don't exist)
   - Dead ends (files with no incoming links)
4. Categorize links by type:
   - Links to other Atlas docs
   - Links to external resources
   - Links to files outside Atlas
5. Generate audit report

**Output Files**:
- `LINK_AUDIT_REPORT.md` - Comprehensive link analysis
- `BROKEN_LINKS.json` - All broken links with fix recommendations
- `DEAD_END_DOCUMENTS.json` - Files with no incoming links

### Task 6.2: Implement Link Standards

**Actions**:
1. Establish link conventions:
   - Use relative paths for all internal links: `../other-file.md`
   - Use markdown link format: `[text](path/to/file.md)`
   - Create consistent anchor naming: `### section-name` → `#section-name`
2. Update all non-compliant links
3. Create link resolver configuration
4. Document special linking cases

**Output Files**:
- `LINK_STANDARDS.md` - Link convention documentation
- `LINK_RESOLVER_CONFIG.json` - Resolver configuration
- Updated all files with standardized links

### Task 6.3: Build Relationship Maps

**Actions**:
1. Create cross-service dependency documentation
2. Build architecture relationship diagrams (text-based)
3. Create data flow documentation
4. Document service-to-service integration points
5. Create impact analysis docs (changes to X affect Y)

**Output Files**:
- `/docs/atlas/architecture/SERVICE_DEPENDENCIES.md`
- `/docs/atlas/architecture/DATA_FLOW_DIAGRAM.md`
- `/docs/atlas/services/INTEGRATION_MAP.md`

### Task 6.4: Create Breadcrumb Navigation

**Actions**:
1. Add breadcrumb metadata to every document
2. Implement parent/child relationships
3. Add "up" links to parent category pages
4. Add "related documents" sections
5. Create navigation consistency

**Frontmatter Format**:
```markdown
---
title: Document Title
category: operations
subcategory: scribe-mcp
parent: ../INDEX.md
related:
  - ../ALERT_RESPONSE_GUIDE.md
  - ../troubleshooting/QUICK_FIX.md
tags: [scribe, operations, guides]
---
```

### Task 6.5: Fix All Links & Verify

**Actions**:
1. Update all broken links
2. Fix all relative path issues
3. Verify all links work (manual spot check)
4. Test navigation paths from different documents
5. Create final link audit report

**Deliverables for TASKSET 6**:
- ✅ `LINK_AUDIT_REPORT.md` (comprehensive)
- ✅ `LINK_STANDARDS.md` (documented)
- ✅ All 400+ files updated with standardized links
- ✅ No broken internal links
- ✅ Cross-service relationship maps created
- ✅ Breadcrumbs added to all documents
- ✅ Navigation tested and verified
- ✅ `TASKSET_6_COMPLETION_REPORT.md` with link verification results

**Location**: `/docs/atlas/`

---

## TASKSET 7: INDEX & NAVIGATION

**Objective**: Create comprehensive navigation and searchability

**Estimated Duration**: 2-3 hours

### Task 7.1: Build Master Index System

**Actions**:
1. Update main `INDEX.md` with:
   - 400+ searchable documents
   - Tagging system
   - Category quick links
   - Most popular documents
   - New/updated documents
2. Create 8 category indices:
   - `/docs/atlas/architecture/INDEX.md`
   - `/docs/atlas/operations/INDEX.md`
   - `/docs/atlas/development/INDEX.md`
   - `/docs/atlas/testing/INDEX.md`
   - `/docs/atlas/deployment/INDEX.md`
   - `/docs/atlas/security/INDEX.md`
   - `/docs/atlas/services/INDEX.md`
   - `/docs/atlas/project-archive/INDEX.md`
3. Create service indices (one per service)
4. Create topic indices (cross-category topics)

**Output Files**:
- `/docs/atlas/INDEX.md` - Master index (searchable, tagged)
- 8 category landing pages
- 7 service landing pages
- Topic-based indices

### Task 7.2: Implement Search Infrastructure

**Actions**:
1. Create searchable metadata for all 400+ documents
2. Build keyword index
3. Create tag taxonomy
4. Document indexing strategy
5. Create search guide for users

**Metadata Format**:
```json
{
  "title": "Scribe MCP Operator Guide",
  "description": "Daily operations guide for Scribe MCP infrastructure",
  "path": "/docs/atlas/operations/scribe-mcp/OPERATOR_GUIDE.md",
  "keywords": ["scribe", "operations", "mcp", "monitoring", "alerts"],
  "tags": ["operations", "guide", "production", "scribe-mcp"],
  "category": "operations",
  "audience": ["operators", "devops", "platform-team"],
  "last_updated": "2026-01-08"
}
```

**Output Files**:
- `SEARCH_METADATA.json` - Metadata for all 400+ files
- `SEARCH_INDEX.md` - Keyword search guide
- `TAG_TAXONOMY.md` - Complete tag system

### Task 7.3: Create Quick Reference System

**Actions**:
1. Create `/docs/atlas/QUICK_START.md`
   - Quick navigation guide
   - Common user paths
   - Search shortcuts
2. Create `/docs/atlas/QUICK_REFERENCE_BY_ROLE.md`
   - For developers (what to read first)
   - For operators (on-call quick refs)
   - For architects (architecture overviews)
   - For DBAs (database guides)
3. Create `/docs/atlas/QUICK_REFERENCE_BY_TASK.md`
   - Deploy the system
   - Respond to alerts
   - Troubleshoot issues
   - Monitor performance
   - Add new features
   - Add new service
4. Create service quick references

**Output Files**:
- `QUICK_START.md`
- `QUICK_REFERENCE_BY_ROLE.md`
- `QUICK_REFERENCE_BY_TASK.md`
- Service quick references

### Task 7.4: Build Documentation Maps

**Actions**:
1. Create visual hierarchy diagrams (text-based markdown)
2. Create service dependency maps
3. Create user journey maps:
   - Developer journey
   - Operator/on-call journey
   - Architect journey
4. Create learning path recommendations
5. Create "how to get started" flowcharts

**Output Files**:
- `/docs/atlas/DOCUMENTATION_MAP.md` (visual hierarchy)
- `/docs/atlas/services/SERVICE_DEPENDENCY_MAP.md`
- `/docs/atlas/learning/DEVELOPER_LEARNING_PATH.md`
- `/docs/atlas/learning/OPERATOR_LEARNING_PATH.md`
- `/docs/atlas/learning/ARCHITECT_LEARNING_PATH.md`

### Task 7.5: Create Navigation Summary

**Deliverables for TASKSET 7**:
- ✅ Enhanced `INDEX.md` (all 400+ docs searchable and tagged)
- ✅ 8 category landing pages (fully populated)
- ✅ 7 service landing pages (with cross-references)
- ✅ Quick reference documents (3 main + service-specific)
- ✅ Search metadata for all documents
- ✅ Documentation maps (visual hierarchy, dependencies, learning paths)
- ✅ Navigation tested and verified (5 user paths tested)
- ✅ `TASKSET_7_COMPLETION_REPORT.md` with navigation verification

**Location**: `/docs/atlas/`

---

## TASKSET 8: VERIFICATION & QUALITY ASSURANCE

**Objective**: Ensure completeness, consistency, and correctness

**Estimated Duration**: 2-3 hours

### Task 8.1: Comprehensive Audit

**Actions**:
1. Verify all 400+ files migrated/archived
2. Check for orphaned documents
3. Identify content gaps
4. Validate metadata completeness
5. Create audit checklist

**Verification Items**:
- [ ] All 400+ documented files processed
- [ ] No orphaned files left in old locations
- [ ] No duplicate files in new location
- [ ] All files have appropriate metadata
- [ ] All files in correct category
- [ ] No missing critical documentation
- [ ] Archive structure correct (`_old/` directory)

**Output Files**:
- `AUDIT_CHECKLIST.md` - Complete verification checklist
- `AUDIT_RESULTS.json` - Automated audit results

### Task 8.2: Link & Reference Validation

**Actions**:
1. Run comprehensive link check on all documents
2. Verify cross-references work
3. Check external links (report status)
4. Test navigation paths from different documents
5. Validate all breadcrumbs

**Testing Scenarios**:
- [ ] Navigate from root INDEX to every category
- [ ] Navigate from each category to each document
- [ ] Follow cross-references between documents
- [ ] Test related documents sections
- [ ] Verify service dependency maps are accurate

**Output Files**:
- `LINK_VERIFICATION_REPORT.md`
- `NAVIGATION_TEST_RESULTS.md`

### Task 8.3: Content Quality Review

**Actions**:
1. Check for outdated information
2. Identify documents needing updates
3. Flag inconsistencies across docs
4. Review for completeness
5. Check for style/formatting consistency

**Quality Checks**:
- [ ] No references to deprecated systems
- [ ] No broken links
- [ ] Consistent formatting across category
- [ ] All documents have proper headings
- [ ] Code examples are formatted correctly
- [ ] File paths are current
- [ ] Product/service names are consistent

**Output Files**:
- `CONTENT_QUALITY_REPORT.md`
- `DOCUMENTATION_UPDATES_NEEDED.md` - Documents needing refresh

### Task 8.4: User Testing & Feedback

**Actions**:
1. Test documentation findability (5 key scenarios)
2. Test navigation flow for different user types
3. Gather team feedback
4. Validate index accuracy
5. Test search functionality (if implemented)

**Testing Scenarios**:
- Scenario 1: New developer finds setup instructions
- Scenario 2: On-call engineer finds alert procedures
- Scenario 3: Architect finds service architecture docs
- Scenario 4: Operator finds deployment runbook
- Scenario 5: Team member finds their service docs

**Feedback Collection**:
- Developer feedback
- Operations feedback
- Architecture feedback
- Management feedback

**Output Files**:
- `USER_TESTING_REPORT.md`
- `FEEDBACK_SUMMARY.md`

### Task 8.5: Final Cleanup & Handoff

**Actions**:
1. Delete or archive temporary/old locations
   - Move old docs to `/docs/atlas/_old/` with `README.md`
   - Create `_old/README.md` explaining deprecation
   - Keep old paths as git history (don't delete from git)
2. Create redirects for moved files (if using web-based docs)
3. Document any special cases
4. Create post-consolidation maintenance guide
5. Update main `README.md` to point to Atlas

**Output Files**:
- `CONSOLIDATION_COMPLETE_CHECKLIST.md`
- `/docs/atlas/_old/README.md` - Archive explanation
- `POST_CONSOLIDATION_MAINTENANCE_GUIDE.md`
- Updated main `/Users/alexarno/materi/README.md`

### Task 8.6: Create Final Reports

**Deliverables for TASKSET 8**:
- ✅ `CONSOLIDATION_AUDIT_REPORT.md` (comprehensive)
- ✅ `MIGRATION_COMPLETION_REPORT.md` (executive summary)
- ✅ `LINK_VERIFICATION_REPORT.md` (all links verified)
- ✅ `CONTENT_QUALITY_REPORT.md` (consistency check)
- ✅ `USER_TESTING_REPORT.md` (5 scenarios validated)
- ✅ `POST_CONSOLIDATION_MAINTENANCE_GUIDE.md`
- ✅ All 400+ files verified in final location
- ✅ Navigation tested and working
- ✅ Quality assurance checklist complete (all items checked)
- ✅ `TASKSET_8_COMPLETION_REPORT.md` (final summary)

**Location**: `/docs/atlas/` and `/Users/alexarno/materi/docs/`

---

## EXECUTION ROADMAP

### Sequential Execution Flow

```
START
  ↓
TASKSET 1: Analysis & Baseline (2-3 hours)
├─ Generate complete inventory (400+ files)
├─ Classify all documents
├─ Design target architecture
├─ Create migration mapping
└─ Request confirmation
  ↓ (GO TASKSET 2)
TASKSET 2: Structure & Foundation (1-2 hours)
├─ Create /docs/atlas/ directory structure
├─ Establish metadata & configuration
├─ Build master index & navigation
├─ Set up documentation standards
└─ Request confirmation
  ↓ (GO TASKSET 3)
TASKSET 3: Content Migration - Phase 1 (2-3 hours, 105 files)
├─ Migrate architecture docs (40 files)
├─ Migrate operations docs (35 files)
├─ Consolidate Scribe MCP docs (15 files)
├─ Update all cross-references
└─ Request confirmation
  ↓ (GO TASKSET 4)
TASKSET 4: Content Migration - Phase 2 (2-3 hours, 250 files)
├─ Migrate development docs (60 files)
├─ Migrate testing docs (40 files)
├─ Organize project management (100 files)
├─ Migrate deployment docs (50 files)
└─ Request confirmation
  ↓ (GO TASKSET 5)
TASKSET 5: Content Migration - Phase 3 (2-3 hours, 160 files)
├─ Migrate service-specific docs (120 files)
├─ Migrate security docs (15 files)
├─ Migrate miscellaneous (25 files)
├─ Set up service indices
└─ Request confirmation
  ↓ (GO TASKSET 6)
TASKSET 6: Cross-Reference & Link Management (2-3 hours, 400+ files)
├─ Audit all internal links
├─ Implement link standards
├─ Build relationship maps
├─ Create breadcrumb navigation
└─ Request confirmation
  ↓ (GO TASKSET 7)
TASKSET 7: Index & Navigation (2-3 hours, 8 indices)
├─ Build master index system
├─ Implement search infrastructure
├─ Create quick reference system
├─ Build documentation maps
└─ Request confirmation
  ↓ (GO TASKSET 8)
TASKSET 8: Verification & QA (2-3 hours, comprehensive audit)
├─ Comprehensive audit (400+ files)
├─ Link & reference validation
├─ Content quality review
├─ User testing & feedback
├─ Final cleanup & handoff
└─ COMPLETION
  ↓
PROJECT COMPLETE
All 400+ files consolidated into /docs/atlas/ ✅
```

### Timeline Summary

| Phase | Duration | Files | Key Activities |
|-------|----------|-------|-----------------|
| TASKSET 1 | 2-3h | Analysis | Inventory, classification, design, mapping |
| TASKSET 2 | 1-2h | Foundation | Directory structure, config, standards |
| TASKSET 3 | 2-3h | 105 files | Architecture, operations, Scribe MCP |
| TASKSET 4 | 2-3h | 250 files | Development, testing, project, deployment |
| TASKSET 5 | 2-3h | 160 files | Services, security, miscellaneous |
| TASKSET 6 | 2-3h | 400+ files | Links, relationships, breadcrumbs |
| TASKSET 7 | 2-3h | 8 indices | Master index, search, quick refs, maps |
| TASKSET 8 | 2-3h | Audit | Verification, validation, QA, testing |
| **TOTAL** | **16-24h** | **400+** | **Complete consolidation** |

---

## SUCCESS CRITERIA

### Completeness
- ✅ All 400+ documentation files consolidated into `/docs/atlas/`
- ✅ Zero orphaned files left in old locations
- ✅ All duplicate content handled (consolidated or archived)
- ✅ No missing critical documentation

### Organization
- ✅ 8 category hierarchies with proper organization
- ✅ 7 service-specific directories with dedicated docs
- ✅ Consistent naming conventions (directories and files)
- ✅ Metadata complete for all documents

### Discoverability
- ✅ Master index with all 400+ documents searchable and tagged
- ✅ Service-specific indices populated
- ✅ Topic-based indices created
- ✅ Quick reference materials for all user roles
- ✅ Search metadata and keyword index complete

### Navigation
- ✅ Breadcrumb navigation on all documents
- ✅ Parent-child relationships established
- ✅ Related documents sections populated
- ✅ Cross-service dependency maps created
- ✅ User journey maps created (developer, operator, architect)

### Quality
- ✅ All internal links verified and working
- ✅ No broken links
- ✅ Content quality review completed
- ✅ Consistency checks passed
- ✅ Documentation standards enforced

### Verification
- ✅ User testing: 5 key scenarios validated
- ✅ Navigation tested and working
- ✅ All team feedback incorporated
- ✅ Quality assurance checklist complete

### Accessibility
- ✅ Documentation standards guide created
- ✅ Contribution guidelines established
- ✅ Template files available for new docs
- ✅ Post-consolidation maintenance guide created

### Final Status
- ✅ All files migrated and verified
- ✅ Old locations cleaned up/archived
- ✅ New Atlas collection fully operational
- ✅ Team trained on new structure
- ✅ Ready for ongoing maintenance

---

## NOTES FOR WORKSPACE AGENTS

### How to Use This Document

1. **Before Starting**: Read this entire document to understand the complete strategy
2. **Per Taskset**: Follow the explicit task breakdown
3. **Request Confirmation**: After each taskset, wait for user confirmation (e.g., "GO TASKSET 2")
4. **Track Progress**: Update todo list as you complete tasks
5. **Generate Reports**: Create completion reports for each taskset
6. **Verify Deliverables**: Ensure all outputs match expectations

### Key Principles

- **Sequential Execution**: Complete tasksets in order (1→2→3→...→8)
- **Verification**: Verify each task before moving to next
- **Discoverability**: Create clear navigation at each stage
- **Consistency**: Apply standards uniformly across all documents
- **Quality Assurance**: Test navigation and links before proceeding
- **Documentation**: Create completion reports for each taskset

### Success Indicators

Each taskset is complete when:
- ✅ All specified files processed/migrated
- ✅ Deliverables created and verified
- ✅ Completion report generated
- ✅ No blocking issues remain

---

## APPENDIX: KEY LOCATIONS & MAPPINGS

### Source Locations (12 major locations)

```
/Users/alexarno/materi/                           (32 files - root docs)
/Users/alexarno/materi/docs/                      (10 files - main docs)
/Users/alexarno/materi/docs/operations/           (4 files)
/Users/alexarno/materi/.atlas/TODO/               (280+ files - phases/tasksets)
/Users/alexarno/materi/domain/docs/               (150+ files - services)
/Users/alexarno/materi/.clari/                    (30+ files - completion reports)
/Users/alexarno/materi/.sparki/docs/              (10 core files)
/Users/alexarno/materi/.scribe/                   (4 files)
/Users/alexarno/materi/domain/docs/api/           (API service docs)
/Users/alexarno/materi/domain/docs/shield/        (Shield service docs)
/Users/alexarno/materi/domain/docs/relay/         (Relay service docs)
/Users/alexarno/materi/platform/intelligence/     (Other docs)
```

### Target Location

```
/Users/alexarno/materi/docs/atlas/                (All 400+ files)
```

### Key Destination Directories

```
/docs/atlas/
├── architecture/              ← 40 files (design, decisions, C4)
├── operations/                ← 35 files (ops, runbooks, monitoring)
│   └── scribe-mcp/           ← 15 files (Scribe MCP guides)
├── development/               ← 60 files (implementation, API refs)
├── testing/                   ← 40 files (test architecture, coverage)
├── deployment/                ← 50 files (deployment, CI/CD, K8s)
├── security/                  ← 15 files (RBAC, encryption, compliance)
├── services/                  ← 120 files (service-specific)
│   ├── api/
│   ├── shield/
│   ├── relay/
│   ├── manuscript/
│   ├── printery/
│   └── sparki/
├── project-archive/           ← 100 files (phases, tasksets, roadmap)
├── learning/                  ← 25 files (onboarding, tutorials, team)
└── planning/                  ← (Consolidation project docs)
```

---

**Document Version**: 1.0
**Last Updated**: 2026-01-08
**Status**: Ready for Execution
**Next Step**: Send "GO TASKSET 1" to begin TASKSET 1: Analysis & Baseline
