use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

// Constants for server configuration
const HOST: &str = "127.0.0.1";
const PORT: &str = "8080";

fn main() {
    // Bind to the host and port
    let endpoint = format!("{}:{}", HOST, PORT); // se format! es un macro, permite incluir variables en cadenas
    let listener = TcpListener::bind(endpoint).unwrap(); // se crea una variable que es el listener, se hace un bind y se pasa el parametro de la direccion y posteriormente se hace un unwrap para manejar errores (vacios)
    print!("servidor iniciado con exito en puerto {} /n", PORT);

    for flujo in listener.incoming() {
        let mut trafico = flujo.unwrap();
        handle_conects(&mut trafico);
    }
}

// funcion para manejar conecciones. requiere de un flujo que es un
fn handle_conects(trafico: &mut TcpStream) {
    //aloca memoria en un buffer usando read() para leer los bytes de informacion que llega y maneja nulos con unwrap
    let mut buffer = [0; 1024];

    //lee los bytes de trafico y a punta al mutable de (buffer)
    trafico.read(&mut buffer).unwrap();

    //Ahora se convierten los bytes puros en strings (cadenas)
    let trafic_str = String::from_utf8_lossy(&buffer);

    print!("request body {}", trafic_str);
}
