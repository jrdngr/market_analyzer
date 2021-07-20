const ACCESS_TOKEN_KEY = `ACCESS_TOKEN`;

const BASE_URL = `https://api.tradier.com/v1`;

export function storeAccessToken(accessToken) {
    localStorage.setItem(ACCESS_TOKEN_KEY, accessToken);
}

export function accessToken() {
    return localStorage.getItem(ACCESS_TOKEN_KEY);
}

export async function getOptionExpirations(symbol) {
    const url = `${BASE_URL}/markets/options/expirations?symbol=${symbol}&includeAllRoots=true`;

    const data = await (await fetch(url, {
        method: 'GET',
        headers: {
            'Accept': `application/json`,
            'Authorization': `Bearer ${accessToken()}`,
        },
    })).json();
    console.log(data);
    return data.expirations;
}

export async  function getOptionChain(symbol) {
    const expiration = (await getOptionExpirations(symbol)).date[0];
    const url = `${BASE_URL}/markets/options/chains?symbol=${symbol}&expiration=${expiration}&greeks=true`;

    const data = await (await fetch(url, {
        method: 'GET',
        headers: {
            'Accept': `application/json`,
            'Authorization': `Bearer ${accessToken()}`,
        },
    })).json();
    console.log(data);
    return data.options.option;
}

export async function getQuote(symbol) {
    const url = `${BASE_URL}/markets/quotes?symbols=${symbol}`;

    const data = await (await fetch(url, {
        method: 'GET',
        headers: {
            'Accept': `application/json`,
            'Authorization': `Bearer ${accessToken()}`,
        },
    })).json();
    console.log(data);
    return data.quotes.quote;
}
