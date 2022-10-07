use std::io::SeekFrom;

use fltk::{
    app::{self, App}, enums,
    prelude::{GroupExt, WidgetExt},
    window::{self, DoubleWindow}, button::Button,
};
use fltk_table::{SmartTable, TableOpts};

use fltk::{app::*, browser::*, button::*, enums::*, input::*, prelude::*, window::*};
use serde::__private::de;

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
    Save,
}

use crate::entidad::{Persona, ScreenOutput};
use crate::entidad::PersonaDAO;

pub struct GUI{
    app : App,
    wind : DoubleWindow,
    sender : Sender<Message>,
    receiver : Receiver<Message>,
    model : Vec<Persona>,
    personaDAO : PersonaDAO,
    filter_input : Input,
    list_browser : HoldBrowser,
    ident_input : Input,
    name_input : Input,
    surname_input : Input,
    create_button : Button,
    update_button : Button,
    delete_button : Button,
    save_button : Button
}

impl GUI {
    
    pub fn new() -> GUI {
        let mut app = app::App::default().with_scheme(app::Scheme::Gtk);
        let mut wind = Window::default().with_label("CRUD");
        let (sender, receiver) = channel::<Message>();

        let mut filter_input = Input::default().with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(WIDGET_PADDING + WIDGET_WIDTH * 2, WIDGET_PADDING)
        .with_label("Filter prefix:");

        let mut list_browser = HoldBrowser::default().with_pos(
            WIDGET_PADDING,
            filter_input.y() + filter_input.height() + WIDGET_PADDING,
        )
        .with_size(WIDGET_WIDTH * 3, WIDGET_HEIGHT * 4);

        let ident_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .with_pos(
            list_browser.x() + list_browser.width() + WIDGET_PADDING + WIDGET_WIDTH,
            list_browser.y(),
        )
        .with_label("Id:");

        let name_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&ident_input, WIDGET_PADDING)
        .with_label("Nombres:");

        let surname_input = Input::default()
        .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
        .below_of(&name_input, WIDGET_PADDING)
        .with_label("Apellidos:");

        let mut create_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .with_pos(
                WIDGET_PADDING,
                list_browser.y() + list_browser.height() + WIDGET_PADDING,
            )
            .with_label("Crear");

        let mut update_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&create_button, WIDGET_PADDING)
            .with_label("Modificar");

        let mut delete_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&update_button, WIDGET_PADDING)
            .with_label("Borrar");

        let mut save_button = Button::default()
            .with_size(WIDGET_WIDTH, WIDGET_HEIGHT)
            .right_of(&delete_button, WIDGET_PADDING)
            .with_label("Guardar");

        let personaDAO = PersonaDAO::new();
        let model = personaDAO.asVector();

        GUI {
            app : app,
            wind : wind,
            sender : sender,
            receiver : receiver,
            filter_input : filter_input,
            list_browser : list_browser,
            personaDAO : personaDAO,
            model : model,
            ident_input : ident_input,
            name_input : name_input,
            surname_input : surname_input,
            create_button : create_button,
            update_button : update_button,
            delete_button : delete_button,
            save_button : save_button
        }
    }

    pub fn build(&mut self) {
        self.filter_input.set_trigger(CallbackTrigger::Changed);
        self.filter_input.emit(self.sender, Message::Filter);

        self.list_browser.emit(self.sender, Message::Select);        

        //self.sender.send(Message::Filter);

        self.create_button.emit(self.sender, Message::Create);

        self.update_button.emit(self.sender, Message::Update);
        self.update_button.deactivate();

        self.delete_button.emit(self.sender, Message::Delete);
        self.delete_button.deactivate();

        self.save_button.emit(self.sender, Message::Save);

        self.wind.set_size(
            self.name_input.x() + self.name_input.width() + WIDGET_PADDING,
            self.create_button.y() + self.create_button.height() + WIDGET_PADDING,
        );

        self.sender.send(Message::Filter);

    }

    fn clear_edit(&mut self) {
        self.ident_input.set_value("");
        self.name_input.set_value("");
        self.surname_input.set_value("");
    }

    pub fn show(&mut self) {
        self.wind.end();
        self.wind.show();
        while self.app.wait() {
            match self.receiver.recv() {
                Some(Message::Create) => {
                    self.model.push(Persona { 
                        identificacion : self.ident_input.value(),
                        apellidos : self.surname_input.value(),
                        nombres : self.name_input.value()
                    });
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                }
                Some(Message::Update) => {
                    if self.list_browser.value() > 0 {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter_mut().filter(|e| {
                            return e.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();
                        match search_result {
                            Some(persona) => {
                                persona.nombres = self.name_input.value();
                                persona.apellidos = self.surname_input.value();
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA MODIFICAR!!!");
                    }
                }
                Some(Message::Delete) => {
                    if self.list_browser.value() > 0 {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter().enumerate().filter(|e| {
                            return e.1.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();
                        match search_result {
                            Some((index,persona)) => {
                                self.model.remove(index);
                                self.clear_edit();
                                self.sender.send(Message::Filter);
                                self.sender.send(Message::Select);
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }
                    } else {
                        println!("NO HAY ELEMENTO PARA ELIMINAR!!!");
                    }
                }
                Some(Message::Save) => {
                    self.personaDAO.save_and_refresh(&self.model);
                    self.model = self.personaDAO.asVector();
                    self.clear_edit();
                    self.sender.send(Message::Filter);
                    self.sender.send(Message::Select);
                }
                Some(Message::Select) => {
                    if self.list_browser.value() == 0 {
                        self.update_button.deactivate();
                        self.delete_button.deactivate();
                    } else {
                        let text_selection = self.list_browser.text(self.list_browser.value()).unwrap();
                        let search_result = self.model.iter().filter(|e| {
                            return e.toScreen().eq_ignore_ascii_case(&text_selection)
                        }).next();

                        match search_result {
                            Some(persona) => {
                                self.ident_input.set_value(&persona.identificacion);
                                self.name_input.set_value(&persona.nombres);
                                self.surname_input.set_value(&persona.apellidos);
                                self.update_button.activate();
                                self.delete_button.activate();
                            },
                            _ => {
                                println!("ELEMENTO NO ENCONTRADO!!!");
                            } 
                        }                        
                    }
                }
                Some(Message::Filter) => {
                    let prefix = self.filter_input.value().to_lowercase();
                    let filter_empty = prefix.trim().eq_ignore_ascii_case("");
                    self.list_browser.clear();
                    for (i,p) in self.model.iter().enumerate() {
                        if (p.identificacion.eq_ignore_ascii_case(prefix.as_str()) && !filter_empty) || (filter_empty)  {
                            let item = p.toScreen();
                            self.list_browser.add(&item);    
                        }
                    }                                 
                    self.sender.send(Message::Select);    
                }
                None => {},
                _ => {}
            }
        }
    }
    
    pub fn refresh(&mut self, data : Vec<Persona>) {
        for (i,p) in data.iter().enumerate() {
            println!("{} {:?} ",i, p);
        }    
    }

}
