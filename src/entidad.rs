use std::{path::Path, fs::{File, self}, collections::HashMap, hash::Hash, iter, result};
use serde::{Deserialize, Serialize, ser::SerializeStruct};
use std::str::FromStr;
use std::error::Error;
use csv::Reader;
use csv::Writer;
pub trait ScreenOutput {
    fn toScreen(&self) -> String;
}

#[derive(Debug, Deserialize,Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct Persona {
    pub identificacion : String,
    pub apellidos : String,
    pub nombres : String
}

#[derive(Debug, Deserialize,Serialize,Clone)]
pub struct Otra {}

#[derive(Debug, Deserialize,Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub enum Tipo {
    Apartamento,
    Casa,
    Chalet
}


#[derive(Debug, Deserialize,Serialize,Clone)]
#[serde(rename_all = "camelCase")]
pub struct TipoVivienda {
    pub identificacion : String,
    pub calle: String,
    pub numero: i32,
    pub piso: String,
    pub codigo_postal: String,
    pub metros_cuadrados: i32,
    pub numero_aseos: i32,
    pub numero_habitaciones: i32,
    pub tipo: Tipo
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

impl ScreenOutput for Persona {
    fn toScreen(&self) -> String {
        format!("{:?},{:?},{:?}",self.identificacion,self.nombres,self.apellidos)
    }
}

impl ScreenOutput for TipoVivienda {
    fn toScreen(&self) -> String {
        format!("{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?},{:?}", self.identificacion,self.calle,self.numero,self.piso,self.codigo_postal,
        self.metros_cuadrados,self.numero_aseos,self.numero_habitaciones, self.tipo)
    }
} 


pub struct PersonaDAO {
    indice : HashMap<String,Persona>
}

pub struct TipoViviendaDAO {
    indice : HashMap<String,TipoVivienda>
}

impl ScreenOutput for PersonaDAO {
    fn toScreen(&self) -> String {
        format!("{:?}",self.indice)
    }
}

impl ScreenOutput for TipoViviendaDAO {
    fn toScreen(&self) -> String {
        format!("{:?}",self.indice)
    }
}

impl PersonaDAO {

    pub fn new() -> PersonaDAO {
        let mut p = PersonaDAO { indice : HashMap::new() };
        p.refresh();
        p
    }

    pub fn refresh(&mut self)  {
        let path_json =  Path::new("./src/json/personas.json");
        let data_str = fs::read_to_string(path_json).expect("Unable to read file");
        let personas : Vec<Persona> = serde_json::from_str(&data_str).expect("JSON does not have correct format.");
        self.indice.clear();
        for p in personas {
            self.indice.insert(p.clone().identificacion,p);
        }
    }

    pub fn save_state(&self) {
        let datos = self.indice.values().cloned().collect::<Vec<Persona>>();
        self.save(&datos);
    }

    pub fn save(&self, datos : &Vec<Persona>) {
        let path_json =  Path::new("./src/json/personas.json");
        std::fs::write(
            path_json,
            serde_json::to_string_pretty(&datos).unwrap(),
        )
        .unwrap();        
    }

    pub fn save_and_refresh(&mut self, datos: &Vec<Persona>) {
        self.save(datos);
        self.refresh();
    }


    pub fn asVector(&self) -> Vec<Persona> {
        let datos = self.indice.values().cloned().collect::<Vec<Persona>>();
        datos
    }

    pub fn add(&mut self, p : Persona) {
        if !self.indice.contains_key(&p.identificacion) {
            self.indice.insert(p.clone().identificacion, p);
        }
    } 

    pub fn update(&mut self, p : Persona) {
        if self.indice.contains_key(&p.identificacion) {
            self.indice.insert(p.clone().identificacion, p);
        }
    } 

    pub fn remove(&mut self, key : &String) -> Option<Persona> {
        self.indice.remove(key)
    }        

}
impl TipoViviendaDAO {
    pub fn new() -> TipoViviendaDAO {
        let mut p = TipoViviendaDAO { indice : HashMap::new() };
        p.refresh();
        p
    }

    pub fn refresh(&mut self) -> Result<(), Box<dyn Error>> {
        let path_json =  Path::new("./src/csv/tipo-vivienda.csv");
        self.indice.clear();
    let mut rdr = Reader::from_path(path_json).unwrap();
        let mut iter = rdr.deserialize();
        if let Some(result) = iter.next() {
            let record: TipoVivienda = result?;
            self.indice.insert(record.clone().identificacion,record);
            Ok(())
        } else {
            Err(From::from("expected at least one record but got none"))
        }
    }
    pub fn save_state(&self) {
        let datos = self.indice.values().cloned().collect::<Vec<TipoVivienda>>();
        self.save(&datos);
    }
/* 
    pub fn save(&self, datos : &Vec<TipoVivienda>) {
        let path_json =  Path::new("./src/json/tipo-vivienda.json");
        std::fs::write(
            path_json,
            serde_json::to_string_pretty(&datos).unwrap(),
        )
        .unwrap();        
    }
*/
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
fn to_screen_persona() {
    let persona = super::Persona{identificacion: String::from("6"),
    apellidos: String::from("VALERA VAZQUEZ"),
    nombres: String::from("RAMON ALEJANDRO")};

    assert_eq!(persona.toScreen(),"\"6\",\"RAMON ALEJANDRO\",\"VALERA VAZQUEZ\"");
}
#[test]
fn to_screen_tipo_vivienda() {
    let tipo_vivienda = super::TipoVivienda {
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
    let tipo_vivienda = super::TipoVivienda {
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
    let tipo_vivienda = super::TipoVivienda {
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
