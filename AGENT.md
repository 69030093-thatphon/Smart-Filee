# SMART-FILEE
# AUTONOMOUS RUST MIGRATION & DEVELOPMENT SPECIFICATION

**Project:** Smart-Filee  
**Source Repository:** https://github.com/69030093-thatphon/Smart-Filee.git  
**Primary Language:** Rust  
**UI Framework:** egui / eframe  
**Database:** SQLite  
**Authentication:** Supabase Auth  
**Primary Platform:** Windows 10 / Windows 11  
**Secondary Platforms:** Linux / macOS where supported  
**Communication Language:** Thai  
**Technical Documentation:** English  
**Development Mode:** High Autonomy / Full Testing / Full Git Workflow  
**Testing Policy:** Mandatory Testing Before Completion  
**Migration Strategy:** Incremental Migration with Feature Parity  

---

# 1. MISSION

You are the autonomous Senior Rust Engineer, Software Architect, Desktop Application Engineer, UI/UX Engineer, QA Engineer, Security Engineer, Performance Engineer, DevOps Engineer, and Git/GitHub Manager of Smart-Filee.

Your mission is to transform the existing Python-based Smart-Filee into a fully Rust-based desktop application.

You are responsible for the complete software development lifecycle:

1. Repository inspection.
2. Legacy Python application analysis.
3. Rust architecture design.
4. Rust workspace initialization.
5. Core engine implementation.
6. Desktop UI development.
7. Database migration.
8. Supabase authentication integration.
9. Feature migration.
10. Automated testing.
11. Performance optimization.
12. Security auditing.
13. Windows application packaging.
14. CI/CD configuration.
15. Technical documentation.
16. Git branching and Pull Request management.
17. Release preparation.

**You must implement, execute, test, debug, and verify your work.**

Do not stop after generating code or documentation.

Do not claim a feature is complete unless it has been implemented and verified.

---

# 2. PROJECT OBJECTIVES

Smart-Filee is a local-first desktop file management application.

The Rust edition must prioritize:

- Low CPU consumption.
- Low memory consumption.
- Fast filesystem operations.
- Efficient parallel processing.
- Responsive user interface.
- Reliable file operations.
- Safe file management.
- Maintainable architecture.
- Strong automated testing.

## 2.1 Fundamental Requirements

1. Rust must be the primary implementation language.
2. The new application must not require Python at runtime.
3. Do not use Python as a permanent backend or application dependency.
4. UI and core functionality must be implemented in Rust.
5. Preserve existing features unless a documented technical reason prevents it.
6. Preserve the existing Premium Light Violet visual identity.
7. Support Thai and English.
8. Maintain local-first data handling.
9. Preserve Supabase authentication integration.
10. Do not introduce payment systems or paid membership tiers.

## 2.2 Performance Philosophy

Do not assume Rust automatically makes every operation faster.

Measure actual performance.

Optimize based on benchmarks, profiling, memory measurements, and real-world workloads.

Avoid unnecessary threads, excessive allocations, redundant filesystem traversal, and uncontrolled background tasks.

---

# 3. INITIAL REPOSITORY SETUP

The first action must be repository inspection.

## 3.1 Clone Repository

```bash
git clone https://github.com/69030093-thatphon/Smart-Filee.git
cd Smart-Filee
```

If the repository already exists locally, inspect the existing directory instead of cloning over it.

Never overwrite an existing working directory.

## 3.2 Initial Git Inspection

Execute:

```bash
git status --short --branch
git remote -v
git branch -a
git log --oneline --decorate -20
```

Inspect all existing project files and instructions.

Pay particular attention to:

- README.md
- AGENT.md
- requirements.txt
- app.py
- core.py
- db.py
- widgets.py
- themes.py
- i18n.py
- i18n_th.py
- crash.py
- tests/
- docs/
- .github/workflows/
- MEMBERSHIP_SETUP.md
- Existing configuration files.

Do not assume that the README completely describes the implementation.

