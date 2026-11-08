import urllib.error
import urllib.parse
import urllib.request
import unittest
from types import SimpleNamespace
from unittest.mock import patch

import membership_client


class EmailConfirmationListenerTests(unittest.TestCase):
    def test_callback_accepts_one_time_hash_and_state(self):
        listener = membership_client.start_email_confirmation_listener()
        try:
            url = listener.redirect_url
            callback = url + "?" + urllib.parse.urlencode({
                "token_hash": "unit-test-one-time-hash",
                "type": "email",
            })
            with urllib.request.urlopen(callback, timeout=3) as response:
                body = response.read().decode("utf-8")
                self.assertEqual(response.status, 200)
                self.assertIn("Return to Smart File Manager Pro", body)
                self.assertNotIn("unit-test-one-time-hash", body)
            result = listener.take_result()
            self.assertEqual(result, {
                "token_hash": "unit-test-one-time-hash",
                "type": "email",
            })
            self.assertIsNone(listener.take_result())
        finally:
            listener.close()

    def test_callback_rejects_wrong_state_path(self):
        listener = membership_client.start_email_confirmation_listener()
        try:
            wrong_url = listener.redirect_url.rsplit("/", 1)[0] + "/wrong-state?token_hash=bad&type=email"
            with self.assertRaises(urllib.error.HTTPError) as raised:
                urllib.request.urlopen(wrong_url, timeout=3)
            self.assertEqual(raised.exception.code, 404)
            self.assertIsNone(listener.take_result())
        finally:
            listener.close()

    def test_email_confirmation_uses_email_and_one_time_hash(self):
        fake_user = SimpleNamespace(id="user-id", email="member@example.com", user_metadata={})
        fake_session = SimpleNamespace(access_token="test-access-token")

        class FakeAuth:
            def __init__(self):
                self.params = None

            def verify_otp(self, params):
                self.params = params
                return SimpleNamespace(user=fake_user, session=fake_session)

        fake_auth = FakeAuth()
        fake_client = SimpleNamespace(auth=fake_auth)
        with patch.object(membership_client, "_supabase", return_value=fake_client), \
             patch.object(membership_client, "_enrich_user", return_value={"id": "user-id"}), \
             patch.object(membership_client, "_track_device"):
            result = membership_client.complete_email_confirmation(
                {"token_hash": "one-time", "type": "email"}, "Member@Example.com"
            )

        self.assertEqual(fake_auth.params, {
            "email": "member@example.com",
            "token_hash": "one-time",
            "type": "email",
        })
        self.assertEqual(result["access_token"], "test-access-token")


if __name__ == "__main__":
    unittest.main()
