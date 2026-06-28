# Simulación Sol-Tierra-Luna

## Objetivo

Crear la primera simulación astronómica mínima del motor.

## Enfoque inicial

La primera versión usa órbitas circulares analíticas para validar:

- Separación entre simulación y render.
- Avance temporal.
- Retroceso temporal.
- Posiciones globales en f64.
- Integración futura con Floating Origin.

## Cuerpos iniciales

- Sol.
- Tierra.
- Luna.

## Estado

Implementado como lógica matemática pura en:

engine/src/simulation/solar_system.rs

## Próximo paso

Conectar estas posiciones al render Bevy usando entidades visuales simples.
