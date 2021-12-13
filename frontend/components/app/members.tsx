import { useState } from 'react';

export default function Members() {

    const [groupMembers, setGroupMembers] = useState(["@mihir", "@kavika", "@madderz", "@bushby"]);
    const listMembers = groupMembers.map((d) => <li style={{listStyle: 'none'}} key={d}>{d}</li>);

    return(
        
        <div className="py-4 px-4 shadow-lg inline-block rounded-2xl bg-white hover:shadow-xl duration-150" style={{fontFamily: "Roboto Mono"}}>
        
            <h1 className="font-bold text-2xl pb-2 text-exclpurple">Members</h1>
            <div>
                { listMembers }
            </div>            
        </div>
    )
}