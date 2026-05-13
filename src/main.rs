use anyhow::{anyhow, Context};
use eframe::egui::{self, Color32, RichText, Rounding, Stroke, Vec2};
use std::process::Command;

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(780.0, 980.0))
            .with_min_inner_size(Vec2::new(680.0, 900.0))
            .with_title("Stake"),
        ..Default::default()
    };

    eframe::run_native(
        "Stake",
        options,
        Box::new(|_cc| Ok(Box::<StakeApp>::default())),
    )
}

#[derive(Default)]
struct StakeApp {
    website: String,
    title: String,
    last_status: Option<String>,
    last_error: Option<String>,
}

impl eframe::App for StakeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        self.apply_theme(ctx);

        egui::CentralPanel::default().show(ctx, |ui| {
            egui::Frame::none()
                .fill(Color32::from_rgb(7, 7, 10))
                .stroke(Stroke::new(1.2, Color32::from_rgb(130, 22, 22)))
                .rounding(Rounding::same(20.0))
                .inner_margin(egui::Margin::same(26.0))
                .show(ui, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading(
                            RichText::new("STAKE")
                                .size(76.0)
                                .color(Color32::from_rgb(210, 27, 27)),
                        );
                        ui.label(
                            RichText::new("Forge web apps with Pake")
                                .size(22.0)
                                .color(Color32::from_rgb(162, 78, 78)),
                        );
                    });

                    ui.add_space(30.0);
                    labeled_input(ui, "Website", "https://example.com", &mut self.website);
                    ui.add_space(20.0);
                    labeled_input(ui, "Title", "app name", &mut self.title);

                    ui.add_space(32.0);
                    ui.vertical_centered(|ui| {
                        let button = egui::Button::new(
                            RichText::new("Create")
                                .size(42.0)
                                .color(Color32::from_rgb(255, 77, 77)),
                        )
                        .fill(Color32::from_rgb(34, 10, 10))
                        .stroke(Stroke::new(1.5, Color32::from_rgb(207, 26, 26)))
                        .min_size(Vec2::new(320.0, 88.0));

                        if ui.add(button).clicked() {
                            match run_pake(&self.website, &self.title) {
                                Ok(msg) => {
                                    self.last_status = Some(msg);
                                    self.last_error = None;
                                }
                                Err(err) => {
                                    self.last_error = Some(err.to_string());
                                    self.last_status = None;
                                }
                            }
                        }
                    });

                    if let Some(status) = &self.last_status {
                        ui.add_space(20.0);
                        ui.colored_label(Color32::from_rgb(90, 200, 90), status);
                    }

                    if let Some(error) = &self.last_error {
                        ui.add_space(12.0);
                        ui.colored_label(Color32::from_rgb(230, 80, 80), error);
                    }
                });
        });
    }
}

impl StakeApp {
    fn apply_theme(&self, ctx: &egui::Context) {
        let mut style = (*ctx.style()).clone();
        style.visuals.override_text_color = Some(Color32::from_rgb(235, 205, 205));
        style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(12, 12, 16);
        style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(18, 20, 28);
        style.visuals.widgets.active.bg_fill = Color32::from_rgb(32, 14, 14);
        style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(38, 16, 16);
        style.visuals.extreme_bg_color = Color32::from_rgb(7, 8, 10);
        style.visuals.window_fill = Color32::from_rgb(4, 5, 8);
        style.visuals.faint_bg_color = Color32::from_rgb(16, 16, 20);
        ctx.set_style(style);
    }
}

fn labeled_input(ui: &mut egui::Ui, label: &str, hint: &str, value: &mut String) {
    ui.label(
        RichText::new(label)
            .size(40.0)
            .color(Color32::from_rgb(230, 66, 66)),
    );

    ui.add_space(10.0);
    ui.add(
        egui::TextEdit::singleline(value)
            .hint_text(hint)
            .desired_width(f32::INFINITY)
            .margin(egui::Vec2::new(16.0, 20.0)),
    );
}

fn run_pake(url: &str, app_name: &str) -> anyhow::Result<String> {
    let normalized_name = app_name.trim();
    let normalized_url = url.trim();

    if normalized_name.is_empty() {
        return Err(anyhow!("Title is required."));
    }

    url::Url::parse(normalized_url).context("Please enter a valid URL including protocol.")?;

    let pake = which::which("pake").context("Could not find `pake` binary in PATH.")?;

    let output = Command::new(pake)
        .arg(normalized_url)
        .arg("--name")
        .arg(normalized_name)
        .output()
        .context("Failed to execute pake.")?;

    if !output.status.success() {
        return Err(anyhow!(
            "pake failed: {}",
            String::from_utf8_lossy(&output.stderr)
        ));
    }

    Ok(format!(
        "Created app `{}` for {} via Pake.",
        normalized_name, normalized_url
    ))
}
