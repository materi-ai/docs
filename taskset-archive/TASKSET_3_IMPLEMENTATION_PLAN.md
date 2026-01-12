# TASKSET 3: Printery Event-Driven Coordination - Implementation Plan

**Date**: January 8, 2026
**Status**: Planning Phase
**Target**: Implement event-driven deployment coordination via Redis Streams and Printery workers
**Previous**: TASKSET 2 completed - Real deployers with API integrations

---

## Executive Summary

TASKSET 3 adds event-driven coordination for multi-service deployments:

### Current State (TASKSET 2)
- Scribe orchestrates deployment synchronously
- Stage executors deploy to Vercel, Railway, Folio
- Shield webhook records deployment events
- No cross-service event distribution
- No background worker processing

### Target State (TASKSET 3)
- Deployment events published to Redis Streams
- Printery workers consume and process events
- Service dependency validation via events
- Cross-service orchestration handlers
- Event replay and dead-letter queue (DLQ) support

---

## Architecture Overview

### Event Flow

```
┌─────────────────────────────────────────────────────────────┐
│  scribe deploy --version v1 --environment production        │
│           (Orchestrator from TASKSET 2)                     │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  Deployment Orchestration (TASKSET 2)                       │
│  • Validate dependencies                                    │
│  • Deploy Vercel (Canvas/Office/Frame)                      │
│  • Deploy Railway (API/Shield/Relay)                        │
│  • Deploy Folio (Observability)                             │
└────────────────────┬────────────────────────────────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  Event Publisher (NEW in TASKSET 3)                         │
│  Publishes to Redis Streams:                                │
│  • deployment:started                                       │
│  • deployment:stage:vercel:started                          │
│  • deployment:stage:vercel:completed                        │
│  • deployment:stage:railway:started                         │
│  • deployment:stage:railway:completed                       │
│  • deployment:stage:folio:started                           │
│  • deployment:stage:folio:completed                         │
│  • deployment:completed (success/failure)                   │
└────────────────────┬────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┬──────────────┐
        │            │            │              │
        ▼            ▼            ▼              ▼
┌──────────────┐ ┌──────────────┐ ┌──────────┐ ┌─────────┐
│ Printery     │ │ Printery     │ │ Printery │ │ Printery│
│ Validator    │ │ Dependency   │ │ Health   │ │ Notifier│
│ Worker       │ │ Checker      │ │ Monitor  │ │ Worker  │
│ [NEW]        │ │ [NEW]        │ │ [NEW]    │ │ [NEW]   │
└──────────────┘ └──────────────┘ └──────────┘ └─────────┘
        │            │            │              │
        └────────────┼────────────┴──────────────┘
                     │
                     ▼
┌─────────────────────────────────────────────────────────────┐
│  Redis Streams Event Distribution                           │
│  • materi:events:deployment:validation                      │
│  • materi:events:deployment:health                          │
│  • materi:events:deployment:dependency                      │
└────────────────────┬────────────────────────────────────────┘
                     │
        ┌────────────┼────────────┬──────────────┐
        │            │            │              │
        ▼            ▼            ▼              ▼
    ┌─────────┐ ┌─────────┐ ┌─────────┐ ┌──────────┐
    │API      │ │Shield   │ │Relay    │ │Manifest  │
    │Service  │ │Service  │ │Service  │ │Update    │
    └─────────┘ └─────────┘ └─────────┘ └──────────┘
```

---

## Implementation Tasks

### Task 1: Define Event Schemas (Protobuf)
**File**: `shared/proto/deployment.proto` (NEW)
**Duration**: 1 hour

Define Protocol Buffer schemas for all deployment events:

```protobuf
// Deployment Events

message DeploymentStartedEvent {
    string deployment_id = 1;
    string version = 2;
    string environment = 3;
    string strategy = 4;
    repeated string services = 5;
    int64 timestamp = 6;
    string correlation_id = 7;
}

message DeploymentStageStartedEvent {
    string deployment_id = 1;
    string stage = 2;  // vercel, railway, folio
    string version = 3;
    int64 timestamp = 4;
    string correlation_id = 5;
}

message DeploymentStageCompletedEvent {
    string deployment_id = 1;
    string stage = 2;
    string status = 3;  // success, failure
    int32 duration_seconds = 4;
    string error_message = 5;
    int64 timestamp = 6;
    string correlation_id = 7;
}

message DeploymentCompletedEvent {
    string deployment_id = 1;
    string status = 2;  // success, failure, rolled_back
    int32 total_duration_seconds = 3;
    repeated StageResult stage_results = 4;
    string error_message = 5;
    int64 timestamp = 6;
    string correlation_id = 7;
}

message ServiceDependencyCheckEvent {
    string deployment_id = 1;
    string service = 2;
    repeated string dependencies = 3;
    repeated DependencyStatus dependency_statuses = 4;
    int64 timestamp = 5;
    string correlation_id = 6;
}

message ServiceHealthCheckEvent {
    string deployment_id = 1;
    string service = 2;
    string health_endpoint = 3;
    int32 response_time_ms = 4;
    bool healthy = 5;
    string error_message = 6;
    int64 timestamp = 7;
    string correlation_id = 8;
}
```

