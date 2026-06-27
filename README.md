# Prometheus Universe Engine

Prometheus Universe Engine es un motor de simulación astronómica 3D interactivo, escalable y modular.

## Objetivo inicial

Construir primero una simulación estable Sol-Tierra-Luna antes de escalar hacia el Sistema Solar completo, la Vía Láctea, galaxias externas y un universo procedural.

## Stack principal

- Rust
- Bevy
- wgpu / WGSL
- Python para datos científicos
- Blender para assets puntuales
- Git + GitHub por terminal

## Fase actual

Fase 0: Prototipos críticos.

## Prioridades de Fase 0

- Escena Bevy base.
- Cámara libre.
- SimulationTime.
- Floating Origin.
- Coordenadas globales y locales.
- Prueba Tierra-Luna sin jitter.
- Render de puntos.
- Benchmark de FPS.
- Streaming inicial con mmap.

## Verificación local

Ejecutar:

./scripts/verify_repo.sh
