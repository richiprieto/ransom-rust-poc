use rand::{thread_rng, Rng};
use std::fs;
use std::io::Write;

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
    // Generamos un vector de tamaño 32, que contiene 32 campos
    // con un valor u8 cada uno
    let clave: [u8; 32] = thread_rng().gen();
    clave_file
        .write_all(&clave)
        .expect("No se pudo grabar la clave en el archivo, revise permisos");
}
