# Arquitectura

Prometheus Universe Engine se divide en módulos independientes para evitar acoplar simulación, render, datos y UI.

## Principios

- La simulación no depende del render.
- El render no decide la física.
- Los datos no bloquean el loop principal.
- f64 para simulación global.
- f32 solo para render local.
- Floating Origin desde Fase 0.
