use rgtk::*;
use std::old_io::fs::PathExtensions;
use std::num::FromPrimitive;

fn save_project(
    state: &mut ::utils::State,
    tree: &mut gtk::TreeView,
    filename: String)
{
    state.projects.insert(filename.clone());
    state.selection = Some(filename.clone());
    ::utils::write_prefs(state);
    ::ui::update_project_tree(state, tree);
}

pub fn new_project(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    if let Some(dialog) = gtk::FileChooserDialog::new(
        "New Project",
        None,
        gtk::FileChooserAction::Save
    ) {
        if let Some(gtk::ResponseType::Accept) = FromPrimitive::from_i32(dialog.run()) {
            if let Some(filename) = dialog.get_filename() {
                save_project(state, tree, filename);
                // TODO: cargo new filename --bin
            }
        }
        dialog.destroy();
    }
}

pub fn import_project(state: &mut ::utils::State, tree: &mut gtk::TreeView) {
    if let Some(dialog) = gtk::FileChooserDialog::new(
        "Import",
        None,
        gtk::FileChooserAction::SelectFolder
    ) {
        if let Some(gtk::ResponseType::Accept) = FromPrimitive::from_i32(dialog.run()) {
            if let Some(filename) = dialog.get_filename() {
                save_project(state, tree, filename);
            }
        }
        dialog.destroy();
    }
}

pub fn rename_file(state: &mut ::utils::State) {
    if let Some(_) = ::utils::get_selected_path(state) {
        // TODO: show dialog with a text field
    }
}

pub fn remove_item(state: &mut ::utils::State, tree: &mut gtk::TreeView, fd: ::ffi::c_int) {
    if let Some(path) = ::utils::get_selected_path(state) {
        if let Some(dialog) = gtk::MessageDialog::new_with_markup(
            Some(state.window.clone()),
            gtk::DialogFlags::Modal,
            gtk::MessageType::Question,
            gtk::ButtonsType::OkCancel,
            if state.projects.contains(&path) {
                "Remove this project? It WILL NOT be deleted from the disk."
            } else {
                "Remove this file? It WILL be deleted from the disk."
            }
        ) {
            if let Some(gtk::ResponseType::Ok) = FromPrimitive::from_i32(dialog.run()) {
                if state.projects.contains(&path) {
                    state.projects.remove(&path);
                } else {
                    ::ffi::send_message(fd, ":call delete(expand('%')) | bdelete!".as_slice());
                }
                ::utils::write_prefs(state);
                ::ui::update_project_tree(state, tree);
            }
            dialog.destroy();
        }
    }
}

pub fn set_selection(state: &mut ::utils::State, fd: ::ffi::c_int) {
    if !state.is_refreshing_tree {
        if let Some(ref path) = ::utils::get_selected_path(state) {
            state.selection = Some(path.clone());
            ::utils::write_prefs(state);
            ::ui::update_project_buttons(state);
            ::ffi::send_message(fd, format!("e {}", path).as_slice());
        }
    }
}

pub fn remove_expansion(state: &mut ::utils::State, iter: &gtk::TreeIter) {
    if let Some(path_str) = state.tree_model.get_value(iter, 1).get_string() {
        for p in state.expansions.clone().iter() {
            if *p == path_str ||
                !Path::new(p).exists() ||
                (p.starts_with(path_str.as_slice()) &&
                !::utils::are_siblings(&path_str, p))
            {
                state.expansions.remove(p);
            }
        }
        ::utils::write_prefs(state);
    }
}

pub fn add_expansion(state: &mut ::utils::State, iter: &gtk::TreeIter) {
    if let Some(path_str) = state.tree_model.get_value(iter, 1).get_string() {
        state.expansions.insert(path_str);
        ::utils::write_prefs(state);
    }
}
