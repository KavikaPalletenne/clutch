import { useState } from 'react';
import UserName from './UserName';


export default function Members(props: {
    members: string[]; admins: string[];
    }) {

    
    const listAdmins = props.admins.map((d) => <li style={{listStyle: 'none'}} key={d}>{<UserName userId={d}/>}</li>);
    const listMembers = props.members.map((d) => <li style={{listStyle: 'none'}} key={d}>{<UserName userId={d}/>}</li>);

    return(
        
        <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono"}}>
        
            <h1 className="font-bold text-2xl pb-2 text-exclpurple">Members</h1>
            <div>
                {/* <div className="text-gray-600 font-bold">
                    { listAdmins }
                </div> */}
                
                <div className="text-gray-500 font-bold">
                    { listMembers }
                </div>
            </div>            
        </div>
    )
}