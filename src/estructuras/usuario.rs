use uuid::Uuid;

pub struct Rol {
    id: String, //not a Uuid
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
    fn new() -> Rating{
        Rating { calificacion_comprador: (0,0), calificacion_vendedor: (0,0) }
    }
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
    roles: Vec<String> //id de rol
}

impl Usuario {
    pub fn new(nombre:String,mail:String,roles:Vec<String>) -> Usuario {
        let id = Uuid::new_v4().to_string();
        Usuario { id , nombre, mail, rating: Rating::new(), roles}
    }

    pub fn has_role(&self, rol:&str) ->bool{
        self.roles.contains(&rol.to_string())
    }

    pub fn get_rating_comprador() -> f64 {
        todo!()
    }
    pub fn get_rating_vendedor() -> f64 {
        todo!()
    }

    pub fn agregar_rol(){
        
    }
}