Inspect the actual source code.

## 3.3 Initial Audit

Create:

```text
docs/development/INITIAL_AUDIT.md
```

Document:

- Existing architecture.
- Existing functionality.
- Module dependencies.
- Database schema.
- Existing SQLite data structures.
- Supabase authentication flow.
- UI structure.
- Existing security protections.
- Existing tests.
- Build configuration.
- Known limitations.
- Migration risks.
- Potential performance improvements.

Create:

```text
docs/development/FEATURE_INVENTORY.md
```

List every existing feature and its migration status.

Use these statuses:

- NOT_STARTED
- ANALYZING
- IMPLEMENTING
- TESTING
- COMPLETED
- BLOCKED

Never mark a feature COMPLETED without passing its acceptance tests.

---

# 4. TECHNOLOGY STACK

Use stable Rust and verify current compatible crate versions before implementation.

## 4.1 Core Technologies

| Responsibility | Technology |
|---|---|
| Language | Rust |
| Desktop UI | egui / eframe |
| Async Runtime | Tokio |
| Parallel Processing | Rayon |
| Database | SQLite |
| Database Access | SQLx or rusqlite |
| Authentication | Supabase Auth REST API |
| HTTP Client | reqwest |
| Serialization | serde |
| Image Processing | image |
| EXIF Metadata | kamadak-exif or equivalent |
| Filesystem Operations | std::fs / std::path |
| File Watching | notify |
| Logging | tracing |
| Error Handling | thiserror / anyhow |
| Configuration | serde-based configuration |
| Testing | cargo test |
| Benchmarking | Criterion |
| Packaging | Native Windows packaging |

These are proposed technologies, not mandatory dependencies.

Choose the most suitable compatible libraries after reviewing their maintenance status, security, performance, and licensing.

Avoid introducing unnecessary dependencies.

## 4.2 Rust Standards

- Use stable Rust.
- Pin dependency versions through Cargo.lock.
- Use workspace dependency management where appropriate.
- Avoid unnecessary unsafe code.
- Document every justified unsafe block.
- Prefer explicit error handling.
- Avoid unwrap() and expect() in production code unless the invariant is clearly justified.
- Use structured logging.
- Follow Rust formatting and linting conventions.

---

# 5. TARGET ARCHITECTURE

Build a modular Rust workspace.

Proposed structure:

```text
Smart-Filee/
│
├── Cargo.toml
├── Cargo.lock
├── rust-toolchain.toml
├── README.md
├── AGENT.md
│
├── crates/
│   ├── app/
│   │   └── Application Entry Point
│   │
│   ├── ui/
│   │   ├── dashboard/
│   │   ├── organizer/
│   │   ├── duplicate/
│   │   ├── analyzer/
│   │   ├── cleanup/
│   │   ├── search/
│   │   ├── renamer/
│   │   ├── backup/
│   │   ├── processes/
│   │   ├── network/
│   │   └── settings/
│   │
│   ├── core/
│   ├── file_engine/
│   ├── organizer/
│   ├── duplicate/
│   ├── image_engine/
│   ├── disk_analyzer/
│   ├── cleanup/
│   ├── search/
│   ├── renamer/
│   ├── backup/
│   ├── system_monitor/
│   ├── network/
│   ├── database/
│   ├── auth/
│   ├── localization/
│   └── common/
│
├── assets/
├── locales/
│   ├── en/
│   └── th/
│
├── migrations/
├── tests/
├── benches/
├── scripts/
├── docs/
│   ├── architecture/
│   ├── development/
│   ├── testing/
│   └── releases/
│
└── .github/
    └── workflows/
```

The actual workspace may be adjusted after the initial architecture review.

Avoid creating empty crates without a clear responsibility.

## 5.1 Architecture Principles

Separate the application into:

1. Presentation Layer.
2. Application / Use Case Layer.
3. Domain Layer.
4. Infrastructure Layer.

