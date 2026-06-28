# Prometheus Universe Engine

Prometheus Universe Engine es un motor de simulación astronómica 3D interactivo, escalable y modular.

## Estado actual

Fase 0 MVP funcional.

La demo actual muestra una escena educativa Sol-Tierra-Luna con:

- Cámara libre.
- Simulación temporal.
- Controles de tiempo.
- Floating Origin runtime.
- Órbitas visuales.
- Etiquetas básicas.
- HUD en pantalla.
- Tests unitarios.

## Stack principal

- Rust.
- Bevy.
- wgpu / Vulkan.
- Python para futuros datos científicos.
- Git + GitHub por terminal.

## Ejecutar

    cargo run -p prometheus_engine

## Verificar

    cargo fmt --all
    cargo check --workspace
    cargo test --workspace

## Controles

### Cámara

- W: avanzar.
- S: retroceder.
- A: izquierda.
- D: derecha.
- Q: bajar.
- E: subir.
- Click derecho + mouse: mirar.
- Shift: acelerar.
- Ctrl: reducir velocidad.

### Tiempo

- Space: pausar o reanudar.
- 1: velocidad x1.
- 2: velocidad x100.
- 3: velocidad x1.000.
- 4: velocidad x10.000.
- 5: velocidad x50.000.
- 6: velocidad x1.000.000.
- B: invertir dirección del tiempo.
- R: reiniciar a J2000.

## Arquitectura inicial

- engine/src/time: SimulationTime y controles temporales.
- engine/src/camera: cámara libre.
- engine/src/coordinates: posiciones globales/locales.
- engine/src/floating_origin: Floating Origin runtime.
- engine/src/simulation: simulación astronómica analítica.
- engine/src/render: render Sol-Tierra-Luna.
- engine/src/ui: HUD runtime.

## Roadmap inmediato

- Cierre de Fase 0.
- Tag de versión.
- Inicio Fase 1: Sistema Solar básico.
