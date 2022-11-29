use entidad::Persona;
use entidad::TipoVivienda;
use entidad::Tipo;

use crate::entidad::PersonaDAO;
use crate::entidad::TipoViviendaDAO;

mod entidad;
mod presentacion;

fn main() {
    let mut tipoViviendaDAO = TipoViviendaDAO::new();
    let mut gui = presentacion::GUI::new();
    gui.build();
    gui.show();    
}
