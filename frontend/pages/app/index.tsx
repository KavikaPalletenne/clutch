import Head from 'next/head';
import { useRouter } from 'next/router';
import { useEffect, useState } from 'react';
import Cookies from 'cookies'
import { GetServerSideProps } from "next";
import { NextResponse } from 'next/server';
import { ServerResponse } from 'http';

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

export const getServerSideProps: GetServerSideProps = async ({req, res}) => {
    
    
    let cookies = new Cookies(req, res)
    const groups = await fetch(`https://api.examclutch.com/api/user/get_user_groups/${cookies.get("user_id")}`, {
        credentials: 'include',
        headers: req ? {cookie: req.cookies.value } : undefined
    });
    
    if (!groups.ok) {
        if (groups.status == 401) {
            return {
                redirect: {
                    destination: '/api/login',
                    permanent: false,
                }
            }
        }
    }
    
    const user_groups = await groups.json() as string[];

    if (!user_groups) {
        return {
            redirect: {
                destination: '/api/login',
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