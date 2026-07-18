export function formatTimestamp(rawTime: any) {
    if (!rawTime) return "";
    const fullDate = rawTime.split(" ")[0];
    const time = rawTime.split(" ")[1];
    const date = fullDate.split("-")[2]
    const month = fullDate.split("-")[1]
    const year = fullDate.split("-")[0].slice(-2)
    const seconds = time.split(":")[2]
    const minutes = time.split(":")[1]
    const hours = time.split(":")[0]
    return {date:`${date}/${month}/${year}`, time:`${hours}:${minutes}`};
}