"use client";

export default function NumberForm() {
  async function handleSubmit(event: any) {
    event.preventDefault();

    const data = {
      numbers: String(event.target.numbers.value)
    }
    console.log(data)
    
    const response = await fetch("http://127.0.0.1:8000/train_game", {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify(data),
    })

    console.log(response.json())
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