**Related Types**:
```protobuf
message StageResult {
    string stage = 1;
    string status = 2;
    int32 duration_seconds = 3;
    string error_message = 4;
}

message DependencyStatus {
    string service = 1;
    string status = 2;  // healthy, degraded, unhealthy
    string endpoint = 3;
    int32 response_time_ms = 4;
}
```

---

### Task 2: Event Publisher in Orchestrator
**File**: `internal/deployment/event_publisher.go` (NEW)
**Duration**: 1.5 hours

Implement event publishing to Redis Streams:

```go
type EventPublisher struct {
    redis     *redis.Client
    namespace string
}

// PublishDeploymentStarted publishes deployment_started event
func (p *EventPublisher) PublishDeploymentStarted(ctx context.Context, event *DeploymentStartedEvent) error

// PublishStageStarted publishes stage_started event
func (p *EventPublisher) PublishStageStarted(ctx context.Context, event *DeploymentStageStartedEvent) error

// PublishStageCompleted publishes stage_completed event
func (p *EventPublisher) PublishStageCompleted(ctx context.Context, event *DeploymentStageCompletedEvent) error

// PublishDeploymentCompleted publishes deployment_completed event
func (p *EventPublisher) PublishDeploymentCompleted(ctx context.Context, event *DeploymentCompletedEvent) error

// PublishDependencyCheck publishes dependency_check event
func (p *EventPublisher) PublishDependencyCheck(ctx context.Context, event *ServiceDependencyCheckEvent) error

// PublishHealthCheck publishes health_check event
func (p *EventPublisher) PublishHealthCheck(ctx context.Context, event *ServiceHealthCheckEvent) error
```

**Integration with Orchestrator**:
```go
type Orchestrator struct {
    // ... existing fields ...
    eventPublisher *EventPublisher
}

func (o *Orchestrator) Deploy(ctx context.Context) (*DeploymentResult, error) {
    // Publish deployment_started event
    o.eventPublisher.PublishDeploymentStarted(ctx, &DeploymentStartedEvent{
        DeploymentID: o.deploymentID,
        Version: o.config.Version,
        // ...
    })

    // For each stage:
    //   - Publish stage_started
    //   - Execute stage
    //   - Publish stage_completed
}
```

---

### Task 3: Printery Workers (4 workers)
**Files**: `cmd/printery_workers/*.go` (NEW)
**Duration**: 3 hours

#### Worker 1: Validator Worker
**File**: `cmd/printery_workers/validator_worker.go` (NEW)

Validates deployment prerequisites and services:

```go
type ValidatorWorker struct {
    redis *redis.Client
    db    *Database
}

// Run starts the validator worker
func (w *ValidatorWorker) Run(ctx context.Context) error {
    // Subscribe to deployment:started events
    // For each deployment event:
    //   1. Load deployment manifest
    //   2. Validate service prerequisites (Docker images, configs)
    //   3. Check critical service availability
    //   4. Publish validation_completed event (success/failure)
    //   5. If failed, publish deployment_failed event
}
```

**Validations Performed**:
- Docker images exist for all services
- Database migrations are compatible
- Environment variables configured
- Service discovery accessible
- API credentials valid

#### Worker 2: Dependency Checker Worker
**File**: `cmd/printery_workers/dependency_checker_worker.go` (NEW)

Checks service dependencies during deployment:

```go
type DependencyCheckerWorker struct {
    redis *redis.Client
    http  *http.Client
}

// Run starts the dependency checker worker
func (w *DependencyCheckerWorker) Run(ctx context.Context) error {
    // Subscribe to stage:completed events
    // For each completed stage:
    //   1. If Railway stage completed successfully:
    //      - Check all deployed service endpoints
    //      - Verify inter-service connectivity
    //      - Check database/cache accessibility
    //   2. Publish dependency_check event with results
    //   3. If any dependency unhealthy, fail deployment
}
```