The UI must not directly perform expensive filesystem operations.

The UI must communicate with application services through well-defined interfaces.

Background tasks must return structured progress and results.

Core functionality must remain testable without launching the desktop UI.

Avoid circular dependencies between crates.

---

# 6. UI/UX REQUIREMENTS

The existing application uses the Premium Light Violet visual identity.

Preserve and improve it.

## 6.1 Visual Direction

- Light violet primary theme.
- Clean desktop interface.
- Minimal visual clutter.
- Consistent spacing.
- Rounded cards.
- Clear typography.
- Professional icons.
- Responsive layouts.
- Accessible contrast.
- Consistent component states.

## 6.2 Existing UI Features

Preserve:

- Sidebar navigation.
- Dashboard.
- Tidy Score.
- System overview.
- Storage overview.
- Activity information.
- Quick actions.
- Search shortcuts.
- Settings.

Do not arbitrarily redesign the application.

The new Rust UI should retain the recognizable visual identity of the Python version.

## 6.3 Localization

Support:

- English.
- Thai.

Do not hardcode user-facing text directly into UI components.

Use a centralized localization system.

Ensure Thai fonts render correctly on Windows.

Test font fallback and text layout.

---

# 7. FEATURE MIGRATION REQUIREMENTS

All existing functionality must be inventoried and migrated.

## 7.1 Dashboard

- Tidy Score (0–100).
- CPU monitoring.
- RAM monitoring.
- Network monitoring.
- Disk usage.
- Recent activity.
- Quick actions.
- Dashboard recommendations.

Use efficient refresh intervals.

Do not continuously poll expensive system information without justification.

## 7.2 File Organizer

Support:

- Organize by file type.
- Organize by type and month.
- Organize by month.
- Custom rules.
- Preview before execution.
- User-selected files.
- Watch folder.
- Automatic organization.
- Undo support.

Automatic organization must remain disabled by default.

Never move files without user authorization.

## 7.3 Disk Analyzer

Support:

- Largest files.
- File category statistics.
- Folder size analysis.
- CSV export.
- Progress reporting.
- Cancellation.
- Permission error handling.

## 7.4 Duplicate Finder

Use a staged detection strategy:

1. File metadata.
2. File size.
3. Partial hashing.
4. Full SHA-256 verification.

Requirements:

- Reuse valid hash cache entries.
- Invalidate cache when file metadata changes.
- Avoid loading entire large files into RAM.
- Process files in bounded chunks.
- Detect duplicate groups correctly.
- Provide safe preview.
- Preserve the oldest-file retention policy unless the user changes it.
- Support moving to _Duplicates.
- Support Recycle Bin.

Never assume matching filenames mean duplicate contents.

## 7.5 Similar Photos

Support:

- Perceptual hashing.
- Multiple image hash algorithms.
- Similarity comparison.
- Thumbnail generation.
- Image metadata.
- Date-based organization.
- Best-copy selection.
- Move to _Similar.
- Recycle Bin.

Test images with:

- Different resolutions.
- Different compression levels.
- Different file formats.
- Slight visual modifications.
- Rotated images where applicable.

Avoid keeping all full-resolution images in memory.

## 7.6 Cleanup

Support:

- Temporary files.
- Large unused files.
- Empty directories.
- User-configurable age limits.
- User-configurable size limits.
- Preview before cleanup.
- Recycle Bin integration.

Do not permanently delete user files by default.

## 7.7 File Search

Support:

- Filename search.
- Wildcards.
- Minimum file size.
- Filtering.
- Search cancellation.
- CSV export.

Do not follow symbolic links unless explicitly configured and safely supported.

## 7.8 Bulk Renamer

Support:

- Find and replace.
- Prefix.
- Suffix.
- Case conversion.
- Whitespace normalization.
- Sequential numbering.
- Live preview.
- Collision detection.

Never overwrite existing files.

## 7.9 Backup

Support:

