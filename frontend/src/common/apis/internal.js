const BASE_URL = `http://localhost:3030/graphql`;

export async function getGammaExposure(symbol, options) {
    return (await graphql_request(`query GammaExposure($symbol: String!, $options: GammaExposureOptions) {
        gammaExposure(symbol: $symbol, options: $options) {
            prices {
                strike,
                gammaExposure,
            }
            maximum,
            minimum,
            absoluteMaximum,
            weightedAverageAbsolutePrice,
            absoluteMinimumPrice,
        }
    }`, { symbol, options })).gammaExposure;
}

export async function getQuote(symbol) {
    return (await graphql_request(`query Quote($symbol: String!){
        quote(symbol: $symbol) {
            symbol,
            last,
            change,
            volume,
            open,
            high,
            low,
            close,
        }
    }`, { symbol })).quote
}

export async function getOhlc(symbol, interval) {
    return (await graphql_request(`query Ohlc($symbol: String!, $interval: String = "5min"){
        ohlc(symbol: $symbol, interval: $interval) {
            time,
            price,
            open,
            high,
            low,
            close,
            volume,
            vwap,
        }
    }`, { symbol, interval })).ohlc
}

async function graphql_request(query, variables) {
    const escapedQuery = query
        .replace(/\n/g, "\\n")
        .replace(/"/g, "\\\"");

    const variables_json = variables ? JSON.stringify(variables) : "{}";

    const response = await fetch(BASE_URL, {
        method: 'POST',
        headers: {
            'Content-Type': 'application/json',
        },
        body: `{ "query": "${escapedQuery}", "variables": ${variables_json} }`,
    });
    return (await response.json()).data;
}
