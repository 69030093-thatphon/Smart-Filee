# Smart-Filee — Data Flow & Lifecycle Specification

---

## 1. Asynchronous Execution & Progress Reporting Flow

All heavyweight operations follow a standardized asynchronous actor/worker data flow:

```mermaid
sequenceDiagram
    participant UI as Desktop UI (egui)
    participant Channel as Communication Channel (mpsc)
    participant Worker as Background Worker (Tokio/Rayon)
    participant FS as Filesystem / SQLite

    UI->>Worker: Dispatch Command (e.g. ScanDuplicates { path, min_size }) + cancel_token
    activate Worker
    Worker->>FS: Query HashCache (SQLite)
    Worker->>FS: Parallel Walk Files (Rayon)
    loop Every N files / candidates
        Worker->>Channel: Send ProgressUpdate(scanned, current_file)
        Channel-->>UI: Poll & Display Progress
    end
    Worker->>FS: Verify full SHA-256 for candidate collisions
    Worker->>FS: Write new hashes to HashCache
    Worker->>Channel: Send Completed(DuplicateResult)
    deactivate Worker
    Channel-->>UI: Render Duplicate Groups in Table
```

---

## 2. Destructive Operation & Undo Journal Lifecycle

Every move, rename, cleanup, or organize action strictly adheres to this 8-step lifecycle:

```
1. SCAN & COLLECT
   Target files collected based on user configuration.
          │
2. PATH VALIDATION
   Validate that source and destination are not filesystem roots or OS system folders.
          │
3. PREVIEW GENERATION
   Pre-calculate planned changes and collision resolutions.
          │
4. USER CONFIRMATION
   Present preview table and obtain user confirmation.
          │
5. TRANSACTION JOURNALING
   Begin SQLite transaction: insert row into `undo_batches` with status 'active'.
   Insert planned (src, dst) rows into `undo_moves`.
   Commit SQLite transaction.
          │
6. ATOMIC EXECUTION
   Execute physical file moves using deterministic unique names.
   Log unreadable/locked file errors without aborting entire batch.
          │
7. RE-EVALUATE & CLEANUP
   Prune empty created category/month directories if operations were reverted.
          │
8. LOG & VERIFY
   Append entry to `activity_log`. Update in-memory undo stack.
```

---

## 3. Supabase Email Confirmation Loopback Flow

```mermaid
sequenceDiagram
    actor User
    participant App as Smart-Filee App
    participant Listener as Local HTTP Server (127.0.0.1:8765)
    participant Browser as Web Browser / Email Client
    participant Supabase as Supabase Auth Cloud

    User->>App: Enter Email, Display Name, Password
    App->>Listener: Start Listener with random state UUID
    App->>Supabase: POST /auth/v1/signup with redirect_to=http://127.0.0.1:8765/auth/callback/{state}
    Supabase-->>App: Signup accepted (needs_email_confirmation: true)
    App->>User: Show "Waiting for email confirmation..." dialog
    Supabase->>User: Send Email with confirmation link
    User->>Browser: Click confirmation link
    Browser->>Listener: GET /auth/callback/{state}?token_hash=OTP_HASH&type=email
    Listener-->>Browser: HTTP 200 "Email confirmation received. You may close this tab."
    Listener->>App: Deliver { token_hash, type } result
    App->>Listener: Stop and close HTTP server
    App->>Supabase: POST /auth/v1/verify { email, token_hash, type: "email" }
    Supabase-->>App: Session established (JWT access_token, user_id)
    App->>Supabase: Record device in sfm_user_devices & sfm_login_logs
    App->>User: Open Main Application Workspace
```
