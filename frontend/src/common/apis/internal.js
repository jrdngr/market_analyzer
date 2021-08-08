const BASE_URL = `http://localhost:3030`;

export async function getGammaExposure(symbol, options) {
    return (await graphql_request(`{
        gammaExposure(symbol: "${symbol}") {
            prices {
                strike,
                gammaExposure,
            }
            maximum,
            minimum,
            absoluteMaximum,
        }
    }`)).gammaExposure;
}

export async function getQuote(symbol) {
    return (await graphql_request(`{
        quote(symbol: "${symbol}") {
            symbol,
            last,
            change,
            volume,
            open,
            high,
            low,
            close,
        }
    }`)).quote
}

export async function getOhlc(symbol, interval) {
    return (await graphql_request(`{
        ohlc(symbol: "${symbol}", interval: "${interval}") {
            time,
            price,
            open,
            high,
            low,
            close,
            volume,
            vwap,
        }
    }`)).ohlc
}

async function graphql_request(query) {
    const escapedQuery = query
        .replace(/\n/g, "\\n")
        .replace(/"/g, "\\\"");

    const response = await fetch(BASE_URL, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: `{ "query": "${escapedQuery}" }`,
    });
    return (await response.json()).data;
}