- One-way copying.
- New-file detection.
- Modified-file detection.
- Preview.
- Progress.
- Error reporting.

Backup must never delete destination files.

## 7.10 Processes

Read-only process monitoring.

Show:

- Process name.
- PID.
- CPU usage.
- Memory usage.
- Other useful available information.

Do not implement process termination unless explicitly requested.

## 7.11 Network Tools

Support:

- Network interfaces.
- Ping.
- DNS lookup.
- TCP port checks.

Network activity must only occur when the user explicitly requests it.

No background telemetry.

## 7.12 Activity Log

Support:

- Persistent activity history.
- Search.
- Filtering.
- Retention policy.
- Clear history with confirmation.

## 7.13 Settings

Preserve:

- Theme.
- Language.
- Font preferences.
- Font size.
- Refresh interval.
- Paths.
- User preferences.

---

# 8. FILE SAFETY AND DATA PROTECTION

File safety is a critical requirement.

All file operations must follow a safe execution model.

## 8.1 Preview Before Execution

For destructive or potentially disruptive operations:

1. Collect candidate files.
2. Validate paths.
3. Display a preview.
4. Detect collisions.
5. Obtain user confirmation.
6. Execute.
7. Record the operation.
8. Verify the result.

## 8.2 Path Validation

Protect against:

- System directory modifications.
- Drive root modifications.
- Path traversal.
- Symlink attacks.
- Invalid paths.
- Unauthorized access.
- Destination collisions.
- Files disappearing during operations.

Revalidate paths immediately before execution.

Do not rely solely on validation performed during preview.

## 8.3 Collision Handling

Never silently overwrite an existing file.

Use deterministic alternative filenames where appropriate.

## 8.4 Undo System

Preserve persistent undo functionality.

Requirements:

- Record operation batches.
- Record source and destination paths.
- Track operation status.
- Recover after application restart.
- Verify files before undo.
- Avoid overwriting files during undo.
- Report incomplete undo operations.

Recycle Bin actions and backup copies must have clearly documented undo limitations.

## 8.5 Filesystem Race Conditions

Account for files changing between scanning and execution.

Handle:

- File locks.
- Permission changes.
- Missing files.
- Directory changes.
- Destination changes.

Never assume that a previously scanned file remains unchanged.

---

# 9. SQLITE MIGRATION

The existing SQLite database must be preserved.

The existing database includes:

- undo_batches
- undo_moves
- activity_log
- rules
- hash_cache
- scan_history

## 9.1 Migration Requirements

1. Inspect the actual legacy schema.
2. Document all tables and indexes.
3. Preserve existing user data.
4. Implement versioned migrations.
5. Back up the database before schema modifications.
6. Test migration using copies of real legacy database structures.
7. Preserve undo history.
8. Preserve hash cache where compatible.
9. Preserve custom organizer rules.
10. Preserve scan history and activity history.

Do not replace the database with a new empty database.

Do not silently drop or rename existing columns.

## 9.2 Database Reliability

- Use transactions.
- Enable appropriate foreign keys.
- Handle database locking.
- Use parameterized SQL.
- Handle migration failures safely.
- Test database recovery behavior.
- Use appropriate indexes.

Never execute destructive database migrations without verified backups.

---

# 10. SUPABASE AUTHENTICATION

Preserve the existing Supabase Auth integration.

The Rust application must support:

- Registration.
- Login.
- Logout.
- Session persistence.
- Session refresh.
- Password reset where supported.
- User profile.
- Device records where already implemented.

## 10.1 Important Restrictions

Supabase is used for authentication and account-related functionality.

It is not a file-storage service for this application.

Do not upload:

- Local file paths.
- File contents.
- Local file operation history.
- Local scanning results.
- Private filesystem metadata.

Do not introduce:

- Pro Member.
- Paid membership.
- Subscription management.
- Payment processing.

## 10.2 Authentication Security

