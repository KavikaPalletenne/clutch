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

    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [subject, setSubject] = useState('General');
    const [files, setFiles] = useState([]);
    const [fileData, setFileData] = useState([]);
    const [fileType, setFileType] = useState('');
    const [fileUrls, setFileUrls] = useState('');

    const [cancelUrl, setCancelUrl] = useState(`/app/group/${id}`)
    
    const [loading, setLoading] = useState(false);
    const [submitted, setSubmitted] = useState(false);

    const [tagInput, setTagInput] = useState('');
    const [tags, setTags] = useState([]);
    const [listFiles, setListFiles] = useState(<h1></h1>);
    const [fileProgress, setFileProgress] = useState(0.0);

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
        
      if(!submitted) {
        setSubmitted(true)
        setFileProgress(0.0) 
        if (title.length == 0 || subject.length == 0) {
          return;
        }

        fetch(`https://api.examclutch.com/api/discord/resource/create?token=${props.token}`, {
            method: 'POST',
            credentials: 'include',
            headers: {
                'Content-Type': 'application/json',
                // 'Cookie': `auth_token=${authToken}`,
            },
            body: JSON.stringify({
                'user_id': '',
                'group_id': '',
                'title': title,
                'description': description,
                'subject': subject,
                'tags': tags,
                'files': files,
            })
        }).then(r =>  r.json().then(async function(data) {
          setFileUrls(data['file_put_urls'][0])
          console.log(`Urls: ${fileUrls}`)
          setLoading(true)
          setCancelUrl(`/api/discord/resource/cancel/${data.group_id}/${data['resource_id']}/delete?token=${props.token}`)
          url = `/api/discord/resource/cancel/${data.group_id}/${data['resource_id']}/delete?token=${props.token}`

          return await axios.request({
          headers: {
            'content-type': fileType,
          },
          method: "put", 
          url: `${data['file_put_urls'][0]}`, 
          data: fileData[0],
          onUploadProgress: (p) => {
            console.log(p); 
            setFileProgress(p.loaded/p.total);
          }
          })
          // return fetch(`${data['file_put_urls'][0]}`, {
          //       method: 'PUT',
          //       credentials: 'include',
          //       headers: {           
          //       },
          //       body: fileData[0]
          // });
        }
        ))
        .then(response => {
          console.log('File Upload Status:' + response.status)
          setLoading(false)
          if (response.status == 200) {
            setFileProgress(1.0)
            router.push(`/discord/flow/success`)
          }
          if (response.status == 401) {
            router.push(`/discord/flow/failed`)
          }
        })
        .catch(err => {
          console.error('Request failed', err)
        });
      }
    }

    return (
      
      <div className='bg-gray-800 md:pl-96 pt-5' style={{fontFamily: "Roboto Mono"}}>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Create a new resource" />
          <meta name="robots" content="none" />
          <meta name="googlebot" content="none" />
          <meta name="referrer" content="no-referrer" />
          <title>Create New Resource for {props.group_name} - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />

          <script async src="https://pagead2.googlesyndication.com/pagead/js/adsbygoogle.js?client=ca-pub-7136601653169605"
          crossOrigin="anonymous"></script>
        </Head>

        
        <div className='min-h-screen'>
        <div>
          <div className='text-white text-2xl inline-flex'>
          <h1 className=''>{props.greeting}</h1>
          <div className='flex inline-flex'>
          {/* <img className='rounded-full' width={40} src={`https://cdn.discordapp.com/avatars/${props.user_id}/${props.avatar_hash}.webp?size=512`} /> */}
          <h1 className='pl-2 text-exclpurple'>{props.username}</h1>
          </div>
          </div>
          <div>
          <div className='font-bold text-white text-4xl flex inline-flex text-center justify-items-center align-items-center shadow-sm'>
          <h1>Upload a resource for</h1>
          <h1 className='text-exclpurple pl-4'>{props.group_name}</h1>
          </div>
          </div>
        <div className="drop-shadow-xl md:grid md:grid-cols-3 md:gap-6">
          <div className="mt-5 md:mt-0 md:col-span-2">
            <form onSubmit={submit} id="my-form">
              <div className="shadow sm:rounded-md sm:overflow-hidden">
                <div className="px-4 py-5 bg-gray-800 space-y-6 sm:p-6">
                  <div className="grid grid-cols-3 gap-6">
                    <div className="col-span-3 sm:col-span-2">
                      <label htmlFor="title" className="block text-sm font-medium text-gray-300">
                        Title
                      </label>
                      <div className="mt-1 flex rounded-md shadow-sm">
                        <input
                          onChange={e => setTitle(e.target.value)}
                          type="text"
                          name="title"
                          id="title"
                          className="focus:ring-exclpurple bg-gray-700 text-white focus:border-exclpurple flex-1 block w-full rounded-l-md rounded-none rounded-r-md sm:text-sm border-gray-800"
                          placeholder="Choose a descriptive title..."
                        />
                      </div>
                    </div>
                  </div>

                  <div className="grid grid-cols-3 gap-6">
                    <div className="col-span-3 sm:col-span-2">
                      <label htmlFor="subject" className="block text-sm font-medium text-gray-300">
                        Subject
                      </label>
                      <div className="">
                        <select onChange={e => setSubject(e.target.value)} className="rounded-md bg-gray-700 text-white shadow-sm focus:border-exclpurple focus:ring-exclpurple border-gray-800" name="cars" id="cars">
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
                                <option value="French">French</option>
                                <option value="Japanese">Japanese</option>
                                <option value="Latin">Latin</option>
                                <option value="Chinese">Chinese</option>
                                <option value="Psychology">Psychology</option>
                                <option value="Enviro Science">Enviro Science</option>
                                <option value="Legal Studies">Legal Studies</option>
                                <option value="Accounting">Accounting</option>
                                <option value="Business Management">Business Management</option>
                            </optgroup>
                        </select>
                      </div>
                    </div>
                  </div>

                  <div>
                  <label htmlFor="tags" className="block pb-1 text-sm text-sm font-medium text-gray-300">
                        Tags
                  </label>
                  <div className="flex border bg-gray-700 overflow-x-auto border-gray-800 focus-within:border-exclpurple focus-within:focus:ring-exclpurple shadow-sm rounded-md py-2 px-3">
                    {tags.map((tag, index) => (
                      <div key={index} className="flex max-h-10 inline-grid justify-center bg-exclpurple rounded-2xl py-2 px-2 text-white mr-2">
                        {tag}

                        {/* <button className={styles.tag.button} onClick={() => deleteTag(index)}>
                          <AiOutlineClose color="white" className="text-white" />
                        </button> */}
                        
                      </div>
                    ))}
                    <input 
                    className="outline-none h-10 bg-gray-700 text-white"
                      value={tagInput}
                      placeholder={tagInputPlaceHolder}
                      onKeyDown={onKeyDown}
                      onKeyUp={onKeyUp} 
                      onChange={onChange}
                    />
                </div>
                </div>

                  <div>
                    <label htmlFor="description" className="block text-sm font-medium text-gray-300">
                      Description
                    </label>
                    <div className="mt-1">
                      <textarea
                        onChange={e => setDescription(e.target.value)}
                        id="description"
                        name="description"
                        rows={3}
                        className="shadow-sm focus:ring-exclpurple bg-gray-700 text-white focus:border-exclpurple mt-1 block w-full sm:text-sm border border-gray-800 rounded-md"
                        placeholder="Tell the group a bit about this resource..."
                        defaultValue={''}
                      />
                    </div>
                  </div>

                  <div>
                    <label htmlFor="about" className="block text-sm font-medium text-gray-700">
                      
                    </label>
                    <div className="mt-1">
                      <input 
                      id={'fileUpload'} type={"file"} 
                      className={styles.uploadButton}
                      onChange={e => onFileChange(e.target)} 
                      multiple={false}
                      accept={"image/*, .pdf, .doc, .docx"}
                      />
                      <label htmlFor="fileUpload" className={styles.uploadButtonLabel}>Select files</label>
                      {listFiles}
                      <div className='text-gray-300 pt-2'>
                        Upload Progress: {fileProgress *100 + "%"}
                      </div>
                    </div>
                  </div>
                </div>
                
                <div className="px-4 py-3 bg-gray-800 text-right sm:px-6">
                  <Link href={url}>
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