import Link from 'next/link';
import { useState, useEffect } from 'react';
import { GetServerSideProps } from "next";
import ContentLoader from 'react-content-loader'
import Rect from 'react-content-loader'

// type Resource = {
//     _id: ObjectId;
//     user_id: string;
//     group_id: string;
//     title: string;
//     description: string;
//     tags: string[];
//     files: File[];
//     last_edited_at: Date;
// }

// type ObjectId = {
//     $oid: string;
// }

// type File = {
//     id: string;
//     title: string;
//     size: string;
// }

type User = {
    id: string;
    username: string;
    email: string;
    groups: string[];
}


export default function UserName(props: {
    userId: String;
}) {
    const [data, setData] = useState<User>()
    const [isLoading, setLoading] = useState(false)

    const NameLoader = () => (
      <ContentLoader viewBox='0 0 275 10'>
        <rect x="3" y="0" rx="3" ry="3" width="50" height="7" />
      </ContentLoader>
    );
  
    useEffect(() => {
      setLoading(true)
      fetch(`https://api.examclutch.com/api/user/username/${props.userId}`, {
        credentials: 'include'
      })
        .then((res) => res.json())
        .then((data) => {
          setData(data)
          setLoading(false)
        })
    }, [])
  
    if (isLoading) return (
        <div className='flex inline-flex'>
          <NameLoader />
        </div>
    )

    if (!data) return (
      <div>
        <h1>@deleted</h1>
      </div>
    )
  
    return (
      <div>
        <h1>@{data.username}</h1>
      </div>
    )
}