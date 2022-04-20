import { useEffect, useState } from 'react';
import GroupName from './GroupName';
import Cookies from 'js-cookie';
import Link from 'next/link';


export default function GroupNavigation(props: {
        currentGroupId: string;
    }) {
    
    const [data, setData] = useState(null);
    const [isLoading, setLoading] = useState(false);
    const [userId, setUserId] = useState(Cookies.get('user_id'))

    const [groups, setGroups] = useState([]);

    const listGroups = groups.map((d) => {
        if (d == props.currentGroupId) {
            return (
            <div key={d}>
                <Link href={"/app/group/" + d}>
                    <a>
                        <li className="py-2 px-1 bg-exclpurple rounded-xl text-xl text-white justify-center flex font-medium" style={{listStyle: 'none'}} key={d}>
                            {<GroupName groupId={d}/>}
                        </li>
                    </a>
                </Link>
            </div>
            )
        }

        if (d != props.currentGroupId) {
            return (
            <div key={d}>
                <Link href={"/app/group/" + d}>
                    <a>
                        <li className="py-2 px-1 rounded-xl text-xl text-black justify-center flex font-medium" style={{listStyle: 'none'}} key={d}>
                            {<GroupName groupId={d}/>}
                        </li>
                    </a>
                </Link>
            </div>
            )
        }
        
    });
    
    useEffect(() => {
        setLoading(true)
        setUserId(Cookies.get('user_id'))
        console.log("User_id: " + userId)
        fetch(`http://127.0.0.1:443/api/user/get_user_groups/${userId}`, {
            credentials: 'include',
        })
        .then((res) => res.json())
        .then((data) => {
            setGroups(data)
            setLoading(false)
        })
    }, [])
    

    return(
        
        <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", minWidth: "250px"}}>
        
            <div className="flex">
                <h1 className="font-bold text-2xl pb-2 text-exclpurple">Groups</h1>
                <Link href="/app/join">
                    <a>
                        <div className="pl-2 text-2xl text-gray-500">+</div>
                    </a>
                </Link>
            </div>
            <div>
                <div className="text-exclpurple-dark font-bold">
                    { listGroups }
                </div>
            </div>            
        </div>
    )
}