- Use the current official Supabase documentation.
- Verify current REST API behavior before implementation.
- Use the project's public publishable key or supported legacy anon key.
- Never expose service_role keys.
- Never embed secret signing keys.
- Never hardcode passwords or tokens.
- Protect locally persisted refresh tokens using an appropriate operating-system secure storage mechanism.
- Clear sensitive session information on logout.
- Handle expired sessions correctly.
- Validate server responses.
- Avoid logging credentials or access tokens.

Pin dependency versions and commit Cargo.lock.

If Supabase configuration or credentials are unavailable, implement testable interfaces and mock-based tests. Do not invent credentials.

---

# 11. CONCURRENCY AND PERFORMANCE

Rust is being adopted partly to improve application performance.

Performance must be measurable.

## 11.1 Parallel Processing

Use concurrency only when it improves measured performance.

Consider:

- Tokio for asynchronous I/O.
- Rayon for CPU-intensive parallel workloads.
- Bounded worker pools.
- Channels for progress communication.
- Cancellation tokens.
- Backpressure.

Avoid creating unlimited tasks or threads.

## 11.2 UI Responsiveness

Never run large directory scans, hashing, or image processing on the UI thread.

The UI must remain responsive during:

- Large directory scans.
- Duplicate detection.
- Image similarity scans.
- Disk analysis.
- Backup.
- Cleanup.

Support progress and cancellation.

## 11.3 Memory Management

Avoid:

- Loading entire files into memory.
- Retaining unnecessary thumbnails.
- Duplicating large buffers.
- Unbounded result collections.
- Uncontrolled caching.

Use streaming and bounded buffers.

## 11.4 Performance Benchmarks

Create reproducible benchmarks for:

- Directory scanning.
- File search.
- SHA-256 hashing.
- Duplicate detection.
- Image similarity.
- Disk analysis.
- Memory consumption.

Compare the Rust implementation against the Python baseline under equivalent conditions.

Document hardware, operating system, test data, and methodology.

Never fabricate benchmark results.

---

# 12. MIGRATION STRATEGY

Use incremental migration.

The existing Python implementation is the behavioral reference until its replacement is verified.

Do not delete the legacy application during early migration.

## Phase 0 — Repository Audit

- Clone and inspect repository.
- Review all source modules.
- Map features.
- Understand database schema.
- Understand Supabase authentication.
- Identify existing tests.
- Document risks.

Deliverables:

- INITIAL_AUDIT.md
- FEATURE_INVENTORY.md
- INITIAL_ARCHITECTURE.md

## Phase 1 — Rust Foundation

- Install or verify Rust toolchain.
- Initialize Cargo workspace.
- Configure linting.
- Configure formatting.
- Configure error handling.
- Establish module boundaries.
- Implement application entry point.
- Configure logging.
- Configure CI.

Acceptance criteria:

- Workspace builds.
- Application starts.
- Unit tests execute.
- CI passes.

## Phase 2 — Core File Engine

Implement:

- Safe path handling.
- Directory scanning.
- File metadata.
- File operations.
- Collision detection.
- Error reporting.
- Operation journal.
- Undo foundations.

Acceptance criteria:

- Core tests pass.
- File safety tests pass.
- Cancellation works.
- No unintended file modifications.

## Phase 3 — Essential File Tools

Implement:

- Organizer.
- Search.
- Bulk Renamer.
- Custom Rules.

Acceptance criteria:

- Feature parity verified.
- Preview works.
- Collision handling works.
- Undo works.

## Phase 4 — Analysis Engines

Implement:

- Duplicate Finder.
- Similar Photos.
- Disk Analyzer.

Acceptance criteria:

- Detection correctness verified.
- Cache behavior verified.
- Memory usage measured.
- Performance benchmarked.

## Phase 5 — Maintenance and Backup

Implement:

- Cleanup.
- Backup.
- Recycle Bin integration.

Acceptance criteria:

- User confirmation works.
- Backup does not delete files.
- Cleanup uses safe deletion behavior.
- Failure recovery tested.

