"""Supabase Auth client for Smart File Manager Pro.

Only the Supabase project URL and publishable key are used by this desktop app.
No service_role key, database password, or server-side secret belongs here.
The auth session is kept in memory for this process; SQLite remains local.
"""
from __future__ import annotations

import os
import platform
import socket
import sys
import secrets
import threading
from http.server import BaseHTTPRequestHandler, HTTPServer
from urllib.parse import parse_qs, urlsplit
from datetime import datetime, timezone
from pathlib import Path
from typing import Any

from dotenv import load_dotenv
from supabase import Client, create_client

APP_DIR = Path(__file__).resolve().parent
APP_VERSION = "4.2"
_client: Client | None = None


class MembershipError(Exception):
    """Safe, user-facing authentication/configuration error."""


def _load_settings() -> None:
    candidates = [
        APP_DIR / ".env",
        Path.home() / ".smart_file_manager_pro" / ".env",
    ]
    if getattr(sys, "frozen", False):
        candidates.insert(0, Path(sys.executable).resolve().parent / ".env")
    for candidate in candidates:
        if candidate.is_file():
            load_dotenv(candidate, override=False)


def _config() -> tuple[str, str]:
    _load_settings()
    url = os.getenv("SUPABASE_URL", "").strip().rstrip("/")
    key = os.getenv("SUPABASE_PUBLISHABLE_KEY", "").strip()
    if not url or not key:
        raise MembershipError(
            "Supabase is not configured. Copy .env.example to .env and set "
            "SUPABASE_URL and SUPABASE_PUBLISHABLE_KEY."
        )
    if not url.startswith("https://") or ".supabase.co" not in url:
        raise MembershipError("SUPABASE_URL must be the HTTPS URL for your Supabase project.")
    if key.startswith("sb_secret_") or "service_role" in key.lower():
        raise MembershipError(
            "A secret/service_role key was detected. Use only the Supabase publishable key."
        )
    return url, key


def _supabase() -> Client:
    global _client
    if _client is None:
        url, key = _config()
        try:
            _client = create_client(url, key)
        except Exception as exc:
            raise MembershipError("Could not initialize the Supabase client. Check the app configuration.") from exc
    return _client


def check_server() -> None:
    """Validate configuration before showing the auth dialog; network errors surface on login."""
    _config()
    _supabase()


def _attr(obj: Any, name: str, default=None):
    if isinstance(obj, dict):
        return obj.get(name, default)
    return getattr(obj, name, default)


def _session(response):
    return _attr(response, "session")


def _user(response):
    return _attr(response, "user")


def _user_dict(user) -> dict:
    if not user:
        return {}
    metadata = _attr(user, "user_metadata", {}) or {}
    return {
        "id": str(_attr(user, "id", "")),
        "email": _attr(user, "email", "") or "",
        "display_name": metadata.get("display_name") or metadata.get("full_name") or "",
        "avatar_url": metadata.get("avatar_url") or "",
        "role": "user",
    }


def _enrich_user(user) -> dict:
    result = _user_dict(user)
    if not result.get("id"):
        return result
    client = _supabase()
    try:
        profile_response = (
            client.table("sfm_profiles")
            .select("display_name,username,avatar_url")
            .eq("id", result["id"])
            .limit(1)
            .execute()
        )
        if profile_response.data:
            profile = profile_response.data[0]
            result["display_name"] = profile.get("display_name") or profile.get("username") or result["display_name"]
            result["avatar_url"] = profile.get("avatar_url") or result["avatar_url"]
    except Exception:
        # Auth is still valid if optional profile enrichment is temporarily unavailable.
        pass
    try:
        role_response = (
            client.table("sfm_user_roles")
            .select("role")
            .eq("user_id", result["id"])
            .limit(1)
            .execute()
        )
        if role_response.data:
            role = role_response.data[0].get("role")
            if role in ("user", "admin"):
                result["role"] = role
    except Exception:
        pass
    return result


