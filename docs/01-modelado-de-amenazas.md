# Modelado de amenazas

## Concepto y problema

Un modelo de amenazas convierte una preocupación vaga en decisiones
verificables: qué activo importa, dónde cruza una frontera, qué capacidad se
asume para un atacante y qué control reduce el riesgo. Sin ese modelo, la
seguridad se vuelve una lista de herramientas sin justificación ni prioridad.

STRIDE ofrece un lenguaje para clasificar amenazas: suplantación, alteración,
repudio, divulgación, denegación de servicio y elevación de privilegios. No es
un escáner ni una prueba de vulnerabilidad; ayuda a formular preguntas sobre un
diseño autorizado.

## Contrato e invariantes

El modelo del crate recibe una severidad de impacto y una probabilidad
documentada, ambas de uno a cinco. Devuelve una prioridad reproducible y no
observa redes, archivos ni servicios. Un riesgo tiene contexto humano: el
número orienta una conversación, no decide por sí solo qué aceptar.

## Alternativas y decisión

Un equipo puede usar STRIDE, PASTA, árboles de ataque o una matriz propia. Para
un curso inicial, una matriz impacto-probabilidad es transparente y se prueba
fácilmente. La decisión se revisa cuando cambia el activo, la arquitectura o
la evidencia de abuso.

## Límites

Clasificar un riesgo no demuestra que exista una vulnerabilidad ni autoriza
probar un sistema. Una revisión defensiva valida controles con pruebas locales,
logs sintéticos y los permisos definidos en el contrato de laboratorio.
