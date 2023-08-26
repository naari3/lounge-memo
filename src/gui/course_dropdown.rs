// taken from https://github.com/ItsEthra/egui-dropdown
// modify to use with Course struct

use eframe::egui::{Id, Key, Response, Ui, Widget};
use std::hash::Hash;

use crate::courses::COURSE_SHORTHAND_MAP;

/// Dropdown widget
pub struct DropDownBox<
    'a,
    F: FnMut(&mut Ui, &str) -> Response,
    V: AsRef<str>,
    I: Iterator<Item = V>,
> {
    buf: &'a mut String,
    popup_id: Id,
    display: F,
    it: I,
}

impl<'a, F: FnMut(&mut Ui, &str) -> Response, V: AsRef<str>, I: Iterator<Item = V>>
    DropDownBox<'a, F, V, I>
{
    /// Creates new dropdown box.
    pub fn from_iter(
        it: impl IntoIterator<IntoIter = I>,
        id_source: impl Hash,
        buf: &'a mut String,
        display: F,
    ) -> Self {
        Self {
            popup_id: Id::new(id_source),
            it: it.into_iter(),
            display,
            buf,
        }
    }
}

impl<'a, F: FnMut(&mut Ui, &str) -> Response, V: AsRef<str>, I: Iterator<Item = V>> Widget
    for DropDownBox<'a, F, V, I>
{
    fn ui(self, ui: &mut Ui) -> Response {
        let Self {
            popup_id,
            buf,
            it,
            mut display,
        } = self;

        let mut r = ui.text_edit_singleline(buf);
        if r.gained_focus() {
            ui.memory_mut(|m| m.open_popup(popup_id));
        }
        let enter_pressed = r.ctx.input(|i| i.key_pressed(Key::Enter));
        if enter_pressed {
            if let Some(course_name) = COURSE_SHORTHAND_MAP.lock().unwrap().get(buf) {
                *buf = course_name.to_owned();
                ui.memory_mut(|m| m.close_popup());
            }
        }

        let mut changed = false;
        eframe::egui::popup_below_widget(ui, popup_id, &r, |ui| {
            eframe::egui::ScrollArea::vertical().show(ui, |ui| {
                for var in it {
                    let text = var.as_ref();
                    if !buf.is_empty() && !text.contains(&*buf) {
                        continue;
                    }

                    if display(ui, text).clicked() {
                        *buf = text.to_owned();
                        changed = true;

                        ui.memory_mut(|m| m.close_popup());
                    }
                }
            });
        });

        if changed {
            r.mark_changed();
        }

        r
    }
}
