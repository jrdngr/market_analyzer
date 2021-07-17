const OPTION_CHAIN_URL = `https://api.tdameritrade.com/v1/marketdata/chains`;
const API_KEY_KEY = `API_KEY`

export async function GetOptionChain(symbol) {
    const apiKey = LoadApiKey();
    const url = `${OPTION_CHAIN_URL}?apikey=${apiKey}&symbol=${symbol}`;
    const data = await (await fetch(url)).json();
    return data;
}

export function StoreApiKey(key) {
    localStorage.setItem(API_KEY_KEY, key);
}

const LoadApiKey = () => {
    return localStorage.getItem(API_KEY_KEY);
}
