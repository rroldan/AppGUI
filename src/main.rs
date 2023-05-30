
use entidad::TipoViviendaBD;
use entidad::Tipo;


use crate::entidad::TipoViviendaDAO;

mod entidad;
mod schema;
mod repository;
mod presentacion;

fn main() {
    let mut tipoViviendaDAO = TipoViviendaDAO::new();
    let mut gui = presentacion::GUI::new();
    gui.build();
    gui.show();    
}
