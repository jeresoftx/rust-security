# Hardening e incidentes

## Concepto y problema

Hardening reduce superficie de ataque al elegir configuraciones explícitas,
privilegios mínimos, secretos fuera del código y actualizaciones controladas.
No es una lista estática: el control depende del activo, entorno y capacidad
operativa para detectar y recuperar.

## Contrato e invariantes

El modelo del crate evalúa una configuración sintética: exige TLS habilitado,
modo de depuración deshabilitado y acceso administrativo restringido. No cambia
configuraciones, no ejecuta comandos y no inspecciona una máquina real. El
resultado explica qué control falta para dirigir una revisión humana.

## Respuesta a incidentes

Preparar respuesta significa tener propietario, canal de comunicación, fuentes
de evidencia, criterio de contención y recuperación verificable. Ante un
evento, preservar evidencia y limitar impacto tiene prioridad sobre improvisar
acciones irreversibles. La lección del curso es diseñar ese proceso, no operar
sistemas ajenos.

## Límites

Los ejemplos no enumeran servicios, no deshabilitan controles ni elevan
privilegios. Toda evaluación se limita a estructuras locales y escenarios de
laboratorio autorizados.
