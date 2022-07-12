import Head from 'next/head'
import Link from 'next/link'
import { useRouter } from 'next/router';
import {SyntheticEvent, useEffect, useState} from  'react'
import styles from '../../../styles/newResource.module.css'
import { GetServerSideProps } from "next";
import { FileReference } from "../../app/index"
import FileRender from "../../../components/app/FileRender"
import { AiOutlineClose } from "react-icons/ai"
import { randomUUID } from 'crypto';
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
    var url = `/discord/flow/cancelled  `

    const [isKeyReleased, setIsKeyReleased] = useState(false);

    const [title, setTitle] = useState('');
    const [description, setDescription] = useState('');
    const [subject, setSubject] = useState('General');
    const [files, setFiles] = useState([]);
    const [fileData, setFileData] = useState([]);
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
                'group_id': id,
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
            router.push(`/discord/create/success`)
          }
          if (response.status == 401) {
            router.push(`/discord/create/failed`)
          }
        })
        .catch(err => {
          console.error('Request failed', err)
        });
      }
    }

    return (
      
      <div className='bg-slate-800 pt-5' style={{fontFamily: "Roboto Mono"}}>

        <Head>
          <meta charSet="utf-8" />
          <meta name="description" content="Create a new resource" />
          <meta name="robots" content="none" />
          <meta name="googlebot" content="none" />
          <meta name="referrer" content="no-referrer" />
          <title>Create New Resource for {props.group_name} - ExamClutch</title>
          <link rel="icon" href="/gradient_logo.svg" />
        </Head>

        <div className='min-h-screen flex justify-center justify-items-center align-items-center'>
            <div>
                <h1 className='text-exclpurple font-bold text-7xl text-center'>Cancelled uploading a resource</h1>
                <h1 className='text-white text-5xl text-center'>You can close this window now</h1>
            </div>
        </div>
        
        
      </div>
  )
}