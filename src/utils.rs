// SPDX-License-Identifier: GPL-3.0-only
use gtk4::{gio, glib};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone, Debug, Default, glib::Boxed)]
#[boxed_type(name = "BoxedSearchResult")]
pub struct BoxedSearchResult(pub Option<pop_launcher::SearchResult>);

pub fn icon_source(icon: &Rc<RefCell<gtk4::Image>>, source: &Option<pop_launcher::IconSource>) {
    match source {
        Some(pop_launcher::IconSource::Name(name)) => {
            icon.borrow().set_from_icon_name(Some(name));
        }
        Some(pop_launcher::IconSource::Mime(content_type)) => {
            icon.borrow()
                .set_from_gicon(&gio::content_type_get_icon(content_type));
        }
        _ => {
            icon.borrow().set_from_icon_name(None);
        }
    }
}