def _track_device(user_id: str, app_version: str, event_type: str) -> str | None:
    """Record device/login metadata without sending file paths or file activity."""
    client = _supabase()
    device_name = socket.gethostname()[:120] or "Unknown device"
    platform_name = f"{platform.system()} {platform.release()}"[:120]
    device_id = None
    try:
        existing = (
            client.table("sfm_user_devices")
            .select("id")
            .eq("user_id", user_id)
            .eq("device_name", device_name)
            .is_("revoked_at", "null")
            .limit(1)
            .execute()
        )
        if existing.data:
            device_id = existing.data[0]["id"]
            client.table("sfm_user_devices").update({
                "platform": platform_name,
                "app_version": app_version,
                "last_seen_at": datetime.now(timezone.utc).isoformat(),
            }).eq("id", device_id).eq("user_id", user_id).execute()
        else:
            created = client.table("sfm_user_devices").insert({
                "user_id": user_id,
                "device_name": device_name,
                "platform": platform_name,
                "app_version": app_version,
            }).execute()
            if created.data:
                device_id = created.data[0].get("id")
    except Exception:
        # Do not lock a legitimate user out if device tracking is temporarily unavailable.
        pass
    try:
        client.table("sfm_login_logs").insert({
            "user_id": user_id,
            "event_type": event_type,
            "device_id": device_id,
            "details": {"app_version": app_version, "platform": platform_name},
        }).execute()
    except Exception:
        pass
    return device_id


class EmailConfirmationListener:
    """One-shot loopback callback for Supabase email confirmation.

    The callback receives only Supabase's single-use token_hash, never an
    access token or refresh token. Verification happens in the app process.
    """

    HOST = "127.0.0.1"
    PORT = 8765

    def __init__(self):
        self._state = secrets.token_urlsafe(24)
        self._result = None
        self._lock = threading.Lock()
        owner = self

        class Handler(BaseHTTPRequestHandler):
            def do_GET(self):
                parsed = urlsplit(self.path)
                expected_path = f"/auth/callback/{owner._state}"
                if parsed.path != expected_path:
                    self.send_error(404)
                    return

                query = parse_qs(parsed.query, keep_blank_values=True)
                token_hash = (query.get("token_hash") or [""])[0]
                otp_type = (query.get("type") or [""])[0]
                error = (query.get("error_description") or query.get("error") or [""])[0]
                if len(token_hash) > 2048:
                    token_hash = ""
                if token_hash and otp_type == "email":
                    result = {"token_hash": token_hash, "type": "email"}
                elif error:
                    result = {"error": error[:300]}
                else:
                    result = {"error": "The confirmation link is incomplete or invalid."}

                with owner._lock:
                    if owner._result is None:
                        owner._result = result

                body = (
                    "<!doctype html><html><head><meta charset='utf-8'>"
                    "<meta name='viewport' content='width=device-width,initial-scale=1'>"
                    "<title>Smart File Manager Pro</title>"
                    "<script>history.replaceState({}, document.title, location.pathname);</script>"
                    "</head>"
                    "<body style='font:16px system-ui;max-width:560px;margin:12vh auto;padding:24px;color:#302943'>"
                    "<h2>Email confirmation received</h2>"
                    "<p>Return to Smart File Manager Pro. The app is completing your sign-in.</p>"
                    "<p>You can close this browser tab.</p></body></html>"
                ).encode("utf-8")
                self.send_response(200)
                self.send_header("Content-Type", "text/html; charset=utf-8")
                self.send_header("Content-Length", str(len(body)))
                self.send_header("Cache-Control", "no-store")
                self.send_header("Referrer-Policy", "no-referrer")
                self.send_header("X-Content-Type-Options", "nosniff")
                self.send_header("Pragma", "no-cache")
                self.end_headers()
                self.wfile.write(body)

            def log_message(self, fmt, *args):
                # Do not log callback URLs: they contain a single-use OTP hash.
                return

        try:
            self._server = HTTPServer((self.HOST, self.PORT), Handler)
        except OSError as exc:
            raise MembershipError(
                f"Could not start the email callback on {self.HOST}:{self.PORT}. "
                "Close the app instance using this port and try again."
            ) from exc
        self._server.daemon_threads = True
        self._thread = threading.Thread(
            target=self._server.serve_forever, name="SFMEmailCallback", daemon=True
        )
        self._thread.start()

    @property
    def redirect_url(self) -> str:
        return f"http://{self.HOST}:{self.PORT}/auth/callback/{self._state}"

    def take_result(self) -> dict | None:
        with self._lock:
            result = self._result
            self._result = None
            return result

    def close(self) -> None:
        server = getattr(self, "_server", None)
        if server is not None:
            self._server = None
            try:
                server.shutdown()
                server.server_close()
            except Exception:
                pass


