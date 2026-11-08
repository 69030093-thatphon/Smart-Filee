# Smart File Manager Pro — Supabase Membership

## Architecture

- Desktop client: Python + PySide6.
- Identity: Supabase Auth (email + password).
- Online account data: `public.sfm_profiles`, `public.sfm_user_roles`, `public.sfm_user_devices`, and `public.sfm_login_logs`.
- Authorization: PostgreSQL Row Level Security (RLS) policies already configured in the connected Supabase project.
- Local-only data: the existing SQLite database (`sfm.db`) remains local for undo history, organizer rules, hash cache, scan history, and activity logs. File paths and file contents are not uploaded.

There is no Pro Member tier. Accounts have a standard `user` role by default; `admin` is assigned only through a trusted administrative operation.

## Configure

The included `.env` is preconfigured for the connected Supabase project. `.env.example` is the template if you need to recreate it.

```text
SUPABASE_URL=https://fyuzbacxqbrqkmeubwdb.supabase.co
SUPABASE_PUBLISHABLE_KEY=sb_publishable_...
```

Only a Supabase publishable key belongs in this desktop app. Never add a `service_role` or `sb_secret_` key, database password, or JWT signing secret to the client. The publishable key is not a password; RLS is what restricts account data.

For a Windows build, place `.env` beside `SmartFileManagerPro.exe`. Do not commit `.env` to a public repository. The `.env.example` file is safe to keep as a template.

## Run

1. Install Python 3.11+ (recommended) and run `run.bat`, or install dependencies using `python -m pip install -r requirements.txt`.
2. Launch with `python app.py`.
3. Create an account using a valid email and a password of at least 12 characters.
4. If email confirmation is enabled, keep Smart File Manager Pro open and click the confirmation link. The app receives the one-time token hash through a loopback callback and signs you in automatically.
5. Use **Account → Profile & devices…** to edit the display name and view device records.
6. Use **Account → Sign out and exit** to sign out and close the app.

## Session and device notes

- The Supabase SDK manages the active session in process memory. This version does not save access/refresh tokens to a local file; users sign in again after restarting the app.
- Device metadata contains the machine name, OS/platform, and app version. No file paths, file contents, or local activity logs are uploaded.
- Marking a device as revoked updates the account's device record. It does not forcibly invalidate a different active Supabase Auth session. Enforced remote session revocation requires a trusted server-side/admin operation and is intentionally not performed using a client key.
- Login/device logging is best-effort and does not block sign-in if the log endpoint is temporarily unavailable.

## Supabase Auth settings — required for automatic confirmation

In the Supabase Dashboard, open **Authentication → URL Configuration** and add this Redirect URL (keep the existing URLs):

```text
http://127.0.0.1:8765/auth/callback/**
```

Then open **Authentication → Email Templates → Confirm signup** and set the confirmation link to use the one-time token hash directly with the redirect URL supplied by the desktop app. The link in the template should be:

```html
<a href="{{ .RedirectTo }}?token_hash={{ .TokenHash }}&type=email">Confirm email address</a>
```

Do not use `{{ .ConfirmationURL }}` for this desktop callback template. The app verifies the one-time `token_hash` with Supabase Auth; it never accepts access or refresh tokens from the browser URL. Keep the existing **Site URL** unless you have a separate website that needs it.

The app binds only to `127.0.0.1` on port `8765`, checks a random per-registration callback path, suppresses HTTP request logging, and stops the listener after completion or dialog close. Open the confirmation email in a browser on the same computer and keep Smart File Manager Pro open until the confirmation finishes. If port 8765 is occupied, close the other app using it and retry.

After these dashboard settings are saved, register a new test account. The browser should return to the local callback page, then the desktop app should verify the email and open the workspace automatically. Existing accounts can still sign in normally after confirming their email.

## Local database

The original SQLite file and local file-management features are unchanged. Online account features use Supabase Auth and the `sfm_*` tables; they do not replace the local SQLite database.
