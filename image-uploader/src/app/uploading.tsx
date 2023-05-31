import { Progress, Text } from "@mantine/core";

const Uploading = ({ progressValue }) => {
    return (
        <div className="shadow-lg w-80 mx-auto my-40 px-10 py-2 rounded-xl">
            <Text c={"#4F4F4F"} fz={"18px"}>Uploading...</Text>
            <Progress value={progressValue} my={30} />
        </div>
    )
}

export default Uploading;