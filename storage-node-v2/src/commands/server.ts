import { flags } from '@oclif/command'
import { createApp } from '../services/webApi/app'
import ApiCommandBase from '../command-base/ApiCommandBase'

// TODO: fix command not found error (error handling)
// TODO: custom IP address?

export default class Server extends ApiCommandBase {
  static description = 'Starts the storage node server.'

  static flags = {
    worker: flags.integer({
      char: 'w',
      required: true, // TODO: for dev
      description: 'Storage provider worker ID',
    }),
    uploads: flags.string({
      char: 'u',
      required: true,
      description: 'Data uploading directory.',
    }),
    port: flags.integer({
      char: 'p',
      required: true,
      description: 'Server port.',
    }),
    ...ApiCommandBase.keyflags,
  }

  static args = [{ name: 'file' }]

  async run(): Promise<void> {
    const { flags } = this.parse(Server)

    if (flags.dev) {
      await this.ensureDevelopmentChain()
    }

    const account = this.getAccount(flags)
    const api = await this.getApi()

    try {
      const port = flags.port
      const workerId = flags.worker ?? 0 // TODO: don't require on dev???
      const app = await createApp(api, account, workerId, flags.uploads)
      console.info(`Listening on http://localhost:${port}`)
      app.listen(port)
    } catch (err) {
      console.error(`Error: ${err}`)
    }
  }

  // Override exiting.
  /* eslint-disable @typescript-eslint/no-empty-function */
  async finally(): Promise<void> {}
}
