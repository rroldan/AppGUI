use fltk::{
    app::{self, App}, enums,
    prelude::{GroupExt, WidgetExt},
    window::{self, DoubleWindow}, button::Button,
};
use fltk_table::{SmartTable, TableOpts};

use fltk::{app::*, browser::*, button::*, enums::*, input::*, prelude::*, window::*};

const WIDGET_WIDTH: i32 = 70;
const WIDGET_HEIGHT: i32 = 25;
const WIDGET_PADDING: i32 = 10;

#[derive(Clone, Copy)]
enum Message {
    Create,
    Update,
    Delete,
    Select,
    Filter,
}

use crate::entidad::Persona;

pub struct GUI {
    app : App,
    wind : DoubleWindow,
    table : SmartTable
}

impl GUI {
    
    pub fn new() -> GUI {
        let mut app = app::App::default().with_scheme(app::Scheme::Gtk);
        let mut wind = window::Window::default().with_size(800, 600);

        let mut table = SmartTable::default()
        .with_size(790, 590)
        .center_of_parent()
        .with_opts(TableOpts {
            rows: 30,
            cols: 15,
            editable: true,
            ..Default::default()
        });
        GUI {
            app : app,
            wind : wind,
            table : table
        }
    }

    pub fn build(&mut self) {
        let mut b0 = Button::default().with_label("won't work")
            .with_size(96, 64)
            .with_pos(20, 32);
        let mut b1 = Button::default().with_label("is hacky")
            .with_size(96, 64)
            .with_pos(130, 32);        
    }

    pub fn show(&mut self) {
        self.wind.show();    
        self.app.run().unwrap();
    }
    
    pub fn refresh(&mut self, data : Vec<Persona>) {
        self.table.clear();
        for (i,p) in data.iter().enumerate() {
            println!("{} {:?} ",i, p);
            self.table.set_cell_value(i as i32, 0, p.identificacion.as_str());
            self.table.set_cell_value(i as i32, 1, p.apellidos.as_str());
            self.table.set_cell_value(i as i32, 2, p.nombres.as_str());           
        }    
        //self.wind.end();
    }

}
