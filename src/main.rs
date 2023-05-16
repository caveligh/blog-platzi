use serde::{Serialize};
use std::fs::File;
use std::io::Write;

#[derive(Serialize)]
struct FieldInfo {
    nombre: String,
    tipo_de_dato: String,
    posicion_inicial: usize,
    posicion_final: usize,
    descripcion: String,
    obligatorio: String,
}

#[derive(Serialize)]
struct TableInfo {
    tabla: Table,
}

#[derive(Serialize)]
struct Table {
    nombre: String,
    campos: Vec<FieldInfo>,
}

fn main() {
    let mut posicion_actual = 0;
    let mut campos: Vec<FieldInfo> = Vec::new();

    // Campo: id_vendedor
    let id_vendedor = FieldInfo {
        nombre: "id_vendedor".to_string(),
        tipo_de_dato: "NUMBER(10)".to_string(),
        posicion_inicial: posicion_actual,
        posicion_final: posicion_actual + 10 - 1,
        descripcion: "Identificador único para cada vendedor (clave primaria)".to_string(),
        obligatorio: "Sí".to_string(),
    };
    posicion_actual += 10;
    campos.push(id_vendedor);

    // Campo: nombre
    let nombre = FieldInfo {
        nombre: "nombre".to_string(),
        tipo_de_dato: "VARCHAR2(50)".to_string(),
        posicion_inicial: posicion_actual,
        posicion_final: posicion_actual + 50 - 1,
        descripcion: "Nombre del vendedor".to_string(),
        obligatorio: "Sí".to_string(),
    };
    posicion_actual += 50;
    campos.push(nombre);

    // Campo: apellido
    let apellido = FieldInfo {
        nombre: "apellido".to_string(),
        tipo_de_dato: "VARCHAR2(50)".to_string(),
        posicion_inicial: posicion_actual,
        posicion_final: posicion_actual + 50 - 1,
        descripcion: "Apellido del vendedor".to_string(),
        obligatorio: "Sí".to_string(),
    };
    posicion_actual += 50;
    campos.push(apellido);

    // Campo: correo_electronico
    let correo_electronico = FieldInfo {
        nombre: "correo_electronico".to_string(),
        tipo_de_dato: "VARCHAR2(100)".to_string(),
        posicion_inicial: posicion_actual,
        posicion_final: posicion_actual + 100 - 1,
        descripcion: "Dirección de correo electrónico del vendedor".to_string(),
        obligatorio: "No".to_string(),
    };
    posicion_actual += 100;
    campos.push(correo_electronico);

    // Campo: telefono
    let telefono = FieldInfo {
        nombre: "telefono".to_string(),
        tipo_de_dato: "VARCHAR2(20)".to_string(),
        posicion_inicial: posicion_actual,
        posicion_final: posicion_actual + 20 - 1,
        descripcion: "Número de teléfono del vendedor".to_string(),
        obligatorio: "No".to_string(),
    };
    posicion_actual += 20;
    campos.push(telefono);

    // Campo: fecha_ingreso
    let fecha_ingreso = FieldInfo {
        nombre: "fecha_ingreso".to_string(),
        tipo_de_dato: "DATE".to_string(),
        posicion_inicial: posicion_actual,
        posicion_final: posicion_actual + 10 - 1,
        descripcion: "Fecha en que el vendedor se unió a la empresa".to_string(),
        obligatorio: "No".to_string(),
    };
    campos.push(fecha_ingreso);

    // Crear la estructura JSON
    let tabla = Table {
        nombre: "vendedores".to_string(),
        campos,
    };

    let table_info = TableInfo {
        tabla,
    };

    // Convertir la estructura a JSON
    let json_data = serde_json::to_string_pretty(&table_info).unwrap();

    // Guardar el archivo JSON
    let mut file = File::create("tabla_vendedores.json").expect("Error al crear el archivo JSON");
    file.write_all(json_data.as_bytes()).expect("Error al escribir en el archivo JSON");
    println!("Archivo JSON generado correctamente.");
}
