import Head from "next/head";

export default function App() {

    const resource = {
        id: "cd505bba-1bb4-4ff7-b3b3-f57854d0099e",
        title: "Electronegativity",
        description: "Electronegativity chart that we will get on the exam",
        files: [
            {"id": "1bf1f868-6aa7-4821-9aff-12002114c360","title": "electronegativity_chart.png", "size": "1.5MB"},
            {"id": "33adf97e-afe8-4d28-95b7-99eba22bee8d", "title": "data_booklet_2021.pdf", "size": "2.5MB"}
        ]
    }

    return(
        <div>
            <Head>
                <title>Dashboard - ExamClutch</title>
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