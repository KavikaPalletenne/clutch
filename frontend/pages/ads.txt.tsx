import { GetServerSideProps } from "next";

export default function AdsTxtPage() {
  return(
    <div></div>
  )
}
export const getServerSideProps: GetServerSideProps = async (context: any) => {

  return {
    redirect: {
        destination: 'https://srv.adstxtmanager.com/19390/examclutch.com',
        permanent: true,
    }
}
}