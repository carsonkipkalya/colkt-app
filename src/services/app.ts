import { invokeCommand } from './ipc'
import type { PingResponse } from '../types/commands'

export function ping(): Promise<PingResponse> {
  return invokeCommand('ping')
}
