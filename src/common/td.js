const AUTH_URL = `https://api.tdameritrade.com/v1/oauth2/token`;
const OPTION_CHAIN_URL = `https://api.tdameritrade.com/v1/marketdata/chains`;
const CLIENT_ID_KEY = `CLIENT_ID`;
const CODE_KEY = `CODE`;

export function storeClientId(clientId) {
    localStorage.setItem(CLIENT_ID_KEY, clientId);
}

export function clientId() {
    let apiKey = localStorage.getItem(CLIENT_ID_KEY);
    return `${apiKey}@AMER.OAUTHAP`;
}

export function clientIdUrl() {
    let apiKey = localStorage.getItem(CLIENT_ID_KEY);
    return `${apiKey}%40AMER.OAUTHAP`;
}

export function storeCode(code) {
    localStorage.setItem(CODE_KEY, code);
}

export function code() {
    return localStorage.getItem(CODE_KEY);
}

export function codeUrl() {
    let code = localStorage.getItem(CODE_KEY);
    return encodeURIComponent(code);
}

export async function login() {
    const body = {
        grant_type: `authorization_code`,
        access_type: `offline`,
        code: code(),
        client_id: clientId(),
        redirect_uri: `https://localhost:5000/auth`,
    };

    const response = await fetch(AUTH_URL, {
        method: 'POST',
        body: JSON.stringify(body),
    });
    
    const result = await response.json();
    console.log(result);
    
    return result;
}

export async function getOptionChain(symbol) {
    const url = `${OPTION_CHAIN_URL}?apikey=${clientId()}&symbol=${symbol}`;
    const data = await (await fetch(url)).json();
    return data;
}
