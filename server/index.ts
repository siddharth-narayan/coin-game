import { Hono } from 'hono'
import { serve } from '@hono/node-server'
import { z } from "zod"
const app = new Hono()

let announceSchema = z.object({
    ip: z.ipv4().or(z.ipv6()),
    port: z.int().positive().lt(65536)
})

let peers: z.infer<typeof announceSchema>[] = []

app.post('/announce', async (c) => {
    let json = await c.req.json()
    
    let res = z.parse(announceSchema, json)
    console.log(res)

    peers.push(res)
    return c.json({peers: peers}, 200)
})

serve(app)