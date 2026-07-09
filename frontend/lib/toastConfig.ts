import { Slide, type ToastPosition } from "react-toastify";

export const toastConfig = {
    position: "bottom-left" as ToastPosition,
    autoClose: 3000,
    hideProgressBar: true,
    closeOnClick: false,
    pauseOnHover: true,
    draggable: true,
    progress: undefined,
    theme: "dark",
    transition: Slide,
}
