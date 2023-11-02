import { color, bold } from 'console-log-colors'

export class Logger {

  _title: string
  _context: string
  _silent: boolean

  constructor(context: string, silent = false) {
    this._title = 'logger'
    this._context = context
    this._silent = silent
  }

  info (message: string, data?: unknown) {
    if (this._silent) return

    const sTitle = color.blue(this._title)
    const sContext = color.cyanBright(bold(this._context))

    if (typeof data === 'string') {
      console.log(`${sTitle} - ${sContext} - ${message} ${color.cyan(data)}`)

    } else if (typeof data === 'number' || typeof data === 'boolean') {
      console.log(`${sTitle} - ${sContext} - ${message} ${color.yellow(data)}`)

    } else {
      console.log(`${sTitle} - ${sContext} - ${message}`)
      if (data) console.log(data)
    }
  }

  warn (message: string, data?: unknown) {
    if (this._silent) return

    const sTitle = color.blue(this._title)
    const sContext = color.yellow(bold(this._context))

    if (typeof data === 'string') {
      console.log(`${sTitle} - ${sContext} - ${message} ${color.cyan(data)}`)

    } else if (typeof data === 'number' || typeof data === 'boolean') {
      console.log(`${sTitle} - ${sContext} - ${message} ${color.yellow(data)}`)

    } else {
      console.log(`${sTitle} - ${sContext} - ${message}`)
      if (data) console.log(data)
    }
  }

  error (message: string) {
    const sTitle = color.blue(this._title)
    const sContext = color.red(bold(this._context))
    console.log(`${sTitle} - ${sContext} - ${color.red(message)}`)
  }
}