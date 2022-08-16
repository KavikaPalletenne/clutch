import '../styles/globals.css'
import type { AppProps } from 'next/app'
import Link from 'next/link';
import Script from 'next/script';

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
    
    <Script id="google-analytics-gtag" strategy="lazyOnload" src={"https://www.googletagmanager.com/gtag/js?id=G-PWCR99JC9Z"} />
    <Script id="google-analytics-script" strategy="lazyOnload">
      {`
        window.dataLayer = window.dataLayer || [];
        function gtag(){dataLayer.push(arguments);}
        gtag('js', new Date());

        gtag('config', 'G-PWCR99JC9Z');
      `}
    </Script>

    <Script async strategy='lazyOnload' src="https://fundingchoicesmessages.google.com/i/pub-7136601653169605?ers=1" nonce="zolXR5gcRhueMtgtxmoTQg"></Script>
    <Script id="google-ad-blocking-popup" strategy='lazyOnload' nonce="zolXR5gcRhueMtgtxmoTQg">
      {`(function() {function signalGooglefcPresent() {if (!window.frames['googlefcPresent']) {if (document.body) {const iframe = document.createElement('iframe'); iframe.style = 'width: 0; height: 0; border: none; z-index: -1000; left: -1000px; top: -1000px;'; iframe.style.display = 'none'; iframe.name = 'googlefcPresent'; document.body.appendChild(iframe);} else {setTimeout(signalGooglefcPresent, 0);}}}signalGooglefcPresent();})();
      `}
    </Script>



    <div style={{fontFamily: 'Roboto Mono'}} className="bg-gradient-to-r from-exclpurple to-exclpurple-dark text-center text-white text-sm md:text-md">
        <h1>This is a beta version of ExamClutch not for public use (v.DEV.0.1.0). <Link href="/beta"><a style={{textDecorationLine:'underline'}}>Learn More</a></Link></h1> 
    </div>

    <Component {...pageProps} />

    <div className='bg-slate-500 text-white py-10 px-5 flex'>
      <Link href="/legal/privacy-policy">
        Contact
      </Link>
      <Link href="/legal/privacy-policy">
        Privacy
      </Link>
    </div>
  </div>
  )
}

export default MyApp
