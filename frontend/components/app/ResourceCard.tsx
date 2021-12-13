import { useState } from 'react';

export default function ResourceCard() {

    const [resource, setResource] = useState({
        "title": "Electronegativity",
        "description": "Electronegativity chart that we will get on the exam",
        "files": [
            {"title": "electronegativity_chart.png", "size": "1.5MB"},
            {"title": "data_booklet_2021.pdf", "size": "2.5MB"}
        ]
    });
    
    const listFiles = resource.files.map((f) => 
        <div className="px-4 py-1">
            <div className="py-4 px-4 border-2 hover:border-gray-400 inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", }}>
                <li className="justify-center flex" style={{listStyle: 'none'}} key={f.title}>
                    <div className="font-bold">
                        {f.title}
                    </div>
                    <div className="pl-10">
                        {f.size}
                    </div>
                </li>
            </div>
        </div>
    );

    return(
        <div className="px-4">
            <div className="py-4 px-4 shadow-lg inline-block rounded-2xl bg-white hover:shadow-xl duration-150" style={{fontFamily: "Roboto Mono"}}>

            <div className="inline-block">
                <h1 className="font-bold text-2xl pb-2 text-black inline-block">{ resource.title }</h1>
                <h1>{ resource.description }</h1>
            </div>

            {listFiles}

            </div>
        </div>
    )
}