// import {BigNumber , ethers } from "ethers";
// import { toEth, toWei } from "./utils";
// import { contract } from "./contract";
// import { TokenContract  } from "./contract";
// import { parse } from "path";
// import exp from "constants";
// import { parseEther } from "ethers/lib/utils";

// export async function swapEthToToken(tokenName,amount){
//     try {
//         let tx={value:toWei(amount)};
//         const contractObj = await contract();
//         const data= await contractObj.swapEthToToken(tokenName,tx);
//         const receipt = await data.wait();

//         return receipt;
//     }
//     catch(e){
//         return parseErrorMsg(e);

//     }
// }

// export async function hasValidAllowance(owner,tokenName,amount){
//     try{
//         const contractObj= await contract();
//         const address = await contractObj.getTokenAddress(tokenName);
//         const TokenContractObj = await TokenContract(address);
//         const data = await TokenContractObj.allowance(owner,"0xdFBD5Df2200490743c55707CFf1Ff28a63C87839");
//         const result = BigNumber.from(data.toString()).gte(BigNumber.from(toWei(amount)));

//         return result;
//     }
//     catch(e){
//         return parseErrorMsg(e);
//     }
// };

// export async function swapTokenToEth(tokenName,amount){
//     try{
//         const contractObj = await contract();
//         const data = await contractObj.swapTokenToEth(tokenName,amount);

//         const receipt = await data.wait();
//         return receipt;
        
//     }
//     catch(e){
//         return parseErrorMsg(e);
//     }
// }

// export async function swapTokenToToken(srcToken,destToken,amount){

//     try{

//         const contractObj= await contract();
//         const data = await contractObj.swapTokenToToken(srcToken,destToken,toWei(amount));

//         const receipt = await data.wait();
//         return receipt;
//     }
//     catch(e){
//         return parseErrorMsg(e);
//     }
    
// }

// export async function getTokenBalance(tokenName,address){
//     const contractObj = await contract();
//     const balance = contractObj.getBalance(tokenName,address);
//     return balance;
// }

// export async function getTokenAddress(tokenName){
//     try{
//         const contractObj= await contract();
//         const address= await contractObj.getTokenAddress(tokenName);

//         return  address;
//     }
//     catch(e){
//         return parseErrorMsg(e);
//     }
// }

// export async function increaseAllowance(tokenName,amount){
//     try{
//         const contractObj= await contract();
//         const address = await contractObj.getTokenAddress(tokenName);
//         const TokenContractObj = await TokenContract(address);

//         const data = await TokenContractObj.approve("0xdFBD5Df2200490743c55707CFf1Ff28a63C87839",toWei(amount));
//         const receipt = await data.wait();
//         return receipt;
//     }
//     catch(e){
//         return parseErrorMsg(e);
//     }

// }


// export async function getAllHistory(){

//     try{
//         const contractObj = await contract();

//         const getAllHistory = await contractObj.getAllHistory();

//         const historyTransaction = getAllHistory.map((history,i)=>({
//             historyId: history.historyId,
//             tokenA: history.tokenA,
//             tokenB: history.tokenB,
//             inputValue: toEth(history.inputValue),
//             outputValue: toEth(history.outputValue),
//             userAddress: history.userAddress,
//         }));

//     }

// }