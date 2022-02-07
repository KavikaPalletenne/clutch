import { useRouter } from "next/router";
import {SyntheticEvent, useEffect, useState} from  'react';

import Head from "next/head";
import Members from "../../../components/app/Members";
import GroupTitle from "../../../components/app/GroupTitle";
import ResourceCard from "../../../components/app/ResourceCard";
import { GetServerSideProps } from "next";
import NewButton from "@components/app/NewButton";

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
    files: File[];
    last_edited_at: Date;
}

export type Tag = {
    name: string;
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

    const members = ["436035620905943041", "436035620905943041","436035620905943041","436035620905943041","436035620905943041","436035620905943041","436035620905943041",]

    const listResources = resources.map((r: Resource) =>
            <div key={r._id.$oid} className="pb-3">
                <ResourceCard propResource={r} />
            </div>
    );

    return(
        <div>
            <Head>
                <title>{group.name} - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
                <link rel="icon" href="/gradient_logo.svg" />
            </Head>

            <div className="flex items-start justify-center pt-10">
                <GroupTitle propGroup={group} />
                <NewButton groupId={group._id}/>
            </div>

            <div className="flex justify-center pt-5 pr-20">
                <div>
                { listResources }
                </div>
                <div className="pl-5">
                <Members admins={group.administrators} members={members} />
                </div>
            </div>

        </div>
    )
}


export const getServerSideProps: GetServerSideProps = async (context: any) => {
    
    const group_res = await fetch(`http://localhost:443/api/group/${context.params.id}`);
    
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
    
    const resources_res = await fetch(`http://localhost:443/api/resource/get_all/${context.params.id}`);
    const resources = await resources_res.json() as Resource[];

    return {
        props: {
            group,
            resources
        }
    }
}