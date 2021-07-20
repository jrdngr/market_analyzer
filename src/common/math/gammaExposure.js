
export function gammaExposureByPrice(optionsData) {
    let strikeToGammaExposure = {};

    for (const option of optionsData) {
        let exposure = option.greeks.gamma * option.open_interest;
        if (option.option_type === "put") {
            exposure *= -1;
        }
        if (!strikeToGammaExposure[option.strike]) {
            strikeToGammaExposure[option.strike] = exposure;
        } else {
            strikeToGammaExposure[option.strike] += exposure;
        }
    }

    return gammaExposureStats(strikeToGammaExposure);
}

function gammaExposureStats(strikeToGammaExposure) {
    let positiveSum = 0.0;
    let positiveCount = 0;
    let negativeSum = 0.0;
    let negativeCount = 0;
    let maximum = 0.0;
    let minimum = 0.0;
    let absoluteMaximum = 0.0;
    let weightedPositiveSum = 0.0;
    let weightedNegativeSum = 0.0;

    for (const [strike, exposure] of  Object.entries(strikeToGammaExposure)) {
        if (exposure >= 0.0) {
            positiveSum += exposure;
            weightedPositiveSum += strike * exposure;
            positiveCount += 1;
        } else {
            negativeSum += exposure;
            weightedNegativeSum += strike * exposure;
            negativeCount += 1;
        }
        maximum = Math.max(maximum, exposure);
        minimum = Math.min(minimum, exposure);
        absoluteMaximum = Math.max(absoluteMaximum, Math.abs(exposure));
    }

    positiveCount = Math.max(positiveCount, 1);
    negativeCount = Math.max(negativeCount, 1);

    let averagePositiveExposure = positiveSum / positiveCount;
    let averageNegativeExposure = negativeSum / negativeCount;
    let averageAbsoluteExposure =
        (Math.abs(positiveSum) + Math.abs(negativeSum)) / (positiveCount + negativeCount);

    let weightedAverageAbsolutePrice =
        (Math.abs(weightedPositiveSum) + Math.abs(weightedNegativeSum))
            / (Math.abs(positiveSum) + Math.abs(negativeSum));
        
    let weightedAveragePositivePrice = weightedPositiveSum / positiveSum;
    let weightedAverageNegativePrice = weightedNegativeSum / negativeSum;

    const prices = Object.entries(strikeToGammaExposure).map(([s, e]) => { 
        return { strike: s, gammaExposure: e};
    });

    prices.sort((p1, p2) => p1.strike - p2.strike);

    const result = {
        prices,
        averageAbsoluteExposure,
        averagePositiveExposure,
        averageNegativeExposure,
        maximum,
        minimum,
        absoluteMaximum,
        weightedAverageAbsolutePrice,
        weightedAveragePositivePrice,
        weightedAverageNegativePrice,
    };

    return result;
}
