# Runner protocol v1

## Envelope

Campos: `protocolVersion`, `messageId`, `correlationId`, `sessionId`, `kind`, `name`, `payload`.

## State machine

`created → authenticated → manifestAccepted → preflighted → planned → dryRunPassed → confirmed → executing → verifying → succeeded|failed|cancelled`.

## Comandos

`runner.hello`, `session.create`, `manifest.submit`, `preflight.start`, `inventory.read`, `plan.build`, `plan.validate`, `dryRun.start`, `confirmation.submit`, `execution.start`, `execution.cancel`, `journal.export`, `session.close`.

## Seguridad

- Compatibilidad negociada antes de aceptar manifest.
- Nonce, identidad peer y expiración.
- Plan hash y manifest hash ligados a confirmación.
- Allowlist de operaciones compiladas.
- Límites y timeouts por mensaje.
- Journal append-only redactado.
- Cancelación solo en safe points.
