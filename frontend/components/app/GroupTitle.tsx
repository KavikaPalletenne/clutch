import { FaDiscord } from "react-icons/fa";
import Link from "next/link";
import { useState } from 'react';

export default function GroupTitle() {
    
    const [groupTitle, setGroupTitle] = useState("MGS Year 11");
    const [groupDescription, setGroupDescription] = useState("Melbourne Grammar School Yr 11 study group");
    const [discordLink, setDiscordLink] = useState("https://discord.gg/V38R3ByGQb");

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