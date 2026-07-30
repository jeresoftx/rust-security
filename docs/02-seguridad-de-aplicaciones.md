# Seguridad de aplicaciones

## Concepto y problema

Toda entrada externa es dato no confiable, aunque llegue de una interfaz propia.
La seguridad de aplicaciones empieza al definir qué formato, tamaño, contexto y
autorización acepta una operación. Rechazar datos fuera de contrato reduce
ambigüedad y evita que una capa interprete algo que otra creyó inofensivo.

## Contrato e invariantes

El modelo local valida identificadores sintéticos con una allowlist: letras
ASCII, dígitos, guion y guion bajo, entre uno y 32 caracteres. No intenta
limpiar ni reparar la entrada; una entrada fuera de contrato se rechaza. La
validación de formato no reemplaza autorización, codificación de salida ni
consultas parametrizadas.

## OWASP y defensas

Las familias OWASP recuerdan fronteras comunes: control de acceso, inyección,
configuración, autenticación, secretos y registro. La defensa se construye por
capas: validación temprana, tipos explícitos, autorización por recurso,
codificación contextual, límites de tamaño, dependencias actualizadas y logs
sin secretos.

## Límites

Este capítulo no crea payloads, no prueba aplicaciones ajenas ni automatiza
solicitudes. Los ejemplos operan sobre texto local para enseñar contratos y
pruebas de regresión defensivas.
