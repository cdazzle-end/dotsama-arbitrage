//15 rmrk -> _ kusd
function calculate_price_1(kusdL, rmrkL) {
    const rmrkSupply = 100;
    // Bo = y - ( y * x / ( x + Ai )) = y * Ai / ( x + Ai )
    const kusdOutput = kusdL * rmrkSupply / (rmrkL + rmrkSupply);
    const slippage = (kusdOutput / kusdL) * kusdOutput;
    const total = kusdOutput - slippage;
    console.log("out kusd: ", kusdOutput)
    console.log("slip: ", slippage)
    console.log("total: ", total);
}

//calculate the slippage in small increments
//use this for accuracy until i figure out calculus
exports.rmrk_to_kusd = async (kusdL, rmrkL, rmrkSupply) => {
    // const rmrkSupply = 100;
    const increments = rmrkSupply / 5000;
    let i = 0;
    let kusdChangingLiq = kusdL;
    let rmrkChangingLiq = rmrkL;
    let totalSlip = 0;
    let totalKusd = 0;
    // let swapFees = increments * 0.003
    const rmrkInput = increments - (increments * 0.003)
    while (i < (rmrkSupply / increments)) {
        let kusdOut = kusdChangingLiq * rmrkInput / (rmrkChangingLiq + rmrkInput);
        let slip = (kusdOut / kusdChangingLiq) * kusdOut;
        totalKusd += kusdOut - slip;
        kusdChangingLiq -= kusdOut - slip;
        rmrkChangingLiq += rmrkInput;
        totalSlip += slip;

        i++;
    }
    console.log("out kusd: ", totalKusd)
    console.log("slip: ", totalSlip)
    // console.log("total: ", total);
    return totalKusd;
}

function calculate_ksm_to_kusd(kusdL, ksmL, ksmSupply) {
    const increments = ksmSupply / 5000;
    let i = 0;
    let kusdChangingLiq = kusdL;
    let ksmChangingLiq = ksmL;
    let totalSlip = 0;
    let totalKusd = 0;
    const ksmInput = increments - (increments * 0.003)
    while (i < (ksmSupply / increments)) {
        let kusdOut = kusdChangingLiq * ksmInput / (ksmChangingLiq + ksmInput);
        let slip = (kusdOut / kusdChangingLiq) * kusdOut;
        totalKusd += kusdOut - slip;
        kusdChangingLiq -= kusdOut - slip;
        ksmChangingLiq += ksmInput;
        totalSlip += slip;
        i++;
    }
    console.log("%d KSM to kusd: ", ksmSupply, totalKusd)
    console.log("slip: ", totalSlip)
    // console.log("total: ", total);
    return totalKusd;
}

function calculate_kusd_to_ksm(kusdL, ksmL, kusdSupply) {
    const increments = kusdSupply / 5000;
    let i = 0;
    let kusdChangingLiq = kusdL;
    let ksmChangingLiq = ksmL;
    let totalSlip = 0;
    let totalKsm = 0;
    const kusdInput = increments - (increments * 0.003)
    while (i < (kusdSupply / increments)) {
        let ksmOut = ksmChangingLiq * kusdInput / (kusdChangingLiq + kusdInput);
        let slip = (ksmOut / ksmChangingLiq) * ksmOut;
        totalKsm += ksmOut - slip;
        ksmChangingLiq -= ksmOut - slip;
        kusdChangingLiq += kusdInput;
        totalSlip += slip;
        i++;
    }
    console.log("%d KUSD to KSM: ", kusdSupply, totalKsm)
    console.log("slip: ", totalSlip)
    // console.log("total: ", total);
    return totalKsm;
}

//rmrk -> kusd -> ksm
function calculate_rmrk_to_ksm(rmrkPool, ksmPool, rmrkSupply) {
    // const kusdLiquidity = rmrkPool[0].toString() / 10 ** 12;
    // const rmrkLiquidity = rmrkPool[1].toString() / 10 ** 10;
    const kusdMiddle = rmrk_to_kusd(rmrkPool[0].toString() / 10 ** 12, rmrkPool[1].toString() / 10 ** 10, rmrkSupply);
    const outKsm = calculate_kusd_to_ksm(ksmPool[0].toString() / 10 ** 12, ksmPool[1].toString() / 10 ** 12, kusdMiddle);
    console.log("%d RMRK -> %d KUSD -> %d KSM", rmrkSupply, kusdMiddle, outKsm);
}