use std::hash::Hash;
use uuid::Uuid;

pub struct Publicacion {
    id: String,
    id_prod: String, //id del producto que contiene
    id_user: String, //id del user que publica
    stock: u8,
    activa: bool,
}

impl Publicacion {
    pub fn toggle_activa(&mut self) {
        self.activa = !self.activa;
    }

    pub fn is_activa(&self) -> bool {
        self.activa
    }

    pub fn new(id_producto:String,id_user: String, stock:u8) -> Publicacion{
        let id= Uuid::new_v4().to_string();
        Publicacion{id, id_prod:id_producto, id_user,stock,activa:true}
    }
}
