/**
 * Central frontend/backend command contract.
 *
 * Every Tauri command exposed to the frontend must be declared here, mapping
 * the command name to its argument and result types. `src/services/ipc.ts`
 * derives its public API from this map, so callers cannot select arbitrary
 * argument or result types and unknown command names fail type-checking.
 *
 * Keep payload types consistent with `docs/API.md` and the Rust command
 * signatures.
 */
export type PingResponse = {
  ok: true
}

export interface CommandContracts {
  ping: {
    args: void
    result: PingResponse
  }
}
