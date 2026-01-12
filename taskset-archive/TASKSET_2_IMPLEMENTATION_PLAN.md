# TASKSET 2: Deployers - Implementation Plan

**Date**: January 8, 2026
**Status**: In Progress
**Target**: Implement real Vercel, Railway, and Folio deployers with API integrations
**Previous**: TASKSET 1 completed - CLI framework and manifest system in place

---

## Executive Summary

TASKSET 2 replaces the stub implementations from TASKSET 1 with real API integrations:

### From TASKSET 1 (Stubs)
- Stage executors were simulation/placeholder implementations
- Validation checks were empty stubs
- No actual API calls to Vercel, Railway, or Folio

### To TASKSET 2 (Real Implementations)
- Vercel API deployer - Deploy frontend to Vercel
- Railway CLI deployer - Deploy backend services via Railway CLI
- Folio API deployer - Configure observability (Prometheus scraping, dashboards, alerts)
- Real validation checks - Verify Docker images, database connectivity, API access
- Real rollback mechanisms - Revert deployments on failure
- Shield CICD webhook integration - Record unified deployment records

---

## Architecture

### Layer 1: Deployer Interfaces (New)
```
├── vercel_deployer.go       [NEW] Vercel API client implementation
├── railway_deployer.go      [NEW] Railway CLI wrapper implementation
├── folio_deployer.go        [NEW] Folio API configuration deployer
└── deployer.go              [NEW] Common deployer interface
```

### Layer 2: External API Clients (New)
```
├── clients/
│   ├── vercel_client.go     [NEW] Vercel REST API client
│   ├── railway_client.go    [NEW] Railway CLI wrapper
│   ├── folio_client.go      [NEW] Folio HTTP API client
│   └── shield_client.go     [NEW] Shield webhook client for deployment recording
```

### Layer 3: Validators (Replace Stubs)
```
├── validators/
│   ├── docker_validator.go  [NEW] Verify Docker images exist
│   ├── db_validator.go      [NEW] PostgreSQL connectivity check
│   ├── redis_validator.go   [NEW] Redis connectivity check
│   ├── api_validator.go     [NEW] Vercel/Railway/Folio API access
│   └── service_validator.go [NEW] Service registry health checks
```

### Layer 4: Rollback Managers (New)
```
├── rollback/
│   ├── vercel_rollback.go   [NEW] Revert Vercel deployments
│   ├── railway_rollback.go  [NEW] Revert Railway deployments
│   └── rollback_manager.go  [NEW] Coordinate multi-stage rollback
```

### Layer 5: Webhook Integration (New)
```
├── webhooks/
│   └── shield_webhook.go    [NEW] Send deployment records to Shield CICD
```

---

## Implementation Tasks (In Order)

### Task 1: Create Deployer Interfaces
**File**: `internal/deployment/deployer.go` (NEW)
**Duration**: 30 min

Interface definitions for pluggable deployers:
```go
type Deployer interface {
    Deploy(ctx context.Context, config *Config) error
    Validate(ctx context.Context) error
    Rollback(ctx context.Context) error
}
```

---

### Task 2: Implement Vercel API Client
**File**: `internal/clients/vercel_client.go` (NEW)
**Duration**: 1.5 hours

**Vercel API Integration Points**:
- **Create Deployment**: POST `/v13/deployments`
  - Input: Git reference, production flag, environment variables
  - Output: Deployment ID, URL
  - Wait time: 2-4 minutes per deployment

- **Get Deployment Status**: GET `/v13/deployments/{id}`
  - States: CREATED, BUILDING, READY, ERROR, CANCELED
  - Poll every 10 seconds until READY or ERROR

- **List Deployments**: GET `/v13/projects/{id}/deployments`
  - Get previous version for rollback reference

- **Get Project**: GET `/v13/projects/{id}`
  - Validate project access before deployment

**Required Environment Variables**:
- `VERCEL_API_TOKEN` - Personal access token from Vercel dashboard
- `VERCEL_TEAM_ID` (optional) - For team deployments
- `VERCEL_PROJECT_IDS` - Comma-separated: canvas,office,frame

