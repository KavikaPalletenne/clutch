import '../styles/globals.css'
import type { AppProps } from 'next/app'

import Router from 'next/router';
import axios from 'axios'
import NProgress from 'nprogress';
import '../styles/custom.nprogress.css'; //styles of nprogress//Binding events. 
Router.events.on('routeChangeStart', () => NProgress.start()); Router.events.on('routeChangeComplete', () => NProgress.done()); Router.events.on('routeChangeError', () => NProgress.done());

function MyApp({ Component, pageProps }: AppProps) {
  return (
  <div>
    <link rel="preconnect" href="https://fonts.googleapis.com" />
    <link rel="preconnect" href="https://fonts.gstatic.com" crossOrigin="true" /> 
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Space+Mono:wght@400;700&display=swap" />
    <link rel="stylesheet" href="https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@400;700&display=swap" />
    <link rel="preconnect" href="https://fonts.googleapis.com/css2?family=Space+Mono:wght@400;700&display=swap" />
    <link rel="preconnect" href="https://fonts.googleapis.com/css2?family=Roboto+Mono:wght@400;700&display=swap" />

    <div style={{fontFamily: 'Roboto Mono'}} className="bg-gradient-to-r from-exclpurple to-exclpurple-dark text-center text-white text-sm md:text-md">
        <h1>This is a beta version of ExamClutch not for public use (v.DEV.0.1.0). <a href="/beta" style={{textDecorationLine:'underline'}}>Learn More</a> </h1> 
    </div>

    <Component {...pageProps} />
  </div>
  )
}

export default MyApp