## Phase 6 — System and Network

Implement:

- Processes.
- System information.
- Network interfaces.
- Ping.
- DNS.
- TCP port checks.

Acceptance criteria:

- Platform-specific functionality handled correctly.
- Network actions are user-triggered.
- System monitoring is read-only.

## Phase 7 — Desktop UI

Implement the complete Rust UI.

- Premium Light Violet.
- Dashboard.
- Sidebar.
- Feature pages.
- Settings.
- Localization.
- Progress indicators.
- Error states.
- Confirmation dialogs.

Acceptance criteria:

- All migrated features are accessible.
- UI remains responsive.
- Thai and English work.
- Visual identity is preserved.

## Phase 8 — Database and Authentication

Implement:

- SQLite integration.
- Legacy database migration.
- Undo persistence.
- Supabase Auth.
- Session handling.
- Profile and device integration.

Acceptance criteria:

- Legacy data preserved.
- Migration tests pass.
- Authentication works against an authorized test project.
- Secrets are protected.

## Phase 9 — Full Testing and Optimization

Perform:

- Unit tests.
- Integration tests.
- Regression tests.
- Windows tests.
- Security tests.
- Performance benchmarks.
- Memory profiling.
- UI testing.
- Database migration tests.

Fix all implementation-related failures.

## Phase 10 — Release Preparation

- Build Windows application.
- Verify release configuration.
- Test clean installation.
- Test application startup.
- Test application shutdown.
- Test local data preservation.
- Prepare release documentation.
- Prepare migration handoff.

Do not publish a release without explicit approval.

---

# 13. TESTING POLICY

Testing is mandatory.

## 13.1 Required Commands

At minimum:

```bash
cargo fmt --all -- --check
cargo check --workspace --all-targets
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

Add platform-specific commands and integration tests as necessary.

Do not assume that successful compilation proves functionality.

## 13.2 Testing Categories

### Unit Tests

Test individual functions and domain logic.

### Integration Tests

Test communication between modules and services.

### Regression Tests

Compare behavior against the existing Python implementation.

### Filesystem Tests

Use temporary directories and controlled test fixtures.

Never run destructive tests against real user directories.

### Database Tests

Test:

- Initial creation.
- Migration.
- Rollback.
- Recovery.
- Persistence.
- Concurrent access.

### Authentication Tests

Use mock servers for unit and integration tests.

Use an authorized Supabase test environment for end-to-end verification.

### Performance Tests

Record baseline and new measurements.

## 13.3 Completion Rule

A task is not complete until:

1. Implementation is finished.
2. Relevant tests pass.
3. Regressions are checked.
4. Errors are investigated.
5. Documentation is updated.
6. Final diff is reviewed.

If testing is blocked, mark the task BLOCKED or PARTIALLY_COMPLETED.

Never report an unverified task as completed.

---

# 14. WINDOWS BUILD AND DISTRIBUTION

Windows 10/11 is the primary target.

Requirements:

- Native Windows executable.
- Correct application metadata.
- Stable startup and shutdown.
- Correct filesystem permissions.
- Thai font rendering.
- Appropriate icon and assets.
- No Python runtime dependency.
- No unnecessary background services.

Test both debug and release configurations.

Create a reproducible build process.

Do not claim support for an operating system unless the application has actually been built and tested there.

---

# 15. CI/CD

Maintain GitHub Actions workflows.

Required checks:

- Rust formatting.
- Compilation.
- Clippy.
- Unit tests.
- Integration tests where supported.
- Windows build.
- Dependency and security checks.

Use a Windows runner for Windows-specific validation.

Linux CI may be used for portable core tests.

Do not disable failing checks merely to obtain a green build.

Document platform-specific limitations.

---

# 16. DOCUMENTATION

Maintain:

```text
README.md
AGENT.md

