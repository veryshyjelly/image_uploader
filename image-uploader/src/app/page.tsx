"use client"

import { Text, Image, FileButton, Button } from "@mantine/core";
import axios from "axios";
import { useState } from "react";
import Dropzone from "react-dropzone";
import Uploading from "./uploading";
import Uploaded from "./uploaded";

export default function Home() {
  const [progress, setProgress] = useState(0);
  const [isuploading, setisUploading] = useState(false);
  const [result, setResult] = useState({ link: "" });

  const baseurl = typeof window !== 'undefined' && window.location.origin
    ? window.location.origin
    : '';

  return (
    <>
      {!isuploading && <Upload setResult={setResult}
        setisUploading={setisUploading} setProgress={setProgress} />}
      {isuploading && result.link === "" && <Uploading progressValue={progress} />}
      {result.link !== "" && <Uploaded link={baseurl + result.link} />}
    </>
  )

}

// @ts-ignore
const Upload = ({ setisUploading, setProgress, setResult }) => {
  const baseurl = typeof window !== 'undefined' && window.location.origin
    ? window.location.origin
    : '';

  const setFileHandler = async (file: File) => {
    console.log(file);
    const formdata = new FormData();
    formdata.append("file", file);
    setisUploading(true);

    let res = await axios.request({
      method: "post",
      url: baseurl + "/upload",
      data: formdata,
      onUploadProgress: (p) => {
        setProgress((p.loaded * 100) / (p.total || 100))
        console.log(p);
      }
    });
    console.log(res.data);
    setResult(res.data);
  }

  return (
    <div className="flex flex-col w-72 px-2 mx-auto rounded-lg my-20 text-center shadow-lg">
      <Text fz={"18px"} c={"#4F4F4F"}>Upload your image</Text>
      <Text c={"#828282"} my={15} fz={"10px"}>File should be Jpeg, Png,..</Text>
      <Dropzone onDrop={(acceptedFiles) => acceptedFiles.forEach(f => setFileHandler(f))}
        // @ts-ignore
        accept={"image/png,image/jpeg"}>
        {({ getRootProps, getInputProps, isDragActive, isDragReject }) =>
          <div {...getRootProps()} className="px-10 my-3 justify-center py-5 select-none
          border-2 rounded-xl  border-[#97BEF4]" style={{
              borderStyle: !isDragActive ? "dashed" : "solid",
              borderColor: isDragReject ? "red" : "#97BEF4",
            }}>
            <input {...getInputProps()} />
            <Image src={"/home/image.svg"} width={100} mx={"auto"} alt="Image" />
            <Text c={"#BDBDBD"} fz={"12px"} mt={25}>Drag & Drop your image here</Text>
          </div>}</Dropzone>
      <Text c={"#BDBDBD"} fz={"12px"}>Or</Text>
      <div className="py-5">
        <FileButton onChange={setFileHandler} accept="image/png,image/jpeg">
          {(props) => <Button size="sm" radius={"md"} fz={"12px"} color="blue" className="bg-[#2F80ED]" {...props}>Choose a file</Button>}
        </FileButton>
      </div>
    </div >
  )

}
