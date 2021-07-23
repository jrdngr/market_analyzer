const BASE_URL = `http://localhost:3030`;

export async  function getOptionChain(symbol) {
    const url = `${BASE_URL}/gamma/${symbol}`;
    const data = await (await fetch(url)).json();
    return data;
}

export async function getQuote(symbol) {
    const url = `${BASE_URL}/quote/${symbol}`;
    const data = await (await fetch(url)).json();
    return data;
}
