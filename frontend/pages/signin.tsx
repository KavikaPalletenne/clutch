import Head from 'next/head'
import Link from 'next/link'
import { useRouter } from 'next/router';
import {SyntheticEvent, useEffect, useState} from  'react'


export default function Login() {
    
    const router = useRouter()

    const [isBusy, setBusy] = useState(true);

    const [errorMessage, setErrorMessage] = useState('')

    const {autologin} = router.query

    var isMounted = false
    var userIdLoaded = false
    var loggedIn = false

    const [username, setUsername] = useState('');
    const [password, setPassword] = useState('');
    
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

    
    const submit = async (e: SyntheticEvent) => {
        e.preventDefault();
        
        await fetch("https://api.scantag.co/v1/auth/authenticate", {
            method: 'POST',
            headers: {'Content-Type': 'application/json'},
            body: JSON.stringify({
                'username': username,
                'password': password
            })
        }).then(function(response) {
            return response.json();
        }).then(function(json) {
            
            if(json.jwt == null) {
                document.getElementById("invalidCredentialsText").className = "text-red-500 text-sm float-left pl-1 pb-5 pt-2"
                setErrorMessage("Invalid email or password")
                localStorage.removeItem('token')
                return
            }

            localStorage.setItem('token', json.jwt)
            localStorage.setItem('userId', json.userId)
            
            loggedIn = true

            router.push("/account/tags")
        });

    }
    
    return (
      
      <div>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Login to ScanTag - The Modern Name Tag" />
          <meta name="robots" content="all" />
          <meta name="googlebot" content="all" />
          <title>Sign In - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />
        </Head>


        <div className="min-h-screen flex items-center justify-center py-12 px-4 sm:px-6 lg:px-8 bg-gray-50" style={{fontFamily: 'Roboto Mono'}}>
            <div className="max-w-md w-full space-y-8">
                <div>
                <Link href="/">
                    <a>
                        <img className="mx-auto w-48" src="/gradient_logo.svg" alt="Workflow"/>
                    </a>
                </Link>
                <h2 className="mt-6 text-center text-3xl text-gray-900" style={{fontFamily: "Space Mono", fontWeight: 'bold'}}>
                    Sign in to your account
                </h2>
                <p className="mt-2 text-center text-sm text-gray-600">
                    Or
                    <Link href="/register">
                        <a className="pl-1 font-medium text-exclpurple hover:text-exclpurple-dark">
                            sign up for an account
                        </a>
                    </Link>
                </p>
                </div>
                <form className="mt-8 space-y-6" onSubmit={submit}>
                <input type="hidden" name="remember" value="true"/>
                <div className="rounded-md -space-y-px">
                    
                    <div className="pb-5">
                    <label htmlFor="username" className="sr-only">Email address</label>
                    <input id="username" name="username" type="email" autoComplete="email" required className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md rounded-b-md focus:outline-none focus:ring-exclpurple focus:border-exclpurple focus:z-10 sm:text-sm" placeholder="Email address" onChange={e => setUsername(e.target.value)}/>
                    </div>

                    <div>
                    <label htmlFor="password" className="sr-only">Password</label>
                    <input id="password" name="password" type="password" autoComplete="current-password" required className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md rounded-b-md focus:outline-none focus:ring-exclpurple focus:border-exclpurple focus:z-10 sm:text-sm" placeholder="Password" onChange={e => setPassword(e.target.value)}/>
                    </div>
                    <p id="invalidCredentialsText" className="text-transparent text-sm float-left pl-1 pb-5 pt-2">{errorMessage}</p>
                </div>

                <div>
                    <button type="submit" className="group relative w-full flex justify-center py-2 px-4 border border-transparent text-sm font-medium rounded-md text-white bg-exclpurple hover:bg-exclpurple-dark rounded-3xl focus:outline-none ">
                    Sign in
                    </button>
                </div>               

                <div className="flex items-center justify-center">

                    <div className="text-sm">
                    <Link href="/auth/forgot-password">
                        <a className="pl-1 font-medium text-exclpurple hover:text-exclpurple-dark">
                            Forgot your password?
                        </a>
                    </Link>
                    </div>
                </div>

                </form>
            </div>
        </div>


    </div>
  )
}