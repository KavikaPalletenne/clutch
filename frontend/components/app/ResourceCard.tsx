import Link from 'next/link';
import { useState } from 'react';
import type { Resource, Tag } from '../../pages/app/group/[id]';
import UserName from './UserName';

// type Resource = {
//     _id: ObjectId;
//     user_id: string;
//     group_id: string;
//     title: string;
//     description: string;
//     tags: string[];
//     files: File[];
//     last_edited_at: Date;
// }

// type ObjectId = {
//     $oid: string;
// }

// type File = {
//     id: string;
//     title: string;
//     size: string;
// }


export default function ResourceCard(props: {
    propResource: Resource;
}) {

    const resource = {
        "id": props.propResource._id.$oid,
        "group_id": props.propResource.group_id,
        "author": props.propResource.user_id,
        "title": props.propResource.title,
        "description": props.propResource.description,
        "subject": props.propResource.subject,
        "files": props.propResource.files,
        "tags": props.propResource.tags,
    };
    
    
    const listFiles = resource.files.map((f) => 
        <div className="px-4 py-1" key={f.id}>
            {/* <Link href={ "https://examclutch.com/cdn/file/" + f.id }> */}
            <Link href={ "http://localhost:443/cdn/file/" + f.id }> 
                <a>
                    <div className="py-4 px-4 shadow-sm hover:shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", minWidth: "675px", maxWidth: "750px"}}>
                        
                        <li className="justify-center float-left flex" style={{listStyle: 'none'}} key={f.title}>
                            <div className="pl-5 font-bold">
                                {f.title}
                            </div>
                            
                        </li>
                        <div className="pr-5 float-right">
                            {f.size}
                        </div>
                    
                    </div>
                </a>
            </Link>
        </div>
    );

    const listTags = resource.tags.map((t) =>
        <div className="">
            <Link href={`https://examclutch.com/app/group/${resource.group_id}/tag/${t.name}`}>
                <a>
                    <div className="flex justify-center">
                        <li className="justify-items-center flex" style={{listStyle: 'none'}} key={t.name}>
                            {t.name}
                        </li>
                    </div>
                </a>
            </Link>
        </div>
    )

    return(
            <div>
                <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white hover:shadow-lg duration-150" style={{fontFamily: "Roboto Mono", maxWidth: "750px", minWidth: "750px"}}>

                <div className="inline-block">
                    <Link href={ "/app/resource/"+ resource.id }>
                        <a>
                            <h1 className="font-bold text-2xl pb-2 text-black inline-block hover:text-exclpurple duration-200">{ resource.title }</h1>
                        </a>
                    </Link>

                    <Link href={ `/app/group/${resource.group_id}/subject/${resource.subject}` }>
                        <a>
                            <h3 className="float-right text-exclpurple font-bold" >{ resource.subject }</h3>
                        </a>
                    </Link>
                    <div className="flex">
                        <UserName userId={resource.author} />
                    </div>
                    <h1>{ resource.description }</h1>
                </div>

                {listFiles}
                </div>
            </div>
        
    )
}