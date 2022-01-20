import { useState } from 'react';

type GroupUser = {
    id: string;
    username: string;
}

export default function Members(props: {
    members: GroupUser[]; admins: GroupUser[];
    }) {

    
    const listAdmins = props.admins.map((d) => <li style={{listStyle: 'none'}} key={d.username}>{d.username}</li>);
    const listMembers = props.members.map((d) => <li style={{listStyle: 'none'}} key={d.username}>{d.username}</li>);

    return(
        
        <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white hover:shadow-lg duration-150" style={{fontFamily: "Roboto Mono"}}>
        
            <h1 className="font-bold text-2xl pb-2 text-exclpurple">Members</h1>
            <div>
                <div className="text-exclpurple-dark font-bold">
                    { listAdmins }
                </div>
                
                { listMembers }
            </div>            
        </div>
    )
}