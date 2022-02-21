import Head from 'next/head';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import Cookies from 'js-cookie';

export default function App() {

    const router = useRouter()
    const [groups, setGroups] = useState([])
    const [userId, setUserId] = useState(Cookies.get('user_id'))

    useEffect(() => {

        fetch(`http://localhost:443/api/user/get_user_groups/${userId}`)
        .then((res) => res.json())
        .then((data) => {
            setGroups(data)
        })

        router.push(`/app/group/${groups[0]}`)
    })


    return(
        <div className="bg-bg-gray-50">
            <Head>
                <title>Dashboard - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
        <       link rel="icon" href="/favicon.png" />
            </Head>

            <div className="flex items-start justify-center pt-10">
                
            </div>

            <div className="flex inline-grid">
                
            </div>

        </div>
    )
}