// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'

type Data = {
  message: string
}

export default function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {
  res.status(301)
    .setHeader("Location", "https://examclutch.com/")
    .setHeader("Set-Cookie", "auth_token=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT")
    .setHeader("Set-Cookie", "user_id=deleted; path=/; expires=Thu, 01 Jan 1970 00:00:00 GMT")
    .json({ message: 'Logged Out' })
}
