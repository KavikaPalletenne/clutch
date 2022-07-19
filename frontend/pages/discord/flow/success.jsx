import Head from 'next/head'

export default function NewResourcePage(props) {

    return (
      
      <div className='bg-slate-800 pt-5' style={{fontFamily: "Roboto Mono"}}>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Create a new resource" />
          <meta name="robots" content="none" />
          <meta name="googlebot" content="none" />
          <meta name="referrer" content="no-referrer" />
          <title>Create New Resource for {props.group_name} - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />
        </Head>

        <div className='min-h-screen flex justify-center justify-items-center align-items-center'>
            <div>
                <h1 className='text-exclpurple font-bold text-7xl text-center'>Successfully uploaded a resource</h1>
                <h1 className='text-white text-5xl text-center'>You can close this window now</h1>
            </div>
        </div>
        
        
      </div>
  )
}