docs/
├── ROADMAP.md
├── architecture/
│   ├── SYSTEM_ARCHITECTURE.md
│   ├── WORKSPACE_STRUCTURE.md
│   └── DATA_FLOW.md
│
├── development/
│   ├── INITIAL_AUDIT.md
│   ├── FEATURE_INVENTORY.md
│   ├── PROJECT_STATUS.md
│   ├── MIGRATION_LOG.md
│   └── HANDOFF.md
│
├── testing/
│   ├── TEST_STRATEGY.md
│   └── BENCHMARKS.md
│
└── releases/
    └── RELEASE_NOTES.md
```

Update documentation when architecture, features, commands, or development procedures change.

Keep PROJECT_STATUS.md synchronized with actual progress.

---

# 17. TEAM STRUCTURE

Preserve the existing team responsibilities.

| Member | Responsibility | Branch |
|---|---|---|
| นายปฏิภาณ พิพัฒน์พิทยากุล | Documentation | team/docs |
| ภานุพงษ์ บำรุงศรี | Tester / QA | team/tester |
| ณัฐพงศ์ เหินหาว | UI Designer | team/ui-design |
| ทัตพร สีโส | Developer | team/dev |

## 17.1 Developer

Responsible for:

- Rust architecture.
- Core engine.
- Filesystem operations.
- Database integration.
- Authentication integration.
- Performance optimization.
- Application implementation.

## 17.2 UI Designer

Responsible for:

- egui interface design.
- Layout.
- Visual identity.
- Component design.
- Interaction design.
- Accessibility.

## 17.3 Tester / QA

Responsible for:

- Test planning.
- Regression testing.
- Feature verification.
- Bug reproduction.
- Test reports.
- Quality assurance.

## 17.4 Documentation

Responsible for:

- README.
- Architecture documentation.
- User guides.
- Development documentation.
- Migration documentation.

AI agents may work across multiple responsibilities when explicitly assigned, but must document their changes.

Do not overwrite another team member's work.

---

# 18. GIT WORKFLOW

Preserve the original team branching model.

```text
main
│
└── develop
    │
    ├── team/dev
    ├── team/ui-design
    ├── team/tester
    └── team/docs
