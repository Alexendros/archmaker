# ADR-0001: Rust es autoridad semántica

- Estado: proposed
- Fecha: 2026-10-05

## Decisión

Dominio, reglas, resolución, migración, validación, manifest y exportación se implementan una vez en Rust. React consume DTO mediante `CorePort`; WASM y Tauri adaptan la misma fachada.

## Consecuencias

Se evita divergencia entre browser y desktop; se asume coste de bindings, serialización y pruebas de paridad.
