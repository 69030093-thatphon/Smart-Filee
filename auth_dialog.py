"""PySide6 sign-in and registration dialog for online membership."""
from PySide6.QtCore import Qt, QTimer
from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QHBoxLayout, QLabel, QLineEdit, QPushButton,
    QStackedWidget, QMessageBox, QFrame,
)
import membership_client as membership_db
from i18n import tr


class AuthDialog(QDialog):
    def __init__(self, app_version="unknown", parent=None):
        super().__init__(parent)
        self.app_version = app_version
        self.user = None
        self.access_token = None
        self._confirmation_listener = None
        self._confirmation_timer = QTimer(self)
        self._confirmation_timer.setInterval(500)
        self._confirmation_timer.timeout.connect(self._poll_email_confirmation)
        self.setWindowTitle(tr("Smart File Manager Pro — Sign in"))
        self.setModal(True)
        self.setFixedWidth(430)
        self.setMinimumHeight(470)
        self.setObjectName("authDialog")

        outer = QVBoxLayout(self)
        outer.setContentsMargins(30, 28, 30, 28)
        outer.setSpacing(12)
        logo = QLabel("SF")
        logo.setObjectName("authLogo")
        logo.setAlignment(Qt.AlignCenter)
        logo.setFixedSize(54, 54)
        outer.addWidget(logo, alignment=Qt.AlignHCenter)
        title = QLabel("Smart File Manager Pro")
        title.setObjectName("authTitle")
        title.setAlignment(Qt.AlignCenter)
        outer.addWidget(title)
        self.subtitle = QLabel(tr("Sign in to your account"))
        self.subtitle.setObjectName("authSubtitle")
        self.subtitle.setAlignment(Qt.AlignCenter)
        outer.addWidget(self.subtitle)

        self.pages = QStackedWidget()
        outer.addWidget(self.pages, 1)
        self.login_page = self._make_login()
        self.register_page = self._make_register()
        self.pages.addWidget(self.login_page)
        self.pages.addWidget(self.register_page)
        self.setStyleSheet("""
            QDialog#authDialog { background: #F7F5FC; }
            QLabel#authLogo { background: #7652C8; color: white; border-radius: 16px; font-size: 20px; font-weight: 700; }
            QLabel#authTitle { color: #29243A; font-size: 21px; font-weight: 700; }
            QLabel#authSubtitle { color: #777187; font-size: 12px; }
            QLabel { color: #383246; }
            QLineEdit { background: white; border: 1px solid #E3DDEE; border-radius: 9px; padding: 11px 12px; min-height: 20px; color: #29243A; }
            QLineEdit:focus { border: 1px solid #8B68D5; }
            QPushButton { background: #7652C8; color: white; border: 0; border-radius: 9px; padding: 11px 14px; font-weight: 600; }
            QPushButton:hover { background: #6845B8; }
            QPushButton#linkButton { background: transparent; color: #7652C8; padding: 7px; }
            QPushButton#linkButton:hover { background: #EEE8FA; }
        """)

    def _field(self, placeholder, password=False):
        edit = QLineEdit()
        edit.setPlaceholderText(placeholder)
        if password:
            edit.setEchoMode(QLineEdit.Password)
        return edit

    def _make_login(self):
        page = QFrame()
        layout = QVBoxLayout(page)
        layout.setContentsMargins(0, 12, 0, 0)
        layout.setSpacing(10)
        self.login_email = self._field(tr("Email address"))
        self.login_password = self._field(tr("Password"), True)
        layout.addWidget(self.login_email)
        layout.addWidget(self.login_password)
        button = QPushButton(tr("Sign in"))
        button.clicked.connect(self._login)
        layout.addWidget(button)
        switch = QPushButton(tr("Create an account"))
        switch.setObjectName("linkButton")
        switch.clicked.connect(self.show_register)
        layout.addWidget(switch)
        layout.addStretch(1)
        return page

    def _make_register(self):
        page = QFrame()
        layout = QVBoxLayout(page)
        layout.setContentsMargins(0, 12, 0, 0)
        layout.setSpacing(9)
        self.register_name = self._field(tr("Display name"))
        self.register_email = self._field(tr("Email address"))
        self.register_password = self._field(tr("Password (at least 12 characters)"), True)
        self.register_confirm = self._field(tr("Confirm password"), True)
        for field in (self.register_name, self.register_email, self.register_password, self.register_confirm):
            layout.addWidget(field)
        button = QPushButton(tr("Create account"))
        button.clicked.connect(self._register)
        layout.addWidget(button)
        switch = QPushButton(tr("Back to sign in"))
        switch.setObjectName("linkButton")
        switch.clicked.connect(self.show_login)
        layout.addWidget(switch)
        layout.addStretch(1)
        return page

    def show_register(self):
        self.pages.setCurrentWidget(self.register_page)
        self.subtitle.setText(tr("Create your free account"))

    def show_login(self):
        self.pages.setCurrentWidget(self.login_page)
        self.subtitle.setText(tr("Sign in to your account"))

    def _login(self):
        try:
            result = membership_db.authenticate(self.login_email.text(), self.login_password.text(), self.app_version)
            self.user = result.get("user")
            self.access_token = result.get("access_token")
            self.accept()
        except membership_db.MembershipError as exc:
            QMessageBox.warning(self, tr("Sign in"), str(exc))

    def _register(self):
        if self.register_password.text() != self.register_confirm.text():
            QMessageBox.warning(self, tr("Create account"), tr("Passwords do not match."))
            return

        listener = None
        try:
            # Start the loopback listener before signup so the callback is ready
            # before the user opens the confirmation email.
            listener = membership_db.start_email_confirmation_listener()
            result = membership_db.register(
                self.register_email.text(), self.register_name.text(),
                self.register_password.text(), self.app_version,
                email_redirect_to=listener.redirect_url,
            )
            self.user = result.get("user")
            self.access_token = result.get("access_token")
            if result.get("needs_email_confirmation"):
                self._confirmation_listener = listener
                listener = None  # ownership transferred to the dialog
                self.show_login()
                self.login_email.setText(self.register_email.text().strip())
                self.login_password.clear()
                self.subtitle.setText("Waiting for email confirmation…")
                QMessageBox.information(
                    self, tr("Confirm your email"),
                    "Your account was created. Open the confirmation email on this same computer and click the link. "
                    "Smart File Manager Pro will verify it and sign you in automatically.",
                )
                self._confirmation_timer.start()
                return

            QMessageBox.information(self, tr("Account created"), tr("Your account has been created successfully."))
            self.accept()
        except membership_db.MembershipError as exc:
            QMessageBox.warning(self, tr("Create account"), str(exc))
        finally:
            if listener is not None:
                listener.close()

    def _poll_email_confirmation(self):
        listener = self._confirmation_listener
        if listener is None:
            self._confirmation_timer.stop()
            return
        callback_result = listener.take_result()
        if callback_result is None:
            return

        self._confirmation_timer.stop()
        try:
            if callback_result.get("error"):
                raise membership_db.MembershipError(
                    "Email confirmation failed: " + callback_result["error"]
                )
            result = membership_db.complete_email_confirmation(
                callback_result, self.login_email.text(), self.app_version
            )
            self.user = result.get("user")
            self.access_token = result.get("access_token")
            QMessageBox.information(
                self, tr("Email confirmed"),
                "Your email has been confirmed. You are now signed in to Smart File Manager Pro.",
            )
            self.accept()
        except membership_db.MembershipError as exc:
            QMessageBox.warning(self, tr("Email confirmation"), str(exc))
            self.subtitle.setText(tr("Sign in to your account"))
        finally:
            self._stop_confirmation_listener()

    def _stop_confirmation_listener(self):
        self._confirmation_timer.stop()
        listener = self._confirmation_listener
        self._confirmation_listener = None
        if listener is not None:
            listener.close()

    def accept(self):
        self._stop_confirmation_listener()
        super().accept()

    def reject(self):
        self._stop_confirmation_listener()
        super().reject()