**Dependency Checks**:
```
API →
  - PostgreSQL (database)
  - Redis (cache)
  - Shield (auth)
  - Relay (WebSocket)

Shield →
  - PostgreSQL (database)
  - Redis (cache)

Relay →
  - PostgreSQL (database)
  - Redis (cache)
  - API (service registry)

Printery →
  - PostgreSQL (database)
  - Redis (cache)
  - API (service registry)
```

#### Worker 3: Health Monitor Worker
**File**: `cmd/printery_workers/health_monitor_worker.go` (NEW)

Monitors service health during and after deployment:

```go
type HealthMonitorWorker struct {
    redis *redis.Client
    http  *http.Client
}

// Run starts the health monitor worker
func (w *HealthMonitorWorker) Run(ctx context.Context) error {
    // Subscribe to stage:completed events
    // For each completed stage:
    //   1. Poll service /health and /ready endpoints
    //   2. Check response times and error rates
    //   3. Verify Prometheus metrics are flowing
    //   4. Publish health_check events
    //   5. If unhealthy, trigger rollback
}
```

**Health Checks**:
- HTTP /health endpoint (basic health)
- HTTP /ready endpoint (readiness)
- Prometheus metrics /metrics
- Response time < 1s (warning), < 5s (ok)
- Error rate < 1% (healthy), < 5% (degraded)

#### Worker 4: Notifier Worker
**File**: `cmd/printery_workers/notifier_worker.go` (NEW)

Sends notifications based on deployment events:

```go
type NotifierWorker struct {
    redis *redis.Client
    slack *SlackClient
    email *EmailClient
}

// Run starts the notifier worker
func (w *NotifierWorker) Run(ctx context.Context) error {
    // Subscribe to deployment events
    // For each event:
    //   1. Format notification message
    //   2. Send to configured channels (Slack, email, etc.)
    //   3. Include deployment ID, version, status
    //   4. On failure: notify on-call engineer
    //   5. On success: notify team
}
```

**Notifications**:
- Deployment started → #materi-deployments
- Stage completed → #materi-deployments
- Deployment success → #materi-deployments + team
- Deployment failure → #materi-critical + on-call
- Rollback initiated → #materi-critical + team

---

### Task 4: Event Consumer Groups (Printery)
**File**: `internal/workers/consumer_group.go` (NEW)
**Duration**: 1.5 hours

Implement Redis Streams consumer group pattern:

```go
type ConsumerGroup struct {
    name             string
    stream           string
    redis            *redis.Client
    handlers         map[string]EventHandler
    maxConcurrent    int
    claimTimeout     time.Duration
}

// EventHandler processes a single event
type EventHandler interface {
    Handle(ctx context.Context, event interface{}) error
}

// Run starts consuming from Redis Stream
func (cg *ConsumerGroup) Run(ctx context.Context) error {
    // 1. Create consumer group (ignore if exists)
    // 2. Subscribe to stream
    // 3. Process messages with max concurrency
    // 4. Handle failures:
    //    - Retry with backoff
    //    - Send to DLQ after max retries
    //    - Ack successful messages
}

// HandlePendingMessages handles messages from previous crashes
func (cg *ConsumerGroup) HandlePendingMessages(ctx context.Context) error {
    // Claim pending messages
    // Re-deliver to handler
}
```

**Consumer Groups Configuration**:

```yaml
consumer_groups:
  deployment_validator:
    stream: materi:events:deployment
    group: printery-validator
    max_concurrent: 1
    claim_timeout: 5m

  dependency_checker:
    stream: materi:events:deployment
    group: printery-dependency-checker
    max_concurrent: 5

  health_monitor:
    stream: materi:events:deployment
    group: printery-health-monitor
    max_concurrent: 10

  notifier:
    stream: materi:events:deployment
    group: printery-notifier
    max_concurrent: 5
```

---

### Task 5: Dead Letter Queue (DLQ) Handler
**File**: `internal/workers/dlq_handler.go` (NEW)
**Duration**: 1 hour

Handle permanently failed events:

