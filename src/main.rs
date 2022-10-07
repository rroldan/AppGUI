use entidad::Persona;

use crate::entidad::PersonaDAO;

mod entidad;
mod presentacion;

fn main() {
    let mut personaDAO = PersonaDAO::new();
    let mut gui = presentacion::GUI::new();
    gui.build();
    gui.show();    
}
