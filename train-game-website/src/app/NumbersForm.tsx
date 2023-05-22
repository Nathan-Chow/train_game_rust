"use client";

import { Dispatch, SetStateAction } from "react";

export type solObj = {
  allSolutions: string[],
  numSolutions: number,
}

export interface formProp {
  setSol: Dispatch<SetStateAction<solObj>>,
  setErr: Dispatch<SetStateAction<string>>,
  setSub: Dispatch<SetStateAction<boolean>>,
}

export default function NumberForm(props: formProp) {
  async function handleSubmit(event: any) {
    event.preventDefault();

    const data = {
      numbers: String(event.target.numbers.value)
    }
    
    const response = await fetch("https://train-game-v2.shuttleapp.rs/train_game", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    })
    .then((response) => {
      if (response.ok) {
        return response.json()
      }
    })
    .then((response) => {
      const sols = response as solObj;
      props.setSol(sols);
      props.setErr("");
      props.setSub(true);
    })
    .catch(err => {
      //todo
    });
  }

  return (
    <form onSubmit={handleSubmit}>
      <div className="flex flex-col">
        <label>
          Numbers
        </label>
        <input autoComplete="off" type="text" id="numbers"></input>
      </div>
      <button className="my-2 p-2 bg-grey text-yellow rounded-md" type="submit">Solve</button>
    </form>
  );
}