def start_email_confirmation_listener() -> EmailConfirmationListener:
    """Start the local callback before requesting the signup email."""
    return EmailConfirmationListener()


def complete_email_confirmation(
    callback_result: dict, email: str, app_version: str = APP_VERSION
) -> dict:
    """Verify a one-time email OTP hash and establish the Supabase session."""
    token_hash = (callback_result or {}).get("token_hash", "")
    otp_type = (callback_result or {}).get("type", "email")
    email = (email or "").strip().lower()
    if not email or not token_hash or otp_type != "email":
        raise MembershipError("The email confirmation link is invalid or incomplete. Request a new email.")
    try:
        response = _supabase().auth.verify_otp({
            "email": email, "token_hash": token_hash, "type": "email"
        })
        user = _user(response)
        session = _session(response)
        if not user or not session:
            raise MembershipError("Email verification completed, but no session was returned. Please sign in.")
        enriched = _enrich_user(user)
        _track_device(enriched["id"], app_version, "signup")
        return {"user": enriched, "access_token": _attr(session, "access_token")}
    except MembershipError:
        raise
    except Exception as exc:
        message = str(getattr(exc, "message", "") or exc).lower()
        if "expired" in message or "invalid" in message or "already" in message:
            safe = "This confirmation link has expired or was already used. Request a new confirmation email."
        elif any(x in message for x in ("timeout", "connection", "network", "name resolution")):
            safe = "Cannot reach Supabase to finish email confirmation. Check your internet connection."
        else:
            safe = "Could not verify the email confirmation. Check the link and try again."
        raise MembershipError(safe) from exc


def register(
    email: str, display_name: str, password: str, app_version: str = APP_VERSION,
    email_redirect_to: str | None = None,
) -> dict:
    email = (email or "").strip().lower()
    display_name = (display_name or "").strip()
    if not display_name:
        raise MembershipError("Please enter a display name.")
    if len(display_name) > 80:
        raise MembershipError("Display name must be 80 characters or fewer.")
    if len(password or "") < 12:
        raise MembershipError("Password must be at least 12 characters long.")
    try:
        response = _supabase().auth.sign_up({
            "email": email,
            "password": password,
            "options": {
                "data": {"display_name": display_name},
                **({"email_redirect_to": email_redirect_to} if email_redirect_to else {}),
            },
        })
        user = _user(response)
        session = _session(response)
        if not user:
            raise MembershipError("Supabase did not return a user. Please try again.")
        if not session:
            # Supabase email confirmation is enabled. Do not open the workspace without a session.
            return {
                "user": _user_dict(user),
                "access_token": None,
                "needs_email_confirmation": True,
            }
        enriched = _enrich_user(user)
        _track_device(enriched["id"], app_version, "signup")
        return {
            "user": enriched,
            "access_token": _attr(session, "access_token"),
            "needs_email_confirmation": False,
        }
    except MembershipError:
        raise
    except Exception as exc:
        message = str(getattr(exc, "message", "") or exc)
        lowered = message.lower()
        if "already registered" in lowered or "already exists" in lowered:
            message = "An account with this email already exists."
        elif "password" in lowered and ("weak" in lowered or "short" in lowered):
            message = "The password does not meet the project's password requirements."
        elif "invalid email" in lowered:
            message = "Please enter a valid email address."
        elif any(x in lowered for x in ("timeout", "connection", "network", "name resolution")):
            message = "Cannot reach Supabase. Check your internet connection and try again."
        else:
            message = "Registration failed. Check your email/password and Supabase Auth settings."
        raise MembershipError(message) from exc


