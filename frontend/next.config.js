/** @type {import('next').NextConfig} */
module.exports = {
  reactStrictMode: true,
  async redirects() {
    return [
      {
        source: '/ads.txt',
        destination: 'http://srv.adstxtmanager.com/19390/examcldutch.com',
        permanent: true,
      },
    ]
  }
}
