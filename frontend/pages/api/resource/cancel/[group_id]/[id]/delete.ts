// Next.js API route support: https://nextjs.org/docs/api-routes/introduction
import type { NextApiRequest, NextApiResponse } from 'next'

type Data = {
  message: string
}

export default async function handler(
  req: NextApiRequest,
  res: NextApiResponse<Data>
) {

    let group_id = req.query
    let resource_id = req.query

    let response = await fetch(`https://api.examclutch.com/api/resource/delete/${resource_id}`, {
        method: "GET",
        headers: req.headers.cookie ? {"Cookie": req.headers.cookie} : {}
    })

    if (response.status == 200) {
    res.status(301)
        .setHeader("Location", `https://examclutch.com/app/${group_id}`)
        .json({ message: 'Deleted resource' })
    }

    res.status(301)
        .setHeader("Location", `https://examclutch.com/`)
        .json({ message: 'Could not delete resource' })
}
