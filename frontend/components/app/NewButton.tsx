import { FaDiscord } from "react-icons/fa";
import { AiOutlinePlus } from "react-icons/ai"
import Link from "next/link";
import { useState } from 'react';

type Group = {
    _id: string;
    name: string;
    description: string;
    discord_link: string;
    members: string[];
    administrators: string[];
}

export default function NewButton(props: {
    groupId: string;
    }) {
    
    return(  
        <Link href={`/app/group/${props.groupId}/new`}>  
        <a>    
        <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-exclpurple hover:shadow-lg duration-150" style={{fontFamily: "Roboto Mono", fontWeight: "bold"}}>
            <div className="float-right inline-block text-2xl text-white flex items-center">
                <h1>
                    New
                </h1>
                <div className="pl-1 pt-1">
                    <AiOutlinePlus />
                </div>
            </div>
        </div>
        </a>
        </Link>
    )
}