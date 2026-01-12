---
title: Onboarding
description: How new team members get productive in the Materi monorepo
relatedPages: []
---

<Info>
**SDD Classification:** L4-Operational | **Authority:** Engineering Leadership | **Review Cycle:** Quarterly
</Info>

## Overview

This page is the canonical onboarding entry point for new team members.

## First 20 minutes (engineering)

Goal: verify your environment, run tests, and understand system boundaries.

### Context & architecture (5 minutes)

-   Read: /internal/architecture/specs/concept-of-operations
-   Read: /internal/architecture/specs/technology-strategy

### Environment verification (5 minutes)

```bash
node --version
pnpm --version
python3 --version
docker --version
```

### Frontend: Canvas (5 minutes)

```bash
cd products/canvas
pnpm install
pnpm test
```

### Intelligence: Aria (5 minutes)

```bash
cd platform/intelligence/aria
make setup
make test-unit
```

## Deliverables

-   Confirm you’ve read the CONOPS and Technology Strategy.
-   Provide test results for Canvas and Aria.
-   Ask one question about architecture boundaries.
