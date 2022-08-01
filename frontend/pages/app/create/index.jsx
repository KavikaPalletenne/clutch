import Head from 'next/head'
import Link from 'next/link'
import { useRouter } from 'next/router';
import {SyntheticEvent, useEffect, useState} from  'react'
import styles from '../../../styles/newResource.module.css'
import { GetServerSideProps } from "next";
import { FileReference } from "../../app/index"
import FileRender from "../../../components/app/FileRender"
import { AiOutlineClose } from "react-icons/ai"
import { pbkdf2Sync, randomUUID } from 'crypto';
import axios from 'axios';
import jwt_decode from "jwt-decode";
import { decode } from 'punycode';

export default function NewResourcePage(props) {
    
    const router = useRouter()
    const { id } = router.query

    const fileReader = () => new FileReader();

    

    const [isBusy, setBusy] = useState(true);

    const [errorMessage, setErrorMessage] = useState('')

    const {autologin} = router.query

    const [authToken, setAuthToken] = useState('')

    const [tagInputPlaceHolder, setTagInputPlaceHolder] = useState('Separate by commas...')

    var isMounted = false
    var userIdLoaded = false
    var loggedIn = false
    var url = `/discord/flow/cancelled`

    const [isKeyReleased, setIsKeyReleased] = useState(false);

    const [name, setName] = useState('');
    const [description, setDescription] = useState('');
    const [privateGroup, setPrivateGroup] = useState(false);
    
    const [loading, setLoading] = useState(false);
    const [submitted, setSubmitted] = useState(false);
    const [createFailed, setCreateFailed] = useState(false);


    useEffect(() => {
        /**
        function CheckLoggedIn() {
            if (localStorage.getItem('token') != null && autologin != 'false') {
                router.push("/account/tags")
            }
            return
        }
        CheckLoggedIn()
        */
        
        setAuthToken(localStorage.getItem('auth_token'))
    }, [])

    const onChange = (e) => {
      const { value } = e.target;
      setTagInput(value);
    };

    const onKeyDown = (e) => {
      const { key } = e;
      const trimmedInput = tagInput.trim();
      
      if (tags.length > 0) {
        setTagInputPlaceHolder('')
      }
      
      if (tags.length == 0) {
        setTagInputPlaceHolder('Separate by commas...')
      }
      
      
      if (key === ',' && trimmedInput.length && !tags.includes(trimmedInput)) {
        e.preventDefault();
        setTags(prevState => [...prevState, trimmedInput]);
        setTagInput('');
      }
    
      if (key === "Backspace" && !tagInput.length && tags.length && isKeyReleased) {
        const tagsCopy = [...tags];
        const poppedTag = tagsCopy.pop();
        e.preventDefault();
        setTags(tagsCopy);
        setTagInput(poppedTag);
      }
    
      setIsKeyReleased(false);
    };
    
    const onKeyUp = () => {
      setIsKeyReleased(true);
    }

    const onFileChange = (fileInput) => {
      // setFiles([])
      // setFileData([])
      var fileArray = Array.from(fileInput.files)
      var file;
      fileArray.forEach( (f) =>
      { file = {
        "name": f.name,
        "size": f.size,
      }
      files.push(file) }
      );
      
      setFileData(fileArray)
      setFileType(fileArray[0].type)

      setListFiles(
      fileArray.map((f) => 
      <div key={f.name} className={"pt-3"}>
        <a className={"rounded-lg bg-white shadow-md py-1 px-1 text-black"}>{f.name}</a>
      </div>
      ))
      
    }

    const deleteTag = (index) => {
      setTags(prevState => prevState.filter((tag, i) => i !== index))
    }

    const submit = async (e) => {
        e.preventDefault()
        setCreateFailed(false)
      if(!submitted) {
        setSubmitted(true)

        if (name.length == 0) {
          setCreateFailed(true)
          return;
        }

        fetch(`https://api.examclutch.com/api/group/create`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
                // 'Cookie': `auth_token=${authToken}`,
            },
            body: JSON.stringify({
                'name': name,
                'description': description,
                'discord_id': '',
                'private': privateGroup,
            })
        }).then(r =>  r.json().then(async function(data) {
          if (r.ok) {
            router.push(`/app/group/${data.id}`)
            return
          }
          setCreateFailed(true);
          setSubmitted(false);
        
        }
        ))
      }
    }

    return (
      
      <div className='bg-gray-50 md:pl-96 pt-5' style={{fontFamily: "Roboto Mono"}}>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Create a new group. Share files and learn together." />
          <meta name="robots" content="index" />
          <meta name="googlebot" content="index" />
          <meta name="referrer" content="no-referrer" />
          <title>Create New Group - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />          
        </Head>

        
        <div className='min-h-screen'>
        <div>
          <div className='flex justify-items-center align-items-center text-center pb-5'>
          <div className='font-bold text-exclpurple text-4xl flex inline-flex text-center justify-items-center align-items-center shadow-sm'>
          <h1>Create a new Group</h1>
          </div>
          <div className='flex justify-items-center align-items-center'>
          <p className="mt-2 text-center text-sm text-gray-600">
            Or
            <Link href="/app/join">
                <a className="pl-1 font-medium text-exclpurple hover:text-exclpurple-dark duration-300">
                    Join a group
                </a>
            </Link>
          </p>
          </div>
          </div>
        <div className="drop-shadow-xl md:grid md:grid-cols-3 md:gap-6">
          <div className="mt-5 md:mt-0 md:col-span-2">
            <form onSubmit={submit} id="my-form">
              <div className="shadow sm:rounded-md sm:overflow-hidden">
                <div className="px-4 py-5 bg-white space-y-6 sm:p-6">
                  <div className="grid grid-cols-3 gap-6">
                    <div className="col-span-3 sm:col-span-2">
                      <label htmlFor="name" className="block text-sm font-medium text-gray-500">
                        Name
                      </label>
                      <div className="mt-1 flex rounded-md shadow-sm">
                        <input
                          onChange={e => setName(e.target.value)}
                          type="text"
                          name="name"
                          id="name"
                          className="focus:ring-exclpurple bg-white text-black focus:border-exclpurple flex-1 block w-full rounded-l-md rounded-none rounded-r-md sm:text-sm border-white"
                          placeholder="Choose a short & descriptive name..."
                        />
                      </div>
                    </div>
                  </div>

                  

                  

                  <div>
                    <label htmlFor="description" className="block text-sm font-medium text-gray-500">
                      Description
                    </label>
                    <div className="mt-1">
                      <textarea
                        onChange={e => setDescription(e.target.value)}
                        id="description"
                        name="description"
                        rows={3}
                        className="shadow-sm focus:ring-exclpurple bg-white text-black focus:border-exclpurple mt-1 block w-full sm:text-sm border border-white rounded-md"
                        placeholder="A group for friends..."
                        defaultValue={''}
                      />
                    </div>
                  </div>

                  <label htmlFor="description" className="block text-sm font-medium text-gray-500">
                      Privacy
                  </label>
                  <label htmlFor="default-toggle" className="inline-flex relative items-center cursor-pointer">
                  <input type="checkbox" value="" id="default-toggle" className="sr-only peer" onChange={e => setPrivateGroup(e.target.checked)} />
                  <div className="w-11 h-6 bg-gray-200 peer-focus:outline-none peer-focus:ring-2 peer-focus:ring-exclpurple dark:peer-focus:ring-exclpurple rounded-full peer dark:bg-gray-700 peer-checked:after:translate-x-full peer-checked:after:border-white after:content-[''] after:absolute after:top-[2px] after:left-[2px] after:bg-white after:border-gray-300 after:border after:rounded-full after:h-5 after:w-5 after:transition-all dark:border-gray-600 peer-checked:bg-exclpurple-dark"></div>
                  { privateGroup ?
                    <span className="ml-3 font-medium text-exclpurple font-bold text-md">Private</span> :
                    <span className="ml-3 font-medium text-exclpurple font-bold text-md">Public</span>
                  }
                </label>
                  
                </div>
                
                <div className="px-4 py-3 bg-white text-right sm:px-6">
                  <Link href={'/app'}>
                  <a className="pr-1">
                  <button
                    className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-red-500 hover:shadow-md duration-150"
                  >
                    Cancel
                  </button>
                  </a>
                  </Link>
                  
                  <button
                    type="submit"
                    disabled={submitted}
                    className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-exclpurple hover:shadow-md duration-150">
                    Create
                  </button>
                </div>
              </div>
            </form>
            { createFailed ?
              <div className='float-right pr-10 text-red-500'>
                Failed create group
              </div> : null
            }
          </div>
        </div>
      </div>
      </div>


    </div>
  )
}

export async function getServerSideProps(context) {

  let { token } = context.query
  
  if (token == null) {
    return { props: {
      "greeting": "Hello",
      "token": "token",
      "group_name": "Group",
      "user_id": "user_id",
      "username": "Username",
      "avatar_hash": "avatar_hash",
    }
  }
  }

  
  var decoded = jwt_decode(token)

  let greeting_array = [
    "Hello", "Hola", "Bonjour", "Ciao", "G'day"
  ]

  let greeting = greeting_array[Math.floor(Math.random() * greeting_array.length)];

  return { props: {
      "greeting": greeting,
      "token": token,
      "group_name": decoded["group_name"],
      "user_id": decoded["sub"],
      "username": decoded["username"],
      "avatar_hash": decoded["avatar_hash"],
    }
  }

}