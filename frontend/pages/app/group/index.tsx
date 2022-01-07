import { useRouter } from "next/router";
import {SyntheticEvent, useEffect, useState} from  'react';

import Head from "next/head";
import Members from "../../../components/app/Members";
import GroupTitle from "../../../components/app/GroupTitle";
import ResourceCard from "../../../components/app/ResourceCard";

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

export default function GroupPage() {

    const router = useRouter();

    const queryKey = 'id';
    const groupId = router.query[queryKey] || router.asPath.match(new RegExp(`[&?]${queryKey}=(.*)(&|$)`))?.[1];

    const [username, setUsername] = useState('' as string|null);
    const [group, setGroup] = useState({} as Group);
    const [groupResources, setGroupResouurces] = useState();

    useEffect(() => {
        
        setUsername(localStorage.getItem("username"))
        
        async function getGroup() {
            await fetch(`http://localhost:441/api/group/${groupId}`, {
                method: 'GET'
            }).then(res => res.json()).then(data => {
                setGroup(data);
            });
        }

        getGroup()

    }, [])

    const resource = {
        id: "cd505bba-1bb4-4ff7-b3b3-f57854d0099e",
        title: "Electronegativity",
        description: "Electronegativity chart that we will get on the exam",
        files: [
            {"id": "1bf1f868-6aa7-4821-9aff-12002114c360","title": "electronegativity_chart.png", "size": "1.5MB"},
            {"id": "33adf97e-afe8-4d28-95b7-99eba22bee8d", "title": "data_booklet_2021.pdf", "size": "2.5MB"}
        ]
    }

    // const resource = {
    //     id: groupId,
    //     title: group.title,
    //     description: group.description,
    //     files: group.files
    // }


    return(
        <div>
            <Head>
                <title>Dashboard - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
        <       link rel="icon" href="/favicon.png" />
            </Head>

            <div className="flex items-start justify-center pt-10">
                <GroupTitle propGroup={group} />
                <Members />
            </div>

            <div className="flex inline-grid">
                <ResourceCard propResource={resource} />
            </div>

        </div>
    )
}