**Implementation Strategy**:
```go
type VercelClient struct {
    apiToken   string
    baseURL    string
    httpClient *http.Client
}

// Deploy creates new deployment
func (c *VercelClient) Deploy(projectID, gitRef string, production bool) (*Deployment, error)

// GetStatus polls deployment status
func (c *VercelClient) GetStatus(deploymentID string) (*DeploymentStatus, error)

// GetPreviousVersion gets last stable deployment for rollback
func (c *VercelClient) GetPreviousVersion(projectID string) (*Deployment, error)
```

---

### Task 3: Implement Railway CLI Deployer
**File**: `internal/deployment/railway_deployer.go` (NEW) + `internal/clients/railway_client.go` (NEW)
**Duration**: 2 hours

**Railway Deployment Process**:
1. Authenticate with Railway CLI: `railway login --token ${RAILWAY_TOKEN}`
2. Link project: `railway link --project ${RAILWAY_PROJECT_ID}`
3. Deploy backend services via Nixpacks:
   - API: `railway deploy --service api --from ./domain/api`
   - Shield: `railway deploy --service shield --from ./domain/shield`
   - Relay: `railway deploy --service relay --from ./domain/relay`
   - Printery: `railway deploy --service printery --from ./domain/printery`
4. Monitor: `railway logs --service api --follow` (30 sec timeout)
5. Health check: `curl http://{service}.railway.internal:8080/ready`

**Required Environment Variables**:
- `RAILWAY_TOKEN` - API token from Railway dashboard
- `RAILWAY_PROJECT_ID` - Project ID (UUID)
- `RAILWAY_SERVICE_IDS` - Service names in Railway

**Implementation Strategy**:
```go
type RailwayDeployer struct {
    projectID string
    token     string
    services  []string
}

// Deploy orchestrates multi-service deployment
func (d *RailwayDeployer) Deploy(ctx context.Context, config *Config) error
    // 1. Authenticate and link project
    // 2. Deploy each service in parallel
    // 3. Wait for readiness (poll /ready endpoint)
    // 4. Health check all services

// deployService deploys single service
func (d *RailwayDeployer) deployService(service string) error
    // Execute: railway deploy --service {service}
    // Parse output for deployment status
    // Return deployment ID for tracking

// waitForReady waits until service /ready endpoint returns 200
func (d *RailwayDeployer) waitForReady(service string, timeout time.Duration) error
```

---

### Task 4: Implement Folio API Deployer
**File**: `internal/deployment/folio_deployer.go` (NEW) + `internal/clients/folio_client.go` (NEW)
**Duration**: 1.5 hours

**Folio Configuration Updates**:
1. **Update Prometheus Scrape Targets**: POST `/api/v1/config/scrape-targets`
   - Add new service instances
   - Interval: 15s, Timeout: 10s

2. **Deploy Grafana Dashboards**: POST `/api/v1/dashboards`
   - Upload dashboard JSON
   - Set dashboard UIDs for linking

3. **Configure Alert Rules**: POST `/api/v1/alerts/rules`
   - Create alert rules (HighErrorRate, ServiceDown, HighLatency)
   - Configure notification channels

4. **Health Check Folio**: GET `/health`
   - Verify Folio is accessible
   - Check Prometheus connectivity

**Required Environment Variables**:
- `FOLIO_URL` - Folio service URL
- `FOLIO_API_TOKEN` - Bearer token for Folio API

**Implementation Strategy**:
```go
type FolioDeployer struct {
    url       string
    apiToken  string
    httpClient *http.Client
}

// Deploy updates Folio configuration
func (d *FolioDeployer) Deploy(ctx context.Context, config *Config) error
    // 1. Update Prometheus scrape targets for deployed services
    // 2. Deploy Grafana dashboards
    // 3. Configure alert rules
    // 4. Health check

// UpdateScrapeTargets adds services to Prometheus scraping
func (d *FolioDeployer) UpdateScrapeTargets(targets []ScrapeTarget) error

// DeployDashboards uploads Grafana dashboards
func (d *FolioDeployer) DeployDashboards(dashboards []Dashboard) error

// ConfigureAlerts sets up alert rules
func (d *FolioDeployer) ConfigureAlerts(rules []AlertRule) error
```

