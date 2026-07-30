# Plan de implementación de Rust Security

**Estado:** draft

## Objetivo

Entregar un curso y crate educativo de seguridad defensiva en Rust. Enseña a
identificar amenazas, reducir superficie, validar datos, analizar telemetría y
endurecer sistemas en entornos autorizados. No enseña explotación reutilizable
ni operación sobre objetivos reales.

## Alcance y límites

El curso cubre marco legal y ético, threat modeling, OWASP, validación de
entradas, análisis de tráfico sintético, vulnerabilidades de memoria desde la
defensa, hardening, gestión de secretos e incident response. Los modelos usan
datos locales y seguros: clasificación de riesgos, validación de entrada,
redacción de telemetría y evaluación de configuración.

No se usa `unsafe`, acceso a red, escaneo, payloads, evasión, escalación de
privilegios, explotación ni dependencias externas sin autorización.

## Fases

1. Fundación: alcance legal, ética y contrato de laboratorio.
2. Amenazas: activos, fronteras, STRIDE y priorización de riesgos. [x]
3. Aplicación: validación, OWASP y tratamiento de datos no confiables. [x]
4. Telemetría: análisis sintético, secretos y observabilidad defensiva. [x]
5. Hardening: configuraciones, mínimos privilegios y respuesta a incidentes. [x]
6. Cierre: glosario, referencias, auditoría y estado `draft`.

## Ruta crítica

Fundación → amenazas → aplicación → telemetría → hardening → cierre. Cada
fase se divide en especificación, modelo probado y capítulo.

## Criterio de cierre

El curso queda completo como `draft` cuando cada unidad tenga concepto,
problema, defensas, Mermaid, ejemplos seguros, ejercicios, soluciones,
límites, referencias y trazabilidad GitHub; no haya pendientes ni milestones
abiertos.
