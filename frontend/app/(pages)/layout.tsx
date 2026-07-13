import { Toaster } from "react-hot-toast";

export default function PagesLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return <>
    {children}
    <Toaster
      position="bottom-left"
      reverseOrder={true}
      toastOptions={{
        style: {
          background: '#333',
          color: '#fff',
        },
      }}
    />
  </>;
}
