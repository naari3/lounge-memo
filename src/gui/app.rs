use eframe::{
    egui::{
        self, CentralPanel, ComboBox, FontData, FontDefinitions, Grid, Key, Layout, ScrollArea,
    },
    emath::Align,
    epaint::{ColorImage, FontFamily},
    CreationContext, Frame,
};
use egui_extras::{Column, RetainedImage, TableBuilder};
use image::imageops::FilterType;
use std::{
    sync::{Arc, Mutex},
    time::{Duration, Instant},
};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    capture::{open_directshow_device, open_msmf_device},
    capture_raw::{
        capture_with_escapi, capture_with_opencv, get_directshow_device_name_map,
        get_msmf_device_name_map,
    },
    courses::{Course, COURSES, STRING_COURSE_MAP},
    mogi_result::MogiResult,
    race_result::Position,
    settings::Settings,
};

use super::course_dropdown::DropDownBox;

const PPP: f32 = 1.25;

const LOG_LEVELS: [&str; 6] = ["OFF", "ERROR", "WARN", "INFO", "DEBUG", "TRACE"];

#[derive(Debug, Clone)]
pub enum Event {
    EditMogiResult(MogiResult),
}

#[derive(Debug, Clone)]
enum OpenedIndex {
    Result(usize),
    Current,
}

#[derive(Debug, Clone)]
struct OpenedRace {
    index: OpenedIndex,
    buffer: String,
    position: Option<Position>,
    position_input: String,
}

impl OpenedRace {
    fn new(index: OpenedIndex, course: Option<Course>, position: Option<Position>) -> Self {
        Self {
            index,
            buffer: course.map(|c| c.to_string()).unwrap_or("".to_string()),
            position,
            position_input: position.map(|p| p.to_string()).unwrap_or("".to_string()),
        }
    }
}

#[derive(Debug, Clone)]
struct BufSettings {
    device_name: String,
    directshow: bool,
    log_level: String,
    write_log_to_file: bool,
}

// Settings と BufSettingts は相互に変換できるようにする
impl From<Settings> for BufSettings {
    fn from(settings: Settings) -> Self {
        Self {
            device_name: settings.device_name().to_string(),
            directshow: settings.directshow(),
            log_level: settings.log_level().to_string(),
            write_log_to_file: settings.write_log_to_file(),
        }
    }
}

impl From<BufSettings> for Settings {
    fn from(buf_settings: BufSettings) -> Self {
        Self::new(
            buf_settings.device_name,
            buf_settings.directshow,
            buf_settings.log_level,
            buf_settings.write_log_to_file,
        )
    }
}

pub struct App {
    // Sender/Receiver for async notifications.
    tx: Arc<Mutex<Sender<Event>>>,
    rx: Arc<Mutex<Receiver<MogiResult>>>,

    settings_tx: Arc<Mutex<Sender<Settings>>>,
    on_settings: bool,
    msmf_device_names: Vec<String>,
    directshow_device_names: Vec<String>,
    buf_settings: BufSettings,

    courses: Vec<Course>,
    mogi_result: MogiResult,
    draft_mogi_result: Option<MogiResult>,
    opened_race: Option<OpenedRace>,

    capture_preview: Option<RetainedImage>,
    last_preview_updated: Instant,
}

