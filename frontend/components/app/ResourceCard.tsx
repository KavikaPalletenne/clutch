import Link from 'next/link';
import { useState } from 'react';

export default function ResourceCard() {

    const [resource, setResource] = useState({
        "id": "cd505bba-1bb4-4ff7-b3b3-f57854d0099e",
        "title": "Electronegativity",
        "description": "Electronegativity chart that we will get on the exam",
        "files": [
            {"id": "1bf1f868-6aa7-4821-9aff-12002114c360","title": "electronegativity_chart.png", "size": "1.5MB"},
            {"id": "33adf97e-afe8-4d28-95b7-99eba22bee8d", "title": "data_booklet_2021.pdf", "size": "2.5MB"}
        ]
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