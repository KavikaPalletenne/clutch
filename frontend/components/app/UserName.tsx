import Link from 'next/link';
import { useState, useEffect } from 'react';
import { GetServerSideProps } from "next";

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
  
    useEffect(() => {
      setLoading(true)
      fetch(`http://localhost:443/api/user/${props.userId}`)
        .then((res) => res.json())
        .then((data) => {
          setData(data)
          setLoading(false)
        })
    }, [])
  
    if (isLoading) return (
        <div>
          <h1>@loading</h1>
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