```go
type DLQHandler struct {
    redis *redis.Client
    db    *Database
}

// ProcessFailedMessage handles event that failed all retries
func (h *DLQHandler) ProcessFailedMessage(ctx context.Context, msg *FailedMessage) error {
    // 1. Log failure with full context
    // 2. Store in database for manual review
    // 3. Send alert to operations team
    // 4. Provide replay command for manual re-execution
}

// ReplayMessage replays a failed message to original stream
func (h *DLQHandler) ReplayMessage(ctx context.Context, messageID string) error {
    // 1. Retrieve message from DLQ
    // 2. Restore to original stream
    // 3. Reset retry counter
    // 4. Continue processing
}

// ListFailedMessages lists all messages in DLQ
func (h *DLQHandler) ListFailedMessages(ctx context.Context) ([]*FailedMessage, error)
```

**DLQ Scenarios**:
- Validation failed 3 times → Move to DLQ
- Service became unavailable during health check → Retry, then DLQ
- Invalid event format → DLQ immediately
- Database connection lost → Retry indefinitely (eventually succeeds)

---

### Task 6: Integration with Orchestrator
**File**: `internal/deployment/orchestrator.go` (MODIFY)
**Duration**: 1.5 hours

Update orchestrator to publish events:

```go
// In Deploy() method:

// 1. Publish deployment_started
o.eventPublisher.PublishDeploymentStarted(ctx, ...)

// 2. For each stage:
o.eventPublisher.PublishStageStarted(ctx, ...)
stageResult := executeStage(...)
o.eventPublisher.PublishStageCompleted(ctx, ...)

// 3. After all stages:
o.eventPublisher.PublishDeploymentCompleted(ctx, ...)

// 4. On error/rollback:
o.eventPublisher.PublishDeploymentFailed(ctx, ...)
o.eventPublisher.PublishRollbackInitiated(ctx, ...)
```

---

### Task 7: Integration with Shield CICD
**File**: `internal/deployment/shield_integration.go` (NEW)
**Duration**: 1 hour

Listen for events and update Shield records:

```go
type ShieldIntegration struct {
    shield *clients.ShieldClient
    redis  *redis.Client
}

// Run subscribes to deployment events and updates Shield
func (s *ShieldIntegration) Run(ctx context.Context) error {
    // Subscribe to all deployment events
    // For each event:
    //   1. Find or create UnifiedDeployment record
    //   2. Update status based on event type
    //   3. Record stage results
    //   4. Store duration and error information
}

// This enables Shield CICD dashboard to be real-time
```

---

## File Structure

### New Files (8 total)

```
/Users/alexarno/materi/

shared/proto/
├── deployment.proto              [NEW] Event schemas (400 lines)

domain/printery/
├── internal/
│   ├── deployment/
│   │   ├── event_publisher.go   [NEW] Redis Streams publisher (200 lines)
│   │   └── shield_integration.go [NEW] Shield CICD updates (150 lines)
│   │
│   └── workers/
│       ├── consumer_group.go     [NEW] Event consumer pattern (250 lines)
│       └── dlq_handler.go        [NEW] Dead-letter queue (200 lines)
│
└── cmd/printery_workers/
    ├── validator_worker.go       [NEW] Service validation (200 lines)
    ├── dependency_checker_worker.go [NEW] Dependency validation (250 lines)
    ├── health_monitor_worker.go  [NEW] Health monitoring (200 lines)
    └── notifier_worker.go        [NEW] Deployment notifications (150 lines)
```

### Modified Files (1 total)

```
domain/printery/
├── internal/deployment/
│   └── orchestrator.go           [MODIFIED] Add event publishing
```

---

## Redis Streams Configuration

### Event Streams

```
materi:events:deployment
├── deployment:started
├── deployment:stage:vercel:started
├── deployment:stage:vercel:completed
├── deployment:stage:railway:started
├── deployment:stage:railway:completed
├── deployment:stage:folio:started
├── deployment:stage:folio:completed
├── deployment:completed
└── deployment:failed
```

### Consumer Groups

```
materi:events:deployment:group
├── printery-validator (consumer: validator-1)
├── printery-dependency-checker (consumers: checker-1..5)
├── printery-health-monitor (consumers: monitor-1..10)
└── printery-notifier (consumers: notifier-1..5)
```

### Dead Letter Queue

```
materi:dlq:deployment-failed
├── validator-failures
├── dependency-check-failures
├── health-check-failures
└── notification-failures
```

---

## Event Flow Examples

### Successful Deployment