impl App {
    pub fn new(
        ctx: &CreationContext,
        tx: Arc<Mutex<Sender<Event>>>,
        rx: Arc<Mutex<Receiver<MogiResult>>>,
        settings_tx: Arc<Mutex<Sender<Settings>>>,
        default_settings: Settings,
    ) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "notosansjp".to_owned(),
            FontData::from_static(include_bytes!("../assets/NotoSansJP-VariableFont_wght.ttf")),
        );
        fonts
            .families
            .get_mut(&FontFamily::Proportional)
            .unwrap()
            .insert(0, "notosansjp".to_owned());
        ctx.egui_ctx.set_fonts(fonts);
        ctx.egui_ctx.set_pixels_per_point(PPP);
        Self {
            tx,
            rx,
            settings_tx,
            on_settings: false,
            msmf_device_names: get_msmf_device_name_map()
                .unwrap()
                .values()
                .cloned()
                .collect(),
            directshow_device_names: get_directshow_device_name_map()
                .unwrap()
                .values()
                .cloned()
                .collect(),
            buf_settings: default_settings.into(),
            courses: COURSES.try_lock().unwrap().clone(),
            mogi_result: MogiResult::new(),
            draft_mogi_result: None,
            opened_race: None,
            capture_preview: None,
            last_preview_updated: Instant::now(),
        }
    }

    fn save_settings(&mut self) {
        let settings: Settings = self.buf_settings.clone().into();
        self.settings_tx.lock().unwrap().try_send(settings).unwrap();
    }

    fn refresh_capture_preview(&mut self, width: f32) {
        if self.buf_settings.device_name.is_empty() {
            return;
        }
        if self.last_preview_updated.elapsed() > Duration::from_millis(1000 / 30) {
            self.last_preview_updated = Instant::now();
            let device_name = self.buf_settings.device_name.clone();
            let img = if self.buf_settings.directshow {
                let device = match open_directshow_device(&device_name) {
                    Ok(device) => device,
                    Err(err) => {
                        log::error!("failed to open directshow device: {}", err);
                        return;
                    }
                };
                let mut device = device.lock().unwrap();
                capture_with_opencv(&mut device).ok()
            } else {
                let device = match open_msmf_device(&device_name) {
                    Ok(device) => device,
                    Err(err) => {
                        log::error!("failed to open msmf device: {}", err);
                        return;
                    }
                };
                let device = device.lock().unwrap();
                capture_with_escapi(&device).ok()
            };

            if let Some(img) = img {
                let img = image::imageops::resize(
                    &img,
                    width as _,
                    (width / 16.0 * 9.0) as _,
                    FilterType::Nearest,
                );
                let width = img.width();
                let height = img.height();
                self.capture_preview = Some(RetainedImage::from_color_image(
                    "aaa",
                    ColorImage::from_rgb([width as _, height as _], img.as_raw()),
                ))
            };
        }
    }
}

fn course_dropdown(ui: &mut egui::Ui, courses: &[Course], buffer: &mut String) {
    ui.group(|ui| {
        ui.add(DropDownBox::from_iter(
            courses.iter().map(|f| f.to_string()),
            "Course",
            buffer,
            |ui, text| ui.selectable_label(false, text),
        ));
    });
}

