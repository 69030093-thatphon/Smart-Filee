# Smart File Manager Pro — Development Roadmap

**Document Version:** 1.0.0
**Last Updated:** 2026-11-10
**Current Phase:** Phase 1 — Repository Understanding & Initial Audit

---

## 1. Project Objectives

1. Deliver a robust, responsive, local-first file management desktop application with zero background telemetry for user file contents.
2. Provide seamless automated file organization, duplicate identification, safe disk cleanup, bulk renaming, and differential backup synchronization.
3. Support hybrid cloud user accounts (via Supabase Auth) with local loopback email verification, while maintaining 100% local persistence for file paths, activity logs, and metadata.
4. Maintain bilingual parity across English and Thai languages.
5. Provide a deterministic, cross-platform build and CI/CD automation pipeline ensuring stability across Windows, Linux, and macOS.

---

## 2. Priority & Status Definitions

### Priority Levels
- **P0 — Critical:** Architectural blockers, security vulnerabilities, data integrity risks, build breaks.
- **P1 — High:** Core file management capabilities, major UI/UX defects, CI test coverage.
- **P2 — Normal:** Localization gaps, secondary feature enhancements, non-critical optimizations.
- **P3 — Low:** Minor cosmetic adjustments, documentation polish, nice-to-have tooling.

### Status Definitions
- `Not Started`, `Planned`, `In Progress`, `Blocked`, `In Review`, `Testing`, `Completed`, `Deferred`

---

## 3. Major Development Phases

```text
Phase 1: Repository Understanding & Audit (Current)
    │
Phase 2: Architecture & Planning
    │
Phase 3: Core Implementation & Bug Remediation
    │
Phase 4: UI/UX & Localization Parity
    │
Phase 5: Full QA & Comprehensive Testing
    │
Phase 6: Security & Performance Benchmarking
    │
Phase 7: Build Packaging & CI/CD Workflows
    │
Phase 8: Comprehensive Documentation & Handoff
    │
Phase 9: Git Integration & Release Candidate Preparation
```

---

## 4. Phase Breakdown and Task Catalog

### Phase 1 — Repository Understanding & Audit
- **Goal:** Clone, inspect repository structure, configure team branches, audit architecture and dependencies, record baseline health.
- **Milestones:**
  - [x] **TSK-01 (P0):** Inspect repository, branches, remotes, and author configuration. *(Status: Completed)*
  - [x] **TSK-02 (P0):** Initialize branching hierarchy (`develop`, `team/dev`, `team/ui-design`, `team/tester`, `team/docs`). *(Status: Completed)*
  - [x] **TSK-03 (P0):** Create `.gitignore` to prevent secret and cache leaks. *(Status: Completed)*
  - [x] **TSK-04 (P0):** Author Initial Codebase Audit (`docs/development/INITIAL_AUDIT.md`). *(Status: Completed)*
  - [x] **TSK-05 (P0):** Establish Project Roadmap & Status Tracking (`docs/ROADMAP.md`, `docs/development/PROJECT_STATUS.md`). *(Status: Completed)*
  - [x] **TSK-06 (P0):** Initialize Phase Log & Handoff documents (`docs/PHASE-LOG.md`, `docs/HANDOFF.md`). *(Status: Completed)*

### Phase 2 — Architecture & Technical Planning
- **Goal:** Formulate ADRs (Architectural Decision Records) addressing discovered technical debt (packaging inconsistencies, headless selftest, i18n scope).
- **Milestones:**
  - [ ] **TSK-07 (P1):** Document packaging strategy ADR (single-file vs directory installer). *(Status: Planned)*
  - [ ] **TSK-08 (P1):** Document headless self-test architecture to enable automated CI verification without interactive Supabase sign-in blocking. *(Status: Planned)*
  - [ ] **TSK-09 (P2):** Define internationalization expansion architecture for account and auth dialogs. *(Status: Planned)*

### Phase 3 — Core Development & Defect Remediation
- **Goal:** Resolve core and packaging bugs identified during the initial audit.
- **Milestones:**
  - [ ] **TSK-10 (P0):** Fix `smartFileManagerPro.spec` missing `fonts/` and `icon.ico` assets in `datas`. *(Status: Planned)*
  - [ ] **TSK-11 (P1):** Reconcile `build_exe.bat` and `installer.iss` packaging targets. *(Status: Planned)*
  - [ ] **TSK-12 (P1):** Implement headless `--selftest` bypass in `app.py` for automated smoke testing in CI. *(Status: Planned)*
  - [ ] **TSK-13 (P1):** Add error recovery and offline graceful fallback for Supabase client initialization. *(Status: Planned)*

