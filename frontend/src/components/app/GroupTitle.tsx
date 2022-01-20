import { FaDiscord } from "react-icons/fa";
import Link from "next/link";
import { useState } from 'react';


type GroupUser = {
    id: string;
    username: string;
}

type Group = {
    _id: string;
    name: string;
    description: string;
    discord_link: string;
    members: GroupUser[];
    administrators: GroupUser[];
}

export default function GroupTitle(props: {
    propGroup: Group;
    }) {
    
    const groupTitle = props.propGroup.name;
    const groupDescription = props.propGroup.description;
    const discordLink = props.propGroup.discord_link;

    return(
        
        <div className="px-4">
        <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white hover:shadow-lg duration-150" style={{fontFamily: "Roboto Mono", minWidth: "750px"}}>
        
            <div className="inline-block">
                <h1 className="font-bold text-2xl pb-2 text-exclpurple inline-block">{ groupTitle }</h1>
                <h1>{ groupDescription }</h1>
            </div>
            
            <a target="_blank" href={ discordLink } rel="noopener noreferrer" className="float-right inline-block pt-6 flex items-center">
                <FaDiscord className="pr-2" size="25px"/>
                <h1>
                    Discord
                </h1>
            </a>

        </div>
        </div>
    )
}