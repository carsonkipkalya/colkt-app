export type ErrorCode =
  | 'not_found'
  | 'invalid_input'
  | 'conflict'
  | 'unavailable_offline'
  | 'engine_error'
  | 'permission_denied'
  | 'io_error'
  | 'locked'
  | 'unsupported'
  | 'internal'

export type AppError = {
  code: ErrorCode
  message: string
  details: unknown | null
}
