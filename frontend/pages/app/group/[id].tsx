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

export type Resource = {
    _id: ObjectId;
    user_id: string;
    group_id: string;
    title: string;
    description: string;
    tags: string[];
    files: File[];
    last_edited_at: Date;
}

export type ObjectId = {
    $oid: string;
}

export type File = {
    id: string;
    title: string;
    size: number;
}

export default function GroupPage({ group, resources }: {
    group: Group;
    resources: Resource[];
}) {

    const listResources = resources.map((r: Resource) =>
            <div key={r._id.$oid}>
                <ResourceCard propResource={r} />
            </div>
    );

    return(
        <div>
            <Head>
                <title>{group.name} - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
        <       link rel="icon" href="/favicon.png" />
            </Head>

            <div className="flex items-start justify-center pt-10">
                <GroupTitle propGroup={group} />
                <Members admins={group.administrators} members={group.members} />
            </div>

            <div className="flex inline-grid justify-center pt-5">
                { listResources }
            </div>

        </div>
    )
}


export async function getServerSideProps(context: any) {
    
    const group_res = await fetch(`http://localhost:441/api/group/${context.params.id}`);
    const group = await group_res.json() as Group;

    const resources_res = await fetch(`http://localhost:440/api/resource/get_all/${context.params.id}`);
    const resources = await resources_res.json() as Resource[];

    return {
        props: {
            group,
            resources
        }
    }
}