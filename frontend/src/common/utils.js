export function randomId(length = 10) {
    return [...Array(length)].map(() => Math.random().toString(36)[2]).join('');
}

export function roundToStep(value, step) {
    if (value % step === 0) {
        return value;
    }

    const offset = value % step;
    
    const low = value - offset;
    const high = low + step;
    const mid = low + ((high - low) / 2);

    if (value < mid) {
        return low;
    } else {
        return high;
    }
}
