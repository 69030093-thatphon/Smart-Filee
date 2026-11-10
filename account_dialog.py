"""Account profile and device list dialog."""
from PySide6.QtCore import Qt
from PySide6.QtWidgets import (
    QDialog, QVBoxLayout, QHBoxLayout, QLabel, QLineEdit, QPushButton,
    QListWidget, QListWidgetItem, QMessageBox,
)
import membership_client
from i18n import tr


class AccountDialog(QDialog):
    def __init__(self, user=None, parent=None):
        super().__init__(parent)
        self.setWindowTitle(tr("Smart File Manager Pro — Account"))
        self.setMinimumSize(560, 520)
        self.user = user or {}
        self.setStyleSheet("""
            QDialog { background: #F7F5FC; }
            QLabel { color: #383246; }
            QLineEdit, QListWidget { background: white; border: 1px solid #E3DDEE; border-radius: 8px; padding: 8px; color: #29243A; }
            QPushButton { background: #7652C8; color: white; border: 0; border-radius: 8px; padding: 9px 13px; font-weight: 600; }
            QPushButton:hover { background: #6845B8; }
            QPushButton#secondary { background: #EEE8FA; color: #5C3BA5; }
            QPushButton#danger { background: #FBE9ED; color: #9A314A; }
        """)
        layout = QVBoxLayout(self)
        layout.setContentsMargins(24, 24, 24, 24)
        layout.setSpacing(10)

        title = QLabel(tr("Your account"))
        title.setStyleSheet("font-size: 20px; font-weight: 700;")
        layout.addWidget(title)
        self.email_label = QLabel(self.user.get("email", ""))
        self.email_label.setTextInteractionFlags(Qt.TextSelectableByMouse)
        layout.addWidget(self.email_label)
        layout.addWidget(QLabel(tr("Display name")))
        self.name_edit = QLineEdit(self.user.get("display_name", ""))
        self.name_edit.setMaxLength(80)
        self.name_edit.setPlaceholderText(tr("Display name"))
        layout.addWidget(self.name_edit)
        row = QHBoxLayout()
        self.save_button = QPushButton(tr("Save profile"))
        self.save_button.clicked.connect(self.save_profile)
        row.addWidget(self.save_button)
        self.refresh_button = QPushButton(tr("Refresh devices"))
        self.refresh_button.setObjectName("secondary")
        self.refresh_button.clicked.connect(self.load_devices)
        row.addWidget(self.refresh_button)
        layout.addLayout(row)

        devices_title = QLabel(tr("Devices"))
        devices_title.setStyleSheet("font-size: 15px; font-weight: 700; margin-top: 8px;")
        layout.addWidget(devices_title)
        self.devices = QListWidget()
        layout.addWidget(self.devices, 1)
        self.revoke_button = QPushButton(tr("Mark selected device as revoked"))
        self.revoke_button.setObjectName("danger")
        self.revoke_button.clicked.connect(self.revoke_selected)
        layout.addWidget(self.revoke_button)
        note = QLabel(tr("Note: marking a device as revoked updates the device record; it does not forcibly terminate a separate active Supabase session."))
        note.setWordWrap(True)
        note.setStyleSheet("color: #777187; font-size: 11px;")
        layout.addWidget(note)
        close = QPushButton(tr("Close"))
        close.setObjectName("secondary")
        close.clicked.connect(self.accept)
        layout.addWidget(close, alignment=Qt.AlignRight)
        self.load_devices()

    def save_profile(self):
        try:
            self.user = membership_client.update_profile(self.name_edit.text())
            self.name_edit.setText(self.user.get("display_name", ""))
            QMessageBox.information(self, tr("Account"), tr("Profile updated."))
        except membership_client.MembershipError as exc:
            QMessageBox.warning(self, tr("Account"), str(exc))

    def load_devices(self):
        self.devices.clear()
        try:
            rows = membership_client.list_devices()
            for row in rows:
                status = tr("Revoked") if row.get("revoked_at") else tr("Active")
                name = row.get("device_name") or tr("Unknown device")
                platform = row.get("platform") or tr("Unknown platform")
                version = row.get("app_version") or tr("Unknown version")
                seen = row.get("last_seen_at") or row.get("created_at") or ""
                item = QListWidgetItem(f"{name} — {platform} — v{version} — {status}\n{tr('Last seen:')} {seen}")
                item.setData(Qt.UserRole, row.get("id"))
                item.setData(Qt.UserRole + 1, bool(row.get("revoked_at")))
                self.devices.addItem(item)
        except membership_client.MembershipError as exc:
            QMessageBox.warning(self, tr("Devices"), str(exc))

    def revoke_selected(self):
        item = self.devices.currentItem()
        if not item:
            QMessageBox.information(self, tr("Devices"), tr("Select a device first."))
            return
        if item.data(Qt.UserRole + 1):
            QMessageBox.information(self, tr("Devices"), tr("This device is already marked as revoked."))
            return
        answer = QMessageBox.question(
            self, tr("Mark device revoked"),
            tr("Mark this device as revoked in the account record?\nThis does not terminate a separate active session."),
            QMessageBox.Yes | QMessageBox.No, QMessageBox.No,
        )
        if answer != QMessageBox.Yes:
            return
        try:
            membership_client.revoke_device(str(item.data(Qt.UserRole)))
            self.load_devices()
        except membership_client.MembershipError as exc:
            QMessageBox.warning(self, tr("Devices"), str(exc))
