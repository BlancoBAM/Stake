use anyhow::{anyhow, Context};
use eframe::egui::{
    self, Color32, ColorImage, FontData, FontDefinitions, FontFamily, Rect, RichText, Rounding,
    Stroke, TextureHandle, TextureOptions, Vec2,
};
use std::process::Command;

// ── embed assets at compile time ─────────────────────────────────────────────
// Hero image is embedded in the binary so it works without an install prefix.
const HERO_BYTES: &[u8] = include_bytes!("../assets/stake-hero.png");

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(Vec2::new(420.0, 680.0))
            .with_min_inner_size(Vec2::new(360.0, 560.0))
            .with_title("Stake")
            .with_resizable(true),
        ..Default::default()
    };

    eframe::run_native(
        "Stake",
        options,
        Box::new(|cc| Ok(Box::new(StakeApp::new(cc)))),
    )
}

// ── app state ────────────────────────────────────────────────────────────────
struct StakeApp {
    website: String,
    title: String,
    last_status: Option<String>,
    last_error: Option<String>,
    hero_texture: Option<TextureHandle>,
    // animation state
    button_hover_t: f32,
}

impl StakeApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        setup_fonts(&cc.egui_ctx);

        // Decode hero PNG and upload to GPU
        let hero_texture = load_png_texture(&cc.egui_ctx, HERO_BYTES, "stake_hero");

        Self {
            website: String::new(),
            title: String::new(),
            last_status: None,
            last_error: None,
            hero_texture,
            button_hover_t: 0.0,
        }
    }
}

// ── rendering ────────────────────────────────────────────────────────────────
impl eframe::App for StakeApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        apply_theme(ctx);

        // Black window background
        egui::CentralPanel::default()
            .frame(egui::Frame::none().fill(Color32::BLACK))
            .show(ctx, |ui| {
                // Centre the card
                let avail = ui.available_size();
                let card_w = avail.x.min(440.0);

                ui.horizontal(|ui| {
                    // left spacer
                    ui.add_space((avail.x - card_w) / 2.0);

                    ui.vertical(|ui| {
                        ui.set_width(card_w);
                        // top spacer
                        ui.add_space(((avail.y - 600.0) / 2.0).max(8.0));

                        self.draw_card(ui, ctx);
                    });
                });
            });
    }
}

