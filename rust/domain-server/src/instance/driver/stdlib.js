function gainFactorToDb(gainFactor) {
    return 20 * Math.log10(gainFactor);
}

function dbToGainFactor(db) {
    return Math.pow(10, db / 20);
}

function lpad(_value, count, padding = ' ') {
    let value = _value.toString();
    while (value.length < count) {
        value = padding + value;
    }
    return value;
}