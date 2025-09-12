import { Hono } from 'hono'
import { serve, type HttpBindings } from '@hono/node-server'

const app = new Hono<{Bindings: HttpBindings}>

app.all('/announce', (c) => {
    let ip = c.env.incoming.socket.remoteAddress
    console.log(ip)
    return c.text("", 200)
})
serve(app)