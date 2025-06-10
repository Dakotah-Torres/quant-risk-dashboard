use wasm_bindgen::prelude::*;


#[wasm_bindgen]
pub struct TradeParams {
    pub entry: f64,
    pub stop: f64,
    pub target: f64,
    pub account_size: f64,
    pub risk_pct: f64,
    pub tick_value: f64
}

#[wasm_bindgen]
impl TradeParams {
    pub fn new(entry: f64, stop: f64, target: f64, account_size: f64, risk_pct: f64, tick_value: f64) -> TradeParams {
        TradeParams { 
            entry, 
            stop, 
            target, 
            account_size,
            risk_pct, 
            tick_value
        }
    }
}

#[wasm_bindgen]
pub struct RiskResults {
    pub risk_per_contract: f64,
    pub reward_per_contract: f64,
    pub contracts: f64,
    pub rr_ratio: f64,
    pub total_risk: f64,
    pub total_reward: f64
}

#[wasm_bindgen]
pub fn calculate_risk(params: &TradeParams) -> RiskResults { 
    let risk_per_contract: f64 = ((params.entry - params.stop)/0.25).abs() * params.tick_value;
    let reward_per_contract: f64 = ((params.entry - params.target)/0.25).abs() * params.tick_value;

    let max_risk: f64 = params.account_size * (params.risk_pct / 100.0);
    let contracts: f64 = (max_risk / risk_per_contract).floor(); 


    let rr_ratio: f64 = reward_per_contract / risk_per_contract;
    let total_risk: f64 = contracts * risk_per_contract;
    let total_reward: f64 = contracts * reward_per_contract;

    RiskResults { 
        risk_per_contract, 
        reward_per_contract,
        contracts,
        rr_ratio,
        total_risk,
        total_reward
    }
}

#[cfg(test)]
mod tests{
    use  super::*;

    #[test]
    fn test_risk() {
        let p = TradeParams::new(100.0,98.0,104.0,10000.0,1.0,5.0);
        let r = calculate_risk(&p);
        println!("Contracts: {}, RR Ratio: {}", r.contracts, r.rr_ratio);
    }
}