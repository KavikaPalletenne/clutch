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
    
    const groupTitle = props.groupId;
    const [searchTerm, setSearchTerm] = useState('')


    const searchTermUpdate = async (e) => {
        e.preventDefault()
        
        setSearchTerm(e.target.value)

        let results = await fetch(`http://localhost:7700/indexes/resources/search`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            body: JSON.stringify({
                'attributesToHighlight' : [
                    '*'
                ],
                'limit': 50,
                'filter': [
                    {'group_id': props.groupId}
                ],
                'q': searchTerm
            })
        }).then(r => r.json().then(function(data) {
            return data['hits']
        }))

        console.log(results[0].title)
    }



    return(
        
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