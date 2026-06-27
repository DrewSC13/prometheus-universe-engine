# Architecture Decision Records

## ADR-001: Rust + Bevy 0.17 como base inicial

Se usará Rust como lenguaje principal y Bevy 0.17 como motor base durante las primeras fases.

## ADR-002: Python solo para datos científicos

Python será usado para descarga, limpieza, conversión y preprocesamiento de datos astronómicos.

## ADR-003: Floating Origin desde Fase 0

El sistema implementará Floating Origin desde el inicio para evitar jitter y pérdida de precisión.

## ADR-004: f64 para simulación, f32 para render local

Las posiciones globales usarán f64. El render trabajará con posiciones locales f32 cercanas a cámara.

## ADR-005: No crear entidades Bevy individuales para estrellas masivas

Las estrellas se manejarán mediante buffers GPU, instancing y culling.

## ADR-006: SimulationTime basado en J2000/TDB

La simulación usará días TDB desde J2000 internamente. La UI podrá mostrar UTC.
