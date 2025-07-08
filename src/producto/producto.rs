pub struct Categoria {
    id: String,
    nombre: String,
}

impl Categoria {
    pub fn crear_categoria() -> Categoria {
        todo!("debe devolver un Result<Categoria>")
    }
}

pub struct Producto {
    id: String,
    nombre: String,
    desc: String,
    precio: f64,
    stock: u8,
    categ: String,
}

impl Producto {
    pub fn new() -> Producto {
        todo!("debe devolver un Result<Producto>")
    }
}
