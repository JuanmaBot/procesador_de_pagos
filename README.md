# procesador_de_pagos
Este repositorio se creo para poder llevar a cabo el desafío de PrexCORE Challenge 2024 y desarrollar un mini procesador de pagos en RUST utilizando una Rest API concurrente.

# desafio
El challenge consiste en realizar un “mini procesador de pagos” con la
capacidad de llevar el saldo de los clientes en memoria y persistirlos (cuando
se solicite) en un archivo.
Se deberá implementar un micro servicio que exponga una API REST al usuario*, a
través de la cual pueda llevar un registro del saldo de sus clientes.
* Al referirnos a un procesador de pagos, el usuario del mismo es un emisor de tarjetas
y/o servicios de pago, no así el cliente final.

# requerimientos
El servicio recibirá instrucciones a través de su API REST para acreditar o debitar
saldo a los clientes. Cada cliente deberá ser creado inicialmente mediante el servicio
"new_client" y luego podrá recibir tanto débitos (resta al saldo) como créditos (suma al
saldo).
Se podrá consultar el saldo e información de los clientes mediante el servicio
“client_balance”.
El saldo y toda la base de clientes deberá poder persistirse en un archivo mediante
el servicio "store_balances". El servicio deberá implementar los siguientes
endpoints:
* POST "new_client"
* POST “new_credit_transaction”
* POST “new_debit_transaction”
* POST “store_balances”
* GET “client_balance”

# recomendaciones
Se describen a continuación recomendaciones generales alineadas con la forma de
trabajo interna de PrexCORE:
* Utilizar Actix Web para la realización del web server.
* Utilizar formato Decimal para los saldos de clientes.
* Utilizar asincronismo mediante runtime de Tokio.
* Utilizar Serde para serialización y deserialización de payloads.
* Evitar la repetición de código mediante el uso de funciones y/o macros.
* Buenas prácticas de diseño: KISS, DRY y SOLID.

# modo_de_uso ToDo