// TODO: 複雑すぎる どうにかしろ
fn edit_view(
    draft_mogi_result: &mut MogiResult,
    opened_race: &mut Option<OpenedRace>,
    courses: &[Course],
    ui: &mut egui::Ui,
) {
    ui.strong("編集モード");

    let previous_buffer = opened_race
        .as_ref()
        .map(|or| or.buffer.clone())
        .unwrap_or_else(|| "".to_string());
    let mut save_editing_course = false;

    // TODO: スクロールさせたいが、なぜかできない
    ScrollArea::horizontal().max_height(280.0).show(ui, |ui| {
        Grid::new("edit_view").show(ui, |ui| {
            ui.with_layout(Layout::top_down(Align::Min), |ui| {
                draft_mogi_result
                    .iter_races()
                    .enumerate()
                    .for_each(|(index, race)| {
                        ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                            ui.label(format!("{:02}: ", index + 1));
                            if ui.button(race.course_name()).clicked() {
                                *opened_race = Some(OpenedRace::new(
                                    OpenedIndex::Result(index),
                                    race.course(),
                                    Some(race.position()),
                                ));
                            };
                            if ui.button(race.position().to_string()).clicked() {
                                *opened_race = Some(OpenedRace::new(
                                    OpenedIndex::Result(index),
                                    race.course(),
                                    Some(race.position()),
                                ));
                            };
                        });
                        if let Some(OpenedRace {
                            index: OpenedIndex::Result(r_index),
                            buffer,
                            position_input,
                            ..
                        }) = opened_race.as_mut()
                        {
                            if *r_index == index {
                                ui.group(|ui| {
                                    course_dropdown(ui, courses, buffer);
                                    let resp = ui.text_edit_singleline(position_input);
                                    save_editing_course = ui.button("Save").clicked()
                                        || resp.ctx.input(|i| i.key_pressed(Key::Enter));
                                });
                            }
                        }
                    });
            });
        });
    });
    ui.label("現在のコース: ");
    let current_course = draft_mogi_result.current_course().clone();
    let label = current_course
        .as_ref()
        .map_or("(Empty)".to_string(), |course| course.to_string());
    if ui.button(label).clicked() {
        opened_race.replace(OpenedRace::new(OpenedIndex::Current, current_course, None));
    }

    let total_score = draft_mogi_result.total_score();
    ui.label(format!("合計得点: {total_score}"));

    if let Some(OpenedRace {
        index: OpenedIndex::Current,
        buffer,
        position_input,
        ..
    }) = opened_race.as_mut()
    {
        ui.group(|ui| {
            course_dropdown(ui, courses, buffer);
            let resp = ui.text_edit_singleline(position_input);
            save_editing_course =
                ui.button("Save").clicked() || resp.ctx.input(|i| i.key_pressed(Key::Enter));
        });
    }
    if let Some(OpenedRace {
        index,
        buffer,
        position_input,
        position,
    }) = opened_race.as_mut()
    {
        if previous_buffer != *buffer {
            let binding = STRING_COURSE_MAP.lock().unwrap();
            if let Some(course) = binding.get(buffer) {
                match index {
                    OpenedIndex::Result(idx) => draft_mogi_result.set_course(*idx, course.clone()),
                    OpenedIndex::Current => draft_mogi_result.set_current_course(course.clone()),
                }
            }
        }
        if save_editing_course {
            let new_position = position_input
                .parse::<usize>()
                .map(|n| Position::from_index(n - 1))
                .ok()
                .flatten();
            match index {
                OpenedIndex::Result(idx) => {
                    if let Some(new_position) = new_position {
                        draft_mogi_result.set_position(*idx, new_position);
                        *position = Some(new_position);
                        position_input.clear();
                        buffer.clear();
                        opened_race.take();
                    }
                }
                OpenedIndex::Current => {
                    if draft_mogi_result.current_course().is_some() {
                        log::debug!("set current position");
                        if let Some(new_position) = new_position {
                            draft_mogi_result.set_current_position(new_position);
                        }
                    }
                    if let Some(new_position) = new_position {
                        draft_mogi_result.set_current_position(new_position);
                        *position = Some(new_position);
                    }
                    position_input.clear();
                    buffer.clear();
                    opened_race.take();
                }
            }
        }
    }
}

fn show_view(mogi_result: &MogiResult, ui: &mut egui::Ui, tx: &Arc<Mutex<Sender<Event>>>) {
    egui::ScrollArea::horizontal()
        .max_height(420.0)
        .show(ui, |ui| {
            let table = TableBuilder::new(ui)
                .striped(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::initial(100.0).range(40.0..=300.0))
                .column(Column::auto())
                .column(Column::remainder());
            table
                .header(18.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("　");
                    });
                    header.col(|ui| {
                        ui.strong("コース");
                    });
                    header.col(|ui| {
                        ui.strong("順位");
                    });
                    header.col(|ui| {
                        ui.strong("得点");
                    });
                })
                .body(|mut body| {
                    mogi_result.iter_races().enumerate().for_each(|(i, race)| {
                        body.row(30.0, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{:02}", i + 1));
                            });
                            row.col(|ui| {
                                ui.label(race.course_name());
                            });
                            row.col(|ui| {
                                ui.label(race.position().to_string());
                            });
                            row.col(|ui| {
                                ui.label(race.to_score().to_string());
                            });
                        });
                    });
                });
        });

    ui.horizontal(|ui| {
        if ui.button("Copy").clicked() {
            ui.output_mut(|o| {
                mogi_result.iter_races().for_each(|race| {
                    let race_str = format!("{}\t{}\n", &race.course_name(), &race.position());
                    o.copied_text.push_str(&race_str);
                });
            })
        }
        if ui.button("Clear").clicked() {
            tx.lock()
                .unwrap()
                .try_send(Event::EditMogiResult(MogiResult::new()))
                .unwrap();
        }
    });
    ui.separator();
    let current_course_name = mogi_result
        .current_course()
        .as_ref()
        .map_or("(Empty)".to_string(), |course| course.to_string());
    ui.label(format!("現在のコース: {current_course_name}",));

    let total_score = mogi_result.total_score();
    ui.label(format!("合計得点: {total_score}"));
}

