import type { NextPage } from 'next'
import Head from 'next/head'
import Link from 'next/link'
import Image from 'next/image'
import styles from '../styles/index.module.css'
import Features from '../components/index/features'
import Pricing from '../components/index/pricing'

const Home: NextPage = () => {
  return (
    <div style={{fontFamily: "Roboto Mono"}}>
      <Head>
        <title>ExamClutch - Study together</title>
        <meta name="description" content="Share notes and more with your group. Improve together and clutch your exams." />
        <meta name="robots" content="all" />
        <link rel="icon" href="/gradient_logo.svg" />
      </Head>
      <div style={{backgroundImage: "linear-gradient(225deg, rgba(140,154,255,1) 0%, rgba(194,144,255,1) 100%)"}}>
        <div>
          <section className="lg:pl-20 lg:pt-12 pl-5 pt-10 lg:pr-20 pr-5">
            <Link href="/">
              <a>
              <img src="/logo_white.svg" className="lg:w-80 w-36 relative float-left"/>
              </a>
            </Link>
            
            <nav>  
                <div className={styles.headerButtonSection}>
                    <Link href="/">
                        <a className={styles.headerSignInButton}>Sign in</a>
                    </Link>
                    <Link href="/">
                        <a className={styles.headerSignUpButton}>
                            Sign up
                        </a>
                    </Link>
                </div>
            </nav>
          </section>
        </div>
        
        <div className={styles.heroSection}>
            <h1 className={styles.heroSectionH1}>Study together.</h1>
            <h2 className={styles.heroSectionH2}>Clutch your exams.</h2>
            <div className={styles.heroImageContainer} >
              <img src="/homepage.svg" width="1000px"/>
            </div>
        </div>
      </div>
      <Features />
      <Pricing />
    </div>
  )
}

export default Home
