import Head from 'next/head'
import Link from 'next/link'
import { useRouter } from 'next/router';
import {SyntheticEvent, useEffect, useState} from  'react'


export default function Login() {
    
    const router = useRouter()
    const [errorMessage, setErrorMessage] = useState('')
    const [groupId, setGroupId] = useState('');
    

    
    const submit = async (e: SyntheticEvent) => {
        e.preventDefault();

        await fetch(`http://api.examclutch.com/api/group/join/${groupId}`, {
            method: "GET",
            credentials: 'include'  
        }).then(function(response) {
            if (response.status == 200 || response.status == 400) {
                router.push(`/app/group/${groupId}`)
            }
            if (response.status == 401) {
                router.push(`/api/login`)
            }
        })
    }
    
    return (
      
      <div>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Login to ScanTag - The Modern Name Tag" />
          <meta name="robots" content="all" />
          <meta name="googlebot" content="all" />
          <title>Join a Group - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />
        </Head>


        <div className="min-h-screen flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8 bg-gray-50" style={{fontFamily: 'Roboto Mono'}}>
            <div className="max-w-md w-full space-y-8">
                <div>
                <Link href="/app">
                    <a>
                        <img className="mx-auto w-48" src="/gradient_logo.svg" alt="Workflow"/>
                    </a>
                </Link>
                <h2 className="mt-6 text-center text-4xl text-gray-900" style={{fontFamily: "Space Mono", fontWeight: 'bold'}}>
                    Enter a group ID
                </h2>
                <p className="mt-2 text-center text-sm text-gray-600">
                    Or
                    <Link href="/app">
                        <a className="pl-1 font-medium text-exclpurple hover:text-exclpurple-dark duration-300">
                            return to the dashboard
                        </a>
                    </Link>
                </p>
                </div>
                <form className="mt-8 space-y-6" onSubmit={submit}>
                <input type="hidden" name="remember" value="true"/>
                <div className="rounded-md -space-y-px">
                    
                    <div className="pb-5">
                    <label htmlFor="group_id" className="sr-only">Group Id</label>
                    <input id="group_id" name="group_id" type="text" autoComplete="off" required className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md rounded-b-md focus:outline-none focus:ring-exclpurple focus:border-exclpurple focus:z-10 sm:text-sm" placeholder="Group ID" onChange={e => setGroupId(e.target.value)}/>
                    </div>
                    <p id="invalidCredentialsText" className="text-transparent text-sm float-left pl-1 pb-5 pt-2">{errorMessage}</p>
                </div>

                <div>
                    <button type="submit" className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-exclpurple hover:bg-exclpurple-dark rounded-3xl focus:outline-none duration-300 ">
                    Join
                    </button>
                </div>               
                </form>
            </div>
        </div>


    </div>
  )
}