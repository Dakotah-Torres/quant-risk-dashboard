import init, { calculate_risk, TradeParams } from "./pkg/quant_risk_dashboard.js";



async function main() {
        await init();    
        console.log("WASM initialized");

        const test = new TradeParams(100,98,104,10000,1,5,1.75);
        console.log("created TradeParams:", test);

        console.log("trying to calc risk")
        const cal = calculate_risk(test);
        console.log("Result from calculate_risk:", cal);

    const form = document.getElementById('trade-form');
    form.addEventListener('submit', (e) => {
        e.preventDefault();

        //collecting all of the values from the form in HTML
        const entry         = parseFloat(document.getElementById('entry').value)
        const stop          = parseFloat(document.getElementById('stop').value)
        const target        = parseFloat(document.getElementById('target').value)
        const risk_pct      = parseFloat(document.getElementById('risk_pct').value)
        const account_size  = parseFloat(document.getElementById('account_size').value)
        const tick_value    = parseFloat(document.getElementById('tick_value').value)
        const cost          = parseFloat(document.getElementById('cost').value)

        const inputs        = [entry,stop,target,account_size, risk_pct, tick_value, cost]
        if(inputs.some(isNaN)){
            alert(" One or More inputs are invalkid or empty")
        }
        const params        = new TradeParams(entry,stop,target,account_size, risk_pct, tick_value, cost)
        const result        = calculate_risk(params)

        document.getElementById('output').textContent =
        "Contracts:              " + result.contracts + "\n" +
        "R:R Ratio:              " + result.rr_ratio.toFixed(2) + "\n" +
        "Total Reward Ticks      " + result.total_tick_reward.toFixed(2) + "\n" +
        "Total Risk Ticks        " + result.total_tick_risk.toFixed(2) + "\n" +
        "Risk/Contract:         $" + result.risk_per_contract.toFixed(2) + "\n" +
        "Reward/Contract:       $" + result.reward_per_contract.toFixed(2) + "\n" +
        "Total Risk:            $" + result.total_risk.toFixed(2) + "\n" +
        "Total Reward:          $" + result.total_reward.toFixed(2) + "\n" +
        "Breakeven Price:       $" + result.break_even_price.toFixed(2) + "\n" +
        "Total Cost of Trade    $" + result.total_trade_cost.toFixed(2); 
       
    });

}

main();