# Alcance legal y laboratorio

## Concepto y problema

La seguridad defensiva estudia cómo reducir daño, detectar abuso y recuperar un
sistema. Ese aprendizaje pierde su propósito cuando se aplica a objetivos sin
autorización. Un laboratorio no es una nota al pie: delimita datos, sistemas,
personas y consecuencias para que la práctica sea ética y repetible.

## Contrato de laboratorio

Este curso usa únicamente datos sintéticos, fixtures locales y servicios
creados para el ejercicio. Antes de una práctica, se identifica propietario,
permiso explícito, alcance técnico, ventana de trabajo y mecanismo de parada.
No se escanean dominios, direcciones, cuentas ni repositorios ajenos.

Los modelos del crate clasifican riesgos, validan texto, redactan secretos y
evalúan configuraciones ficticias. No generan payloads, no evaden controles, no
elevan privilegios ni automatizan interacción con redes.

## Modelo de amenaza responsable

Una amenaza se formula como activo, frontera, capacidad del atacante, impacto y
defensa. El objetivo es conocer qué control reduce riesgo y cómo verificarlo,
no demostrar acceso. Si una práctica muestra una debilidad, se documenta una
mitigación, una prueba de regresión y un plan de comunicación responsable.

## Lista de verificación

- [x] El objetivo es propio o tiene autorización verificable.
- [x] Las entradas son sintéticas y no incluyen secretos reales.
- [x] La práctica tiene alcance, parada y registro de resultados.
- [x] Cada técnica se acompaña de su defensa y límite.
