//! Network Tools page.

use crate::theme::ThemeColors;
use crate::widgets::{Card, StatCard};
use egui::Ui;
use smart_filee_localization::tr;
use smart_filee_network::{check_port, local_ip, ping_host, valid_host};
use std::time::Duration;

pub struct NetworkPage {
    pub local_ip_addr: String,
    pub ping_host_input: String,
    pub ping_result: String,
    pub port_host_input: String,
    pub port_number: u16,
    pub port_result: String,
}

impl Default for NetworkPage {
    fn default() -> Self {
        Self {
            local_ip_addr: local_ip(),
            ping_host_input: "google.com".to_string(),
            ping_result: String::new(),
            port_host_input: "google.com".to_string(),
            port_number: 443,
            port_result: String::new(),
        }
    }
}

impl NetworkPage {
    pub fn show(&mut self, ui: &mut Ui, colors: &ThemeColors) {
        ui.heading(tr("Network Tools"));
        ui.colored_label(
            colors.muted,
            "Inspect network configuration, test DNS and ping latency, and check TCP port reachability",
        );
        ui.add_space(12.0);

        // Local IP Card
        StatCard::new("Local IPv4 Address", &self.local_ip_addr)
            .subtitle("Primary active network interface")
            .show(ui, colors);

        ui.add_space(12.0);

        // Ping & DNS Tool
        Card::show(ui, colors, |ui| {
            ui.heading(tr("Ping & DNS Resolution"));
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label("Target Host:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.ping_host_input)
                        .hint_text("e.g. google.com or 8.8.8.8")
                        .desired_width(220.0),
                );

                if ui.button(tr("Ping Host")).clicked() {
                    let host = self.ping_host_input.trim();
                    if valid_host(host) {
                        self.ping_result = ping_host(host);
                    } else {
                        self.ping_result =
                            "Invalid host format. Use domain or IPv4 address.".to_string();
                    }
                }
            });

            if !self.ping_result.is_empty() {
                ui.add_space(6.0);
                ui.colored_label(colors.accent_btn, &self.ping_result);
            }
        });

        ui.add_space(12.0);

        // TCP Port Connectivity Check
        Card::show(ui, colors, |ui| {
            ui.heading("TCP Port Check");
            ui.add_space(6.0);

            ui.horizontal(|ui| {
                ui.label("Host:");
                ui.add(
                    egui::TextEdit::singleline(&mut self.port_host_input)
                        .hint_text("e.g. 127.0.0.1")
                        .desired_width(180.0),
                );

                ui.label("Port:");
                let mut port_str = self.port_number.to_string();
                if ui
                    .add(egui::TextEdit::singleline(&mut port_str).desired_width(60.0))
                    .changed()
                {
                    if let Ok(p) = port_str.parse::<u16>() {
                        self.port_number = p;
                    }
                }

                if ui.button("Check Port").clicked() {
                    let host = self.port_host_input.trim();
                    if valid_host(host) {
                        self.port_result =
                            check_port(host, self.port_number, Duration::from_secs(3));
                    } else {
                        self.port_result = "Invalid host name.".to_string();
                    }
                }
            });

            if !self.port_result.is_empty() {
                ui.add_space(6.0);
                ui.colored_label(colors.accent_btn, &self.port_result);
            }
        });
    }
}
