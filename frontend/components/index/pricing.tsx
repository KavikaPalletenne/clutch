import Image from "next/image";
import React from "react";

export default function Pricing() {
    return (
        <>
            <div className="xl:mx-auto xl:container py-20 2xl:px-0 px-6">
                <div className="lg:flex items-center justify-between ml-10">
                    <div className=" lg:w-1/2 w-full">
                        <p className="text-base leading-4 text-gray-600">Choose your plan</p>
                        <h1 role="heading" className="md:text-5xl text-3xl font-bold leading-10 mt-3 text-exclpurple">
                            Our pricing
                        </h1>
                        <p role="contentinfo" className="text-base leading-5 mt-5 text-gray-600 mr-16">
                            Our notes might be free, but servers are not. Support ExamClutch&apos;s development by subscribing.
                        </p>
                        <div className="w-56">
                            
                        </div>
                    </div>
                    <div className="xl:w-1/2 lg:w-7/12 relative w-full lg:mt-0 mt-12 md:px-8" role="list">
                        <Image src="/colour_burst.png" layout="fill" className="absolute w-full -ml-12" alt="background circle images" />
                        <div role="listitem" className="bg-white cursor-pointer shadow rounded-lg p-8 relative z-30">
                            <div className="md:flex items-center justify-between">
                                <h2 className="text-2xl font-semibold leading-6 text-exclpurple">Starter</h2>
                                <p className="text-2xl font-semibold md:mt-0 mt-4 leading-6 text-gray-800">FREE</p>
                            </div>
                            <p className="md:w-80 text-base leading-6 mt-4 text-gray-600">Full access to all features. 20MB upload/download limit.</p>
                        </div>
                        <div role="listitem" className="bg-white cursor-pointer shadow rounded-lg mt-3 flex relative z-30">
                            <div className="w-2.5  h-auto rounded-tl-md rounded-bl-md" />
                            <div className="w-full p-8">
                                <div className="md:flex items-center justify-between">
                                    <h2 className="text-2xl font-semibold leading-6 text-exclpurple">Personal</h2>
                                    <p className="text-2xl md:mt-0 mt-4 font-semibold leading-6 text-gray-800">
                                        $5<span className="font-normal text-base">/mo</span>
                                    </p>
                                </div>
                                <p className="md:w-80 text-base leading-6 mt-4 text-gray-600">200MB upload/download limit.</p>
                            </div>
                        </div>
                        <div role="listitem" className="bg-white cursor-pointer shadow rounded-lg p-8 relative z-30 mt-3">
                            <div className="md:flex items-center justify-between">
                                <h2 className="text-2xl font-semibold leading-6 text-exclpurple">Community</h2>
                                <p className="text-2xl md:mt-0 mt-4 font-semibold leading-6 text-gray-800">
                                    $3<span className="font-normal text-base">/member/mo</span>
                                </p>
                            </div>
                            <p className="md:w-80 text-base leading-6 mt-4 text-gray-600">200MB upload limit for all members. Needs to be paid by single account.</p>
                        </div>
                    </div>
                </div>
            </div>
        </>
    );
}