fn settings_view(this: &mut App, ui: &mut egui::Ui, frame: &Frame) {
    ui.vertical(|ui| {
        ui.strong("設定");
        ui.separator();
        if ui
            .checkbox(
                &mut this.buf_settings.directshow,
                "DirectShowのデバイスを選択する",
            )
            .clicked()
        {
            this.buf_settings.device_name = "".to_string();
        };
        ui.label("キャプチャするデバイスを選択");
        ComboBox::from_id_source(0)
            .width(200.0)
            .selected_text(this.buf_settings.device_name.clone())
            .show_ui(ui, |ui| {
                let device_names = if this.buf_settings.directshow {
                    &this.directshow_device_names
                } else {
                    &this.msmf_device_names
                };

                device_names.iter().for_each(|dn| {
                    ui.selectable_value(&mut this.buf_settings.device_name, dn.clone(), dn.clone());
                })
            });
        let width = ui
            .ctx()
            .input(|i| i.viewport().inner_rect.unwrap().width() - 15.0);
        this.refresh_capture_preview(width);
        if let Some(captured) = this.capture_preview.as_ref() {
            captured.show(ui);
        }
        ui.separator();
        ui.label("以下の設定は再起動後に変更が反映される");
        ui.label("コンソールに出力するログのレベル");
        ComboBox::from_id_source(1)
            .selected_text(this.buf_settings.log_level.to_string())
            .show_ui(ui, |ui| {
                LOG_LEVELS.iter().for_each(|ll| {
                    ui.selectable_value(
                        &mut this.buf_settings.log_level,
                        ll.to_string(),
                        ll.to_string(),
                    );
                })
            });
        ui.checkbox(
            &mut this.buf_settings.write_log_to_file,
            "ファイルにTRACEレベルのログを出力",
        );
    });
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, frame: &mut Frame) {
        {
            let mut rx = match self.rx.lock() {
                Ok(rx) => rx,
                Err(err) => {
                    log::error!("failed to lock rx: {}", err);
                    return;
                }
            };
            if let Ok(mogi_result) = rx.try_recv() {
                self.mogi_result = mogi_result;
                // TODO: UXとしてどうか判断したい
                // とりあえず新しいmogi_resultを受け取ったら編集モードを抜ける
                self.draft_mogi_result = None;
                self.opened_race = None;
            }
        }
        CentralPanel::default().show(ctx, |ui| {
            ui.horizontal(|ui| {
                ui.heading("lounge-memo");
                ui.with_layout(Layout::right_to_left(Align::Max), |ui| {
                    let settings_label = if self.on_settings { "OK" } else { "Settings" };
                    if ui.button(settings_label).clicked() {
                        self.on_settings = !self.on_settings;
                        // 設定画面を閉じるときに設定を保存する
                        if !self.on_settings {
                            self.save_settings();
                        }
                    }
                });
            });

            if self.on_settings {
                settings_view(self, ui, frame);
                return;
            }

            if let Some(draft_mogi_result) = self.draft_mogi_result.as_mut() {
                edit_view(draft_mogi_result, &mut self.opened_race, &self.courses, ui);
            } else {
                show_view(&self.mogi_result, ui, &self.tx);
            }

            ui.separator();

            if let Some(draft_mogi_result) = self.draft_mogi_result.as_mut() {
                if ui.button("Save All").clicked() {
                    let new_mogi_result = draft_mogi_result.clone();
                    self.mogi_result = new_mogi_result.clone();
                    self.tx
                        .lock()
                        .unwrap()
                        .try_send(Event::EditMogiResult(new_mogi_result))
                        .unwrap();
                    self.draft_mogi_result = None;
                    self.opened_race = None;
                }
            } else if ui.button("Edit").clicked() {
                self.draft_mogi_result = Some(self.mogi_result.clone());
            }
        });

        ctx.request_repaint();
    }
}
