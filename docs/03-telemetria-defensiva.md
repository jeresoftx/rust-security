# Telemetría defensiva

## Concepto y problema

Logs, métricas y trazas ayudan a detectar fallas y abuso, pero también pueden
convertirse en una fuga si registran secretos, tokens o contenido sensible. La
telemetría defensiva diseña qué observar, cuánto retener y cómo permitir
investigación sin multiplicar el riesgo de exposición.

## Contrato e invariantes

El modelo del crate opera sobre texto sintético y redacta segmentos con prefijo
`token=`. Conserva la estructura del mensaje para depuración y sustituye el
valor por `[REDACTED]`. No analiza paquetes, no abre archivos ni identifica
secretos reales: es una demostración de una frontera de logging.

## Alternativas y límites

La redacción de texto es una capa final; la mejor defensa es no incluir
secretos en eventos. En producción se combinan esquemas de eventos, controles
de acceso, cifrado en reposo, retención limitada, alertas y respuesta a
incidentes. Un patrón de texto no sustituye clasificación de datos ni revisión
de rutas de observabilidad.
