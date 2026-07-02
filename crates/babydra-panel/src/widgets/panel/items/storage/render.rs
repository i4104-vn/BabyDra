use gtk4::prelude::*;
use super::get_disk_list;

pub fn create_disk_list_box() -> gtk4::Box {
    let card = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    card.add_css_class("control-disk-card");

    let title_row = gtk4::Box::new(gtk4::Orientation::Horizontal, 6);
    let disk_icon = babydra_common::icon::get_icon_colored("server", 12, "#10b981");
    let title_label = gtk4::Label::new(Some(&babydra_common::i18n::t("panel.storage_usage")));
    title_label.add_css_class("control-slider-title");
    
    title_row.append(&disk_icon);
    title_row.append(&title_label);
    card.append(&title_row);

    let list_container = gtk4::Box::new(gtk4::Orientation::Vertical, 8);
    
    let disks = get_disk_list();
    if disks.is_empty() {
        let no_disks = gtk4::Label::new(Some(&babydra_common::i18n::t("panel.no_storage")));
        no_disks.add_css_class("tile-subtitle");
        list_container.append(&no_disks);
    } else {
        for disk in disks.into_iter().take(3) {
            let disk_item = gtk4::Box::new(gtk4::Orientation::Vertical, 3);
            disk_item.add_css_class("control-disk-item");

            let label_box = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
            label_box.add_css_class("control-disk-title-box");

            let name_label = gtk4::Label::new(Some(&disk.mount_point));
            name_label.add_css_class("control-disk-name");
            name_label.set_hexpand(true);
            name_label.set_halign(gtk4::Align::Start);

            let usage_label = gtk4::Label::new(Some(&format!(
                "{} / {} ({:.0}%)",
                disk.used, disk.size, disk.percent
            )));
            usage_label.add_css_class("control-disk-usage");
            usage_label.set_halign(gtk4::Align::End);

            label_box.append(&name_label);
            label_box.append(&usage_label);

            let progress = gtk4::ProgressBar::new();
            progress.set_fraction(disk.percent / 100.0);
            progress.set_hexpand(true);

            disk_item.append(&label_box);
            disk_item.append(&progress);

            list_container.append(&disk_item);
        }
    }

    card.append(&list_container);
    card
}
