import Head from 'next/head'
import Link from 'next/link'

export default function NewResourcePage(props) {
    

    return (
      
      <div className='bg-slate-800 pt-5' style={{fontFamily: "Roboto Mono"}}>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Create a new resource" />
          <meta name="robots" content="none" />
          <meta name="googlebot" content="none" />
          <meta name="referrer" content="no-referrer" />
          <title>Create an Account to Link - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />
        </Head>

        <div className='min-h-screen flex justify-center justify-items-center align-items-center'>
            <div>
                <h1 className='text-exclpurple font-bold text-7xl text-center'>Make an Account</h1>
                <h1 className='text-white text-5xl text-center'>Then ask bot for another URL</h1>
                <div className='flex pt-10 justify-center justify-content-center align-items-center'>
                  <Link href={"/sign-up"}>
                    <a className='bg-exclpurple hover:bg-exclpurple-dark duration-200 drop-shadow-lg py-5 px-10 rounded-xl'>
                      <h1 className='text-6xl text-white font-bold'>Sign Up</h1>
                    </a>
                  </Link>
                </div>
            </div>
            
        </div>
        
        
      </div>
  )
}