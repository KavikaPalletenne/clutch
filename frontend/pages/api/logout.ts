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
    .setHeader("Cookie", "auth_token=; path=/")
    .setHeader("Cookie", "user_id=; path=/")
    .json({ message: 'Discord Login' })
}
