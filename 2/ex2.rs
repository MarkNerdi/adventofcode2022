use std::str::FromStr;

fn main() {

    let data: &str = "A Z\nA Y\nB X\nB X\nC X\nB X\nA X\nA X\nC X\nA X\nA X\nA Y\nB X\nA Y\nC X\nC X\nA X\nA Y\nC X\nB X\nA X\nB X\nA X\nA X\nB Y\nB Y\nA X\nA X\nA X\nA X\nA X\nA Y\nA X\nB X\nB Y\nA Y\nC Z\nC X\nA X\nA Y\nA X\nA X\nA Y\nA X\nA Y\nA X\nB X\nC X\nA X\nB X\nA X\nC X\nA Y\nB X\nB X\nA X\nA X\nC Y\nB X\nA X\nC X\nB X\nB X\nA X\nA X\nB X\nA Y\nC X\nA Y\nA X\nA X\nA Y\nB X\nA X\nA X\nA X\nB Y\nA X\nA X\nA Y\nA X\nA X\nB Y\nB Y\nB X\nC X\nB Y\nA X\nC X\nC X\nA Y\nA X\nA Z\nB X\nC X\nA Y\nB X\nA X\nA X\nA X\nA Y\nC X\nA X\nA Z\nA X\nA Y\nB Y\nA X\nC X\nB X\nA Y\nB Z\nB X\nC Y\nB X\nB Z\nA Y\nB X\nC X\nC Y\nA X\nA Y\nA X\nB X\nA Y\nB Y\nA X\nC Y\nA Y\nB Y\nA X\nB Y\nB X\nB Y\nA X\nB Y\nA X\nC Z\nA X\nC X\nA Y\nA X\nC X\nA X\nA X\nA X\nC X\nB Z\nA X\nB Y\nB X\nA Y\nA X\nA X\nB Y\nB Y\nA Y\nB X\nA X\nC Y\nA X\nC X\nA X\nA X\nA X\nA X\nA X\nC Z\nC X\nA Z\nC Z\nA X\nA Y\nB X\nC X\nB X\nB Y\nC X\nB X\nA X\nA X\nA X\nA Y\nB X\nB Y\nA X\nC X\nA X\nA X\nA X\nA X\nC Y\nA Z\nC X\nA X\nA Y\nB X\nA Y\nA X\nA Y\nA X\nA X\nA X\nA X\nA Y\nA X\nB X\nA X\nA X\nC Y\nC Y\nA X\nC X\nA X\nA X\nC X\nC X\nB Z\nB X\nB X\nA X\nA Y\nB Y\nA X\nA X\nC X\nC X\nC X\nB X\nC X\nB Y\nA X\nB Y\nB X\nB X\nA X\nB Y\nA Y\nC X\nB X\nC X\nA Y\nB X\nC X\nA X\nC X\nA X\nB Y\nA X\nA X\nB Z\nC X\nB X\nC X\nB X\nA X\nA X\nA X\nA X\nA X\nC X\nC X\nA X\nA Z\nA Y\nA X\nA X\nA X\nA X\nC Y\nC Y\nB Z\nA X\nA X\nB X\nB X\nA Y\nA X\nB Y\nA X\nB Y\nA X\nC Y\nC X\nA Y\nB Y\nC X\nA X\nA X\nA X\nA Y\nA X\nA X\nA X\nB Y\nB X\nC X\nC Y\nA Y\nA Z\nB X\nC X\nB X\nC X\nA X\nB Z\nB Y\nA X\nC X\nA Y\nB X\nB X\nA Y\nA X\nA X\nA Y\nA X\nB X\nA X\nA X\nC Y\nC Y\nA Y\nA Y\nB Y\nB Y\nB Y\nB X\nA X\nA Y\nB Y\nB X\nA Y\nA X\nA Y\nC X\nC X\nB Y\nA Y\nB Y\nA Y\nA X\nB Y\nB X\nC X\nA X\nA X\nA X\nB Y\nB X\nA X\nB Y\nA Y\nA X\nC X\nA X\nB Y\nA X\nA X\nB Y\nA Y\nA X\nC Y\nB Z\nA Y\nC X\nA Y\nA X\nB Y\nA X\nB Y\nB Y\nB X\nB X\nA X\nA X\nC X\nB Y\nB X\nC X\nB Y\nA X\nA X\nA X\nA X\nB Y\nC X\nB X\nA X\nA X\nA Y\nC Y\nA Y\nB X\nA X\nB X\nA X\nA Y\nB Y\nA X\nC X\nC X\nA Y\nB X\nC Y\nA Y\nB X\nA X\nA Y\nC X\nA X\nA X\nB Y\nA Y\nB X\nB Y\nB Y\nA X\nA X\nA X\nB Y\nA Y\nA X\nB X\nA Y\nA Y\nB X\nC Y\nA X\nA Y\nA X\nB X\nA X\nB X\nC Z\nA X\nA X\nB Y\nA X\nA X\nA X\nC Z\nC X\nA Z\nC Y\nB X\nA Y\nB X\nA X\nA Y\nA X\nC X\nA X\nA X\nC Y\nA Z\nC Y\nB Y\nB X\nB X\nA Y\nC X\nB X\nA Z\nA X\nA Z\nC X\nA X\nC Z\nC X\nB Z\nC Y\nB Y\nA X\nB Y\nA X\nA X\nC X\nB X\nA X\nC Z\nB Y\nB Y\nA Y\nB Y\nA X\nA X\nB Y\nA X\nA X\nC Y\nC X\nA X\nC Y\nC X\nB X\nB X\nA X\nB Y\nA Y\nA X\nB Y\nB Y\nB X\nB X\nA X\nB X\nB X\nB Y\nB Y\nC X\nB X\nB X\nA X\nA X\nA X\nA X\nA Y\nC Y\nA X\nA Y\nC Z\nA Y\nC X\nA X\nC Y\nA X\nA X\nA Y\nA Z\nA Y\nA Y\nA X\nA Y\nA Z\nC X\nC Z\nB X\nA X\nA X\nB X\nA X\nA X\nA X\nC X\nA X\nA Y\nA X\nA X\nA Y\nC Z\nC X\nA X\nA X\nA Z\nA X\nC X\nA Y\nC X\nA Y\nC X\nC X\nA X\nA X\nA Y\nB Y\nC X\nC X\nB X\nB Z\nB X\nB X\nA X\nC X\nA Y\nC X\nA Z\nB X\nB Y\nA Y\nA Y\nA X\nA Y\nA X\nA X\nA Y\nA X\nA X\nB Z\nA X\nA X\nA X\nB Y\nB Y\nC Z\nA Y\nA Y\nB Y\nA X\nB X\nC X\nC Y\nC X\nB X\nC X\nA Y\nA X\nA X\nB X\nC X\nB X\nA Y\nB X\nB Z\nA Y\nA X\nA Y\nB X\nB Z\nA X\nA X\nC X\nA X\nA Y\nA X\nB X\nA X\nA X\nA Z\nB X\nB Y\nC Y\nC X\nA Y\nA X\nB Y\nC X\nA X\nC Y\nA X\nA Y\nA Y\nA X\nA X\nC X\nC X\nA Y\nA Y\nC X\nA X\nB X\nC X\nB Y\nC Y\nC X\nB X\nB Y\nC X\nB X\nB Y\nA X\nA X\nB X\nA Y\nA X\nB X\nA X\nC Y\nA X\nB X\nA Y\nC Y\nA X\nA Y\nA Z\nC Z\nA X\nA X\nC X\nA X\nB X\nA X\nC X\nC X\nA Y\nA X\nC Y\nA Y\nA X\nB Y\nA X\nA Y\nA X\nC Z\nC Z\nC X\nA X\nA X\nA X\nA X\nC X\nA X\nC X\nB X\nB X\nA X\nA Y\nB Y\nB X\nB X\nA Y\nA Y\nA X\nA X\nB Y\nB X\nA X\nC X\nB X\nC X\nB Y\nB Z\nB Y\nA X\nA X\nA X\nA X\nA X\nB Y\nA X\nC X\nB Z\nA X\nC Y\nA X\nB X\nC Y\nA X\nA Y\nB X\nA X\nB Y\nB X\nA Y\nC Y\nC X\nA Y\nA X\nA X\nA X\nC X\nB X\nC X\nA Y\nA X\nA Y\nC Z\nC X\nA X\nA Y\nB X\nB X\nB X\nB X\nA X\nB X\nA Z\nB Y\nA X\nA X\nA X\nC Z\nA Y\nA Y\nC X\nC X\nC Y\nB Y\nA Y\nA X\nA X\nC Y\nA X\nC X\nA X\nA X\nC Y\nA Y\nA X\nC X\nA X\nA Y\nC X\nA Y\nA X\nA X\nA X\nB X\nB X\nA X\nB Y\nC Y\nC X\nA Y\nB X\nB X\nB Z\nC X\nB Y\nA Y\nC Z\nB X\nC X\nA X\nA Y\nB Y\nB X\nC Z\nB X\nA X\nC X\nC X\nA X\nA X\nA Y\nB Y\nB X\nA Y\nC Y\nB Y\nB Y\nA X\nA Y\nA X\nB X\nA Y\nA X\nA Y\nA X\nB Z\nB X\nA X\nA Y\nA X\nB Y\nA Y\nA X\nA X\nC X\nA X\nA X\nA X\nA X\nB Y\nB X\nA X\nA X\nC Y\nC X\nA Y\nA X\nB X\nA X\nB X\nB X\nB Y\nA X\nB X\nB Y\nA X\nB X\nC X\nA X\nA X\nC Z\nA X\nB X\nA X\nB X\nB X\nB Y\nA X\nB Y\nC Y\nA X\nC X\nA X\nB X\nB Y\nA X\nC X\nB X\nA X\nC Y\nC X\nB X\nA X\nB X\nA Z\nC Y\nA X\nA X\nB X\nA Y\nB X\nA X\nA Y\nB X\nA X\nC Y\nA X\nC X\nB X\nA X\nC Y\nC X\nA X\nB Y\nB X\nA Y\nB X\nA X\nB X\nA Y\nB X\nA X\nA X\nC X\nA X\nA X\nA X\nA X\nC X\nC X\nB X\nB Y\nB X\nB Z\nA X\nB X\nB Y\nB X\nB X\nA X\nA Z\nA Z\nA X\nB Y\nA X\nB X\nB Y\nA X\nB Y\nB X\nA X\nC X\nC Y\nA X\nA X\nB X\nB X\nB Y\nA X\nC X\nB Y\nA X\nA Z\nA X\nC Y\nA Y\nB X\nC X\nC X\nA X\nB X\nC X\nA Z\nA X\nA X\nB Y\nC X\nA X\nA Y\nC Z\nB X\nA Y\nA X\nA Z\nC Y\nA X\nC X\nA X\nC Y\nB X\nA X\nA X\nB Y\nA X\nB X\nB X\nB X\nC X\nC X\nA X\nC Z\nA X\nC Y\nA X\nC Y\nA Y\nB Y\nB X\nB Y\nB X\nC Y\nB Y\nB Y\nB X\nB Y\nA Y\nA Y\nC Z\nA X\nA X\nA X\nA Z\nB X\nB Y\nB X\nB Y\nA X\nC X\nB X\nA X\nC Y\nB X\nC X\nC Y\nB Y\nA X\nB Y\nB X\nC X\nB X\nC X\nB X\nA X\nC X\nB Z\nB Z\nC X\nA Y\nB X\nA Y\nA X\nB X\nC Y\nC X\nA Y\nC Y\nB Z\nB Y\nB Y\nB Y\nB Z\nB Y\nA X\nA X\nA Y\nB Y\nB Y\nA X\nA Z\nC X\nA X\nA Y\nC X\nA Y\nA X\nB X\nA X\nA X\nC X\nA X\nA X\nC Y\nB Y\nA Y\nA X\nA X\nB Y\nA X\nB X\nA X\nA X\nB X\nB X\nA X\nB Y\nA X\nA Y\nB Y\nA Y\nA X\nC X\nA Y\nA X\nC X\nA X\nA X\nB X\nC Y\nC Y\nB X\nB Z\nA X\nA Y\nA X\nA X\nC Z\nA X\nB X\nA Z\nA X\nA X\nA X\nA X\nA X\nA X\nC X\nB X\nB X\nC X\nA X\nA X\nA X\nA X\nA X\nA X\nB X\nA X\nC X\nA Y\nA X\nA X\nA X\nA X\nB X\nB Y\nA Y\nA X\nA X\nC X\nC X\nA X\nA X\nC X\nA X\nA X\nA X\nA X\nA X\nA X\nC X\nA X\nA X\nA X\nA X\nB X\nA X\nA X\nB Y\nC Y\nA X\nA Y\nC Y\nA X\nB X\nB X\nA X\nA X\nA X\nB X\nA X\nA X\nA X\nA Y\nA X\nB Y\nA Y\nA X\nA Y\nA X\nC X\nA X\nC Y\nB X\nB X\nA X\nA X\nA X\nA X\nA X\nC Y\nA Z\nA X\nB X\nB X\nA X\nA X\nC Z\nA X\nB Z\nB Y\nC Y\nA X\nA X\nC X\nB X\nB X\nB X\nC X\nA X\nA Y\nB Y\nC X\nA X\nB Y\nA X\nC Z\nB X\nA Y\nA X\nA X\nC Y\nA X\nA X\nC X\nA X\nB X\nB Y\nB Y\nB X\nA X\nA Y\nC X\nA X\nA X\nA X\nB X\nC X\nB Y\nA X\nA X\nA Y\nA X\nA X\nB X\nC X\nC X\nA Y\nB X\nA X\nA X\nA X\nB Y\nA X\nB X\nA X\nA X\nA X\nA Y\nA X\nA Z\nC X\nB Y\nB X\nA X\nB X\nC X\nA X\nA X\nB X\nA X\nA X\nA X\nA X\nB X\nA X\nC X\nA Y\nB X\nC X\nA X\nB X\nA X\nB Y\nA X\nC Y\nA X\nC Y\nC X\nB Y\nA Y\nA X\nA X\nB X\nA X\nC X\nA X\nA Z\nB X\nA X\nA X\nA Y\nA X\nA X\nC X\nB Y\nC Z\nA X\nA Y\nB X\nB X\nA X\nB Y\nC X\nA X\nB X\nA Y\nB X\nA X\nA Y\nA X\nA X\nC Y\nB X\nA Y\nC X\nC X\nA Y\nA Y\nA X\nB Z\nA Y\nC X\nC X\nC Y\nA Y\nB X\nB Z\nA X\nB Y\nA X\nA X\nB X\nA Y\nC X\nC Y\nA X\nB Y\nA Y\nA X\nB Z\nB Y\nA X\nB X\nA X\nA X\nB Y\nA Y\nA X\nA X\nA X\nA Y\nA X\nA X\nC X\nA X\nA X\nC X\nA X\nA X\nC Z\nB X\nA Y\nA X\nB Y\nB X\nA X\nA X\nA X\nC Y\nC Y\nB Y\nA X\nC X\nC X\nB X\nA X\nA X\nA X\nA Y\nA X\nA X\nB Z\nC X\nA X\nA X\nA X\nB Y\nB X\nC X\nB X\nA X\nB X\nA X\nC X\nA X\nC X\nA X\nC X\nB Y\nB X\nA X\nA X\nB Y\nB X\nB X\nA X\nB X\nC Y\nA X\nA Y\nA Y\nA Y\nA Z\nB Y\nA X\nA X\nA X\nA X\nA Z\nA X\nA X\nA X\nB Z\nB Y\nC Y\nA X\nA X\nB X\nC X\nA Y\nA X\nB X\nA X\nA X\nB X\nB Y\nA X\nB X\nC X\nB X\nA X\nA X\nA X\nA X\nB X\nB X\nC Y\nB X\nB X\nA X\nB X\nA X\nB Y\nB X\nA X\nA X\nA Y\nB X\nA X\nB Y\nA Y\nB Y\nB X\nA Z\nA X\nB Y\nA X\nB X\nA X\nA X\nA X\nB Y\nB X\nA X\nC Z\nA Y\nA Y\nC X\nB Z\nB X\nC Z\nB X\nB X\nA X\nA X\nB Y\nC X\nA X\nA Z\nB X\nA X\nA X\nB Y\nA X\nA X\nA X\nB Y\nA X\nC Y\nA X\nC Y\nA X\nC Y\nB X\nA Y\nA X\nB X\nA X\nB Y\nA X\nB X\nB X\nA Z\nA X\nC Y\nB Y\nA X\nA X\nC X\nB Y\nC Y\nC X\nA Y\nB Y\nA Y\nA Y\nB X\nA X\nA X\nA X\nA X\nA Y\nA X\nA X\nA X\nB Y\nA X\nA Y\nC X\nA X\nB X\nA X\nA X\nA X\nC Z\nA X\nA Y\nA X\nC Z\nB Y\nB X\nA X\nA Z\nA X\nA X\nA Y\nA X\nA X\nA X\nB Y\nA X\nB X\nA X\nA X\nA X\nA X\nB X\nB Z\nC X\nC X\nC X\nB Y\nA X\nB Y\nB X\nA Y\nB X\nB X\nA X\nC Y\nB X\nA X\nA X\nA X\nA X\nA X\nA X\nB Y\nC Y\nB X\nC Y\nB Y\nC X\nA Y\nC X\nB X\nC X\nB X\nA X\nA X\nB Y\nA X\nC X\nA Y\nC X\nC X\nB Y\nA X\nC X\nC X\nB Z\nA X\nC X\nA Y\nB Y\nA X\nA Y\nA X\nB X\nA X\nA X\nB Y\nA Z\nA X\nB Y\nB X\nA X\nA Y\nB X\nB X\nA X\nA X\nA Y\nA X\nC X\nA X\nB X\nC X\nC X\nC Y\nA X\nB X\nB Y\nA X\nC X\nA X\nB Y\nA X\nB X\nB Z\nA X\nA X\nA X\nB X\nC X\nC X\nB X\nA Y\nA X\nA Y\nA X\nA X\nA X\nA X\nC Y\nB X\nA X\nA X\nA X\nB Y\nA X\nB X\nB Y\nA Z\nA Y\nA X\nA Y\nA X\nA Y\nB Z\nA X\nB Y\nB Z\nB X\nA X\nC X\nC Y\nA X\nA X\nA X\nB X\nA Y\nC X\nA X\nB Y\nA Y\nB X\nB X\nA X\nA Y\nB X\nA Y\nC X\nA X\nC X\nA X\nC X\nA X\nA X\nC X\nA X\nA X\nA Y\nB X\nC X\nB Y\nA Y\nA X\nA X\nA X\nA Z\nB Y\nA X\nA X\nB Y\nA Y\nB Y\nA X\nB Y\nB X\nA X\nA X\nA X\nB X\nB Y\nA X\nC Z\nB Z\nA X\nA X\nA X\nB X\nB Y\nA X\nB Z\nA X\nA Y\nA Y\nA Y\nC X\nB Y\nA X\nA X\nC Y\nB X\nA Y\nA X\nA X\nB Y\nC X\nC X\nC X\nA X\nA X\nA X\nB Z\nA X\nC X\nA X\nA X\nB X\nB X\nA X\nC Y\nA X\nB X\nC X\nB Y\nA X\nB Y\nC X\nA X\nC X\nB Y\nA X\nA Y\nC X\nC Z\nA X\nC X\nA X\nA X\nC X\nA Y\nC X\nB Y\nA X\nC Z\nA X\nB X\nB Y\nA Y\nA X\nA X\nA X\nA X\nA Y\nA Y\nB X\nC X\nA Y\nA X\nC X\nB X\nB Y\nC X\nB X\nA X\nB X\nA X\nC Y\nA X\nA X\nA X\nA X\nC X\nA X\nB X\nA X\nB X\nA X\nA Y\nC X\nC Y\nC X\nA Z\nB X\nA Z\nA X\nB Y\nA Y\nB X\nA X\nB Y\nA Y\nA X\nB Y\nA X\nB X\nA X\nA X\nA X\nA X\nB Y\nA Y\nA X\nA X\nB X\nB Z\nA Y\nA Y\nC Y\nC Y\nA X\nC X\nA X\nB Y\nB X\nB Y\nB X\nC X\nB X\nC X\nA X\nB Y\nA X\nB X\nA X\nB X\nC Y\nB X\nA X\nA Y\nA X\nB Y\nC X\nC X\nA X\nB Z\nA X\nA X\nB Y\nA X\nA X\nB X\nA Y\nC X\nA X\nC Y\nA Y\nC X\nA Y\nA X\nA X\nA X\nA Y\nB X\nB X\nA Y\nC X\nB Y\nA Y\nB X\nB X\nB X\nB X\nA X\nB X\nB X\nA X\nC X\nA X\nC X\nC X\nA X\nC X\nB X\nA Y\nA Z\nA X\nA X\nA X\nA Y\nA X\nA X\nC Z\nA X\nB X\nC X\nA X\nB X\nC X\nC X\nA X\nB Y\nA X\nA X\nA X\nA Y\nB Y\nB Y\nA X\nB X\nA X\nC X\nA X\nB Y\nC Y\nC X\nC X\nB X\nB X\nC X\nA Y\nB X\nA X\nA X\nA X\nA X\nA Y\nB X\nB X\nC Y\nC Y\nB Y\nB X\nA X\nC Y\nA X\nC Z\nC X\nA X\nB Z\nA X\nA Z\nC X\nB Y\nA X\nA Y\nA X\nB X\nA Y\nC X\nC Y\nB X\nB X\nB Y\nB Y\nC X\nB Y\nB Y\nB X\nC X\nC X\nA X\nA X\nA Y\nA X\nA Y\nB X\nA X\nB Y\nB X\nA X\nA X\nA Y\nB X\nB X\nA X\nB X\nC Y\nA Y\nB Y\nC Y\nB Y\nA X\nB Y\nB X\nB Z\nA Y\nA Y\nC Y\nB X\nA X\nA X\nB Y\nA X\nA X\nB X\nA X\nA X\nA X\nB X\nB X\nB Y\nA X\nC X\nA X\nB X\nA X\nA X\nA Y\nB X\nB X\nB X\nC X\nB Y\nC Z\nA X\nC X\nA X\nB X\nA X\nA Y\nB Y\nA Z\nA X\nC X\nA Y\nA X\nA X\nA X\nA X\nA Y\nA X\nC X\nA X\nC Y\nB X\nA X\nA X\nA X\nA X\nA X\nC X\nA X\nB X\nB Y\nB Y\nB X\nC Z\nC X\nB X\nC Y\nA X\nA X\nA X\nA X\nA X\nC X\nA X\nB X\nC X\nC X\nA X\nA X\nB X\nA X\nA X\nA X\nB X\nA X\nC X\nA X\nA Y\nA X\nB Y\nA X\nB X\nA Y\nC X\nB X\nC Y\nC Y\nB X\nA X\nB X\nC X\nA X\nC X\nA X\nC X\nB X\nB X\nC X\nC X\nB X\nB X\nC Y\nB X\nA X\nB X\nB X\nB Y\nA Y\nB X\nA X\nC Y\nA Y\nC Y\nA X\nB Y\nA X\nA X\nA X\nB Y\nB X\nA Y\nA X\nA X\nA Y\nB X\nB Y\nA Y\nB X\nC X\nC Z\nB X\nA Y\nA X\nC X\nA X\nC X\nB Z\nC X\nA Y\nA X\nC X\nB Y\nA Z\nB Y\nA X\nA X\nA Y\nC X\nA Z\nA X\nB Y\nA X\nB X\nA Y\nA Y\nA X\nA X\nA X\nC Y\nB X\nA X\nB X\nB Y\nB Y\nA X\nA X\nA Y\nA X\nC X\nB X\nC X\nB X\nB X\nC Y\nA Y\nB X\nA X\nA X\nA X\nC X\nC X\nA X\nB X\nA X\nA X\nA Z\nA X\nC X\nB X\nA X\nC Y\nA Y\nC Y\nB Y\nB Y\nB Z\nA X\nC X\nA X\nB Y\nB Y\nA X\nB Y\nA X\nB Y\nB X\nB X\nC X\nB Y\nB X\nA X\nA Y\nB X\nA X\nB Z\nA Y\nB X\nB X\nA Y\nC Y\nB X\nB Y\nA X\nB X\nA Y\nA Y\nB X\nB Y\nB X\nA X\nC X\nA X\nA X\nA X\nB Y\nA Y\nB X\nC X\nC Z\nA X\nB Y\nA Y\nA X\nB X\nA X\nA Y\nA X\nA Y\nA X\nA X\nA X\nC X\nA Y\nC Y\nA X\nA X\nC Z\nB X\nA Y\nB X\nB X\nB X\nB X\nB X\nA X\nA X\nB X\nB Y\nA X\nC Y\nC X\nA X\nA X\nA X\nA X\nA X\nA X\nA X\nA Y\nB Y\nA X\nA X\nA Z\nC Y\nC Y\nB X\nC Z\nA X\nA X\nA X\nB Z\nB Y\nA X\nA X\nA X\nB X\nA X\nC X\nC X\nC X\nB X\nA X\nA X\nC Z\nB Z\nC Y\nA X\nA X\nA X\nC Z\nA X\nA X\nC X\nC X\nA X\nC Y\nA X\nA Y\nA X\nA X\nA Z\nC X\nC X\nA Y\nB X\nB X\nA Y\nA X\nB X\nC Y\nA X\nA Y\nB Z\nA X\nC Y\nA X\nB Y\nA X\nA Y\nA X\nC Z\nA X\nB X\nA X\nB Y\nA X\nA X\nA X\nA X\nA X\nB Y\nC X\nA X\nB X\nA X\nA Y\nA Z\nA X\nA X\nB Y\nB X\nC X\nA X\nA X\nB X\nB Y\nB Y\nA Y\nB Z\nB Y\nA X\nA X\nA X\nA X\nB X\nC Y\nA X\nA X\nA X\nA X\nA X\nA X\nA X\nA X\nB Y\nA X\nB X\nC X\nC X\nC X\nC X\nA X\nB Y\nA X\nB X\nB X\nA X\nA X\nC Z\nA X\nB X\nB X\nC X\nA X\nB X\nB X\nB X\nB X\nB Y\nA X\nB X\nA Y\nA X\nA Y\nB Y\nA X\nC Y\nB X\nA X\nA X\nA X\nA Z\nA Y\nA X\nC X\nC X\nB X\nA X\nA X\nB X\nA X\nA Y\nC Z\nA Y\nB X\nB Y\nA X\nC Y\nB Z";

    let battles: Vec<&str> = data.split("\n").collect();
    
    let mut sum: i32 = 0;

    for battle in battles.iter() {
        let tokenized_battle: Vec<&str> = battle.split(" ").collect::<Vec<&str>>();
        let enemy_choice: Choice = Choice::from_str(&tokenized_battle[0]).unwrap();
        let result_choice: &str = &tokenized_battle[1];

        let my_choice: Choice = Choice::get_move_by_result(&enemy_choice, result_choice);
       
        let outcome_score: i32 = get_outcome_score_from_battle(&my_choice, &enemy_choice);
        let choice_score: i32 = Choice::get_score(&my_choice);
        
        sum += outcome_score + choice_score
    }
    println!("Total: {:?}", sum);
}

