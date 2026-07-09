import Image from "next/image";

export default function Home() {
  return (
    <div className="text-4xl font-bold">
      home

      <p className="font-sans bg-testbg">This reads in clean, rounded Quicksand text.</p>
      <p className="font-mono">This pulls your Nunito style properties.</p>
    </div>
  );
}
