import Link from 'next/link';
import { useState } from 'react';

type Resource = {
    id: string;
    title: string;
    description: string;
    files: File[];
}

type File = {
    id: string;
    title: string;
    size: string;
}


export default function ResourceCard(props: {
    propResource: Resource;
}) {

    const [resource, setResource] = useState({
        "id": props.propResource.id,
        "title": props.propResource.title,
        "description": props.propResource.description,
        "files": props.propResource.files,
    });
    
    
    const listFiles = resource.files.map((f) => 
        <div className="px-4 py-1">
            <Link href={ "https://cdn.examclutch.com/file/" + f.id }>
                <a>
                    <div className="py-4 px-4 shadow-sm hover:shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", }}>
                        
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
        <Link href={ "/app/resource/"+ resource.id }>
            <a>
                <div className="px-4 py-2">
                    <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white hover:shadow-lg duration-150" style={{fontFamily: "Roboto Mono"}}>

                    <div className="inline-block">
                        <h1 className="font-bold text-2xl pb-2 text-black inline-block">{ resource.title }</h1>
                        <h1>{ resource.description }</h1>
                    </div>

                    {listFiles}

                    </div>
                </div>
            </a>
        </Link>
    )
}