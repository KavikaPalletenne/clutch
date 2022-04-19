import Head from 'next/head'
import Link from 'next/link'
import { useRouter } from 'next/router';
import {SyntheticEvent, useEffect, useState} from  'react'
import { GetServerSideProps } from "next";
import { File } from "."
import FileRender from "../../../../components/app/FileRender"
import { AiOutlineClose } from "react-icons/ai"

export default function NewResourcePage(props) {
    
    const router = useRouter()
    const { id } = router.query

    

    const [isBusy, setBusy] = useState(true);

    const [errorMessage, setErrorMessage] = useState('')

    const {autologin} = router.query

    const [tagInputPlaceHolder, setTagInputPlaceHolder] = useState('Enter tags separated by commas...')

    var isMounted = false
    var userIdLoaded = false
    var loggedIn = false

    const [isKeyReleased, setIsKeyReleased] = useState(false);

    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [subject, setSubject] = useState('');
    const [files, setFiles] = useState([]);

    const [tagInput, setTagInput] = useState('');
    const [tags, setTags] = useState([]);

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
        setTagInputPlaceHolder('Enter tags separated by commas...')
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

    const onFileChange = (e) => {
      files.push(e.target.files)
    }

    const deleteTag = (index) => {
      setTags(prevState => prevState.filter((tag, i) => i !== index))
    }

    const submit = async (e) => {
        e.preventDefault()

        fetch(`http://localhost:443/resource/create`, {
            method: 'POST',
            headers: {
                'Content-Type': 'application/json'
            },
            body: JSON.stringify({
                'group_id': id,
                'title': title,
                'description': description,
                'subject': subject,
                'tags': tags,
                'files': [
                    {
                        'id': 'asldijflaskdjf',
                        'title': fileTitle,
                        'size': fileSize,
                    }
                ]
            })
        })
    }
    
    return (
      
      <div style={{fontFamily: "Roboto Mono"}}>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Create a new resource" />
          <meta name="robots" content="none" />
          <meta name="googlebot" content="none" />
          <title>Create New Resource - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />
        </Head>
        <Link href={`/app/group/${id}`}>
        <a className="text-exclpurple">
            Return to Group
        </a>
        </Link>

        <div className="justify-center">
        <div className="">
        <div className="md:grid md:grid-cols-3 md:gap-6">
          <div className="mt-5 md:mt-0 md:col-span-2">
            <form onSubmit={submit} id="my-form">
              <div className="shadow sm:rounded-md sm:overflow-hidden">
                <div className="px-4 py-5 bg-white space-y-6 sm:p-6">
                  <div className="grid grid-cols-3 gap-6">
                    <div className="col-span-3 sm:col-span-2">
                      <label htmlFor="company-website" className="block text-sm font-medium text-gray-700">
                        Title
                      </label>
                      <div className="mt-1 flex rounded-md shadow-sm">
                        <input
                          onChange={e => setTitle(e.target.value)}
                          type="text"
                          name="title"
                          id="title"
                          className="focus:ring-exclpurple focus:border-exclpurple flex-1 block w-full rounded-l-md rounded-none rounded-r-md sm:text-sm border-gray-300"
                          placeholder="Physics notes..."
                        />
                      </div>
                    </div>
                  </div>

                  <div className="grid grid-cols-3 gap-6">
                    <div className="col-span-3 sm:col-span-2">
                      <label htmlFor="company-website" className="block text-sm font-medium text-gray-700">
                        Subject
                      </label>
                      <div className="">
                        <select onChange={e => setSubject(e.target.value)} className="rounded-md shadow-sm focus:border-exclpurple focus:ring-exclpurple border-gray-300" name="cars" id="cars">
                            <optgroup style={{fontFamily: "Roboto Mono"}}>
                                <option value="General">General</option>
                                <option value="Physics">Physics</option>
                                <option value="Maths Methods">Maths Methods</option>
                                <option value="Specialist Maths">Specialist Maths</option>
                                <option value="Biology">Biology</option>
                                <option value="Chemistry">Chemistry</option>
                                <option value="English">English</option>
                                <option value="Literature">Literature</option>
                                <option value="Economics">Economics</option>
                            </optgroup>
                        </select>
                      </div>
                    </div>
                  </div>

                  <div className="flex border overflow-x-auto border-gray-300 shadow-sm rounded-md py-2 px-3">
                    {tags.map((tag, index) => (
                      <div key={index} className="flex max-h-10 inline-grid justify-center bg-exclpurple rounded-2xl py-2 px-2 text-white mr-2">
                        {tag}

                        {/* <button className={styles.tag.button} onClick={() => deleteTag(index)}>
                          <AiOutlineClose color="white" className="text-white" />
                        </button> */}
                        
                      </div>
                    ))}
                    <input 
                    className="outline-none h-10"
                      value={tagInput}
                      placeholder={tagInputPlaceHolder}
                      onKeyDown={onKeyDown}
                      onKeyUp={onKeyUp} 
                      onChange={onChange}
                    />
                </div>
                  

                  <div>
                    <label htmlFor="about" className="block text-sm font-medium text-gray-700">
                      Description
                    </label>
                    <div className="mt-1">
                      <textarea
                        onChange={e => setDescription(e.target.value)}
                        id="description"
                        name="description"
                        rows={3}
                        className="shadow-sm focus:ring-exclpurple focus:border-exclpurple mt-1 block w-full sm:text-sm border border-gray-300 rounded-md"
                        placeholder="Tell the group a bit about this resource..."
                        defaultValue={''}
                      />
                    </div>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">Files</label>
                    <div className="mt-1 flex justify-center px-6 pt-5 pb-6 border-2 border-gray-300 border-dashed rounded-md">
                      <div className="space-y-1 text-center">
                        <svg
                          className="mx-auto h-12 w-12 text-gray-400"
                          stroke="currentColor"
                          fill="none"
                          viewBox="0 0 48 48"
                          aria-hidden="true"
                        >
                          <path
                            d="M28 8H12a4 4 0 00-4 4v20m32-12v8m0 0v8a4 4 0 01-4 4H12a4 4 0 01-4-4v-4m32-4l-3.172-3.172a4 4 0 00-5.656 0L28 28M8 32l9.172-9.172a4 4 0 015.656 0L28 28m0 0l4 4m4-24h8m-4-4v8m-12 4h.02"
                            strokeWidth={2}
                            strokeLinecap="round"
                            strokeLinejoin="round"
                          />
                        </svg>
                        <div className="flex text-sm text-gray-600">
                          <label
                            htmlFor="file-upload"
                            className="relative cursor-pointer bg-white rounded-md font-medium text-exclpurple hover:text-exclpurple-dark focus-within:outline-none focus-within:ring-2 focus-within:ring-offset-2 focus-within:ring-exclpurple"
                          >
                            <span>Upload files</span>
                              <input 
                                id="file-upload"
                                name="file-upload"
                                type="file"
                                className="sr-only"
                                onChange={e => onFileChange(e)}
                              />
                          </label>
                          <p className="pl-1">or drag and drop</p>
                        </div>
                        <p className="text-xs text-gray-500">PNG, JPG, PDF, DOCX, up to 20MB</p>                      
                      </div>
                    </div>
                    <div id="uploaded-files">
                        {/* <FileRender propFiles={files} /> */}
                    </div>
                  </div>
                </div>
                
                <div className="px-4 py-3 bg-gray-50 text-right sm:px-6">
                  <button
                    type="submit"
                    className="inline-flex justify-center py-2 px-4 border border-transparent shadow-sm text-sm font-medium rounded-md text-white bg-exclpurple hover:shadow-md duration-150 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-exclpurple"
                  >
                    Create
                  </button>
                </div>
              </div>
            </form>
          </div>
        </div>
      </div>
      </div>
        


    </div>
  )
}

export async function getServerSideProps(context) {

    let groupId = context.params.id

    return {
        props: { groupId: groupId },
    }
}