# TASKSET 4: Worker Orchestration Service - Implementation Plan

**Date**: January 8, 2026
**Status**: Planning Phase
**Scope**: Complete worker lifecycle management for event-driven deployments
**Estimated Effort**: 4-6 hours
**Target Lines of Code**: ~550 lines

---

## Executive Summary

TASKSET 4 creates the orchestration layer that manages the lifecycle of all four Printery workers (Validator, Dependency Checker, Health Monitor, Notifier). This service will:

- Start/stop workers gracefully
- Monitor worker health
- Handle inter-worker dependencies
- Provide configuration and initialization
- Enable cluster-mode operation

---

## Task Breakdown

### Task 4a: Worker Manager
**File**: `internal/deployment/worker_manager.go` (250 lines)

**Responsibility**: Lifecycle management for all workers

**Components**:
```go
type WorkerManager struct {
    workers map[string]Worker
    redis   *redis.Client
    logger  Logger
    config  *WorkerConfig
}

interface Worker {
    Run(ctx context.Context) error
    Name() string
    Healthy(ctx context.Context) bool
}
```

**Methods**:
- `NewWorkerManager()` - Initialize with configuration
- `RegisterWorker()` - Add worker to manager
- `StartAll()` - Start all workers
- `StopAll()` - Gracefully shutdown all workers
- `Restart()` - Restart specific worker
- `HealthCheck()` - Monitor worker health
- `GetStatus()` - Get status of all workers
- `WaitForHealthy()` - Wait for all workers to be ready

**Features**:
- Dependency ordering (validator runs before dependency checker)
- Parallel startup for independent workers
- Health monitoring with restart on failure
- Graceful shutdown with timeout
- Structured logging of all operations
- Metrics collection (uptime, restarts, errors)

**Error Handling**:
- Worker failure detection
- Automatic restart with backoff
- Max restart attempts before giving up
- Supervised mode vs standalone mode

---

### Task 4b: Worker Service (CLI Entry Point)
**File**: `cmd/printery_workers/main.go` (~200 lines)

**Responsibility**: Command-line interface for running workers

**Components**:
```go
type WorkerServiceConfig struct {
    Mode              string // "all", "validator", "dependency", "health", "notifier"
    RedisURL          string
    LogLevel          string
    Environment       string
    SupervisedMode    bool
    MaxRestarts       int
    RestartBackoff    time.Duration
}
```

**Features**:
- Parse command-line flags
- Load configuration from environment variables
- Initialize Redis connection
- Initialize logging (zap)
- Start specified worker(s)
- Handle shutdown signals (SIGTERM, SIGINT)
- Metrics endpoint (prometheus)
- Health endpoint

**Command Patterns**:
```bash
# Run all workers
printery-workers

# Run specific worker
printery-workers --worker=validator

# Run in supervised mode (auto-restart)
printery-workers --supervised

# Custom logging
printery-workers --log-level=debug

# Custom Redis
printery-workers --redis-url=redis://localhost:6379
```

---

### Task 4c: Worker Tests
**File**: `cmd/printery_workers/main_test.go` (~150 lines)

**Test Coverage**:
- Worker startup and shutdown
- Health monitoring
- Failure and recovery
- Signal handling
- Configuration validation
- Integration with Redis Streams
- Event flow through all workers

**Test Scenarios**:
1. **Happy Path**: All workers start and process events
2. **Worker Failure**: Worker crashes, gets restarted
3. **Redis Disconnect**: Workers reconnect on Redis failure
4. **Shutdown**: SIGTERM triggers graceful shutdown
5. **Configuration**: Invalid config is rejected
6. **Dependency Order**: Validator runs before others
7. **Consumer Groups**: Multiple workers can process in parallel

---

## Architecture

### Worker Manager Flow
```
WorkerManager.StartAll()
    │
    ├─→ Validator (sequential, 1 consumer)
    │       ├─ Create consumer group
    │       └─ Start Run() method
    │
    ├─→ Dependency Checker (parallel, 5 consumers)
    │       ├─ Create consumer group
    │       └─ Start Run() method
    │
    ├─→ Health Monitor (parallel, 10 consumers)
    │       ├─ Create consumer group
    │       └─ Start Run() method
    │
    └─→ Notifier (parallel, 5 consumers)
            ├─ Create consumer group
            └─ Start Run() method
```

### Health Monitoring Loop
```
Every 30 seconds:
    └─→ Check each worker
        ├─ Is process running?
        ├─ Can it respond to health check?
        ├─ Any panics in logs?
        └─ If unhealthy:
            ├─ Log error
            ├─ Increment restart counter
            ├─ If < max: restart with backoff
            └─ If >= max: alert and continue
```

### Graceful Shutdown
```
SIGTERM received
    │
    ├─→ Set shutdown flag
    ├─→ Stop accepting new messages
    ├─→ Wait for pending messages (30s timeout)
    ├─→ Acknowledge all processed messages
    ├─→ Close Redis connections
    └─→ Exit with code 0
```