### Phase 4 — UI/UX & Localization (Thai & English)
- **Goal:** Ensure complete visual consistency, accessibility, and 100% Thai localization coverage.
- **Milestones:**
  - [ ] **TSK-14 (P1):** Refactor `account_dialog.py` to use `i18n.tr()` for all UI strings. *(Status: Planned)*
  - [ ] **TSK-15 (P1):** Complete Thai translation dictionary in `i18n_th.py` for all auth, account, and menu dialog strings. *(Status: Planned)*
  - [ ] **TSK-16 (P2):** Update `tests/test_i18n.py` AST scanner to validate `account_dialog.py` and `auth_dialog.py`. *(Status: Planned)*
  - [ ] **TSK-17 (P2):** Audit UI high-DPI scaling, font rendering, and contrast across all 6 themes. *(Status: Planned)*

### Phase 5 — Full QA & Regression Testing
- **Goal:** Execute all test categories across core, db, loopback callbacks, UI headless, and localization.
- **Milestones:**
  - [ ] **TSK-18 (P0):** Run full unit test suite in headless environment and verify 100% pass rate. *(Status: In Progress)*
  - [ ] **TSK-19 (P1):** Add edge-case tests for path traversal, long paths, and permission-denied file trees. *(Status: Planned)*
  - [ ] **TSK-20 (P1):** Create comprehensive QA Test Plan (`docs/QA/TEST_PLAN.md`) and Test Report (`docs/QA/TEST_REPORT.md`). *(Status: Planned)*

### Phase 6 — Security Auditing & Performance Profiling
- **Goal:** Conduct static code analysis, path traversal checks, secret scanning, and file operation benchmarks.
- **Milestones:**
  - [ ] **TSK-21 (P0):** Author Security Policy & Audit Report (`docs/security/SECURITY_POLICY.md`, `docs/security/SECURITY_AUDIT.md`). *(Status: Planned)*
  - [ ] **TSK-22 (P1):** Profile and benchmark file hashing and perceptual image comparison on sample 10,000 file datasets (`docs/performance/PERFORMANCE_BASELINE.md`). *(Status: Planned)*

### Phase 7 — CI/CD & Build Packaging
- **Goal:** Implement multi-platform GitHub Actions workflows for continuous integration and automated installer creation.
- **Milestones:**
  - [ ] **TSK-23 (P0):** Create `.github/workflows/ci.yml` running unit and UI tests on Linux and Windows runners. *(Status: Planned)*
  - [ ] **TSK-24 (P1):** Verify Windows release build workflow (`.github/workflows/build-windows.yml`) with Inno Setup. *(Status: Planned)*
  - [ ] **TSK-25 (P2):** Document CI/CD and deployment procedures in `docs/devops/CI_CD_GUIDE.md`. *(Status: Planned)*

### Phase 8 — Comprehensive Documentation & Knowledge Base
- **Goal:** Complete all technical documentation, architecture diagrams, user guides, and development handoffs.
- **Milestones:**
  - [ ] **TSK-26 (P1):** Update `README.md` and create `CHANGELOG.md` reflecting v4.3 updates and CI statuses. *(Status: Planned)*
  - [ ] **TSK-27 (P1):** Author `docs/development/ARCHITECTURE.md` and `docs/development/DEVELOPMENT_GUIDE.md`. *(Status: Planned)*
  - [ ] **TSK-28 (P0):** Keep `docs/HANDOFF.md` and `docs/PHASE-LOG.md` synchronized across all development steps. *(Status: In Progress)*

### Phase 9 — Git Integration & Release Candidate Preparation
- **Goal:** Review diffs, commit conventional messages, push to team branches, submit Pull Requests against `develop`, and prepare Release Candidate.
- **Milestones:**
  - [ ] **TSK-29 (P0):** Submit PR for Phase 1 documentation and `.gitignore` integration into `develop`. *(Status: In Progress)*
  - [ ] **TSK-30 (P0):** Execute full verification gate prior to any release candidate sign-off. *(Status: Planned)*
  - [ ] **TSK-31 (P0):** Await explicit user authorization before any merge into `main` or production release. *(Status: Planned)*

---

## 5. Development Risks & Tracking

| Risk ID | Description | Severity | Mitigation Plan |
|---|---|---|---|
| **RSK-01** | Missing CI pipeline permits regression introduction on Windows runners. | High | Create `.github/workflows/ci.yml` immediately in Phase 7/DevOps. |
| **RSK-02** | User file corruption during complex organizing/undo operations. | Critical | Strict unit and regression test verification prior to modifying `core.py`. |
| **RSK-03** | Auth dialog blocking automated testing. | Medium | Add `--selftest` and `--headless` bypass hooks in `app.py`. |
