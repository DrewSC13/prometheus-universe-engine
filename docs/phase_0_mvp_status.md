# Phase 0 MVP Status

## Estado general

La Fase 0 del Prometheus Universe Engine ya cuenta con una demo funcional Sol-Tierra-Luna.

## Funcionalidades implementadas

- Repositorio profesional con Git y GitHub.
- Commits y merges firmados.
- Workspace Rust.
- Bevy como base inicial.
- Cámara libre.
- Sistema de tiempo SimulationTime.
- Controles de tiempo por teclado.
- Coordenadas globales en f64.
- Posiciones locales para render.
- Floating Origin runtime.
- Simulación analítica Sol-Tierra-Luna.
- Render educativo Sol-Tierra-Luna.
- Órbitas visuales.
- Etiquetas básicas.
- HUD runtime.
- Tests unitarios.

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
- R: reiniciar a J2000 con velocidad x50.000.

## Validación actual

- cargo fmt --all.
- cargo check --workspace.
- cargo test --workspace.
- 19 tests pasando.

## Estado visual

La escena muestra:

- Sol.
- Tierra.
- Luna.
- Órbita visual de la Tierra.
- Órbita visual de la Luna.
- HUD con información temporal.

## Próxima fase sugerida

Fase 1: MVP Sistema Solar ampliado.

Objetivos sugeridos:

- Añadir Mercurio.
- Añadir Venus.
- Añadir Marte.
- Añadir Júpiter.
- Añadir Saturno.
- Añadir Urano.
- Añadir Neptuno.
- Crear una estructura genérica para cuerpos celestes.
- Separar datos de cuerpos en configuración.
- Mejorar etiquetas.
- Añadir panel UI.
- Añadir screenshots automatizadas.
