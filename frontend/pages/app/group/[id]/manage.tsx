import Head from 'next/head'
import Link from 'next/link'
import { useRouter } from 'next/router';
import {SyntheticEvent, useEffect, useState} from  'react'
import { GetServerSideProps } from "next";

import type { Group } from "./index"

export default function NewResourcePage({ group, loggedIn, isGroupAdmin }: {
    group: Group;
    loggedIn: boolean;
    // resources: Resource[];
    isGroupAdmin: boolean;
}) {
    
    const router = useRouter()
    const { id } = router.query

    const fileReader = () => new FileReader();

    

    const [isBusy, setBusy] = useState(true);

    const [errorMessage, setErrorMessage] = useState('')

    const {autologin} = router.query

    const [authToken, setAuthToken] = useState('')

    const [tagInputPlaceHolder, setTagInputPlaceHolder] = useState('Separate by commas...')

    var isMounted = false
    var userIdLoaded = false
    var loggedIn = false
    var url = `/discord/flow/cancelled`

    const [isKeyReleased, setIsKeyReleased] = useState(false);

    const [name, setName] = useState('');
    const [description, setDescription] = useState('');
    const [privateGroup, setPrivateGroup] = useState(group.private);
    
    const [loading, setLoading] = useState(false);
    const [submitted, setSubmitted] = useState(false);
    const [updateFailed, setUpdateFailed] = useState(false);

    const [updateMessageVisible, setUpdateMessageVisible] = useState(false);
    const [updateMessage, setUpdateMessage] = useState('');


    useEffect(() => {
        /**
        function CheckLoggedIn() {
            if (localStorage.getItem('token') != null && autologin != 'false') {
                router.push("/account/tags")
            }
            return
        }
        CheckLoggedIn()
        */
    
    }, [])

    const setGroupPrivacy = async (e: any) => {
        let is_private = e.target.checked

        let res = await fetch(`https://api.examclutch.com/api/group/${group._id}/private?value=${is_private}`, {
            method: 'POST',
            credentials: 'include',
        })

        setUpdateMessageVisible(true)

        if (res.ok) {
            setUpdateMessage("Successfully updated group privacy")
        } else {
            setUpdateMessage("Failed to update group privacy")
        }

        setTimeout(function () {
            setUpdateMessageVisible(false)
            setUpdateMessage("")
        }, 3000);
    }

    const submit = async (e: any) => {
        e.preventDefault()
        setUpdateFailed(false)
      if(!submitted) {
        setSubmitted(true)

        if (name.length == 0) {
          setUpdateFailed(true)
          return;
        }

        fetch(`https://api.examclutch.com/api/group/${group._id}/update?name=${name}&description=${description}`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
                // 'Cookie': `auth_token=${authToken}`,
            },
            body: JSON.stringify({
                'name': name,
                'description': description,
                'discord_id': '',
                'private': privateGroup,
            })
        }).then(r =>  r.json().then(async function(data) {
            if (r.ok) {
                setUpdateMessage("Successfully updated group")
                setSubmitted(false);
            } else {
                setUpdateMessage("Failed to update group")
            }
    
            setTimeout(function () {
                setUpdateMessageVisible(false)
                setUpdateMessage("")
            }, 3000);
            // setUpdateFailed(true);
            setSubmitted(false);
        }
        ))
      }
    }

    return (
      
      <div className='bg-gray-50 md:pl-96 pt-5' style={{fontFamily: "Roboto Mono"}}>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Create a new group. Share files and learn together." />
          <meta name="robots" content="index" />
          <meta name="googlebot" content="index" />
          <meta name="referrer" content="no-referrer" />
          <title>Manage Group - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />          
        </Head>

        
        <div className='min-h-screen'>
        <div>
          <div className='flex justify-items-center align-items-center text-center pb-5'>
          <div className='font-bold text-exclpurple text-4xl flex inline-flex text-center justify-items-center align-items-center shadow-sm'>
          <h1>Manage {group.name}</h1>
          </div>
          <div className='flex justify-items-center align-items-center'>
          <p className="mt-2 text-center text-sm text-gray-600">
            Or
            <Link href={`/app/group/${group._id}`}>
                <a className="pl-1 font-medium text-exclpurple hover:text-exclpurple-dark duration-300">
                    return to group
                </a>
            </Link>
          </p>
          </div>
          </div>
        <div className="drop-shadow-xl md:grid md:grid-cols-3 md:gap-6">
          <div className="mt-5 md:mt-0 md:col-span-2">
            <form onSubmit={submit} id="my-form">
              <div className="shadow sm:rounded-md sm:overflow-hidden">
                <div className="px-4 py-5 bg-white space-y-6 sm:p-6">
                  <div className="grid grid-cols-3 gap-6">
                    <div className="col-span-3 sm:col-span-2">
                      <label htmlFor="name" className="block text-sm font-medium text-gray-500">
                        Name
                      </label>
                      <div className="mt-1 flex rounded-md shadow-sm">
                        <input
                          onChange={e => setName(e.target.value)}
                          type="text"
                          name="name"
                          id="name"
                          className="focus:ring-exclpurple bg-white text-black focus:border-exclpurple flex-1 block w-full rounded-l-md rounded-none rounded-r-md sm:text-sm border-white"
                          defaultValue={group.name}
                        />
                      </div>
                    </div>
                  </div>

                  

                  

                  <div>
                    <label htmlFor="description" className="block text-sm font-medium text-gray-500">
                      Description
                    </label>
                    <div className="mt-1">
                      <textarea
                        onChange={e => setDescription(e.target.value)}
                        id="description"
                        name="description"
                        rows={3}
                        className="shadow-sm focus:ring-exclpurple bg-white text-black focus:border-exclpurple mt-1 block w-full sm:text-sm border border-white rounded-md"
                        defaultValue={group.description}
                      />
                    </div>
                  </div>

                  <label htmlFor="description" className="block text-sm font-medium text-gray-500">
                      Privacy
                  </label>
                  <label htmlFor="default-toggle" className="inline-flex relative items-center cursor-pointer">
                  <input type="checkbox" defaultChecked={group.private} id="default-toggle" className="sr-only peer" onChange={e => setGroupPrivacy(e)} />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-exclpurple dark:peer-focus:ring-exclpurple rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-exclpurple-dark"></div>
                  { privateGroup ?
                    <span className="ml-3 font-medium text-exclpurple font-bold text-md">Private</span> :
                    <span className="ml-3 font-medium text-exclpurple font-bold text-md">Public</span>
                  }
                </label>
                  
                </div>
                
                <div className="px-4 py-3 bg-white text-right sm:px-6">
                  <Link href={`/app/group/${group._id}`}>
                  <a className="pr-1">
                  <button
                    className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-red-500 hover:shadow-md duration-150"
                  >
                    Cancel
                  </button>
                  </a>
                  </Link>
                  
                  <button
                    type="submit"
                    disabled={submitted}
                    className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-exclpurple hover:shadow-md duration-150">
                    Update
                  </button>
                </div>
              </div>
            </form>
            { updateFailed ?
              <div className='float-right pr-10 text-red-500'>
                Failed to update group
              </div> : null
            }
          </div>
        </div>
      </div>
      { updateMessageVisible ? <div className='flex pt-5'>
          <div className='justify-center text-center px-3 py-2 bg-exclpurple rounded-xl text-white font-bold'>
              {updateMessage}
          </div>
      </div>
       : null}
      </div>


    </div>
  )
}

