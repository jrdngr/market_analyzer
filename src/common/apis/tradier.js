const ACCESS_TOKEN_KEY = `ACCESS_TOKEN`;

const BASE_URL = `https://api.tradier.com/v1/`;

export function storeAccessToken(accessToken) {
    localStorage.setItem(ACCESS_TOKEN_KEY, accessToken);
}

export function clientId() {
    return localStorage.getItem(ACCESS_TOKEN_KEY);
}

export function getOptionExpirations(symbol) {
    const url = `${BASE_URL}/markets/options/expirations?symbol=${symbol}&includeAllRoots=true&strikes=true`;

    const data = await (await fetch(url, {
        method: 'GET',
        headers: {
            'Accept': `application/json`,
            'Authorization': `Bearer ${accessToken}`,
        },
    })).json();
    console.log(data);
    return data;
}

export function getOptionChain(symbol) {
    const expiration = ``;
    const url = `${BASE_URL}/markets/options/chains?symbol=${symbol}&expiration=${expiration}&greeks=true`;

    const data = await (await fetch(url, {
        method: 'GET',
        headers: {
            'Accept': `application/json`,
            'Authorization': `Bearer ${accessToken}`,
        },
    })).json();
    return data;
}