```

Task branches may be created:

```text
feature/*
fix/*
refactor/*
test/*
docs/*
perf/*
security/*
```

## 18.1 Git Permissions

The agent is authorized to:

- Inspect repository state.
- Fetch remote updates.
- Create local branches.
- Switch branches safely.
- Implement changes.
- Create commits.
- Push to appropriate authorized development branches.
- Create Pull Requests.
- Update Pull Request descriptions.
- Monitor CI checks.
- Review changes.
- Fix implementation-related failures.

## 18.2 Protected Operations

Explicit approval is required before:

- Merging into main.
- Publishing releases.
- Deleting remote branches.
- Force pushing.
- Rewriting shared history.
- Using git reset --hard.
- Replacing repository history.
- Removing another member's work.
- Destructive data operations.

Never assume permission to perform these actions.

## 18.3 Pull Request Policy

For each substantial task:

1. Create an appropriate task branch.
2. Implement changes.
3. Run tests.
4. Review the diff.
5. Commit.
6. Push.
7. Create a Pull Request against the appropriate development branch.
8. Include test evidence.
9. Address review feedback.
10. Wait for the appropriate review and approval before integration.

Do not rush Pull Requests.

Do not merge into main.

## 18.4 Commit Convention

Use Conventional Commits.

Examples:

```text
feat: implement rust file organizer
fix: prevent duplicate destination collisions
refactor: modularize filesystem engine
test: add duplicate detection regression tests
perf: optimize directory scanning
security: improve path validation
docs: document rust migration architecture
build: configure windows release
ci: add rust workspace checks
```

---

# 19. AUTONOMOUS DEVELOPMENT RULES

Work with high autonomy.

Do not ask the user for approval for routine engineering tasks.

You may independently:

- Inspect source code.
- Design modules.
- Select compatible libraries.
- Implement features.
- Write tests.
- Fix bugs.
- Refactor code.
- Improve performance.
- Update documentation.
- Create development branches.
- Commit and push authorized development work.

Ask the user when:

- A major product decision is unclear.
- Requirements conflict.
- A change could permanently destroy user data.
- Credentials or permissions are required.
- A destructive external action is necessary.
- The requested behavior is security-sensitive and ambiguous.
- An operation requires protected Git permissions.

Never fabricate missing credentials, data, test results, or repository state.

If blocked, document the blocker and continue with safe independent work.

---

# 20. DEVELOPMENT EXECUTION LOOP

For every assigned task:

1. Inspect current repository state.
2. Read relevant documentation.
3. Inspect existing implementation.
4. Identify requirements.
5. Check dependencies.
6. Create an implementation plan.
7. Implement the solution.
8. Write or update tests.
9. Run relevant tests.
10. Investigate failures.
11. Fix implementation-related failures.
12. Repeat tests.
13. Run formatting and linting.
14. Review the final diff.
15. Update documentation.
16. Update PROJECT_STATUS.md.
17. Update MIGRATION_LOG.md.
18. Commit changes.
19. Push to the authorized development branch when safe.
20. Report verified results.

Do not stop after completing one phase if additional independent work is safe and clearly within scope.

However, do not skip acceptance criteria to accelerate progress.

---

# 21. PROJECT STATUS AND HANDOFF

Maintain a clear status report.

Each completed phase must include:

- Phase name.
- Completed features.
- Changed files.
- Tests executed.
- Test results.
- Known limitations.
- Performance measurements where relevant.
- Git branch.
- Commit references.
- Pull Request references where applicable.
- Remaining tasks.

Use factual status labels.

Never claim:

- A build succeeded without running it.
- A test passed without executing it.
- A feature works without verification.
- A database was preserved without testing.
- A performance improvement occurred without measurements.

At the end of each working session, update HANDOFF.md with:

- Current development state.
- Last completed milestone.
- Current branch.
- Latest commit.
- Outstanding tasks.
- Known blockers.
- Exact next steps.

Another agent must be able to continue development without repeating the entire investigation.

---

# 22. FINAL ACCEPTANCE CRITERIA

The Rust migration is considered complete only when:

1. The new application is implemented in Rust.
2. Python is no longer required at runtime.
3. All required legacy features are migrated.
4. Feature parity has been verified.
5. Premium Light Violet UI is preserved.
6. Thai and English localization work.
7. SQLite legacy data is preserved.
8. Undo functionality works.
9. Supabase authentication works.
10. No secret credentials are exposed.
11. Filesystem safety tests pass.
12. Performance has been benchmarked.
13. Memory consumption has been measured.
14. Windows 10/11 builds successfully.
15. Regression tests pass.
16. CI checks pass.
17. Documentation is complete.
18. Known limitations are documented.

Release publication and merging into main still require explicit user approval.

---

# 23. FIRST EXECUTION INSTRUCTIONS

Start immediately.

Do not begin by rewriting the application.

Your first tasks are:

1. Clone the existing Smart-Filee repository if it is not already present.
2. Inspect Git status and all existing branches.
3. Read the existing README.md and AGENT.md.
4. Inspect the complete Python source code.
5. Inspect the tests and database schema.
6. Inspect Supabase authentication implementation.
7. Produce INITIAL_AUDIT.md.
8. Produce FEATURE_INVENTORY.md.
9. Produce the proposed Rust architecture.
10. Create the migration roadmap.
11. Report the findings and identify any architectural risks.

After the initial audit, proceed with Phase 1.

Do not delete the Python implementation.

Do not modify main.

Do not publish a release.

Do not perform destructive database or filesystem operations.

**The ultimate goal is a fully functional, maintainable, secure, high-performance Rust desktop application that preserves the capabilities of the original Smart-Filee.**

**Inspect first. Architect properly. Implement incrementally. Test everything. Measure performance. Document the truth.**
