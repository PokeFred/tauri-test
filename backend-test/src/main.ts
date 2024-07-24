import express from "express"
import type { Express, Request, Response, NextFunction } from "express"
import { json as jsonParser } from "body-parser"
import Cookies from "cookies"

const app: Express = express()
app.use(jsonParser())
app.use(async (req: Request, res: Response, next: NextFunction): Promise<void> => {
    next()
})

app.post("/", async (req: Request, res: Response): Promise<void> => {
    const cookies: Cookies = new Cookies(req, res)

    console.log("[server] received request")
    console.log(req.body)

    setTimeout(async (): Promise<void> => {
        res
            .writeHead(200, { "Content-Type": "application/json" })
            .end(JSON.stringify({ message: "Hello, World!" }))
    }, 1000)
})

app.listen(3000, "127.0.0.1", (): void => {
    console.log("[server] running on http://127.0.0.1:3000")
})
