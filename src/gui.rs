use eframe::{
    egui::{self, CentralPanel, FontData, FontDefinitions, Layout},
    emath::Align,
    epaint::FontFamily,
    CreationContext, Frame,
};
use egui_dropdown::DropDownBox;
use std::sync::{Arc, Mutex};
use tokio::sync::mpsc::{Receiver, Sender};

use crate::{
    courses::{Course, COURSES, STRING_COURSE_MAP},
    mogi_result::MogiResult,
    race_result::Position,
};

const PPP: f32 = 1.5;

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

#[derive(Debug)]
pub struct App {
    // Sender/Receiver for async notifications.
    tx: Arc<Mutex<Sender<Event>>>,
    rx: Arc<Mutex<Receiver<MogiResult>>>,
    courses: Vec<Course>,
    mogi_result: MogiResult,
    draft_mogi_result: Option<MogiResult>,
    opened_race: Option<OpenedRace>,
}

impl App {
    pub fn new(
        ctx: &CreationContext,
        tx: Arc<Mutex<Sender<Event>>>,
        rx: Arc<Mutex<Receiver<MogiResult>>>,
    ) -> Self {
        let mut fonts = FontDefinitions::default();
        fonts.font_data.insert(
            "notosansjp".to_owned(),
            FontData::from_static(include_bytes!("./assets/NotoSansJP-VariableFont_wght.ttf")),
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
            courses: COURSES.try_lock().unwrap().clone(),
            mogi_result: MogiResult::new(),
            draft_mogi_result: None,
            opened_race: None,
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

fn edit_view(
    draft_mogi_result: &mut MogiResult,
    opened_race: &mut Option<OpenedRace>,
    courses: &[Course],
    ui: &mut egui::Ui,
) {
    ui.label("edit mode");

    let previous_buffer = opened_race
        .as_ref()
        .map(|or| or.buffer.clone())
        .unwrap_or_else(|| "".to_string());

    let mut position_response = None;
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
                        let response = ui.text_edit_singleline(position_input);
                        if response.changed() {
                            position_response = Some(response);
                        }
                    });
                }
            }
        });
    ui.label("current course: ");
    let current_course = draft_mogi_result.current_course().clone();
    let label = current_course
        .as_ref()
        .map_or("(Empty)".to_string(), |course| course.to_string());
    if ui.button(label).clicked() {
        let course = current_course.map(|course| course.clone());
        opened_race.replace(OpenedRace::new(OpenedIndex::Current, course, None));
    }

    if let Some(OpenedRace {
        index: OpenedIndex::Current,
        buffer,
        position_input,
        ..
    }) = opened_race.as_mut()
    {
        ui.group(|ui| {
            course_dropdown(ui, courses, buffer);
            let response = ui.text_edit_singleline(position_input);
            if response.changed() {
                position_response = Some(response);
            }
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
        if position_response.is_some() {
            if let Ok(n) = position_input.parse::<usize>() {
                if n >= 1 && n <= 12 {
                    let new_position = Position::from_index(n - 1);
                    *position = Some(new_position);
                    match index {
                        OpenedIndex::Result(idx) => {
                            draft_mogi_result.set_position(*idx, new_position);
                        }
                        OpenedIndex::Current => {
                            if draft_mogi_result.current_course().is_some() {
                                println!("set current position");
                                draft_mogi_result.set_current_position(new_position);
                                position_input.clear();
                                buffer.clear();
                            }
                        }
                    }
                }
            }
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut Frame) {
        let mut rx = self.rx.lock().unwrap();
        if let Ok(mogi_result) = rx.try_recv() {
            self.mogi_result = mogi_result;
            // TODO: UXとしてどうか判断したい
            // とりあえず新しいmogi_resultを受け取ったら編集モードを抜ける
            self.draft_mogi_result = None;
            self.opened_race = None;
        }
        CentralPanel::default().show(ctx, |ui| {
            ui.heading("lounge-memo");
            if let Some(draft_mogi_result) = self.draft_mogi_result.as_mut() {
                edit_view(draft_mogi_result, &mut self.opened_race, &self.courses, ui);
            } else {
                ui.label(self.mogi_result.to_string());
                if ui.button("Copy").clicked() {
                    ui.output_mut(|o| {
                        self.mogi_result.iter_races().for_each(|race| {
                            let race_str =
                                format!("{}\t{}\n", &race.course_name(), &race.position());
                            o.copied_text.push_str(&race_str);
                        });
                    })
                }
            }

            if let Some(draft_mogi_result) = self.draft_mogi_result.as_mut() {
                if ui.button("Save").clicked() {
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
            } else {
                if ui.button("Edit").clicked() {
                    self.draft_mogi_result = Some(self.mogi_result.clone());
                }
            }
        });
        ctx.request_repaint();
    }
}