fn get_outcome_score_from_battle(my_choice: &Choice, enemy_choice: &Choice) -> i32 {
    match (my_choice, enemy_choice) {
        (Choice::Rock, Choice::Rock) => 3,
        (Choice::Rock, Choice::Scissor) => 6,
        (Choice::Rock, Choice::Paper) => 0,
        (Choice::Scissor, Choice::Rock) => 0,
        (Choice::Scissor, Choice::Scissor) => 3,
        (Choice::Scissor, Choice::Paper) => 6,
        (Choice::Paper, Choice::Rock) => 6,
        (Choice::Paper, Choice::Scissor) => 0,
        (Choice::Paper, Choice::Paper) => 3
    }
}

#[derive(Debug, PartialEq)]
enum Choice {
    Rock,
    Paper,
    Scissor
}

impl FromStr for Choice {

    type Err = ();

    fn from_str(input: &str) -> Result<Choice, Self::Err> {
        match input {
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissor),
            _      => Err(()),
        }
    }
}

impl Choice {

    fn get_score(self: &Choice) -> i32 {
        match self {
            Choice::Rock => 1,
            Choice::Paper  => 2,
            Choice::Scissor  => 3
        }
    }

    fn get_move_by_result(choice: &Choice, result: &str) -> Choice {
        match (choice, result) {
            (Choice::Rock, "X") => Choice::Scissor,
            (Choice::Rock, "Y") => Choice::Rock,
            (Choice::Rock, "Z") => Choice::Paper,
            (Choice::Scissor, "X") => Choice::Paper,
            (Choice::Scissor, "Y") => Choice::Scissor,
            (Choice::Scissor, "Z") => Choice::Rock,
            (Choice::Paper, "X") => Choice::Rock,
            (Choice::Paper, "Y") => Choice::Paper,
            (Choice::Paper, "Z") => Choice::Scissor,
            _      => Choice::Scissor,
        }
    }
}
