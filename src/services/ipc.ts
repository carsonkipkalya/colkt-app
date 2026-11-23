import { invoke, type InvokeArgs } from '@tauri-apps/api/core'
import type { CommandContracts } from '../types/commands'
import type { AppError, ErrorCode } from '../types/errors'

const ERROR_CODES: readonly ErrorCode[] = [
  'not_found',
  'invalid_input',
  'conflict',
  'unavailable_offline',
  'engine_error',
  'permission_denied',
  'io_error',
  'locked',
  'unsupported',
  'internal',
]

function isAppError(value: unknown): value is AppError {
  if (typeof value !== 'object' || value === null) return false
  const candidate = value as Record<string, unknown>
  return (
    typeof candidate.code === 'string' &&
    ERROR_CODES.includes(candidate.code as ErrorCode) &&
    typeof candidate.message === 'string' &&
    'details' in candidate
  )
}

function toAppError(error: unknown): AppError {
  if (isAppError(error)) return error
  return {
    code: 'internal',
    message: 'An unexpected error occurred',
    details: null,
  }
}

type CommandName = keyof CommandContracts

/** Commands declared with `args: void` take no arguments. */
type CommandArgs<K extends CommandName> =
  CommandContracts[K]['args'] extends void
    ? []
    : [args: CommandContracts[K]['args']]

/**
 * Central wrapper around Tauri's `invoke`, constrained by `CommandContracts`.
 *
 * The command name, argument shape, and result type are all derived from the
 * contract map: unknown command names, missing or excess arguments, and
 * arbitrary result types are rejected at compile time.
 *
 * The single internal assertion bridges the contract map to Tauri's broad
 * `invoke` signature, which accepts any serialisable argument payload and
 * returns `Promise<unknown>`-style generics. It is safe because callers can
 * only reach it through the constrained signature above.
 */
export async function invokeCommand<K extends CommandName>(
  command: K,
  ...args: CommandArgs<K>
): Promise<CommandContracts[K]['result']> {
  const payload = (args.length === 0 ? undefined : args[0]) as
    | InvokeArgs
    | undefined
  try {
    return await invoke<CommandContracts[K]['result']>(command, payload)
  } catch (error) {
    throw toAppError(error)
  }
}
