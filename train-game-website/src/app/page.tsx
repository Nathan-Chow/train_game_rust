"use client";

import NumberForm from "./form";

export default function Home() {
  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-yellow text-grey">
      <div className="p-12 font-bold">
        <h1 className="mb-6 text-4xl xl:text-5xl" >Train Game</h1>
        <h2 className="mb2 text-2xl xl:text-3xl tracking-tight">
          <span>Sydney Train Game calculator</span>
        </h2>
        <NumberForm/>
      </div>
    </div>
  )
}
