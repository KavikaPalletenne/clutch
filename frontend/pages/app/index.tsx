import Head from "next/head";
import Members from "../../components/app/Members";
import GroupTitle from "../../components/app/GroupTitle";
import ResourceCard from "../../components/app/ResourceCard";

export default function App() {

    return(
        <div>
            <Head>
                <title>Dashboard - ExamClutch</title>
                <meta name="description" content="Exam Clutch Dashboard" />
                <meta name="robots" content="noindex" />
        <       link rel="icon" href="/favicon.png" />
            </Head>

            <div className="flex items-start justify-center pt-10">
                <GroupTitle />
                <Members />
            </div>

            <div className="flex inline-grid">
                <ResourceCard />
                <ResourceCard />
                <ResourceCard />
                <ResourceCard />
                <ResourceCard />
            </div>

        </div>
    )
}