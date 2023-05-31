use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;

use crate::entidad::{TipoViviendaBD};
use crate::schema::tipo_viviendas::dsl::*;

use std::env;

pub struct TipoViviendaRepository {
    pub conn: SqliteConnection,

}

impl TipoViviendaRepository {

    pub fn new() -> Self {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL")
            .expect("DATABASE_URL must be set");        

            TipoViviendaRepository { 
            conn: SqliteConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
        }
    }

    pub fn find_all(&mut self) -> Result<Vec<TipoViviendaBD>, Error>  {
        {
            tipo_viviendas.load::<TipoViviendaBD>(&mut self.conn)
        }
    }


    pub fn find_by_id(&mut self, uniq_id: String) -> Result<TipoViviendaBD, Error> {
        tipo_viviendas.find(uniq_id).get_result::<TipoViviendaBD>(&mut self.conn)
    }

    pub fn create(&mut self, new_tipo_vivienda: &TipoViviendaBD) -> Result<TipoViviendaBD, Error> {
        diesel::insert_into(tipo_viviendas)
        .values(new_tipo_vivienda)
        .execute(&mut self.conn)
        .expect("Error saving new post");

        tipo_viviendas.order(identificacion.desc()).first(&mut self.conn)
    }

    pub fn update(&mut self, uniq_id: String, tipo_vivienda: TipoViviendaBD) -> Result<TipoViviendaBD, Error> {
        diesel::update(tipo_viviendas.find(tipo_vivienda.identificacion))
        .set((calle.eq(&tipo_vivienda.calle),numero.eq(&tipo_vivienda.numero), piso.eq(&tipo_vivienda.piso), codigo_postal.eq(&tipo_vivienda.codigo_postal), metros_cuadrados.eq(&tipo_vivienda.metros_cuadrados), numero_aseos.eq(&tipo_vivienda.numero_aseos),numero_habitaciones.eq(&tipo_vivienda.numero_habitaciones),tipo.eq(&tipo_vivienda.tipo)))
        .execute(&mut self.conn)
        .unwrap();

        tipo_viviendas.find(uniq_id).first(&mut self.conn)
    }

    pub fn delete(&mut self, uniq_id: String) -> Result<usize, Error> {
         diesel::delete(tipo_viviendas.find(uniq_id)).execute(&mut self.conn)
    }
    
}