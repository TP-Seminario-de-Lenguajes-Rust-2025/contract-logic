use uuid::Uuid;

pub enum EstadoOrden {
    Pendiente,
    Enviada, //solo lo puede modificar el vendedor
    Recibida, //solo lo puede modificar el comprador
    Cancelada, //tienen que estar ambos de acuerdo y tiene que estar en estado pendiente
}

pub struct Orden {
    //info de la orden
    id: String,
    id_vendedor: String,
    id_comprador: String,
    status: EstadoOrden,
    productos: Vec<String>, //vec con uuid de los productos
    cal_vendedor: Option<u8>,  //calificacion que recibe el vendedor
    cal_comprador: Option<u8>, //calificacion que recibe el comprador
}

impl Default for Orden{
    fn default() -> Self {
        Orden{
            id: "".to_string(),
            id_vendedor: "".to_string(),
            id_comprador: "".to_string(),
            status: EstadoOrden::Pendiente,
            productos: Vec::new(),
            cal_vendedor: None,
            cal_comprador: None, 
        }
    }
}

impl Orden {
    pub fn new(id_vendedor:String, id_comprador:String, productos:Vec<String>) -> Orden {
        //verificar que productos no sea vacio
        let id = Uuid::new_v4().to_string();
        Orden{id, id_vendedor, id_comprador,productos, ..Default::default()}
    }

    //pub fn cambiar_estado
    //fn set_enviada() //solamente puede ser modificada por el vendedor
    //fn set_recibida() //solamente puede ser modificada por el comprador
    //fn cancelar() //necesitan estar de acuerdo ambos
}
