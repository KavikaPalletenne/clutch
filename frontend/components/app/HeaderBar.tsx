import { Resource } from "@pages/app/group/[id]";
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
    members: string[];
    administrators: string[];
}

export default function HeaderBar(props: {
    groupId: string;
    }) {
    
    const [searchTerm, setSearchTerm] = useState('')


    const searchTermUpdate = async (e: React.ChangeEvent<any>) => {
        e.preventDefault()
        
        setSearchTerm(e.target.value)

        let results = await fetch(`http://api.scantag.com/api/search/${props.groupId}/${e.target.value}`, {
            method: 'GET',
            credentials: 'include'
        }).then(r => r.json().then(function(data) {
            return data as Resource[]
        }))

        console.log(results)
    }



    return(
        // Header Bar
        <div>
        <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", minWidth: "750px", backgroundImage: "linear-gradient(225deg, rgba(140,154,255,1) 0%, rgba(194,144,255,1) 100%)"}}>
        
            <div className="inline-block flex">
                <h1 className="font-bold text-2xl text-white inline-block">Search</h1>
                <div className="rounded-md shadow-sm">
                        <input
                          onChange={e => searchTermUpdate(e)}
                          type="text"
                          name="search_bar"
                          id="search_bar"
                          className="focus:ring-exclpurple focus:border-exclpurple flex-1 block w-full rounded-l-md rounded-none rounded-r-md sm:text-sm border-gray-300"
                          placeholder="Redox reactions..."
                        />
                    </div>
            </div>

        </div>
        </div>
    )
}