```
1. scribe deploy --version v1 --environment production
2. Orchestrator.Deploy() starts
3. Publish: deployment:started
4. Validator Worker consumes event
5. Validator: ✅ All checks pass
6. Publish: validation:completed (success)
7. Orchestrator continues: Vercel stage
8. Publish: deployment:stage:vercel:started
9. Vercel deployment executes (2-4 min)
10. Publish: deployment:stage:vercel:completed (success)
11. Health Monitor Worker: ✅ Canvas/Office/Frame healthy
12. Publish: health:check (success)
13. Orchestrator continues: Railway stage
14. Publish: deployment:stage:railway:started
15. Railway deployment executes (5-6 min)
16. Publish: deployment:stage:railway:completed (success)
17. Dependency Checker: ✅ All services healthy and connected
18. Publish: dependency:check (success)
19. Health Monitor: ✅ API/Shield/Relay/Printery all healthy
20. Orchestrator continues: Folio stage
21. Publish: deployment:stage:folio:started
22. Folio config updates (1-2 min)
23. Publish: deployment:stage:folio:completed (success)
24. Orchestrator: deployment:completed (success)
25. Notifier Worker: Send success notification to #materi-deployments
```

### Failed Deployment with Rollback

```
1. scribe deploy --version broken --environment staging
2. Orchestrator.Deploy() starts
3. Publish: deployment:started
4. Validator Worker: ❌ Docker image not found
5. Publish: validation:completed (failure)
6. Orchestrator: Validation failed, stop deployment
7. Publish: deployment:failed
8. Notifier Worker: Send alert to #materi-critical
9. No rollback (never deployed)
```

### Failed Deployment Mid-Stream with Rollback

```
1. scribe deploy --version v1 --environment production
2-12. Vercel deployment succeeds (as above)
13. Orchestrator continues: Railway stage
14. Publish: deployment:stage:railway:started
15. Railway deployment hangs (API service fails to start)
16. Publish: deployment:stage:railway:completed (failure, timeout)
17. Health Monitor Worker: ❌ API not responding
18. Publish: health:check (failure)
19. Orchestrator detects Railway failure
20. Publish: rollback:initiated
21. Orchestrator.Rollback() executes
22. Vercel Deployer.Rollback() → Revert to previous deployment
23. Publish: rollback:completed (success)
24. Notifier Worker: Send critical alert to #materi-critical
```

---

## Success Criteria

- [ ] Event schemas defined in Protobuf
- [ ] Event publisher integrated into Orchestrator
- [ ] All 4 Printery workers implemented and functional
- [ ] Consumer group pattern working with Redis Streams
- [ ] DLQ handling failed events
- [ ] Shield CICD integration receiving events
- [ ] Event replay capability for failed messages
- [ ] Dependency validation working across services
- [ ] Health monitoring during and after deployment
- [ ] Notifications sent on all deployment state changes

---

## Estimated Timeline

| Task | Duration | Status |
|------|----------|--------|
| 1. Event Schemas | 1h | Pending |
| 2. Event Publisher | 1.5h | Pending |
| 3. Printery Workers (4) | 3h | Pending |
| 4. Consumer Groups | 1.5h | Pending |
| 5. DLQ Handler | 1h | Pending |
| 6. Orchestrator Integration | 1.5h | Pending |
| 7. Shield CICD Integration | 1h | Pending |
| **TOTAL** | **~10 hours** | **Pending** |

---

## Known Limitations

| Feature | Status | Reason |
|---------|--------|--------|
| Real-time WebSocket updates | Not in TASKSET 3 | TASKSET 4 (Shield CICD) |
| Automatic canary traffic shifting | Not in TASKSET 3 | TASKSET 5 (Progressive deployment) |
| Custom event handling hooks | Not in TASKSET 3 | TASKSET 5+ (Extensibility) |
| Event audit log to database | Not in TASKSET 3 | TASKSET 4 (Unified deployment record) |
| Correlation ID tracing across events | Partial | Full tracing in TASKSET 4 |

---

## Integration with Other Tasksets

### From TASKSET 1 & 2
- Uses orchestrator framework
- Uses deployers (real implementations)
- Uses manifest loading
- Uses config validation

### For TASKSET 4
- Shield CICD reads events
- Real-time dashboard updates
- UnifiedDeployment record creation
- Event correlation ID tracking

### For TASKSET 5
- Event-driven canary deployment
- Health-based rollback decisions
- Progressive traffic shifting
- Feature flag integration

---

## Next Steps (After User Approval)

1. Create Protobuf event schemas
2. Implement event publisher
3. Create 4 Printery workers
4. Implement consumer group pattern
5. Add DLQ handling
6. Integrate with orchestrator
7. Integrate with Shield CICD
8. Test end-to-end event flow
9. Document event schemas and worker behavior

---
