use wasm_bindgen::prelude::*;

//This stuct contains all the input risk values that I use with my trading
#[wasm_bindgen]
pub struct TradeParams {
    pub entry: f64,
    pub stop: f64,
    pub target: f64,
    pub account_size: f64,
    pub risk_pct: f64,
    pub tick_value: f64,
    pub cost_per_contract: f64
}

#[wasm_bindgen]
impl TradeParams {
    #[wasm_bindgen(constructor)]
    pub fn new(entry: f64, stop: f64, target: f64, account_size: f64, risk_pct: f64, tick_value: f64, cost_per_contract: f64) -> TradeParams {
        TradeParams { 
            entry, 
            stop, 
            target, 
            account_size,
            risk_pct, 
            tick_value,
            cost_per_contract
        }
    
    }
    pub fn is_long(&self) -> bool {
        self.target > self.entry
    }

    pub fn is_short(&self) -> bool {
        self.target < self.entry
    }
}

//These are the values i am looking to calulate
#[wasm_bindgen]
pub struct RiskResult {
    pub risk_per_contract: f64,
    pub reward_per_contract: f64,
    pub contracts: f64,
    pub rr_ratio: f64,
    pub total_risk: f64,
    pub total_reward: f64,
    pub break_even_price: f64,
    pub total_trade_cost: f64,
    pub total_tick_risk: f64,
    pub total_tick_reward: f64
}

#[wasm_bindgen]
pub fn calculate_risk(params: &TradeParams) -> RiskResult { 
    let total_tick_risk: f64 = ((params.entry - params.stop)/0.25).abs();
    let total_tick_reward: f64 = ((params.entry - params.target)/0.25).abs();

    let risk_per_contract: f64 = total_tick_risk * params.tick_value;
    let reward_per_contract: f64 = total_tick_reward * params.tick_value;

    let max_risk: f64 = params.account_size * (params.risk_pct / 100.0);
    let contracts: f64 = (max_risk / risk_per_contract).floor().max(1.0); 


    let rr_ratio: f64 = reward_per_contract / risk_per_contract;
    let total_risk: f64 = contracts * risk_per_contract;
    let total_reward: f64 = contracts * reward_per_contract;

    //This is the entry price + or - my trading fees so I can break even on the day. 
    let break_even_price: f64 = match params.is_long(){
        true => params.entry + (contracts * params.cost_per_contract),
        false => params.entry - (contracts * params.cost_per_contract)
    };

    let total_trade_cost: f64 = contracts * params.cost_per_contract;
    


    RiskResult { 
        risk_per_contract, 
        reward_per_contract,
        contracts,
        rr_ratio,
        total_risk,
        total_reward,
        break_even_price,
        total_trade_cost,
        total_tick_reward,
        total_tick_risk
    }
}


#[cfg(test)]
mod tests{
    use  super::*;

    #[test]
    fn test_risk() {
        let p = TradeParams::new(25782.50,25807.50,25732.50,50000.0,0.5,0.50,0.74);
        let r = calculate_risk(&p);
        println!("Contracts: {}, RR Ratio: {}, Break Even: {}, Total Risk: {}", r.contracts, r.rr_ratio, r.break_even_price, r.total_risk);
        println!("Trading Costs: ${}", (r.contracts * p.cost_per_contract))
    }
}