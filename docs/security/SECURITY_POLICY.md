# Smart File Manager Pro — Security Policy

**Document Version:** 1.0.0
**Effective Date:** 2026-11-10
**Status:** Active

---

## 1. Security Principles

1. **Local-First Privacy:** User files, contents, directory paths, and local SQLite metadata never leave the user's computer. The application contains zero background telemetry.
2. **Defensive File Handling:** File operations (organize, duplicate handling, renaming, cleanup) must never overwrite user data without collision resolution, and must refuse operations in protected system folders.
3. **Least Privilege Identity Management:** The desktop application connects to Supabase Auth using public publishable keys only. Secret keys, database superuser credentials, and service roles are strictly prohibited from client distribution.
4. **Input Sanitation:** All external inputs (filenames, custom folder rules, network hostnames) are validated against strict whitelists before execution.

---

## 2. Threat Model & Mitigation Strategy

| Threat Category | Potential Attack Vector | Mitigation Control |
|---|---|---|
| **Path Traversal** | Malicious rule patterns containing `../` or absolute paths | `core.ILLEGAL_NAME` regex check and `Path.resolve()` boundary verification |
| **Command Injection** | Hostname parameter in Network Ping / Port tools | `core.HOST_RE` strict character whitelisting |
| **Overwriting Data** | Renaming or organizing to an existing file path | `core.unique_dest()` generates non-colliding `name (n).ext` targets |
| **Credential Leakage** | Committing secrets or publishing tokens in logs | Automated `.gitignore` rules and suppression of sensitive token parameters in log outputs |
| **Unauthorized File Access** | Tampering with OS critical directories | `core.protected_reason()` rejects `/bin`, `/System`, `/usr`, `C:\Windows`, `C:\Program Files` |

---

## 3. Reporting a Vulnerability

If you discover a security vulnerability in Smart File Manager Pro:
1. Do not open a public GitHub issue.
2. Report the vulnerability privately to the project maintainers via email:
   - ทัตพร สีโส (`69030093@kmitl.ac.th`)
3. Provide a clear description, proof of concept, and affected environment.
4. The team will acknowledge receipt within 48 hours and coordinate remediation.
