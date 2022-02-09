import Link from 'next/link';
import { useState, useEffect } from 'react';
import { GetServerSideProps } from "next";


export default function FileRender(props) {

  const [files, setFiles] = useState([])

  setFiles(props.propFiles)

    if (files.length = 0) { return (
      <div className="font-bold text-exclpurple">
        <h1>No Files</h1>
      </div>
    )}
    
    return (
      <div className="font-bold text-exclpurple">
        <h1>{files[0].name}</h1>
      </div>
    )
}