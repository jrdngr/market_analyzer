const BASE_URL = `http://localhost:3030`;

export async function getGammaExposureStats(symbol) {
    const url = `${BASE_URL}/gamma/${symbol}`;
    const data = await (await fetch(url)).json();
    return data;
}

export async function getGammaExposureAggregate(symbol) {
    const url = `${BASE_URL}/gamma/${symbol}/aggregate`;
    const data = await (await fetch(url)).json();
    return data;
}

export async function getQuote(symbol) {
    const url = `${BASE_URL}/quote/${symbol}`;
    const data = await (await fetch(url)).json();
    return data;
}

export async function getOhlc(symbol, interval) {
    const url = `${BASE_URL}/ohlc/${symbol}/${interval}`;
    const data = await (await fetch(url)).json();
    return data;
}
