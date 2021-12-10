import Head from "next/head";
import Members from "../components/app/members";

export default function App() {

    return(
        <div>
            <Head>
                <title>Dashboard - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
        <       link rel="icon" href="/favicon.png" />
            </Head>

            <body>
                <Members/>
            </body>
        </div>
    )
}