import Head from 'next/head'
import Link from 'next/link'

export default function Custom404() {
    return (
        <div style={{backgroundImage: `url(/hero_background.png)`}}>

        <Head>
          <meta charSet="utf-8"/>
          <meta name="description" content="404 Error - Page not found." />
          <meta name="robots" content="all" />
          <meta name="googlebot" content="all" />
          <title>Page not Found - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />
        </Head>

        
        <body className="h-screen min-h-screen overflow-hidden">
            
            <div className="lg:pt-10 lg:pl-32 text-white lg:pr-32 pt-10 pl-5 pr-5">
                <Link href="/app">
                <a>
                    <img className="float-left lg:w-96 w-12" src="logo_white.svg"/>
                </a>
                </Link>
            </div>

            <div className="container lg:pl-32 lg:pt-48 pt-24 pl-5">
                <div className="items-center lg:flex">
                    <div className="w-full lg:w-2/3">
                        <div className="max-w-2xl pr-4">
                        <h1 className="text-7xl font-bold text-orange dark:text-white lg:text-9xl" style={{fontFamily: 'Roboto Mono'}}>404</h1>
                            <h1 className="text-3xl font-bold text-white dark:text-white lg:text-7xl" style={{fontFamily: 'Roboto Mono'}}>Page not found</h1>
                            {/* <p className="mt-2 lg:text-4xl text-xl text-gray-300 dark:text-gray-400 pb-5">If only it had a ScanTag...</p> */}
                        </div>
                    </div>
            
                    {/* <div className="lg:pt-0 lg:pl-0 pl-10 pt-5 lg:absolute lg:bottom-0 lg:right-20">
                        <img className="lg:w-96 w-60 max-h-screen" src="https://raw.githubusercontent.com/KavikaPalletenne/scantag-assets/main/scantag-mockup-edited2.png" alt=""/>
                    </div> */}
                </div>
            </div>
        </body>



        </div>
    )
}