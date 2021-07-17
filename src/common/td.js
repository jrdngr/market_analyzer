const AUTH_URL = `https://api.tdameritrade.com/v1/oauth2/token`;
const OPTION_CHAIN_URL = `https://api.tdameritrade.com/v1/marketdata/chains`;
const CLIENT_ID_KEY = `CLIENT_ID`;
const CODE_KEY = `CODE`;

export function storeClientId(clientId) {
    localStorage.setItem(CLIENT_ID_KEY, clientId);
}

export function clientId() {
    return localStorage.getItem(CLIENT_ID_KEY);
}

export function storeCode(code) {
    localStorage.setItem(CODE_KEY, code);
}

export function code() {
    return localStorage.getItem(CODE_KEY);
}

export async function login() {
    const body = {
        grant_type: `authorization_code`,
        access_type: `offline`,
        code: code(),
        client_id: `${clientId()}@AMER.OAUTHAP`,
        redirect_uri: `https://localhost:5000/auth`,
    };

    console.log(body);

    const response = await fetch(AUTH_URL, {
        method: 'POST',
        body: JSON.stringify(body),
    });
    
    const result = await response.json();
    console.log(result);
    
    return result;
}

export async function getOptionChain(symbol) {
    const apiKey = apiKey();
    const url = `${OPTION_CHAIN_URL}?apikey=${apiKey}&symbol=${symbol}`;
    const data = await (await fetch(url)).json();
    return data;
}
