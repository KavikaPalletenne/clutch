// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'

type Data = {
  name: string
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  res.status(301).setHeader("Location", "https://discord.com/api/oauth2/authorize?client_id=917954795384500236&redirect_uri=http%3A%2F%2F127.0.0.1%3A443%2Fapi%2Foauth2%2Fredirect&response_type=code&scope=identify%20email%20guilds").json({ message: 'Discord Login' })
}
