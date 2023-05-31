import { Button, Text, Image, TextInput } from "@mantine/core";
import { IconCheck } from "@tabler/icons-react";

//@ts-ignore
const Uploaded = ({ result }) => {
    return (
        <div className="w-96 text-center my-40 mx-auto shadow-lg py-1 px-1">
            <IconCheck size={"2rem"} className="bg-[#219653] rounded-full mx-auto" color="white" />
            <Text fz={"18px"} c={"#4F4F4F"} my={20}>Uploaded Successfully!</Text>
            <img src={result.file} alt="image preview" />
            <div className="flex my-10">
                <TextInput ml={30} />
                <Button mx={"auto"} fz={"8px"} size="sm" className="bg-[#2F80ED]">Copy Link</Button>
            </div>
        </div>
    )
};

export default Uploaded;