def authenticate(email: str, password: str, app_version: str = APP_VERSION) -> dict:
    email = (email or "").strip().lower()
    if not email or not password:
        raise MembershipError("Enter your email address and password.")
    try:
        response = _supabase().auth.sign_in_with_password({"email": email, "password": password})
        user = _user(response)
        session = _session(response)
        if not user or not session:
            raise MembershipError("Sign-in did not create a session. Confirm your email and try again.")
        enriched = _enrich_user(user)
        _track_device(enriched["id"], app_version, "login")
        return {"user": enriched, "access_token": _attr(session, "access_token")}
    except MembershipError:
        raise
    except Exception as exc:
        message = str(getattr(exc, "message", "") or exc).lower()
        if "email not confirmed" in message or "email_not_confirmed" in message:
            safe = "Please confirm your email address before signing in."
        elif "invalid login credentials" in message or "invalid credentials" in message:
            safe = "Email or password is incorrect."
        elif any(x in message for x in ("timeout", "connection", "network", "name resolution")):
            safe = "Cannot reach Supabase. Check your internet connection and try again."
        else:
            safe = "Sign-in failed. Check your credentials and Supabase Auth settings."
        raise MembershipError(safe) from exc


def current_session_user() -> dict | None:
    try:
        session = _supabase().auth.get_session()
        if not session:
            return None
        user = _attr(session, "user")
        return _enrich_user(user) if user else None
    except Exception:
        return None


def sign_out() -> None:
    client = _supabase()
    try:
        session = client.auth.get_session()
        user = _attr(session, "user") if session else None
        if user and _attr(user, "id"):
            _track_device(str(_attr(user, "id")), APP_VERSION, "logout")
        client.auth.sign_out()
    except Exception as exc:
        # The app should still exit if the remote service is temporarily unavailable.
        try:
            client.auth.sign_out()
        except Exception:
            pass


def get_profile() -> dict:
    user = current_session_user()
    if not user:
        raise MembershipError("Please sign in again.")
    return user


def update_profile(display_name: str) -> dict:
    name = (display_name or "").strip()
    if not name or len(name) > 80:
        raise MembershipError("Display name must be between 1 and 80 characters.")
    client = _supabase()
    session = client.auth.get_session()
    user = _attr(session, "user") if session else None
    user_id = str(_attr(user, "id", "")) if user else ""
    if not user_id:
        raise MembershipError("Please sign in again.")
    try:
        client.table("sfm_profiles").update({"display_name": name}).eq("id", user_id).execute()
        client.auth.update_user({"data": {"display_name": name}})
        return get_profile()
    except Exception as exc:
        raise MembershipError("Could not update the profile. Please try again.") from exc


def list_devices() -> list[dict]:
    client = _supabase()
    session = client.auth.get_session()
    user = _attr(session, "user") if session else None
    user_id = str(_attr(user, "id", "")) if user else ""
    if not user_id:
        raise MembershipError("Please sign in again.")
    try:
        response = (
            client.table("sfm_user_devices")
            .select("id,device_name,platform,app_version,created_at,last_seen_at,revoked_at")
            .eq("user_id", user_id)
            .order("last_seen_at", desc=True)
            .execute()
        )
        return response.data or []
    except Exception as exc:
        raise MembershipError("Could not load your devices. Please try again.") from exc


def revoke_device(device_id: str) -> None:
    client = _supabase()
    session = client.auth.get_session()
    user = _attr(session, "user") if session else None
    user_id = str(_attr(user, "id", "")) if user else ""
    if not user_id:
        raise MembershipError("Please sign in again.")
    try:
        client.table("sfm_user_devices").update({
            "revoked_at": datetime.now(timezone.utc).isoformat(),
        }).eq("id", device_id).eq("user_id", user_id).execute()
    except Exception as exc:
        raise MembershipError("Could not revoke this device. Please try again.") from exc
