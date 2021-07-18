const AUTH_URL = `https://api.tdameritrade.com/v1/oauth2/token`;
const OPTION_CHAIN_URL = `https://api.tdameritrade.com/v1/marketdata/chains`;
const AUTH_REDIRECT_URL = `https://localhost:5000/auth`;
const CLIENT_ID_KEY = `CLIENT_ID`;
const REFRESH_TOKEN_KEY = `REFRESH_TOKEN`;
const ACCESS_TOKEN_KEY = `ACCESS_TOKEN`;

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

export function storeRefreshToken(token) {
    localStorage.setItem(REFRESH_TOKEN_KEY, token);
}

export function refreshToken() {
    return localStorage.getItem(REFRESH_TOKEN_KEY);
}

export function storeAccessToken(token) {
    localStorage.setItem(ACCESS_TOKEN_KEY, token);
}

export function accessToken() {
    return localStorage.getItem(ACCESS_TOKEN_KEY);
}

export function logout() {
    localStorage.removeItem(ACCESS_TOKEN_KEY);
    localStorage.removeItem(REFRESH_TOKEN_KEY);
    console.log("Logged out");
}

export async function login(code) {
    const body = {
        grant_type: `authorization_code`,
        access_type: `offline`,
        code: encodeURIComponent(code),
        client_id: clientIdUrl(),
        redirect_uri: AUTH_REDIRECT_URL,
    };

    const response = await fetch(AUTH_URL, {
        method: 'POST',
        headers: {
            'Accept': '*/*',
            'Accept-Encoding': 'gzip',
            'Accept-Language': 'en-US',
            'Content-Type': 'application/x-www-form-urlencoded',
            'DNT': 1,
            'Host': 'api.tdameritrade.com',
            'Sec-Fetch-Dest': 'empty',
            'Sec-Fetch-Mode': 'cors',
            'Sec-Fetch-Site': 'same-site'
        },
        body: objectToFormBody(body),
    });

    const result = await response.json();
    storeAccessToken(result.access_token);
    storeRefreshToken(result.refresh_token);
    
    return result;
}

export async function updateAccessToken() {
    const body = {
        grant_type: `refresh_token`,
        refresh_token: encodeURIComponent(refreshToken()),
        client_id: clientIdUrl(),
    };

    const response = await fetch(AUTH_URL, {
        method: 'POST',
        headers: {
            'Accept': '*/*',
            'Accept-Encoding': 'gzip',
            'Accept-Language': 'en-US',
            'Content-Type': 'application/x-www-form-urlencoded',
            'DNT': 1,
            'Host': 'api.tdameritrade.com',
            'Sec-Fetch-Dest': 'empty',
            'Sec-Fetch-Mode': 'cors',
            'Sec-Fetch-Site': 'same-site'
        },
        body: objectToFormBody(body),
    });

    const result = await response.json();
    storeAccessToken(result.access_token);
    storeRefreshToken(result.refresh_token);
    
    return result;
}

export async function updateRefreshToken() {
    const body = {
        grant_type: `refresh_token`,
        access_type: `offline`,
        refresh_token: encodeURIComponent(refreshToken()),
        client_id: clientIdUrl(),
    };

    const response = await fetch(AUTH_URL, {
        method: 'POST',
        headers: {
            'Accept': '*/*',
            'Accept-Encoding': 'gzip',
            'Accept-Language': 'en-US',
            'Content-Type': 'application/x-www-form-urlencoded',
            'DNT': 1,
            'Host': 'api.tdameritrade.com',
            'Sec-Fetch-Dest': 'empty',
            'Sec-Fetch-Mode': 'cors',
            'Sec-Fetch-Site': 'same-site'
        },
        body: objectToFormBody(body),
    });

    const result = await response.json();
    storeRefreshToken(result.refresh_token);
    
    return result;
}

export async function getOptionChain(symbol) {
    let accessToken = localStorage.getItem(ACCESS_TOKEN_KEY);
    if (accessToken) {
        const url = `${OPTION_CHAIN_URL}?symbol=${symbol.toUpperCase()}`;
        const data = await (await fetch(url, {
            method: 'GET',
            headers: {
                'Authorization': `Bearer ${accessToken}`,
            },
        })).json();
        return data;
    } else {
        const url = `${OPTION_CHAIN_URL}?apikey=${clientIdUrl()}&symbol=${symbol.toUpperCase()}`;
        const data = await (await fetch(url)).json();
        return data;
    }
}

function objectToFormBody(obj) {
    let result = ``;

    for (const [key, value] of Object.entries(obj)) {
        if (result.length > 0) {
            result += `&`;
        }
        result += `${key}=${value}`;
    }

    return result;
}
