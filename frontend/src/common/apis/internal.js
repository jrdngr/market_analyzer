const BASE_URL = `http://localhost:3030`;

export async function getGammaExposure(symbol, options) {
    let params = new URLSearchParams();
    let hasParams = false;
    if (options.aggregate) {
        params.append("aggregate", true);
        hasParams = true;
    }
    if (options.fresh) {
        params.append("fresh", true);
        hasParams = true;
    }

    let url = `${BASE_URL}/gamma/${symbol}`;
    if (hasParams) {
        url += `?${params.toString()}`;
    }

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
