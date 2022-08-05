import ContentLoader from 'react-content-loader'
import Rect from 'react-content-loader'


export default function ResourceCard() {


    const ResourceLoader = () => (
        <ContentLoader>
          <rect x="3" y="17" rx="3" ry="3" width="275" height="25" />
          <rect x="3" y="70" rx="3" ry="3" width="200" height="10" />
          <rect x="3" y="85" rx="3" ry="3" width="175" height="10" />
          <rect x="3" y="100" rx="3" ry="3" width="190" height="10" />
        </ContentLoader>
      );

      const FileLoader = () => (
        <ContentLoader viewBox='0 0 275 10'>
          <rect x="3" y="0" rx="3" ry="3" width="190" height="10" />
        </ContentLoader>
      );


    return(
            <div>
                <div className="py-4 px-4 shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", maxWidth: "735px", minWidth: "735px"}}>

                <div className="inline-block">
                    {/* <Link href={ "/app/resource/"+ resource.id }>
                        <a> */}
                            {/* <h1 className="font-bold text-2xl pb-2 text-black inline-block duration-200">{ resource.title }</h1> */}
                        {/* </a>
                        <Cont
                    </Link> */}
                        <ResourceLoader />
                    <a>
                        {/* <h3 className="float-right text-exclpurple font-bold" >{ resource.subject }</h3> */}
                    </a>
                    <div className="flex">
                        {/* <UserName userId={resource.author} /> */}
                    </div>
                    {/* <h1>{ resource.description }</h1> */}
                </div>

                    <div className="px-4 py-1">
                        <a target={"_blank"} rel="noopener noreferrer">
                            <div className="py-4 px-4 shadow-sm hover:shadow-md inline-block rounded-2xl bg-white duration-150" style={{fontFamily: "Roboto Mono", minWidth: "675px", maxWidth: "750px"}}>
                                
                                <li className="justify-center float-left flex" style={{listStyle: 'none'}} key={"loading"}>
                                    <div className="pl-5 font-bold">
                                        <FileLoader />
                                    </div>
                                    
                                </li>
                                <div className="pr-5 float-right">
                                    {/* {prettyBytes(f.size)} */}
                                </div>
                            
                            </div>
                        </a>
                    </div>
                </div>
            </div>
        
    )
}