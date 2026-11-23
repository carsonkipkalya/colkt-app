import { beforeEach, describe, expect, expectTypeOf, it, vi } from 'vitest'
import { invoke } from '@tauri-apps/api/core'
import { invokeCommand } from './ipc'
import { ping } from './app'
import type { AppError } from '../types/errors'

vi.mock('@tauri-apps/api/core', () => ({
  invoke: vi.fn(),
}))

const mockedInvoke = vi.mocked(invoke)

describe('invokeCommand', () => {
  beforeEach(() => {
    mockedInvoke.mockReset()
  })

  it('returns the typed command result on success', async () => {
    mockedInvoke.mockResolvedValue({ ok: true })

    const result = await invokeCommand('ping')

    expect(mockedInvoke).toHaveBeenCalledWith('ping', undefined)
    expect(result).toEqual({ ok: true })
    expect(result.ok).toBe(true)
  })

  it('rethrows a structured AppError unchanged', async () => {
    const backendError: AppError = {
      code: 'not_found',
      message: 'Media item not found',
      details: null,
    }
    mockedInvoke.mockRejectedValue(backendError)

    const caught = await invokeCommand('ping').catch((error: unknown) => error)

    expect(caught).toEqual(backendError)
    expect((caught as AppError).code).toBe('not_found')
  })

  it('normalises non-AppError rejections to a safe internal error', async () => {
    mockedInvoke.mockRejectedValue(new Error('raw internals'))

    const caught = await invokeCommand('ping').catch((error: unknown) => error)

    expect(caught).toEqual({
      code: 'internal',
      message: 'An unexpected error occurred',
      details: null,
    })
  })

  it('rejects malformed AppError-shaped payloads', async () => {
    mockedInvoke.mockRejectedValue({ code: 'bogus', message: 42 })

    const caught = await invokeCommand('ping').catch((error: unknown) => error)

    expect((caught as AppError).code).toBe('internal')
  })
})

describe('command contract types', () => {
  it('infers the literal ping result type', async () => {
    mockedInvoke.mockResolvedValue({ ok: true })

    const result = await ping()

    expectTypeOf(result).toEqualTypeOf<{ ok: true }>()
    expectTypeOf(result.ok).toEqualTypeOf<true>()
  })

  it('rejects invalid usage at compile time', () => {
    // These closures are never invoked; they exist so TypeScript checks the
    // calls below. Each @ts-expect-error must suppress a real type error —
    // if the contract weakens, tsc reports an unused directive and the build
    // fails.
    const invalidName = () =>
      // @ts-expect-error unknown command names are not in the contract map
      invokeCommand('does_not_exist')
    const invalidArgs = () =>
      // @ts-expect-error ping is declared with void args and takes none
      invokeCommand('ping', { unexpected: true })
    const arbitraryResult = () =>
      // @ts-expect-error invokeCommand accepts only its contract key, not a
      // caller-chosen result type parameter
      invokeCommand<'ping', { ok: boolean }>('ping')

    expect(typeof invalidName).toBe('function')
    expect(typeof invalidArgs).toBe('function')
    expect(typeof arbitraryResult).toBe('function')
  })
})

describe('app service', () => {
  beforeEach(() => {
    mockedInvoke.mockReset()
  })

  it('ping invokes the registered command name', async () => {
    mockedInvoke.mockResolvedValue({ ok: true })

    const result = await ping()

    expect(mockedInvoke).toHaveBeenCalledWith('ping', undefined)
    expect(result.ok).toBe(true)
  })
})
