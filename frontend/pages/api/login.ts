// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'

type Data = {
  message: string
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  res.status(301).setHeader("Location", "https://discord.com/api/oauth2/authorize?client_id=917954795384500236&redirect_uri=https%3A%2F%2Fapi.examclutch.com%2Fapi%2Fauth%2Foauth2%2Fdiscord%2Fredirect&response_type=code&scope=identify%20email%20guilds").json({ message: 'Discord Login' })
}
