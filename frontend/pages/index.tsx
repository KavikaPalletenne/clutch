import type { NextPage } from 'next'
import Head from 'next/head'
import Link from 'next/link'
import Image from 'next/image'
import styles from '../styles/index.module.css'
import Features from '../components/index/features'
import Pricing from '../components/index/pricing'
import { FaDiscord } from 'react-icons/fa'

const Home: NextPage = () => {
  return (
    <div style={{fontFamily: "Roboto Mono"}}>
      <Head>
        <title>ExamClutch - Study together</title>
        <meta name="description" content="Share notes and more with your group. Improve together and clutch your exams." />
        <meta name="robots" content="index" />
        <meta name="googlebot" content="index" />
        <link rel="icon" href="/gradient_logo.svg" />
      </Head>
      
      <div className='-z-30 absolute'>
      <Image className='-z-30 absolute' priority quality={100} width={2560} height={2000} src="/hero_background.png" />
      </div>
      <div style={{backgroundImage: `url(/hero_background.png)`}}>{/* <div className='z-30'> */}
        <div>
          <section className="lg:pl-20 lg:pt-12 pl-5 pt-10 lg:pr-20 pr-5">
            <Link href="/">
              <a>
              <img src="/logo_white.svg" className="lg:w-80 w-36 relative float-left"/>
              </a>
            </Link>
            
            <nav>  
                <div className={styles.headerButtonSection}>
                    <Link href="https://discord.com/api/oauth2/authorize?client_id=917954795384500236&redirect_uri=https%3A%2F%2Fapi.examclutch.com%2Fapi%2Fauth%2Foauth2%2Fdiscord%2Fredirect&response_type=code&scope=identify%20email%20guilds">
                        <a className={styles.headerSignUpButton}>
                            <FaDiscord className="pr-2 lg:w-8"/>
                            Sign in
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
