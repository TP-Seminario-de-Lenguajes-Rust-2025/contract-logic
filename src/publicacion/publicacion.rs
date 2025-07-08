pub struct Publicacion {
    id: String,
    id_prod: String, //id del producto que contiene
    id_user: String, //id del user que publica
    activa: bool,
}

impl Publicacion {
    pub fn toggle_activa(&mut self) {
        self.activa = !self.activa;
    }

    pub fn is_activa(&self) -> bool {
        self.activa
    }
}
