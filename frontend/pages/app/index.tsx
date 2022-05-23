import Head from 'next/head';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import Cookies from 'cookies'
import { GetServerSideProps } from "next";

export default function App() {


    return(
        <div className="bg-bg-gray-50">
            <Head>
                <title>Redirecting... - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
        <       link rel="icon" href="/favicon.png" />
            </Head>

            <div className="flex items-start justify-center pt-10">
            </div>

            <div className="flex inline-grid">
                
            </div>

        </div>
    )
}

export const getServerSideProps: GetServerSideProps = async (context: any) => {
    
    
    let cookies = new Cookies(context.req, context.res)

    if (cookies.get("user_id") == undefined || cookies.get("auth_token") == undefined) {
        return {
            redirect: {
                destination: '/login',
                permanent: false,
            }
        }
    }

    // let check_login = await fetch('https://api.examclutch.com/api/auth/authorize', {
    //     method: 'GET',
    //     credentials: 'include',
    //     // headers: req ? { cookie: req.cookies.value } : undefined
    //     headers: {
    //         'Cookie': `${cookies.get("auth_token")}`
    //     }
    // });

    // // If not valid auth_token, then prompt to login
    // if (check_login.status == 401) {
    //     return {
    //         redirect: {
    //             destination: `/login-no-authorize-${cookies.get("auth_token")}`,
    //             permanent: false,
    //         }
    //     }
    // }

 
    const res = await fetch(`https://api.examclutch.com/api/group/user_groups/${cookies.get("user_id")}`, {
        credentials: 'include',
        headers: context.req ? {cookie: context.req.headers.cookie} : undefined
    });

    if (!res.ok) {
        if (res.status == 401) {
            return {
                redirect: {
                    destination: `/login`,
                    permanent: false,
                }
            } 
        }
        return {
            redirect: {
                destination: `/login`,
                permanent: false,
            }
        } 
    }

    const user_groups = await res.json() as string[]
    

    // if (!user_groups) {
    //     return {
    //         redirect: {
    //             destination: '/login',
    //             permanent: false,
    //         }
    //     }
    // }
    
    if (user_groups.length == 0) {
        return {
            redirect: {
                destination: `/app/join`,
                permanent: false,
            }
        }  
    }    

    return {
        redirect: {
            destination: `/app/group/${user_groups[0]}`,
            permanent: false,
        }
    }
}