---

### Task 5: Implement Real Validators
**File**: `internal/deployment/validators.go` (REPLACE STUBS)
**Duration**: 1.5 hours

**Validator Implementations**:

1. **Docker Images Validator**
   ```go
   func (o *Orchestrator) validateDockerImages(ctx context.Context) error
       // Check Docker daemon is accessible: docker ps
       // List images: docker image ls
       // Verify images exist for each service:
       //   - materi/api:{version}
       //   - materi/shield:{version}
       //   - materi/relay:{version}
   ```

2. **PostgreSQL Validator**
   ```go
   func (o *Orchestrator) validatePostgreSQL(ctx context.Context) error
       // Connect to: ${DATABASE_URL}
       // Run: SELECT version()
       // Check connection pool availability
   ```

3. **Redis Validator**
   ```go
   func (o *Orchestrator) validateRedis(ctx context.Context) error
       // Connect to: ${REDIS_URL}
       // Run: PING command
       // Check memory availability: INFO memory
   ```

4. **Vercel API Validator**
   ```go
   func (o *Orchestrator) validateVercelAccess(ctx context.Context) error
       // Verify ${VERCEL_API_TOKEN} exists
       // Call: GET https://api.vercel.com/v13/user
       // Verify projects accessible
   ```

5. **Railway API Validator**
   ```go
   func (o *Orchestrator) validateRailwayAccess(ctx context.Context) error
       // Verify ${RAILWAY_TOKEN} exists
       // Call: railway whoami
       // Verify project accessible: railway link --check
   ```

6. **Folio Validator**
   ```go
   func (o *Orchestrator) validateFolioAccess(ctx context.Context) error
       // Verify ${FOLIO_URL} accessible: GET /health
       // Verify API token: GET /api/v1/auth/verify
   ```

7. **Service Registry Validator**
   ```go
   func (o *Orchestrator) validateServiceRegistry(ctx context.Context) error
       // Check service discovery service available
       // Query for current deployed version
   ```

---

### Task 6: Implement Real Rollback Mechanisms
**File**: `internal/deployment/rollback.go` (REPLACE STUBS)
**Duration**: 1 hour

**Rollback Strategy**:
1. **Vercel Rollback**: Promote previous stable deployment to production
   ```go
   func (v *VercelDeployer) Rollback(ctx context.Context) error
       // Get previous deployment from project history
       // Alias previous URL to production domain
       // Verify previous version is live
   ```

2. **Railway Rollback**: Rollout previous Docker image version
   ```go
   func (r *RailwayDeployer) Rollback(ctx context.Context) error
       // Redeploy previous image version
       // Wait for all services ready with previous version
       // Verify health checks pass
   ```

3. **Atomic Rollback Manager**
   ```go
   func (o *Orchestrator) rollback(ctx context.Context) error
       // Coordinate rollback across all failed stages
       // If Vercel succeeded but Railway failed:
       //   - Rollback Vercel (revert frontend)
       //   - Keep Railway in failed state (don't partially rollback)
       // Send rollback notification to Shield
       // Log rollback reason and duration
   ```

---

### Task 7: Implement Shield CICD Webhook Integration
**File**: `internal/deployment/shield_webhook.go` (NEW)
**Duration**: 1 hour

**Webhook Event Types** (POST to `${SHIELD_WEBHOOK_URL}/api/webhooks/pipeline-metrics/`):

1. **deployment_started** (at start of deploy command)
   ```json
   {
     "event": "deployment_started",
     "deployment_id": "d-...",
     "version": "v1",
     "environment": "production",
     "strategy": "canary",
     "services": ["vercel", "railway", "folio"],
     "timestamp": "2026-01-08T14:30:00Z"
   }
   ```

2. **stage_completed** (after each stage finishes)
   ```json
   {
     "event": "stage_completed",
     "deployment_id": "d-...",
     "stage": "railway",
     "status": "success|failure",
     "duration_seconds": 345,
     "timestamp": "2026-01-08T14:35:00Z"
   }
   ```

