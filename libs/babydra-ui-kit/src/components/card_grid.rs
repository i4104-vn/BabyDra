//! Responsive card grid layout component 
use gtk4::prelude::*;
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Clone)]
pub struct ResponsiveCardGrid {
    pub grid: gtk4::Grid,
    pub scrolled_win: gtk4::ScrolledWindow,
    cards: Rc<RefCell<Vec<gtk4::Widget>>>,
    cols: Rc<RefCell<usize>>,
}

impl ResponsiveCardGrid {
    pub fn new() -> Self {
        let grid = gtk4::Grid::new();
        grid.add_css_class("settings-card-grid");
        grid.set_column_homogeneous(true);
        grid.set_row_homogeneous(false);
        grid.set_column_spacing(16);
        grid.set_row_spacing(16);
        grid.set_hexpand(true);
        grid.set_halign(gtk4::Align::Fill);
        grid.set_valign(gtk4::Align::Start);

        let scrolled_win = gtk4::ScrolledWindow::new();
        scrolled_win.set_policy(gtk4::PolicyType::Never, gtk4::PolicyType::Automatic);
        scrolled_win.set_vexpand(true);
        scrolled_win.set_hexpand(true);
        scrolled_win.set_valign(gtk4::Align::Fill);
        scrolled_win.set_child(Some(&grid));

        let cards = Rc::new(RefCell::new(Vec::new()));
        let cols = Rc::new(RefCell::new(3usize));

        let grid_c = grid.clone();
        let cards_c = cards.clone();
        let cols_c = cols.clone();

        scrolled_win.hadjustment().connect_page_size_notify(move |adj| {
            let width = adj.page_size();
            if width <= 10.0 {
                return;
            }
            let target_cols = if width >= 720.0 {
                3
            } else if width >= 460.0 {
                2
            } else {
                1
            };

            if *cols_c.borrow() != target_cols {
                *cols_c.borrow_mut() = target_cols;
                Self::relayout_items(&grid_c, &cards_c.borrow(), target_cols);
            }
        });

        Self {
            grid,
            scrolled_win,
            cards,
            cols,
        }
    }

    fn relayout_items(grid: &gtk4::Grid, cards: &[gtk4::Widget], num_cols: usize) {
        while let Some(child) = grid.first_child() {
            grid.remove(&child);
        }
        for (i, card) in cards.iter().enumerate() {
            let col = (i % num_cols) as i32;
            let row = (i / num_cols) as i32;
            grid.attach(card, col, row, 1, 1);
        }
        // Guarantee grid always allocates num_cols columns even if total cards < num_cols
        if !cards.is_empty() && cards.len() < num_cols {
            for col in cards.len()..num_cols {
                let spacer = gtk4::Box::new(gtk4::Orientation::Horizontal, 0);
                spacer.set_hexpand(true);
                grid.attach(&spacer, col as i32, 0, 1, 1);
            }
        }
    }

    pub fn clear(&self) {
        while let Some(child) = self.grid.first_child() {
            self.grid.remove(&child);
        }
        self.cards.borrow_mut().clear();
    }

    pub fn append<W: IsA<gtk4::Widget>>(&self, card: &W) {
        let widget = card.clone().upcast::<gtk4::Widget>();
        let mut cards = self.cards.borrow_mut();
        cards.push(widget);
        let num_cols = *self.cols.borrow();
        Self::relayout_items(&self.grid, &cards, num_cols);
    }
}

impl Default for ResponsiveCardGrid {
    fn default() -> Self {
        Self::new()
    }
}
