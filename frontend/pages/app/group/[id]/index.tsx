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
import LoadingResourceCard from "../../../../components/app/LoadingResourceCard";
import GroupName from "../../../../components/app/GroupName";
import { GetServerSideProps } from "next";

export type Group = {
    id: string;
    name: string;
    description: string;
    discord_link: string;
    private: boolean;
    members: string[];
    administrators: string[];
}

export type Resource = {
    id: bigint;
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
    name: string;
    size: number;
}

export default function GroupPage({ group, loggedIn, isGroupAdmin }: {
    group: Group;
    loggedIn: boolean;
    // resources: Resource[];
    isGroupAdmin: boolean;
}) {
    const router = useRouter();
    let { id, page_num } = router.query;
    const [userId, setUserId] = useState(Cookies.get("user_id"))
    const [userName, setUserName] = useState('')
    const [fullResources, setFullResources] = useState([] as Resource[])
    const [stateResources, setStateResources] = useState([] as Resource[])
    const listResources = stateResources.map((r: Resource) =>
            <>
            <div key={r.title} className="pb-3">
                <ResourceCard propResource={r} />
            </div>
            <ins className="adsbygoogle"
                style={{'display':'block'}}
                data-ad-format="fluid"
                data-ad-layout-key="-fb+5w+4e-db+86"
                data-ad-client="ca-pub-7136601653169605"
                data-ad-slot="2587920457"></ins>
            <script id="google-resource-feed-ad">
                {`
                (adsbygoogle = window.adsbygoogle || []).push({});
                `} 
            </script>
            </>
    );

    const [fetchedResources, setFetchedResources] = useState(false)

    useEffect(() => {

        setUserId(Cookies.get('user_id'))
        setFetchedResources(false)
        setStateResources([] as Resource[])
        setFullResources([] as Resource[])

        fetch(`https://api.examclutch.com/api/resource/get_all/${id}?page=0&num_per_page=2000000`, {
            credentials: 'include'
        }).then(r => {
            if (!r.ok) {
                setStateResources([] as Resource[])
                setFullResources([] as Resource[])
                setFetchedResources(true)
                return
            }
            r.json().then(function(data) {
            setStateResources(data as Resource[])
            setFullResources(data as Resource[])
            setFetchedResources(true)
        })});

        fetch(`https://api.examclutch.com/api/user/username/${userId}`, {
            credentials: 'include'
        })
            .then((res) => res.json())
            .then((data) => {
            setUserName(data.username)
        })
      }, [id])

    const searchTermUpdate = async (e: React.ChangeEvent<any>) => {
        e.preventDefault()

        let results = await fetch(`https://api.examclutch.com/api/search/${id}/${e.target.value}`, {
            method: 'GET',
            credentials: 'include'
        }).then(r => r.json().then(function(data) {
            return data as (Resource[])
        }))
        
        
        setStateResources(results)
        if (((e.target.value) == '')) {
            setStateResources(fullResources)
        }
    }

    return(
        <div className="bg-gray-50 min-h-screen">
            <Head>
                <title>{group.name} - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
                <link rel="icon" href="/gradient_logo.svg" />
                <script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-7136601653169605" crossOrigin="anonymous"></script>


            <script async src="//www.ezojs.com/basicads.js?d=examclutch.com" type="text/javascript"></script>

            </Head>

            { loggedIn ? null :
                <div style={{'fontFamily': 'Roboto Mono'}} className='justify-content-center justify-center float align-items-center sticky top-0 pt-2 px-96'>
                    <div className="bg-exclpurple-dark px-5 py-3 focus:text-white rounded-2xl shadow-xl text-xl font-bold text-center">
                    <Link href={`/login?redirect=${router.route.replace('[id]', group._id)}`}>
                            <a>
                                <h1 style={{fontFamily: "Roboto Mono"}} className="text-lg text-white">Sign up to share resources and create your own groups</h1>
                            </a>
                        </Link>
                    </div>
                </div>
            }
            <div className="pt-10 grid justify-items-center bg-gray">
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
                        { loggedIn ? <h1 style={{fontFamily: "Roboto Mono"}} className="text-white text-lg">Hey {userName}!</h1>: null }
                        { loggedIn ? <Link href="/api/logout">
                            <a>
                                <h1 style={{fontFamily: "Roboto Mono"}} className="text-white text-lg">Logout</h1>
                            </a>
                        </Link>
                        :
                        <Link href={`/login?redirect=/app/group/${id}`}>
                            <a>
                                <h1 style={{fontFamily: "Roboto Mono"}} className="text-white text-lg">Login</h1>
                            </a>
                        </Link>
}
                    </div>

                </div>
                </div>
            </div>
            <div className="flex justify-center">
            <div className="pt-10 grid grid-flow-col auto-cols-min">
                <div className="pr-3 row-span-3 col-span-1">
                {loggedIn ? <GroupNavigation currentGroupId={id as string} /> : null }              
                </div>
                <div className="pr-3 row-span-10 row-start-4 col-span-1 col-start-1">
                <ins className="adsbygoogle"
                    style={{"display":"block"}}
                    data-ad-client="ca-pub-7136601653169605"
                    data-ad-slot="2738580801"
                    data-ad-format="auto"
                    data-full-width-responsive="true"></ins>
                <script id="google-group-side-ad">
                    {`(adsbygoogle = window.adsbygoogle || []).push({});`}
                </script>  
                </div>
                <div className="pr-3 row-span-10 row-start-3 col-span-1 col-start-3">
                <ins className="adsbygoogle"
                    style={{"display":"block"}}
                    data-ad-client="ca-pub-7136601653169605"
                    data-ad-slot="2738580801"
                    data-ad-format="auto"
                    data-full-width-responsive="true"></ins>
                <script id="google-group-side-ad">
                    {`(adsbygoogle = window.adsbygoogle || []).push({});`}
                </script>  
                </div>
                <div className="">
                <GroupTitle propGroup={group} isAdmin={isGroupAdmin} />
                </div>
                <div className="pt-2 row-start-1">
                { loggedIn ? <Link href={`/app/group/${id}/new`}>  
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
                :
                <Link href={`/login?redirect=/app/group/${id}/new`}>  
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
                }
                </div>
                <div className="pl-3 row-span-3 col-span-1">
                { loggedIn ? <Members admins={group.administrators} members={group.members} /> : null }
                </div>
                <div className="row-start-2 col-start-2 col-span-2 pt-5">
                    { listResources } 
                    {
                     fetchedResources ? null :
                     <div>
                     <div className="pb-3">
                        <LoadingResourceCard />
                     </div>
                     <div className="pb-3">
                        <LoadingResourceCard />
                     </div>
                     <div className="pb-3">
                        <LoadingResourceCard />
                     </div>
                     <div className="pb-3">
                        <LoadingResourceCard />
                     </div>
                     <div className="pb-3">
                        <LoadingResourceCard />
                     </div>
                     </div>
                    }
                    
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
    
    const group_res = await fetch(`https://api.examclutch.com/api/group/${context.params.id}`, {
        credentials: 'include',
        headers: context.req ? {cookie: context.req.headers.cookie} : undefined
    });

    const is_admin_res = await fetch(`https://api.examclutch.com/api/group/${context.params.id}/user_is_admin`, {
        credentials: 'include',
        headers: context.req ? {cookie: context.req.headers.cookie} : undefined
    });

    let is_admin = false
    if (is_admin_res.ok) {
        is_admin = true
    }
    
    if (!group_res.ok) {
        if (group_res.status == 401) {
            return {
                redirect: {
                    destination: '/app',
                    permanent: false,
                }
            }
        }
        
        if (group_res.status != 401) {
            return {
                redirect: {
                    destination: '/app',
                    permanent: false,
                }
            }
        }
    }
    
    const group = await group_res.json() as Group;

    if (!group) {
        return {
            notFound: true,
        }
    }
    
    // const resources_res = await fetch(`https://api.examclutch.com/api/resource/get_all/${context.params.id}`, {
    //     credentials: 'include'
    // });
    // const resources = await resources_res.json() as Resource[];

    const logged_in_res = await fetch('https://api.examclutch.com/api/auth/logged_in', {
        credentials: 'include',
        headers: context.req ? {cookie: context.req.headers.cookie} : undefined
    });

    if (logged_in_res.ok) {
        return {
            props: {
                group,
                loggedIn: true,
                isGroupAdmin: is_admin,
            }
        }
    }

    return {
        props: {
            group,
            loggedIn: false,
            isGroupAdmin: is_admin,
        }
    }
    
}