3. **deployment_completed** (at end of deploy)
   ```json
   {
     "event": "deployment_completed",
     "deployment_id": "d-...",
     "status": "success|failure|rolled_back",
     "total_duration_seconds": 600,
     "stages_summary": {...},
     "timestamp": "2026-01-08T14:40:00Z"
   }
   ```

**Implementation**:
```go
type ShieldWebhook struct {
    webhookURL string
    secret     string // HMAC secret for request signing
    httpClient *http.Client
}

// Send posts webhook event to Shield
func (w *ShieldWebhook) Send(ctx context.Context, event *DeploymentEvent) error
    // Serialize event to JSON
    // Sign with HMAC-SHA256: sign(body, secret)
    // POST to webhook URL with X-Hub-Signature-256 header
    // Retry with exponential backoff on failure
```

---

## Environment Variables Required

### Vercel Integration
```
VERCEL_API_TOKEN=<personal access token from Vercel dashboard>
VERCEL_TEAM_ID=<team ID if applicable, optional>
VERCEL_PROJECT_IDS=canvas,office,frame
```

### Railway Integration
```
RAILWAY_TOKEN=<API token from Railway dashboard>
RAILWAY_PROJECT_ID=<UUID from Railway project>
```

### Folio Integration
```
FOLIO_URL=http://folio.materi.internal:3000
FOLIO_API_TOKEN=<bearer token for Folio API>
```

### Shield Integration
```
SHIELD_WEBHOOK_URL=http://shield.materi.internal:8000/api/webhooks/pipeline-metrics/
SHIELD_WEBHOOK_SECRET=<HMAC secret for webhook signing>
```

---

## Integration Points

### 1. Orchestrator → Stage Executors
```go
// OLD (TASKSET 1): StageExecutor methods were stubs
func (e *StageExecutor) deployVercel(ctx context.Context, config *Config) error
    // Simulated: time.Sleep(500ms), logged actions

// NEW (TASKSET 2): Replace with real deployers
type StageExecutor struct {
    vercelDeployer  *VercelDeployer
    railwayDeployer *RailwayDeployer
    folioDeployer   *FolioDeployer
}

func (e *StageExecutor) Execute(ctx context.Context, config *Config) error {
    switch e.stageType {
    case "vercel":
        return e.vercelDeployer.Deploy(ctx, config)
    case "railway":
        return e.railwayDeployer.Deploy(ctx, config)
    case "folio":
        return e.folioDeployer.Deploy(ctx, config)
    }
}
```

### 2. Orchestrator → Validators
```go
// OLD (TASKSET 1): Validators were empty stubs
func (o *Orchestrator) validatePostgreSQL(ctx context.Context) error {
    o.logger.Debug("Validating PostgreSQL connectivity")
    return nil  // Always pass
}

// NEW (TASKSET 2): Real validation
func (o *Orchestrator) validatePostgreSQL(ctx context.Context) error {
    db, err := sql.Open("postgres", os.Getenv("DATABASE_URL"))
    if err != nil {
        return fmt.Errorf("failed to connect to PostgreSQL: %w", err)
    }
    defer db.Close()

    err = db.PingContext(ctx)
    if err != nil {
        return fmt.Errorf("PostgreSQL health check failed: %w", err)
    }
    return nil
}
```

### 3. Orchestrator → Rollback Manager
```go
// OLD (TASKSET 1): Rollback was logged but not executed
func (o *Orchestrator) rollback(ctx context.Context) error {
    o.logger.Warn("Rollback initiated - reverting to previous version")
    return nil  // Stub
}

// NEW (TASKSET 2): Real rollback
func (o *Orchestrator) rollback(ctx context.Context) error {
    // Rollback each deployed service
    for stage := range o.result.StageResults {
        if o.result.StageResults[stage].Status == "success" {
            deployer := o.getDeployer(stage)
            err := deployer.Rollback(ctx)
            if err != nil {
                return fmt.Errorf("rollback of %s failed: %w", stage, err)
            }
        }
    }
    return nil
}
```

