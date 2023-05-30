use std::{path::Path, fs::{File, self}, collections::HashMap, hash::Hash, iter, result, fmt};
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use std::str::FromStr;
use std::error::Error;
use csv::Reader;
use csv::Writer;
use diesel::{Queryable, Insertable, Selectable, Identifiable};
use crate::schema::tipo_viviendas;
pub trait ScreenOutput {
    fn toScreen(&self) -> String;
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub enum Tipo {
    Apartamento,
    Casa,
    Chalet
}

#[derive(Debug, Deserialize,Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct TipoVivienda {
    pub identificacion: String,
    pub calle: String,
    pub numero: i32,
    pub piso: String,
    pub codigo_postal: String,
    pub metros_cuadrados: i32,
    pub numero_aseos: i32,
    pub numero_habitaciones: i32,
    pub tipo: Tipo
}
#[derive(Debug, Queryable, Selectable, Insertable)]
#[diesel(table_name = tipo_viviendas)]
pub struct TipoViviendaBD {
    pub identificacion: String,
    pub calle: String,
    pub numero: i32,
    pub piso: String,
    pub codigo_postal: String,
    pub metros_cuadrados: i32,
    pub numero_aseos: i32,
    pub numero_habitaciones: i32,
    pub tipo: String
}

impl FromStr for Tipo {

    type Err = ();

    fn from_str(input: &str) -> Result<Tipo, Self::Err> {
        match input {
            "Apartamento"  => Ok(Tipo::Apartamento),
            "Casa"  => Ok(Tipo::Casa),
            "Chalet"  => Ok(Tipo::Chalet),
            _      => Err(()),
        }
    }
}

impl fmt::Display for Tipo {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Tipo::Apartamento => write!(f, "Apartamento"),
            Tipo::Casa => write!(f, "Casa"),
            Tipo::Chalet => write!(f, "Chalet")
        }
    }
}


impl ScreenOutput for TipoVivienda {
    fn toScreen(&self) -> String {
        format!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}", self.identificacion,self.calle,self.numero,self.piso,self.codigo_postal,
        self.metros_cuadrados,self.numero_aseos,self.numero_habitaciones, self.tipo)
    }
} 



pub struct TipoViviendaDAO {
    indice : HashMap<String,TipoVivienda>
}


impl ScreenOutput for TipoViviendaDAO {
    fn toScreen(&self) -> String {
        format!("{:?}",self.indice)
    }
}


impl TipoViviendaDAO {
    pub fn new() -> TipoViviendaDAO {
        let mut p = TipoViviendaDAO { indice : HashMap::new() };
        p.refresh();
        p
    }

    pub fn refresh(&mut self) {
        let path_csv =  Path::new("./src/csv/tipo-vivienda.csv");
        self.indice.clear();
        let mut rdr = Reader::from_path(path_csv).unwrap();

        //let mut iter = rdr.deserialize();
        for result in rdr.deserialize() {
            let record: TipoVivienda = result.unwrap();
            self.indice.insert(record.clone().identificacion,record);
        }  
    }
    

    pub fn save (&self, datos : &Vec<TipoVivienda>) -> Result<(), Box<dyn Error>> {
        let path_json =  Path::new("./src/csv/tipo-vivienda.csv");
        let mut wtr = Writer::from_path(path_json)?;
        //let mut wtr = Writer::from_writer(vec![]);
        for tipo_vivienda in datos {
            wtr.serialize(tipo_vivienda)?
        }
        wtr.flush()?;
        Ok(())
    }


    pub fn save_and_refresh(&mut self, datos: &Vec<TipoVivienda>) {
        self.save(datos);
        self.refresh();
    }


    pub fn asVector(&self) -> Vec<TipoVivienda> {
        let datos = self.indice.values().cloned().collect::<Vec<TipoVivienda>>();
        datos
    }

