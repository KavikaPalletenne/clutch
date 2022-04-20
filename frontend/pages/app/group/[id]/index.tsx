import { useRouter } from "next/router";
import {SyntheticEvent, useEffect, useState} from  'react';
import Link from "next/link";
import { AiOutlinePlus } from "react-icons/ai"
import Cookies from 'js-cookie';

import Head from "next/head";
import Members from "../../../../components/app/Members";
import GroupNavigation from "../../../../components/app/GroupNavigation";
import GroupTitle from "../../../../components/app/GroupTitle";
import HeaderBar from "../../../../components/app/HeaderBar";
import ResourceCard from "../../../../components/app/ResourceCard";
import GroupName from "../../../../components/app/GroupName";
import { GetServerSideProps } from "next";

type Group = {
    _id: string;
    name: string;
    description: string;
    discord_link: string;
    members: string[];
    administrators: string[];
}

export type Resource = {
    _id: ObjectId;
    user_id: string;
    group_id: string;
    title: string;
    description: string;
    subject: string;
    tags: Tag[];
    files: FileReference[];
    last_edited_at: Date;
}

export type Tag = {
    name: string;
}

export type ObjectId = {
    $oid: string;
}

export type FileReference = {
    id: string;
    title: string;
    size: number;
}

export default function GroupPage({ group, resources }: {
    group: Group;
    resources: Resource[];
}) {
    const [userId, setUserId] = useState(Cookies.get("user_id"))
    const [userName, setUserName] = useState('')
    const [fullResources, setFullResources] = useState([] as Resource[])
    const [stateResources, setStateResources] = useState([] as Resource[])
    const listResources = stateResources.map((r: Resource) =>
            <div key={r._id.$oid} className="pb-3">
                <ResourceCard propResource={r} />
            </div>
    );

    useEffect(() => {

        setUserId(Cookies.get('user_id'))
        fetch(`http://127.0.0.1:443/api/resource/get_all/${group._id}`, {
            credentials: 'include'
        })
                        .then(r => r.json().then(function(data) {
                            setStateResources(data as Resource[])
                            setFullResources(data as Resource[])
        }));

        fetch(`http://127.0.0.1:443/api/user/${userId}`, {
            credentials: 'include'
        })
            .then((res) => res.json())
            .then((data) => {
            setUserName(data.username)
        })
      }, [])

    const searchTermUpdate = async (e) => {
        e.preventDefault()

        let results = await fetch(`http://127.0.0.1:443/api/search/${group._id}/${e.target.value}`, {
            method: 'GET',
            credentials: 'include'
        }).then(r => r.json().then(function(data) {
            return data as Resource[]
        }))
        
        
        setStateResources(results)
        if (((e.target.value) == '')) {
            setStateResources(fullResources)
        }
    }

    return(
        <div>
            <Head>
                <title>{group.name} - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
                <link rel="icon" href="/gradient_logo.svg" />
            </Head>

            <div className="pt-10 grid justify-items-center">
                {/* Header Bar */}
                <div>
                <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white duration-150 grid" style={{fontFamily: "Roboto Mono", backgroundImage: "linear-gradient(225deg, rgba(140,154,255,1) 0%, rgba(194,144,255,1) 100%)"}}>
                
                    <div className="inline-block flex align-items-center">
                        <h1 style={{fontFamily: "Roboto Mono"}} className="font-bold text-white text-3xl pr-36">ExCl</h1>
                        <div className="rounded-md shadow-sm pr-24">
                                <input
                                onChange={e => searchTermUpdate(e)}
                                type="text"
                                size={50}
                                name="search_bar"
                                id="search_bar"
                                className="focus:ring-exclpurple focus:border-exclpurple flex-1 block rounded-l-md rounded-none rounded-r-md sm:text-sm border-gray-300"
                                placeholder="Redox reactions..."
                                />
                        </div>
                        <h1 style={{fontFamily: "Roboto Mono"}} className="text-white text-lg">Hey {userName}!</h1>
                    </div>

                </div>
                </div>
            </div>
            <div className="flex justify-center">
            <div className="pt-10 grid grid-flow-col auto-cols-min">
                <div className="pr-3 row-span-3 col-span-1">
                <GroupNavigation currentGroupId={group._id} />
                </div>
                <div className="">
                <GroupTitle propGroup={group} />
                </div>
                <div className="pt-2 row-start-1">
                <Link href={`/app/group/${group._id}/new`}>  
                    <a>    
                    <div className="py-3.5 px-5 shadow-md inline-block rounded-2xl hover:shadow-lg duration-150" style={{fontFamily: "Roboto Mono", fontWeight: "bold", backgroundImage: "linear-gradient(225deg, rgba(140,154,255,1) 0%, rgba(194,144,255,1) 100%)"}}>
                        <div className="text-2xl text-white">
                            <h1>
                                New
                            </h1>
                        </div>
                    </div>
                    </a>
                </Link>
                </div>
                <div className="pl-3 row-span-3 col-span-1">
                <Members admins={group.administrators} members={group.members} />
                </div>
                <div className="row-start-2 col-start-2 col-span-2 pt-5">
                    { listResources }
                    <div className="text-gray-300 text-sm grid justify-items-center align-items-center" style={{fontFamily: "Roboto Mono"}}>
                        <div>Help {group.name} by adding more resources</div>
                    </div>
                </div>
            </div>
            </div>

            <div className="flex justify-center pt-5 pr-20">
                
            </div>

        </div>
    )
}


export const getServerSideProps: GetServerSideProps = async (context: any) => {
    
    const group_res = await fetch(`http://127.0.0.1:443/api/group/${context.params.id}`, {
        credentials: 'include'
    });
    
    if (!group_res.ok) {
        return {
            notFound: true,
        }
    }
    
    const group = await group_res.json() as Group;

    if (!group) {
        return {
            notFound: true,
        }
    }
    
    const resources_res = await fetch(`http://127.0.0.1:443/api/resource/get_all/${context.params.id}`, {
        credentials: 'include'
    });
    const resources = await resources_res.json() as Resource[];

    return {
        props: {
            group,
            resources
        }
    }
}