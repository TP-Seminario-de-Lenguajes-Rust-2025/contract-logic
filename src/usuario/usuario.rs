pub struct Rol {
    id: String,
    desc: String,
}

impl Rol {
    pub fn crear_rol() {
        todo!()
    }
}

struct Rating {
    calificacion_comprador: (u16, u32), //cant de compras, valor cumulativo de todas las calificaciones
    calificacion_vendedor: (u16, u32),
}

impl Rating {
    fn get_rating_comprador() -> f64 {
        todo!("debe devolver Result<f64,ErrDivisionZero>")
    }

    fn get_rating_vendedor() -> f64 {
        todo!("debe devolver Result<f64,ErrDivisionZero>")
    }
}

pub struct Usuario {
    id: String,
    nombre: String,
    mail: String,
    rating: Rating,
}

impl Usuario {
    pub fn new() -> Usuario {
        todo!()
    }

    pub fn get_rating_comprador() -> f64 {
        todo!()
    }
    pub fn get_rating_vendedor() -> f64 {
        todo!()
    }
}
