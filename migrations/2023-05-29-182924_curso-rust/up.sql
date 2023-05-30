-- Your SQL goes here
CREATE TABLE tipo_viviendas (
    identificacion VARCHAR NOT NULL PRIMARY KEY,
    calle VARCHAR NOT NULL,
    numero INTEGER NOT NULL,
    piso VARCHAR NOT NULL,
    codigo_postal VARCHAR NOT NULL,
    metros_cuadrados INTEGER NOT NULL,
    numero_aseos INTEGER NOT NULL,
    numero_habitaciones INTEGER NOT NULL,
    tipo VARCHAR NOT NULL
);