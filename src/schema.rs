// @generated automatically by Diesel CLI.

diesel::table! {
    tipo_viviendas (identificacion) {
        identificacion -> Text,
        calle -> Text,
        numero -> Integer,
        piso -> Text,
        codigo_postal -> Text,
        metros_cuadrados -> Integer,
        numero_aseos -> Integer,
        numero_habitaciones -> Integer,
        tipo -> Text,
    }
}
