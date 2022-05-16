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

type Group = {
    id: string;
    name: string;
    description: string;
    discord_link: string;
    members: string[];
    administrators: string[];
}


export default function UserName(props: {
    groupId: String;
}) {
    const [data, setData] = useState<Group>()
    const [isLoading, setLoading] = useState(false)
  
    useEffect(() => {
      setLoading(true)
      fetch(`https://api.examclutch.com/api/group/name/${props.groupId}`, {
        credentials: 'include'
      })
        .then((res) => res.json())
        .then((data) => {
          setData(data)
          setLoading(false)
        })
    }, [])
  
    if (isLoading) return (
        <div className="font-bold">
          <h1>Loading...</h1>
        </div>
    )

    if (!data) return (
      <div className="font-bold">
        <h1>Deleted</h1>
      </div>
    )
  
    return (
      <div className="font-bold text-center" style={{fontFamily: "Roboto Mono"}}>
        <h1>{data.name}</h1>
      </div>
    )
}