impl StakeApp {
    fn draw_card(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
        // ── outer glow (painted manually behind the card) ──────────────────
        let card_rect_probe = ui.available_rect_before_wrap();
        let glow_inflate = 18.0_f32;

        // ── card frame with red border glow ───────────────────────────────
        let card_frame = egui::Frame::none()
            .fill(Color32::BLACK)
            .stroke(Stroke::new(2.0, Color32::from_rgb(220, 38, 38)))
            .rounding(Rounding::same(8.0))
            .inner_margin(egui::Margin::same(0.0));

        let inner = card_frame.show(ui, |ui| {
            ui.set_width(ui.available_width());

            // ── hero image ────────────────────────────────────────────────
            if let Some(tex) = &self.hero_texture {
                let img_w = ui.available_width();
                let aspect = tex.size()[1] as f32 / tex.size()[0] as f32;
                let img_h = img_w * aspect;

                let (rect, _) = ui.allocate_exact_size(
                    Vec2::new(img_w, img_h),
                    egui::Sense::hover(),
                );

                ui.painter().image(
                    tex.id(),
                    rect,
                    Rect::from_min_max(egui::pos2(0.0, 0.0), egui::pos2(1.0, 1.0)),
                    Color32::WHITE,
                );

                // gradient overlay: transparent → black from top to bottom
                let painter = ui.painter();
                let steps = 16_u8;
                let step_h = img_h / steps as f32;
                for i in 0..steps {
                    let t = i as f32 / steps as f32;
                    let alpha = (t * t * 200.0) as u8; // quadratic fade
                    let y0 = rect.top() + i as f32 * step_h;
                    let strip = Rect::from_min_size(
                        egui::pos2(rect.left(), y0),
                        Vec2::new(img_w, step_h + 1.0),
                    );
                    painter.rect_filled(
                        strip,
                        Rounding::ZERO,
                        Color32::from_black_alpha(alpha),
                    );
                }
            } else {
                // Fallback: solid dark header
                let (rect, _) =
                    ui.allocate_exact_size(Vec2::new(ui.available_width(), 220.0), egui::Sense::hover());
                ui.painter()
                    .rect_filled(rect, Rounding::ZERO, Color32::from_rgb(10, 5, 5));
                ui.painter().text(
                    rect.center(),
                    egui::Align2::CENTER_CENTER,
                    "STAKE",
                    egui::FontId::new(64.0, FontFamily::Proportional),
                    Color32::from_rgb(220, 38, 38),
                );
            }

            // ── form section ──────────────────────────────────────────────
            let form_frame = egui::Frame::none()
                .fill(Color32::BLACK)
                .inner_margin(egui::Margin::symmetric(20.0, 16.0));

            form_frame.show(ui, |ui| {
                // blood drip — top-left
                let drip_rect = Rect::from_min_size(
                    ui.min_rect().left_top() + egui::vec2(12.0, -2.0),
                    Vec2::new(3.0, 20.0),
                );
                paint_blood_drip(ui.painter(), drip_rect, 0.7);

                // blood drip — top-right
                let drip_rect2 = Rect::from_min_size(
                    ui.min_rect().right_top() + egui::vec2(-28.0, -2.0),
                    Vec2::new(2.0, 14.0),
                );
                paint_blood_drip(ui.painter(), drip_rect2, 0.5);

                ui.set_width(ui.available_width());
                ui.spacing_mut().item_spacing.y = 4.0;

                // Website label
                ui.label(
                    RichText::new("Website")
                        .size(12.0)
                        .color(Color32::from_rgb(220, 38, 38))
                        .strong(),
                );
                ui.add_space(4.0);
                styled_input(ui, "url", &mut self.website);

                ui.add_space(12.0);

                // Title label
                ui.label(
                    RichText::new("Title")
                        .size(12.0)
                        .color(Color32::from_rgb(220, 38, 38))
                        .strong(),
                );
                ui.add_space(4.0);
                styled_input(ui, "app name", &mut self.title);

                ui.add_space(20.0);

                // ── Create button ─────────────────────────────────────────
                ui.vertical_centered(|ui| {
                    let btn_size = Vec2::new(ui.available_width(), 46.0);
                    let (rect, resp) =
                        ui.allocate_exact_size(btn_size, egui::Sense::click());

                    let is_hovered = resp.hovered();

                    // animate hover
                    let target = if is_hovered { 1.0_f32 } else { 0.0_f32 };
                    self.button_hover_t += (target - self.button_hover_t) * 0.18;
                    if (self.button_hover_t - target).abs() > 0.001 {
                        ctx.request_repaint();
                    }

                    let t = self.button_hover_t;

                    // Background fill: dark red tinted, brighter on hover
                    let bg = Color32::from_rgba_unmultiplied(
                        (30.0 + t * 20.0) as u8,
                        0,
                        0,
                        255,
                    );
                    ui.painter().rect_filled(rect, Rounding::same(6.0), bg);

                    // Border
                    ui.painter().rect_stroke(
                        rect,
                        Rounding::same(6.0),
                        Stroke::new(2.0, Color32::from_rgb(220, 38, 38)),
                    );

                    // Sweep glow overlay on hover
                    if t > 0.01 {
                        let glow_col = Color32::from_rgba_unmultiplied(
                            220,
                            38,
                            38,
                            (t * 40.0) as u8,
                        );
                        ui.painter()
                            .rect_filled(rect, Rounding::same(6.0), glow_col);
                    }

                    // Button text — use Creepster if loaded, else proportional
                    let text_col = Color32::from_rgb(
                        (220_f32 + t * 30.0) as u8,
                        (38_f32 + t * 30.0) as u8,
                        (38_f32 + t * 30.0) as u8,
                    );
                    ui.painter().text(
                        rect.center(),
                        egui::Align2::CENTER_CENTER,
                        "Create",
                        egui::FontId::new(22.0, FontFamily::Name("gothic".into())),
                        text_col,
                    );

                    if resp.clicked() {
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

                ui.add_space(8.0);

                // Status / error messages
                if let Some(status) = &self.last_status {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(status)
                            .size(12.0)
                            .color(Color32::from_rgb(90, 200, 90)),
                    );
                }
                if let Some(error) = &self.last_error {
                    ui.add_space(8.0);
                    ui.label(
                        RichText::new(error)
                            .size(12.0)
                            .color(Color32::from_rgb(230, 80, 80)),
                    );
                }

                // blood drip — bottom-left
                let br = ui.min_rect().left_bottom();
                let d3 = Rect::from_min_size(br + egui::vec2(36.0, -12.0), Vec2::new(2.0, 12.0));
                paint_blood_drip_bottom(ui.painter(), d3, 0.6);

                // blood drip — bottom-right
                let d4 = Rect::from_min_size(br + egui::vec2(ui.available_width() - 52.0, -16.0), Vec2::new(2.0, 16.0));
                paint_blood_drip_bottom(ui.painter(), d4, 0.5);
            });
        });

        // Paint the outer ambient glow around the card border
        let card_rect = inner.response.rect;
        paint_outer_glow(ui.painter(), card_rect, glow_inflate);
        let _ = card_rect_probe; // suppress unused warning
    }
}

// ── helpers ───────────────────────────────────────────────────────────────────

fn styled_input(ui: &mut egui::Ui, hint: &str, value: &mut String) {
    let input = egui::TextEdit::singleline(value)
        .hint_text(
            RichText::new(hint).color(Color32::from_rgb(90, 90, 90)),
        )
        .desired_width(f32::INFINITY)
        .margin(egui::vec2(10.0, 8.0))
        .font(egui::FontId::new(13.0, FontFamily::Proportional));

    let resp = ui.add(input);

    // Custom border: dim normally, red-glow when focused
    let stroke_col = if resp.has_focus() {
        Color32::from_rgb(220, 38, 38)
    } else {
        Color32::from_rgb(55, 55, 55)
    };
    ui.painter().rect_stroke(
        resp.rect,
        Rounding::same(4.0),
        Stroke::new(1.5, stroke_col),
    );
}

fn paint_blood_drip(painter: &egui::Painter, rect: Rect, opacity: f32) {
    let alpha = (opacity * 180.0) as u8;
    let top_col = Color32::from_rgba_unmultiplied(160, 0, 0, alpha);
    let bot_col = Color32::from_rgba_unmultiplied(160, 0, 0, 0);
    // simple vertical gradient via two rects
    painter.rect_filled(
        Rect::from_min_size(rect.min, Vec2::new(rect.width(), rect.height() * 0.6)),
        Rounding::ZERO,
        top_col,
    );
    painter.rect_filled(
        Rect::from_min_size(
            rect.min + egui::vec2(0.0, rect.height() * 0.4),
            Vec2::new(rect.width(), rect.height() * 0.6),
        ),
        Rounding::ZERO,
        bot_col,
    );
}

fn paint_blood_drip_bottom(painter: &egui::Painter, rect: Rect, opacity: f32) {
    let alpha = (opacity * 100.0) as u8;
    let top_col = Color32::from_rgba_unmultiplied(120, 0, 0, 0);
    let bot_col = Color32::from_rgba_unmultiplied(120, 0, 0, alpha);
    painter.rect_filled(
        Rect::from_min_size(rect.min, Vec2::new(rect.width(), rect.height() * 0.5)),
        Rounding::ZERO,
        top_col,
    );
    painter.rect_filled(
        Rect::from_min_size(
            rect.min + egui::vec2(0.0, rect.height() * 0.5),
            Vec2::new(rect.width(), rect.height() * 0.5),
        ),
        Rounding::ZERO,
        bot_col,
    );
}

fn paint_outer_glow(painter: &egui::Painter, card: Rect, inflate: f32) {
    // multi-pass expanding rect with decreasing alpha simulates glow
    for i in 1..=6_u8 {
        let expand = inflate * (i as f32 / 6.0);
        let alpha = (60 / i).max(4);
        let r = card.expand(expand);
        painter.rect_stroke(
            r,
            Rounding::same(8.0 + expand * 0.5),
            Stroke::new(1.0, Color32::from_rgba_unmultiplied(220, 38, 38, alpha)),
        );
    }
}

// ── font setup ────────────────────────────────────────────────────────────────

fn setup_fonts(ctx: &egui::Context) {
    let mut fonts = FontDefinitions::default();

    // Try to load Creepster from disk at runtime (shipped next to the binary
    // in a real install, or just use the system font path).
    // We try several candidate paths so both local dev and installed builds work.
    let font_candidates = [
        "/usr/share/fonts/truetype/stake/Creepster-Regular.ttf",
        "/usr/share/fonts/stake/Creepster-Regular.ttf",
        "assets/Creepster-Regular.ttf",
    ];

    let mut loaded = false;
    for path in &font_candidates {
        if let Ok(bytes) = std::fs::read(path) {
            fonts.font_data.insert(
                "gothic".to_owned(),
                FontData::from_owned(bytes),
            );
            fonts
                .families
                .entry(FontFamily::Name("gothic".into()))
                .or_default()
                .push("gothic".to_owned());
            loaded = true;
            break;
        }
    }

    // Fallback: alias "gothic" → default proportional
    if !loaded {
        fonts
            .families
            .entry(FontFamily::Name("gothic".into()))
            .or_insert_with(|| vec!["Ubuntu-R".to_owned(), "NotoEmoji-Regular".to_owned()]);
    }

    ctx.set_fonts(fonts);
}

// ── theme ─────────────────────────────────────────────────────────────────────

fn apply_theme(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.visuals.override_text_color = Some(Color32::from_rgb(160, 160, 160));
    style.visuals.widgets.noninteractive.bg_fill = Color32::from_rgb(10, 10, 10);
    style.visuals.widgets.inactive.bg_fill = Color32::from_rgb(20, 20, 20);
    style.visuals.widgets.active.bg_fill = Color32::from_rgb(28, 10, 10);
    style.visuals.widgets.hovered.bg_fill = Color32::from_rgb(35, 14, 14);
    style.visuals.extreme_bg_color = Color32::from_rgb(18, 18, 18);
    style.visuals.window_fill = Color32::BLACK;
    style.visuals.faint_bg_color = Color32::from_rgb(12, 12, 12);
    // Kill egui's built-in widget stroke so our custom ones take over
    style.visuals.widgets.inactive.bg_stroke = Stroke::NONE;
    style.visuals.widgets.hovered.bg_stroke = Stroke::NONE;
    style.visuals.widgets.active.bg_stroke = Stroke::NONE;
    style.visuals.selection.bg_fill = Color32::from_rgba_unmultiplied(220, 38, 38, 80);
    ctx.set_style(style);
}

// ── texture loader ────────────────────────────────────────────────────────────

fn load_png_texture(ctx: &egui::Context, bytes: &[u8], name: &str) -> Option<TextureHandle> {
    let img = image::load_from_memory(bytes).ok()?.to_rgba8();
    let (w, h) = img.dimensions();
    let pixels: Vec<egui::Color32> = img
        .pixels()
        .map(|p| Color32::from_rgba_unmultiplied(p[0], p[1], p[2], p[3]))
        .collect();
    let color_image = ColorImage {
        size: [w as usize, h as usize],
        pixels,
    };
    Some(ctx.load_texture(name, color_image, TextureOptions::LINEAR))
}

// ── pake runner ───────────────────────────────────────────────────────────────

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
        "✓ Created app `{}` for {}",
        normalized_name, normalized_url
    ))
}