export const getServerSideProps: GetServerSideProps = async (context: any) => {
    
    const group_res = await fetch(`https://api.examclutch.com/api/group/${context.params.id}`, {
        credentials: 'include',
        headers: context.req ? {cookie: context.req.headers.cookie} : undefined
    });

    const is_admin_res = await fetch(`https://api.examclutch.com/api/group/${context.params.id}/user_is_admin`, {
        credentials: 'include',
        headers: context.req ? {cookie: context.req.headers.cookie} : undefined
    });

    let is_admin = false
    if (is_admin_res.ok) {
        is_admin = true
    }

    if (!is_admin_res.ok) {
        return {
            redirect: {
                destination: `/app/group/${context.params.id}`,
                permanent: false,
            }
        }
    }
    
    if (!group_res.ok) {
        if (group_res.status == 401) {
            return {
                redirect: {
                    destination: '/app',
                    permanent: false,
                }
            }
        }
        
        if (group_res.status != 401) {
            return {
                redirect: {
                    destination: '/app',
                    permanent: false,
                }
            }
        }
    }
    
    const group = await group_res.json() as Group;

    if (!group) {
        return {
            notFound: true,
        }
    }
    
    // const resources_res = await fetch(`https://api.examclutch.com/api/resource/get_all/${context.params.id}`, {
    //     credentials: 'include'
    // });
    // const resources = await resources_res.json() as Resource[];

    const logged_in_res = await fetch('https://api.examclutch.com/api/auth/logged_in', {
        credentials: 'include',
        headers: context.req ? {cookie: context.req.headers.cookie} : undefined
    });

    if (logged_in_res.ok) {
        return {
            props: {
                group,
                loggedIn: true,
                isGroupAdmin: is_admin,
            }
        }
    }

    return {
        props: {
            group,
            loggedIn: false,
            isGroupAdmin: is_admin,
        }
    }
    
}