### 4. Orchestrator → Shield Webhook
```go
// NEW (TASKSET 2): Send deployment events to Shield
func (o *Orchestrator) Deploy(ctx context.Context) (*DeploymentResult, error) {
    // Send deployment_started webhook
    o.webhook.Send(ctx, &DeploymentEvent{
        Type: "deployment_started",
        DeploymentID: o.deploymentID,
        Version: o.config.Version,
        // ...
    })

    // ... deployment execution ...

    // Send deployment_completed webhook
    o.webhook.Send(ctx, &DeploymentEvent{
        Type: "deployment_completed",
        Status: result.Status,
        // ...
    })
}
```

---

## File Changes Summary

### New Files (8 total)
1. `internal/deployment/deployer.go` - Deployer interface
2. `internal/deployment/vercel_deployer.go` - Vercel implementation
3. `internal/deployment/railway_deployer.go` - Railway implementation
4. `internal/deployment/folio_deployer.go` - Folio implementation
5. `internal/clients/vercel_client.go` - Vercel API client
6. `internal/clients/railway_client.go` - Railway CLI wrapper
7. `internal/clients/folio_client.go` - Folio API client
8. `internal/deployment/shield_webhook.go` - Webhook integration

### Modified Files (3 total)
1. `internal/deployment/stage_executor.go` - Replace stubs with real deployers
2. `internal/deployment/orchestrator.go` - Add real validators, rollback, webhook
3. `internal/deployment/validators.go` - Replace stub validators

### Configuration Files (0)
- No changes to `scribe-deployment.yml` (already comprehensive)
- Environment variables added via `.env` files

---

## Success Criteria

- [ ] Vercel deployer can successfully deploy Canvas/Office/Frame to Vercel
- [ ] Railway deployer can deploy all backend services via Railway CLI
- [ ] Folio deployer can update Prometheus scrape targets and Grafana dashboards
- [ ] All 7 validators perform real checks (not stubs)
- [ ] Rollback successfully reverts failed deployments
- [ ] Shield webhook receives and records all deployment events
- [ ] End-to-end deployment works: `scribe deploy --version v1 --environment staging`
- [ ] Deployment can be paused/cancelled mid-execution
- [ ] Error handling and retries work for flaky operations

---

## Known Limitations

| Item | Reason | Solution in TASKSET |
|------|--------|------------------|
| No GitHub Actions integration | Manual CLI only | TASKSET 4 (CI/CD automation) |
| No real-time WebSocket progress | Webhook is one-way | TASKSET 3 (Shield CICD dashboard) |
| No automatic promotion logic | Manual approval flow | TASKSET 5 (Canary/progressive deployment) |
| No Blue-Green deployment | Complex infrastructure change | TASKSET 5+ (advanced strategies) |

---

## Estimated Timeline

| Task | Duration | Status |
|------|----------|--------|
| 1. Deployer Interfaces | 30 min | Pending |
| 2. Vercel API Client | 1.5h | Pending |
| 3. Railway CLI Deployer | 2h | Pending |
| 4. Folio API Deployer | 1.5h | Pending |
| 5. Real Validators | 1.5h | Pending |
| 6. Rollback Mechanisms | 1h | Pending |
| 7. Shield Webhook | 1h | Pending |
| **TOTAL** | **9 hours** | **~1 working day** |

---

## Testing Strategy

### Unit Tests
- Test deployers with mocked HTTP clients
- Test validators with mocked database/API connections
- Test webhook signing and serialization

### Integration Tests
- Deploy to Railway staging
- Deploy to Vercel preview environment
- Record deployment events to Shield staging

### E2E Tests
- Full deployment workflow: `scribe deploy --version v1 --environment staging`
- Rollback on intentional failure: `scribe deploy --version broken --environment staging --skip-tests`
- Dry-run validation: `scribe deploy --version v1 --environment production --dry-run`

---

## Next Steps

1. **Review this plan** - Ensure approach aligns with requirements
2. **Implement Task 1-7** - Create all files and implementations
3. **Test integration** - Run end-to-end deployment
4. **Document completion** - Create TASKSET_2_COMPLETION_SUMMARY.md
5. **Proceed to TASKSET 3** - Printery event-driven coordination

---

