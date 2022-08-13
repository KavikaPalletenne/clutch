import Head from 'next/head'
import Link from 'next/link'
import { useRouter } from 'next/router';
import {SyntheticEvent, useEffect, useState} from  'react'


export default function Login() {
    
    const router = useRouter()

    const [isBusy, setBusy] = useState(true);

    const [errorMessage, setErrorMessage] = useState('')

    const {autologin, redirect} = router.query

    var isMounted = false
    var userIdLoaded = false
    var loggedIn = false

    const [email, setEmail] = useState('');
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

    
    const submit = async (e) => {
        e.preventDefault()

        await fetch("https://api.examclutch.com/api/auth/login", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            credentials: 'include',
            body: JSON.stringify({
                'email': email,
                'password': password,
            })
        }).then((res) => {
            if (res.status == 200) {

                if (redirect == null) {
                    router.push("/app")
                    return
                }
                router.push(redirect)
                return
            } else {
                setErrorMessage("Invalid credentials")
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
                    { redirect == null ?
                    <Link href={`/sign-up`}>
                        <a className="pl-1 font-medium text-exclpurple hover:text-exclpurple-dark">
                            sign up for an account
                        </a>
                    </Link>
                    :
                    <Link href={`/sign-up?redirect=${redirect}`}>
                        <a className="pl-1 font-medium text-exclpurple hover:text-exclpurple-dark">
                            sign up for an account
                        </a>
                    </Link>
                    }
                </p>
                </div>
                <form className="mt-8 space-y-6" onSubmit={submit}>
                <input type="hidden" name="remember" value="true"/>
                <div className="rounded-md -space-y-px">
                    
                    <div className="pb-5">
                    <label htmlFor="email" className="sr-only">Email address</label>
                    <input id="email" name="email" type="email" autoComplete="email" required className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md rounded-b-md focus:outline-none focus:ring-exclpurple focus:border-exclpurple focus:z-10 sm:text-sm" placeholder="Email address" onChange={e => setEmail(e.target.value)}/>
                    </div>

                    <div>
                    <label htmlFor="password" className="sr-only">Password</label>
                    <input id="password" name="password" type="password" autoComplete="current-password" required className="appearance-none rounded-none relative block w-full px-3 py-2 border border-gray-300 placeholder-gray-500 text-gray-900 rounded-t-md rounded-b-md focus:outline-none focus:ring-exclpurple focus:border-exclpurple focus:z-10 sm:text-sm" placeholder="Password" onChange={e => setPassword(e.target.value)}/>
                    </div>
                    <p id="invalidCredentialsText" className="text-red-500 text-sm float-left pl-1 pb-5 pt-2">{errorMessage}</p>
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