import { FaDiscord, FaFile, FaPencilAlt,FaChalkboardTeacher } from 'react-icons/fa'

const features = [
  {
    name: 'Discord integration',
    description:
      'Interact with your community using Discord. Access notes directly using our Discord bot. Join VCs and chat while studying.',
    icon: FaDiscord,
  },
  {
    name: 'Share files',
    description:
      'Upload your notes onto the community hub and share with other students. Search for notes by name, subject or topic.',
    icon: FaFile,
  },
  {
    name: 'Search handwritten notes',
    description:
      'AI automatically identifies the words in your handwritten notes as soon as you upload them. Allowing both you and other students to easily organise and find them.',
    icon: FaPencilAlt,
  },
  {
    name: 'No more teacher disparity',
    description:
      'Get access to notes from the best teachers in your school and around the state. You no longer have to be stuck with bad teachers.',
    icon: FaChalkboardTeacher,
  },
]

export default function Features() {
  return (
    <div className="py-12 bg-white">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="lg:text-center">
          <p className="mt-2 text-3xl leading-8 font-extrabold tracking-tight text-gray-900 sm:text-4xl text-exclpurple">
            A better way to study
          </p>
          <p className="mt-4 max-w-2xl text-xl text-gray-500 lg:mx-auto">
            Software is better when it&apos;s open-source. Why not studying?
          </p>
        </div>

        <div className="mt-10">
          <dl className="space-y-10 md:space-y-0 md:grid md:grid-cols-2 md:gap-x-8 md:gap-y-10">
            {features.map((feature) => (
              <div key={feature.name} className="relative">
                <dt>
                  <div className="absolute flex items-center justify-center h-12 w-12 rounded-md text-white" style={{backgroundImage: "linear-gradient(225deg, rgba(140,154,255,1) 0%, rgba(194,144,255,1) 100%)"}}>
                    <feature.icon className="h-6 w-6" aria-hidden="true" />
                  </div>
                  <p className="ml-16 text-lg leading-6 font-medium text-gray-900">{feature.name}</p>
                </dt>
                <dd className="mt-2 ml-16 text-base text-gray-500">{feature.description}</dd>
              </div>
            ))}
          </dl>
        </div>
      </div>
    </div>
  )
}