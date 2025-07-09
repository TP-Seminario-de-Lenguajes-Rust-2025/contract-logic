pub enum EstadoOrden {
    Pendiente,
    Enviada,
    Recibida,
    Cancelada,
}

pub struct Orden {
    //info de la orden
    id: String,
    id_vendedor: String,
    id_comprador: String,
    status: EstadoOrden,
    productos: Vec<String>,
    cal_vendedor: Option<u8>,  //calificacion que recibe el vendedor
    cal_comprador: Option<u8>, //calificacion que recibe el comprador
}

impl Orden {
    pub fn crear_orden() -> Orden {
        todo!("deberia devolver un Result<Orden>")
    }

    //pub fn cambiar_estado
    //fn set_enviada() //solamente puede ser modificada por el vendedor
    //fn set_recibida() //solamente puede ser modificada por el comprador
    //fn cancelar() //necesitan estar de acuerdo ambos
}
