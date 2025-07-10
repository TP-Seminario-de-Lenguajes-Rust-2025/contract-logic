use uuid::Uuid;

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
    precio: String,
    categ: String,
}

impl Producto {
    pub fn new(nombre: String,descripcion:String, precio:String,categoria: String) -> Producto {
        //TODO: verificar que stock>0 y precio>0 y nombre y desc sean validos
        let id = Uuid::new_v4().to_string();
        Producto{id, nombre, desc: descripcion, precio, categ:categoria}
    }
}
