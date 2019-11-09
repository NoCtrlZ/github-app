const SmeeClient = require('smee-client')

const smee = new SmeeClient({
  source: 'https://smee.io/T4wO9nyZ5WNsRBJt',
  target: 'http://localhost:3000/events',
  logger: console
})

const events = smee.start()

// Stop forwarding events
// events.close()