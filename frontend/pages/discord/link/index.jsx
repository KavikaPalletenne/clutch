import Head from 'next/head'
import Link from 'next/link'
import { useRouter } from 'next/router';
import {SyntheticEvent, useEffect, useState} from  'react'
import styles from '../../../styles/newResource.module.css'
import { GetServerSideProps } from "next";
import { FileReference } from "../../app/index"
import FileRender from "../../../components/app/FileRender"
import { AiOutlineClose } from "react-icons/ai"
import { pbkdf2Sync, randomUUID } from 'crypto';
import axios from 'axios';
import jwt_decode from "jwt-decode";
import { decode } from 'punycode';
import { FaDiscord } from 'react-icons/fa'

export default function NewResourcePage(props) {
    
    const router = useRouter()

    const [isBusy, setBusy] = useState(true);

    const [errorMessage, setErrorMessage] = useState('')

    const {autologin} = router.query

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

        await fetch("https://api.examclutch.com/api/auth/connect/discord", {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json',
            },
            credentials: 'include',
            body: JSON.stringify({
                'email': email,
                'password': password,
                'discord_token': props.token,
            })
        }).then((res) => {
            if (res.status == 200) {
                router.push("/discord/link/flow/success")
                return
            } else {
                setErrorMessage("Invalid credentials")
            }
            
        })
    }

    return (
      
      <div className='bg-gray-800 pt-5 min-h-screen' style={{fontFamily: "Roboto Mono"}}>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Create a new resource" />
          <meta name="robots" content="none" />
          <meta name="googlebot" content="none" />
          <meta name="referrer" content="no-referrer" />
          <title>Link your Discord Account - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />
        </Head>
        
        <div className='flex justify-content-center justify-center align-center align-items-center'>
            <div>
                <h1 className='text-white font-bold text-2xl'>Link your Discord account to ExamClutch</h1>
                <div className='flex justify-content-center text-4xl pt-10 font-bold align-items-center justify-center'>
                    <img width={100} src="/gradient_logo.svg" />
                    <h1 className='text-gray-600 px-3 text-2xl pt-1'>✖</h1>
                    <img width={50} src="/discord_white.svg" />
                </div>

                <div className='flex py-5 justify-content-center align-items-center justify-center'>
                <div className='flex justify-center justify-content-center align-items-center max-w-md w-full bg-gray-800 drop-shadow-lg rounded-lg px-10 py-2'>
                    <img className='rounded-full' width={50} src={`https://cdn.discordapp.com/avatars/${props.user_id}/${props.avatar_hash}.webp?size=4096`} />
                    <div className='flex justify-content-center align-items-center'>
                    <h1 className='text-exclpurple pt-1 pl-3 text-3xl'>{props.username}</h1>
                    </div>
                </div>
                </div>

                <div className='flex justify-content-center align-items-center justify-center'>
                
                <div className="max-w-md w-full bg-gray-800 drop-shadow-lg rounded-lg px-10 py-10 space-y-8">
                    <div>
                    <h2 className="mt-6 text-center text-3xl text-white" style={{fontFamily: "Space Mono", fontWeight: 'bold'}}>
                        Sign in to your ExCl account
                    </h2>
                    <p className="mt-2 text-center text-sm text-gray-300">
                        Or
                        <Link href="/discord/link/flow/sign-up">
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
                        <label htmlFor="email" className="sr-only">Email address</label>
                        <input id="email" name="email" type="email" autoComplete="email" required className="focus:ring-exclpurple bg-gray-700 text-white focus:border-exclpurple flex-1 block w-full rounded-l-md rounded-none rounded-r-md sm:text-sm border-gray-800" placeholder="Email address" onChange={e => setEmail(e.target.value)}/>
                        </div>

                        <div>
                        <label htmlFor="password" className="sr-only">Password</label>
                        <input id="password" name="password" type="password" autoComplete="current-password" required className="focus:ring-exclpurple bg-gray-700 text-white focus:border-exclpurple flex-1 block w-full rounded-l-md rounded-none rounded-r-md sm:text-sm border-gray-800" placeholder="Password" onChange={e => setPassword(e.target.value)}/>
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

            
        </div>

        


    </div>
  )
}

export async function getServerSideProps(context) {

  let { token } = context.query
  
  if (token == null) {
    return {
        redirect: {
            destination: '/discord/link/no-token',
            permanent: false,
        }
    }
  }

  
  var decoded = jwt_decode(token)


  return { props: {
      "token": token,
      "user_id": decoded["sub"],
      "username": decoded["username"],
      "avatar_hash": decoded["avatar_hash"],
    }
  }

}