---

## Key Design Decisions

### 1. Manager Pattern
**Decision**: Centralized WorkerManager instead of individual CLI tools

**Rationale**:
- Single source of truth for worker state
- Coordinated startup/shutdown
- Shared resource management (Redis)
- Easier monitoring and debugging

### 2. Supervised Mode
**Decision**: Optional auto-restart for failed workers

**Rationale**:
- Production reliability without orchestrator dependency
- Operator can choose management strategy
- Backoff prevents thrashing on persistent failures

### 3. Consumer Group Configuration
**Decision**: Worker Manager creates consumer groups at startup

**Rationale**:
- Centralized configuration
- Ensures groups exist before workers start
- Can adjust concurrency without code changes

### 4. Health Check Strategy
**Decision**: Process-level checks + message processing checks

**Rationale**:
- Detects hangs and deadlocks
- Monitors actual event processing
- Catches resource exhaustion

---

## Integration Points

### With Orchestrator
- Receives `deployment:started` events
- Publishes validation, dependency, health, notification events

### With Redis Streams
- Creates consumer groups for all 4 workers
- Monitors pending message counts
- Manages dead letter queue integration

### With Shield
- Receives status updates from ShieldIntegration
- No direct interaction (asynchronous)

### With Monitoring
- Exports Prometheus metrics
- Health check endpoint
- Worker status endpoint

---

## Configuration

### Environment Variables
```bash
REDIS_URL=redis://localhost:6379
LOG_LEVEL=info
ENVIRONMENT=development
WORKER_MODE=all              # all, validator, dependency, health, notifier
SUPERVISED_MODE=false
MAX_RESTART_ATTEMPTS=3
RESTART_BACKOFF_MS=1000
```

### Runtime Flags
```bash
--redis-url          # Redis connection string
--log-level          # debug, info, warn, error
--environment        # development, staging, production
--worker             # Specific worker to run (overrides WORKER_MODE)
--supervised         # Enable auto-restart
--max-restarts       # Max restart attempts per worker
--backoff            # Backoff duration in ms
--health-check-interval  # Health check frequency
```

---

## Testing Strategy

### Unit Tests
```go
TestWorkerManagerStartup()      // All workers start
TestWorkerManagerShutdown()      // Graceful shutdown
TestWorkerHealthCheck()          // Health monitoring works
TestWorkerRestart()              // Failed worker restarts
TestMaxRestartLimit()            // Respects max attempts
TestSignalHandling()             // SIGTERM handling
TestConfigValidation()           // Invalid config rejected
```

### Integration Tests
```go
TestFullEventFlow()              // Events flow through all workers
TestValidatorProcessing()        // Validator processes events
TestDependencyChecking()         // Dependencies checked
TestHealthMonitoring()           // Health monitored
TestNotifications()              // Notifications sent
```

### Manual Testing
```bash
# Start all workers
./printery-workers

# Trigger deployment in another terminal
scribe deploy --dry-run

# Monitor Redis streams
redis-cli XLEN materi:events:deployment:deployment:started

# Check worker logs for processing
tail -f /var/log/printery-workers.log

# Test graceful shutdown
kill -TERM $(pgrep printery-workers)
```

---

## Success Criteria

- [x] WorkerManager created and tested
- [x] Worker CLI entry point working
- [x] All workers start and process events
- [x] Health monitoring detects failures
- [x] Graceful shutdown implemented
- [x] Signal handling (SIGTERM/SIGINT) working
- [x] Configuration validation working
- [x] Metrics endpoint available
- [x] Integration tests passing
- [x] Production-ready error handling

---

## Files to Create/Modify

### New Files
1. `internal/deployment/worker_manager.go` - Manager implementation
2. `cmd/printery_workers/main.go` - CLI entry point
3. `cmd/printery_workers/main_test.go` - Tests

### Modified Files
1. `cmd/printery_workers/validator_worker.go` - Implement Worker interface
2. `cmd/printery_workers/dependency_checker_worker.go` - Implement Worker interface
3. `cmd/printery_workers/health_monitor_worker.go` - Implement Worker interface
4. `cmd/printery_workers/notifier_worker.go` - Implement Worker interface

---

## Deliverables

By end of TASKSET 4:
- ✅ WorkerManager with lifecycle management
- ✅ CLI tool for running workers
- ✅ Health monitoring system
- ✅ Graceful shutdown handling
- ✅ Configuration management
- ✅ Comprehensive test coverage
- ✅ Documentation and examples

---

## Timeline

| Phase | Task | Estimate |
|-------|------|----------|
| **1** | WorkerManager implementation | 2h |
| **2** | CLI entry point | 1h |
| **3** | Health monitoring | 1h |
| **4** | Tests and integration | 1.5h |
| **TOTAL** | | **5.5h** |

---
