import { FaDiscord } from "react-icons/fa";
import Link from "next/link";
import { useState } from 'react';


type GroupUser = {
    id: string;
    username: string;
}

type Group = {
    id: string;
    name: string;
    description: string;
    discord_link: string;
    members: string[];
    administrators: string[];
}

export default function GroupTitle(props: {
    propGroup: Group;
    isAdmin: boolean;
}) {
    
    const groupTitle = props.propGroup.name;
    const groupDescription = props.propGroup.description;
    const discordLink = props.propGroup.discord_link;

    return(
        
        <div>
        <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", minWidth: "632.5px"}}>
        
            <div className="inline-block">
                <h1 className="font-bold text-2xl text-exclpurple inline-block">{ groupTitle }</h1>
                {/* <h1>{ groupDescription }</h1> */}
            </div>
            
            
            
            {
                    props.isAdmin ?
                    <Link href={`/app/group/${props.propGroup.id}/manage`}>
                    <a className="float-right hover:text-exclpurple duration-150 inline-block pt-2 flex items-center">
                    <h1>Manage</h1> 
                    </a>
                    </Link>
                    : null
            }
           
            
        
            {/* <a target="_blank" href={ discordLink } rel="noopener noreferrer" className="float-right hover:text-exclpurple duration-150 inline-block pt-2 flex items-center">
                <FaDiscord className="pr-2" size="25px"/>
                <h1>
                    Discord
                </h1>
            </a> */}

        </div>
        </div>
    )
}