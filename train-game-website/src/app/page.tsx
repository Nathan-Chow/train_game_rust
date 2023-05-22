"use client";

import { useState } from "react";
import NumberForm, { formProp, solObj } from "./NumbersForm";

export default function Home() {
  const [solutions, setSolutions] = useState<solObj>({
    allSolutions: [],
    numSolutions: 0,
  });
  
  const [errors, setErrors] = useState<string>("");

  const [submitted, setSubmitted] = useState<boolean>(false);
  
  const numFormProps: formProp = {
    setSol: setSolutions,
    setErr: setErrors,
    setSub: setSubmitted,
  }

  console.log(solutions);
  console.log(errors);
  console.log(submitted);

  return (
    <div className="min-h-screen flex flex-col items-center justify-center bg-yellow text-grey">
      <div className="p-12 font-bold">
        <h1 className="mb-6 text-4xl xl:text-5xl" >Train Game</h1>
        <h2 className="mb2 text-2xl xl:text-3xl tracking-tight">
          <span>Sydney Train Game calculator</span>
        </h2>
        <NumberForm {...numFormProps}/>
      </div>
    </div>
  )
}
