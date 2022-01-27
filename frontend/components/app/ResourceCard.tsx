import Link from 'next/link';
import { useState } from 'react';
import type { Resource, File, ObjectId } from '../../pages/app/group/[id]';

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
        "title": props.propResource.title,
        "description": props.propResource.description,
        "files": props.propResource.files,
    };
    
    
    const listFiles = resource.files.map((f) => 
        <div className="px-4 py-1" key={f.id}>
            <Link href={ "https://examclutch.com/cdn/file/" + f.id }>
                <a>
                    <div className="py-4 px-4 shadow-sm hover:shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", minWidth: "750px"}}>
                        
                        <li className="justify-center flex" style={{listStyle: 'none'}} key={f.title}>
                            <div className="font-bold">
                                {f.title}
                            </div>
                            <div className="pl-10">
                                {f.size}
                            </div>
                        </li>
                    
                    </div>
                </a>
            </Link>
        </div>
    );

    return(
            <div className="px-4 py-2">
                <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white hover:shadow-lg duration-150" style={{fontFamily: "Roboto Mono", minWidth: "750px"}}>

                <div className="inline-block">
                    <Link href={ "/app/resource/"+ resource.id }>
                        <a>
                            <h1 className="font-bold text-2xl pb-2 text-black inline-block hover:text-exclpurple duration-200">{ resource.title }</h1>
                        </a>
                    </Link>
                    <h1>{ resource.description }</h1>
                </div>

                {listFiles}

                </div>
            </div>
        
    )
}