    pub fn add(&mut self, p : TipoVivienda) {
        if !self.indice.contains_key(&p.identificacion) {
            self.indice.insert(p.clone().identificacion, p);
        }
    } 

    pub fn update(&mut self, p : TipoVivienda) {
        if self.indice.contains_key(&p.identificacion) {
            self.indice.insert(p.clone().identificacion, p);
        }
    } 

    pub fn remove(&mut self, key : &String) -> Option<TipoVivienda> {
        self.indice.remove(key)
    }      
    
}

#[test]
fn to_screen_tipo_vivienda() {
    let tipo_vivienda = super::entidad::TipoVivienda {
    identificacion: String::from("1"),
    calle: String::from("San Isidro"),
    numero: 4,
    piso: String::from("1C"),
    codigo_postal: String::from("28350"),
    metros_cuadrados: 80,
    numero_aseos: 1,
    numero_habitaciones: 2,
    tipo: super::Tipo::Apartamento};
    assert_eq!(tipo_vivienda.toScreen(),"\"1\",\"San Isidro\",4,\"1C\",\"28350\",80,1,2,Apartamento");
}

#[test]
fn as_vector_tipo_vivienda() {
    let mut tipo_vivienda_dao = TipoViviendaDAO::new();
    let  mut datos:  Vec<TipoVivienda> = tipo_vivienda_dao.asVector();
    assert_eq!(&datos[0].toScreen(),"\"1\",\"San Isidro\",4,\"1C\",\"28350\",80,1,2,Apartamento");
}

#[test]
fn add_tipo_vivienda() {
    let tipo_vivienda = super::entidad::TipoVivienda {
        identificacion: String::from("2"),
        calle: String::from("Chile"),
        numero: 40,
        piso: String::from(""),
        codigo_postal: String::from("28350"),
        metros_cuadrados: 100,
        numero_aseos: 3,
        numero_habitaciones: 3,
        tipo: super::Tipo::Chalet
    };

    let mut tipo_vivienda_dao = TipoViviendaDAO::new();   
    tipo_vivienda_dao.add(tipo_vivienda);

    let datos:  Vec<TipoVivienda> = tipo_vivienda_dao.asVector();
    for tipo_vivienda in datos {
        if tipo_vivienda.identificacion == "2" {
            assert_eq!(tipo_vivienda.toScreen(), "\"2\",\"Chile\",40,\"\",\"28350\",100,3,3,Chalet");
            break; 
        }
    }
}

#[test]
fn remove_tipo_vivienda() {
    let mut tipo_vivienda_dao = TipoViviendaDAO::new();   
    tipo_vivienda_dao.remove(&String::from("1"));
    let datos:  Vec<TipoVivienda> = tipo_vivienda_dao.asVector();
    assert_eq!(datos.len(),0);
}

#[test]
fn save_an_refresh_tipo_vivienda() {
    let tipo_vivienda = super::entidad::TipoVivienda {
        identificacion: String::from("2"),
        calle: String::from("Chile"),
        numero: 40,
        piso: String::from(""),
        codigo_postal: String::from("28350"),
        metros_cuadrados: 100,
        numero_aseos: 3,
        numero_habitaciones: 3,
        tipo: super::Tipo::Chalet
    };

    let mut tipo_vivienda_dao = TipoViviendaDAO::new();   
    tipo_vivienda_dao.add(tipo_vivienda);
    tipo_vivienda_dao.save_and_refresh(&tipo_vivienda_dao.asVector());

    let datos:  Vec<TipoVivienda> = tipo_vivienda_dao.asVector();
    for tipo_vivienda in datos {
        if tipo_vivienda.identificacion == "2" {
            assert_eq!(tipo_vivienda.toScreen(), "\"2\",\"Chile\",40,\"\",\"28350\",100,3,3,Chalet");
            break; 
        }
    }
    tipo_vivienda_dao.remove(&String::from("2"));
    tipo_vivienda_dao.save_and_refresh(&tipo_vivienda_dao.asVector());
}
